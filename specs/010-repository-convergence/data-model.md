# Data Model: Repository Convergence and Cleanup

## Retained Surface

Represents one path or path family allowed to remain in the working tree.

| Field | Type | Rules |
|---|---|---|
| `path` | repository-relative path/pattern | Unique and no parent traversal |
| `role` | enum | `product`, `release`, `validation`, `governance`, `roadmap`, `history` |
| `consumer` | string | Named retained entry point, gate or policy |
| `invariants` | set | Safety/privacy/release rules owned by the surface |
| `validation_gate` | command or document check | Required before acceptance |
| `state` | enum | `proposed`, `validated`, `retained` |

A retained surface without a consumer is reclassified as a removal candidate.

## Removal Candidate

Represents tracked or generated material that may leave the working tree.

| Field | Type | Rules |
|---|---|---|
| `path` | repository-relative path/pattern | Must not overlap a prohibited path |
| `category` | enum | `runtime`, `dependency`, `automation`, `test`, `documentation`, `history`, `generated` |
| `reason` | string | Missing/retired consumer or reproducible residue |
| `incoming_references` | set | Must be empty before removal |
| `live_invariants` | set | Must be empty or mapped to a retained surface |
| `migration_target` | optional retained path | Required when live content exists |
| `deletion_wave` | integer | Enforces dependency order |
| `rollback_commit` | commit identity after implementation | No untracked backup path |
| `state` | enum | See transitions below |

## Validation Invariant

| Field | Type | Rules |
|---|---|---|
| `name` | string | Unique behavior or boundary |
| `source_requirement` | requirement/contract reference | Must resolve in 009 or 010 |
| `old_coverage` | optional path | May be removed only after migration |
| `retained_coverage` | path/gate | Required before candidate removal |
| `security_critical` | boolean | Blocks removal when true and unmapped |

Core invariants include read-only output, Markdown/code preservation, allowed
workspace paths, UTF-8/binary rejection, size/cardinality/time/output limits,
diagnostic redaction, bounded child process, verified package activation,
offline operation and Zed-owned removal.

## Historical Record

| Field | Type | Rules |
|---|---|---|
| `id` | ADR or decision ID | Stable |
| `status` | enum | `accepted`, `partially_superseded`, `superseded` |
| `replacement` | optional ID | Required for superseded records |
| `operational` | boolean | False for retired decisions |
| `backlinks` | set | Must resolve after spec/doc pruning |

Historical records explain past choices but do not satisfy a current setup or
runtime requirement.

## Cleanup Tier

| Field | Type | Rules |
|---|---|---|
| `name` | enum | `normal`, `deep` |
| `eligible_paths` | fixed allowlist | Previewed before deletion |
| `preserved_paths` | fixed allowlist | Always excludes source/agent/user data |
| `requires_confirmation` | boolean | True for deep cleanup |
| `active_process_check` | boolean | Required before deleting build outputs |

Normal cleanup includes all project `target/` directories, validation output,
temporary project directories and stale generated extension binaries. Deep
cleanup may additionally include locked/download caches only by explicit choice.

## Gate Evidence

| Field | Type | Rules |
|---|---|---|
| `gate` | command/check identifier | Unique in one final matrix |
| `result` | enum | `pass`, `fail`, `expected_external_block` |
| `artifact_identity` | optional size/hash/version | Required for package gates |
| `content_free_summary` | string | No source/translation/secret content |
| `recorded_in` | path | Feature 009 validation for release evidence |

## State Transitions

```text
discovered
   -> classified
       -> migration_required -> migrated -> removal_ready
       -> removal_ready
           -> removed -> verified
                         -> restored (if a retained gate fails)
```

- A candidate with incoming references cannot become `removal_ready`.
- A candidate with a live security invariant cannot be removed until retained
  coverage passes.
- Generated residue skips migration but still requires classification, preview
  and prohibited-path checks.
- Publication cannot resume until every removed candidate is `verified` and the
  exact package evidence refers to the post-cleanup tree.

## Relationships and Invariants

- Every tracked file belongs to exactly one retained surface or removal
  candidate at final classification.
- Every removal candidate maps to one rollback commit after implementation.
- Every live invariant maps to at least one retained gate.
- Every superseded operational decision points to ADR 0007 or a later accepted
  decision.
- No cleanup tier overlaps `.agents/`, `.codex/`, `.git/`, source files,
  secrets, real `.env` files or persistent user/provider data.
