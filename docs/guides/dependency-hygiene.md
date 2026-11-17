# Dependency Hygiene

Run the same advisory and duplicate-version checks as CI with:

```shell
cargo deny check advisories bans
```

## Temporary Chrono Exception For v0.1.13

On 2026-11-17, PR #86 failed on RUSTSEC-2026-0543 for chrono 0.4.45.
The scheduled check on 2026-11-16 passed with the same chrono version.
The [RustSec database commit](https://github.com/RustSec/advisory-db/commit/233155b4025b854c68a4b199c0e32d7fd53291a2)
added the advisory on 17 November; the
[advisory's own date](https://github.com/RustSec/advisory-db/blob/main/crates/chrono/RUSTSEC-2026-0543.md)
is 2026-09-14. These dates describe different events.

The advisory is informational `unmaintained`, covers all versions, and lists
no patched version. It recommends Jiff or `time`. It does not itself describe
an exploitable vulnerability. Maintenance cessation still creates risk: future
bugs, security issues, or platform incompatibilities may go unfixed.

The decision in [TASK-207](../tasks/repo-process/TASK-207-temporarily-except-chrono-advisory.md)
accepts that maintenance risk temporarily to preserve the existing datetime
contract during v0.1.13 preparation. `deny.toml` ignores only this advisory ID.
New vulnerability advisories, other unmaintained dependencies, yanked crates,
and bans continue to be checked under the existing policy. The dependency
versions, lockfile, CLI help, and runtime behavior are unchanged by this fix.

## Options Assessed

- **Upgrade chrono:** no available version escapes this advisory.
- **Migrate during this release:** removes the maintenance dependency sooner,
  but adds compatibility risk to an otherwise verified release slice. Chrono
  implements RFC parsing, custom formats, offsets, time-only values, fractional
  output, and shared shift anchors in `src/runbook/render.rs`.
- **Ignore unmaintained dependencies globally or use stale advisory data:**
  unnecessarily hides unrelated maintenance risk or new security findings.
- **One advisory-specific temporary exception:** preserves the wider gate and
  avoids rushing datetime changes, at the cost of retaining an unmaintained
  direct dependency and requiring follow-through. This is the selected option.

## Ownership And Removal

Owner: **albertattard**. Review/remove by **2026-12-17**, or review before the
next release after v0.1.13 if sooner. Any extension requires an explicit new
risk assessment. Reassess immediately if security or soundness evidence changes;
remove sooner if the advisory is withdrawn or chrono is replaced.

This deadline is **manual**. The installed
[cargo-deny 0.20.2 parser](https://github.com/EmbarkStudios/cargo-deny/blob/0.20.2/src/advisories/cfg.rs)
supports an advisory ID and reason, but not the `expiry` field described by the
latest online documentation. CI continues to run the actual advisory check;
it does not enforce this calendar deadline.

[TASK-208](../tasks/repo-process/TASK-208-replace-chrono.md) tracks replacement
and removal of the exception and active notice. Evaluate Jiff and `time`
against SPEC-003 before selection. Jiff's
[strtime documentation](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html)
shows unsupported `%+` and specific fractional-second semantics: shared
strftime-style syntax alone does not establish compatibility. Preserve the
existing contract, or intentionally update the spec and task before changing
user-visible behavior. Keep TASK-207 as the historical decision record.
