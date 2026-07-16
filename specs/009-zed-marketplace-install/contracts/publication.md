# Contract: Zed Extension Publication

## Project Release Gate

Before the central-registry PR is submitted, the public project commit must
provide:

- `zed-extension/extension.toml` with permanent ID `en-es-translator`, matching
  semantic version, repository URL, authors, description, direct language
  server and Markdown/Plain Text languages;
- `zed-extension/LICENSE` matching an accepted Zed extension-code license;
- a tagged public release containing the exact Linux `x86_64` archive named by
  `ops/marketplace/package.lock.json`;
- exact released LSP/runner identities, model identities, notices and
  corresponding-source instructions;
- passing locked Rust/native build, format, lint, dependency, license,
  package-budget, offline and clean-install checks;
- user documentation that begins with Gallery install and contains no terminal,
  Docker, binary-path or provider-setting step.

The extension repository must be public and the referenced commit must belong
to a branch.

## Central Registry Change

The submission to `zed-industries/extensions` must:

1. add the public repository as an HTTPS submodule at
   `extensions/en-es-translator`;
2. add the following matching entry to `extensions.toml`:

   ```toml
   [en-es-translator]
   submodule = "extensions/en-es-translator"
   path = "zed-extension"
   version = "<extension.toml version>"
   ```

3. run the upstream `pnpm sort-extensions` command;
4. pass upstream license, manifest, build and ordering checks;
5. describe a real local supported-platform test and link the project release
   evidence.

The `path` is required because the extension lives in a subdirectory; the
accepted license file must therefore also live in `zed-extension/`.

## Clean Gallery Acceptance

After the registry entry is available to a clean Zed installation:

1. open the Extension Gallery and search for English to Spanish Translator;
2. install it with the Gallery control;
3. open the public Markdown fixture without a project checkout/toolchain;
4. invoke the direct translation action;
5. observe automatic preparation status and a real Spanish read-only preview;
6. disable networking and repeat all 20 public cases;
7. verify source bytes and protected structures are unchanged;
8. disable and uninstall through Zed, then confirm the extension work root is
   removed without a terminal cleanup action.

A dev extension, manually copied binary, local checkout setting or controlled
translation double cannot close this gate. If upstream review has not merged
the registry entry, the feature may be implementation-complete and PR-ready
but marketplace acceptance remains explicitly externally blocked.
