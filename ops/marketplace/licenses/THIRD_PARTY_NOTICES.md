# Third-Party Notices

The Linux `x86_64` translation package contains the following fixed inputs:

- Mozilla `translations` / Bergamot native inference code at commit
  `f31423c7c2c6ed8ae57d71a3d19a9db6f156060e`. The project declares
  MPL-2.0; its complete recursive dependency lock is recorded in
  `ops/marketplace/source.lock.json`.
- Mozilla Firefox Translation English-to-Spanish model, vocabulary and lexical
  shortlist records identified exactly in `ops/marketplace/model.lock.json`.
  Those resources declare MPL-2.0.
- The native build uses the ONNX.js portable SGEMM implementation and its
  pinned Eigen 3.3.7 dependency from the recursive gitlinks recorded in
  `ops/marketplace/source.lock.json`; both remain covered by their upstream
  notices and source trees.
- `translator-lsp` and the Zed extension integration are original project code
  distributed under MIT; see `zed-extension/LICENSE`.

The package does not claim authorship of Mozilla or recursive third-party
components. Their copyright notices and license evidence remain available in
the corresponding source trees named in `SOURCE.md`.
