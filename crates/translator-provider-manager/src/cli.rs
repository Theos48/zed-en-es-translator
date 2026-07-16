use crate::error::ManagerError;

/// Exact lifecycle operation selected by the thin project script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Disclose,
    Prepare { consent: String },
    Status,
    Verify,
    Update { consent: String },
    Rollback,
    Clean { confirmation: String },
    BuildRunner,
}

impl Command {
    /// Parse the bounded positional lifecycle interface.
    ///
    /// # Errors
    ///
    /// Rejects missing, extra, empty, or unknown arguments without echoing
    /// their values.
    pub fn parse(arguments: &[String]) -> Result<Self, ManagerError> {
        match arguments {
            [operation] if operation == "disclose" => Ok(Self::Disclose),
            [operation] if operation == "status" => Ok(Self::Status),
            [operation] if operation == "verify" => Ok(Self::Verify),
            [operation] if operation == "rollback" => Ok(Self::Rollback),
            [operation, confirmation]
                if operation == "clean" && confirmation == "remove-embedded-provider-data" =>
            {
                Ok(Self::Clean {
                    confirmation: confirmation.clone(),
                })
            }
            [operation] if operation == "build-runner" => Ok(Self::BuildRunner),
            [operation, consent] if operation == "prepare" && is_sha256(consent) => {
                Ok(Self::Prepare {
                    consent: consent.clone(),
                })
            }
            [operation, consent] if operation == "update" && is_sha256(consent) => {
                Ok(Self::Update {
                    consent: consent.clone(),
                })
            }
            _ => Err(ManagerError::StateInvalid),
        }
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn parse_should_accept_prepare_with_sha256_consent() {
        let arguments = vec!["prepare".to_string(), "a".repeat(64)];

        let command = Command::parse(&arguments);

        assert!(matches!(command, Ok(Command::Prepare { .. })));
    }

    #[test]
    fn parse_should_reject_prepare_without_consent() {
        let arguments = vec!["prepare".to_string()];

        let command = Command::parse(&arguments);

        assert!(command.is_err());
    }

    #[test]
    fn parse_should_reject_extra_arguments() {
        let arguments = vec!["status".to_string(), "private".to_string()];

        let command = Command::parse(&arguments);

        assert!(command.is_err());
    }
}
