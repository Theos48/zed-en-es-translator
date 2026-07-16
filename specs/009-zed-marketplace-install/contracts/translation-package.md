# Contract: Verified Translation Package

## Layout

```text
work/en-es-translator/
├── install.lock
├── state.json
├── staging/
└── packages/
    └── <package-id>/
        ├── installed.json
        ├── bin/
        │   ├── translator-lsp
        │   └── translator-embedded-runtime
        ├── models/
        │   ├── model.enes.intgemm.alphas.bin
        │   ├── vocab.enes.spm
        │   └── lex.50.50.enes.s2t.bin
        └── LICENSES/
            ├── THIRD_PARTY_NOTICES.md
            ├── MPL-2.0.txt
            └── SOURCE.md
```

All relative names are fixed by the package lock. Archive-controlled absolute
paths, `..`, links, devices, extra executables and unknown package-root entries
are rejected.

## Launchability

A package is launchable only when:

- its directory is a non-symlink directory below `packages/`;
- `installed.json` is strict, bounded JSON and names the directory package ID;
- platform and private wire version are compatible;
- the exact two executables and three model resources are non-symlink regular
  files with expected size and SHA-256;
- only the two allowlisted binaries are executable;
- required notices/source instructions exist;
- installed total is within 128 MiB;
- its state is `verified` and the atomic root state names it active or previous.

Validation never repairs or downloads. Any failure returns a content-free
package-not-ready error.

## Native Runner Invocation

`translator-lsp` starts the exact adjacent runner directly, without a shell,
with a fixed current directory and fixed model arguments:

```text
bin/translator-embedded-runtime
  --model models/model.enes.intgemm.alphas.bin
  --vocabulary models/vocab.enes.spm
  --lexical-shortlist models/lex.50.50.enes.s2t.bin
```

The parent clears the environment, sets only the locale and fixed thread cap,
writes a versioned JSON request to stdin, concurrently drains bounded stdout
and stderr, enforces 15 seconds, and kills/reaps the entire child process group
on timeout. It does not retry.

The request contains only the ordered segments already approved by the core,
`source_language=en`, `target_language=es` and tone. The response must preserve
segment count/order and remain inside the 40 KiB semantic output limit. Raw
stderr or malformed output never reaches the user.

## Immutability and Fallback

Promotion is an atomic rename from staging to `packages/<package-id>`. After
promotion no file is changed. Updates create a new package ID. A failed
candidate cannot change `active`; a valid previous package can be selected only
through an atomic state update after complete revalidation.

At most active, previous and one staging package may consume storage. Cleanup
never follows links and never removes entries outside the extension work root.
