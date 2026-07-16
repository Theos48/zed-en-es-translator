use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::embedded_protocol::{decode_response, encode_request};
use crate::{
    ErrorCode, ProviderRequest, ProviderResponse, TranslateFailure, MAX_OUTPUT_BYTES,
    PROVIDER_TIMEOUT_MS,
};

const TRANSPORT_OVERHEAD_BYTES: usize = 8 * 1024;
const STDERR_LIMIT_BYTES: usize = 4 * 1024;
const POLL_INTERVAL: Duration = Duration::from_millis(5);

/// Fixed process resource limits for the embedded runner boundary.
#[derive(Debug, Clone, Copy)]
pub struct EmbeddedRunnerLimits {
    deadline: Duration,
    stdout_bytes: usize,
    stderr_bytes: usize,
}

impl EmbeddedRunnerLimits {
    /// Construct reduced limits for process-boundary contract tests.
    #[doc(hidden)]
    pub const fn for_tests(deadline: Duration) -> Self {
        Self {
            deadline,
            stdout_bytes: MAX_OUTPUT_BYTES + TRANSPORT_OVERHEAD_BYTES,
            stderr_bytes: STDERR_LIMIT_BYTES,
        }
    }
}

impl Default for EmbeddedRunnerLimits {
    fn default() -> Self {
        Self {
            deadline: Duration::from_millis(PROVIDER_TIMEOUT_MS),
            stdout_bytes: MAX_OUTPUT_BYTES + TRANSPORT_OVERHEAD_BYTES,
            stderr_bytes: STDERR_LIMIT_BYTES,
        }
    }
}

/// Exact verified native process invocation boundary.
#[derive(Clone)]
pub struct EmbeddedProcessRunner {
    executable: PathBuf,
    working_directory: PathBuf,
    arguments: Vec<String>,
    limits: EmbeddedRunnerLimits,
}

impl EmbeddedProcessRunner {
    /// Construct a runner from paths already selected by the verified artifact
    /// store. The executable must be a non-symlink regular executable contained
    /// by the working directory.
    ///
    /// # Errors
    ///
    /// Returns `PROVIDER_NOT_CONFIGURED` when containment or executable checks
    /// fail. Paths are never included in the error.
    pub fn from_verified_paths(
        executable: PathBuf,
        working_directory: PathBuf,
        limits: EmbeddedRunnerLimits,
    ) -> Result<Self, TranslateFailure> {
        validate_paths(&executable, &working_directory)?;
        Ok(Self {
            executable,
            working_directory,
            arguments: Vec::new(),
            limits,
        })
    }

    #[doc(hidden)]
    pub fn from_verified_invocation(
        executable: PathBuf,
        working_directory: PathBuf,
        arguments: Vec<String>,
    ) -> Result<Self, TranslateFailure> {
        validate_paths(&executable, &working_directory)?;
        if arguments.iter().any(|argument| {
            argument.is_empty()
                || argument.contains('\0')
                || argument.starts_with('/')
                || argument.split('/').any(|component| component == "..")
        }) {
            return Err(not_configured());
        }
        Ok(Self {
            executable,
            working_directory,
            arguments,
            limits: EmbeddedRunnerLimits::default(),
        })
    }

    /// Execute one bounded request in one child process.
    ///
    /// # Errors
    ///
    /// Returns stable provider errors for spawn, protocol, cap, exit, or
    /// timeout failures. Child stdout/stderr and paths are never propagated.
    pub fn run(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        let deadline = Instant::now() + self.limits.deadline;
        let input = encode_request(request)?;
        let mut command = Command::new(&self.executable);
        command
            .current_dir(&self.working_directory)
            .args(&self.arguments)
            .env_clear()
            .env("LC_ALL", "C.UTF-8")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0);

        let mut child = command.spawn().map_err(|_| process_failed())?;
        let process_id = child.id();
        let mut stdin = child.stdin.take().ok_or_else(process_failed)?;
        stdin.write_all(&input).map_err(|_| process_failed())?;
        drop(stdin);

        let stdout = child.stdout.take().ok_or_else(process_failed)?;
        let stderr = child.stderr.take().ok_or_else(process_failed)?;
        let stdout_reader = read_capped(stdout, self.limits.stdout_bytes);
        let stderr_reader = read_capped(stderr, self.limits.stderr_bytes);

        let status = loop {
            if let Some(status) = child.try_wait().map_err(|_| process_failed())? {
                break status;
            }
            if Instant::now() >= deadline {
                kill_process_group(process_id);
                let _ = child.kill();
                let _ = child.wait();
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Err(TranslateFailure::new(
                    ErrorCode::ProviderTimeout,
                    "Embedded provider timed out.",
                ));
            }
            thread::sleep(POLL_INTERVAL);
        };

        let stdout = join_reader(stdout_reader)?;
        let stderr = join_reader(stderr_reader);
        if !status.success() || stderr.is_err() {
            return Err(process_failed());
        }
        decode_response(request, &stdout)
    }
}

impl fmt::Debug for EmbeddedProcessRunner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmbeddedProcessRunner")
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}

fn validate_paths(executable: &Path, working_directory: &Path) -> Result<(), TranslateFailure> {
    let executable_metadata = fs::symlink_metadata(executable).map_err(|_| not_configured())?;
    let directory_metadata =
        fs::symlink_metadata(working_directory).map_err(|_| not_configured())?;
    if executable_metadata.file_type().is_symlink()
        || !executable_metadata.is_file()
        || executable_metadata.permissions().mode() & 0o111 == 0
        || directory_metadata.file_type().is_symlink()
        || !directory_metadata.is_dir()
    {
        return Err(not_configured());
    }
    let executable = executable.canonicalize().map_err(|_| not_configured())?;
    let working_directory = working_directory
        .canonicalize()
        .map_err(|_| not_configured())?;
    if !executable.starts_with(working_directory) {
        return Err(not_configured());
    }
    Ok(())
}

fn read_capped<R>(mut reader: R, limit: usize) -> thread::JoinHandle<Result<Vec<u8>, ()>>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut output = Vec::with_capacity(limit.min(8 * 1024));
        let mut buffer = [0_u8; 4096];
        let mut exceeded = false;
        loop {
            let count = reader.read(&mut buffer).map_err(|_| ())?;
            if count == 0 {
                break;
            }
            if !exceeded && output.len().saturating_add(count) <= limit {
                output.extend_from_slice(&buffer[..count]);
            } else {
                exceeded = true;
            }
        }
        if exceeded {
            Err(())
        } else {
            Ok(output)
        }
    })
}

fn join_reader(
    reader: thread::JoinHandle<Result<Vec<u8>, ()>>,
) -> Result<Vec<u8>, TranslateFailure> {
    reader
        .join()
        .map_err(|_| process_failed())?
        .map_err(|_| process_failed())
}

fn kill_process_group(process_id: u32) {
    let process_group = i32::try_from(process_id).map_or(0, |id| -id);
    if process_group != 0 {
        // SAFETY: sending SIGKILL to the child-owned process group requires no
        // dereferencing and the process-group id was created for this child.
        unsafe {
            libc::kill(process_group, libc::SIGKILL);
        }
    }
}

fn not_configured() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::ProviderNotConfigured,
        "Embedded provider artifacts are not ready.",
    )
}

fn process_failed() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::ProviderFailed,
        "Embedded provider process failed.",
    )
}
