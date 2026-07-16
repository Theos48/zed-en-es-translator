# Embedded provider license gate

Status: **blocked**.

F012 may not activate, bundle, rehost, redistribute, or publish an embedded
artifact from this directory until the exact manifest digest has a recorded
human review for the requested scope.

The final review must enumerate:

- Mozilla Translations source and every recursive native dependency revision;
- SPDX conclusion and source for the runner, model, vocabulary, shortlist and
  generated/configuration files;
- MPL notices, modifications and source-offer obligations for the exact binary;
- the accepted SBOM/ELF dependency report and manifest digest;
- a human project-maintainer approval limited to local acquisition/activation;
- a separate F009 human approval before bundling, redistribution or publication.

Automated reports are evidence only. Missing, anonymous, automated-only or
scope-mismatched approval leaves the corresponding state blocked.
