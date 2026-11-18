---
id: TASK-208
title: Replace Unmaintained Chrono Dependency
status: done
category: repo-process
related_features:
  - SPEC-003
owner: albertattard
created: 2026-11-17
updated: 2026-11-17
---

## Summary

Replace chrono with a maintained datetime implementation, evaluating Jiff and
`time`, and remove the temporary RUSTSEC-2026-0543 exception from TASK-207.
Target completion before the exception review deadline of 2026-12-17.

## Scope

- Inventory chrono usage in `src/runbook/render.rs` and existing datetime tests.
- Compare Jiff and `time` for contract compatibility, supported Rust version,
  dependency graph, advisory status, and implementation complexity. Jiff is a
  candidate, not a preselected drop-in replacement.
- Establish regression coverage for the existing SPEC-003 datetime contract
  before replacement; intentionally update the spec first if any behavior
  must change, then update this task and CLI help/help-focused tests as needed.
- Preserve RFC 3339 numeric offsets and Z suffixes, fractional widths 1..9,
  RFC 1123 parsing through the current RFC 2822 parser and GMT output,
  custom formats and numeric offsets, time-only base inheritance and midnight
  crossing, default bases, relative distances, and shared anchors.
- Assess format-directive compatibility, invalid input, range/overflow and
  leap-second behavior explicitly; retain established error/exit contracts.
- Refactor datetime logic out of the large rendering module before adding
  migration complexity. Keep business logic outside `main.rs`.
- Remove chrono from the resolved graph and remove the exception and its active
  README notice; retain TASK-207 as a historical record.

## Acceptance Criteria

- [x] Record the replacement decision and compatibility evidence.
- [x] Preserve SPEC-003 behavior or intentionally update the relevant spec/task
      before implementing any user-visible differences.
- [x] Add/update integration CLI tests for compatibility gaps and error cases.
- [x] No chrono remains in `Cargo.toml` or the resolved dependency graph.
- [x] Remove the RUSTSEC-2026-0543 exception and update active documentation.
- [x] `cargo deny check advisories bans` passes against the current database.
- [x] `./tools/verify.sh` passes.

## Notes

Jiff supports RFC and strftime-style interfaces, but similar directive names do
not guarantee identical parsing or formatting. For example, its documented
`%+` support and `%f` behavior differ from chrono conventions. Do not change
custom-format semantics silently to simplify the dependency replacement.

See the [dependency hygiene guide](../../guides/dependency-hygiene.md) for the
advisory and upstream documentation.

## Replacement Decision (2026-11-17)

Select **Jiff 1.0.0**, with only `std` enabled (no system timezone discovery or
bundled timezone database). Its Rust 1.85 minimum and time 0.3.55's Rust 1.88
minimum both fit this repository's Rust 1.95 minimum. Both are maintained
candidates recommended by RustSec; the final resolved graph must pass the live
advisory check, rather than relying on that recommendation alone.

An isolated executable probe compared both libraries with Chrono 0.4.45:

| Contract | Jiff 1.0.0 | time 0.3.55 |
| --- | --- | --- |
| RFC 3339, fixed offsets, nanoseconds | Native primitives; preserve output style explicitly | Native primitives; preserve output style explicitly |
| RFC 1123/GMT matching | Exact GMT strtime shape validates weekday | RFC 2822 description accepts mismatched weekday; needs explicit validation |
| Existing strftime custom formats | Broad coverage, needs adapter | strftime parser rejects `%f`, `%3f`, `%.f`, `%+`, `%Z` |
| `12:34` time-only | Defaults seconds to zero like Chrono | `Time::parse` also defaults seconds to zero; attach base date/offset explicitly |
| Raw `%f` input `123` | Reads .123 seconds; must adapt to 123 nanoseconds | Unsupported |
| `%.3f` input `.1` | Accepts it; adapter must enforce width | Unsupported |
| `%.f` output for .123400 | Prints .1234; adapter must retain .123400 | Unsupported directive |
| `%+`, `%c`, `%x`, `%X`, `%r` | Expand to Chrono's composites | Some POSIX composites supported, `%+` absent |
| Leap second `23:59:60` | Silently clamps to :59 by default | No Chrono leap-second representation |
| Date range | -9999..9999 | -9999..9999 by default; large-dates still differs from Chrono |

Jiff minimizes the compatibility layer and offers fallible parsing/formatting
and checked arithmetic. Keep fixed-offset interpretation and anchor logic in a
dedicated datetime module. Preserve raw fractional parsing, exact widths,
0/3/6/9 output groups, composites, base-offset inheritance, and local-clock GMT
output explicitly. Existing tests cover shared anchors and midnight crossing;
new CLI regressions cover the previously untested compatibility gaps before
switching dependencies.

SPEC-003 is intentionally updated first for unsupported leap seconds and the
representable year range, and for operational errors on invalid formats and
overflow (the old arithmetic/formatting could panic). Help and help tests must
state those limits. No other datetime behavior change is intended.

The year limit is a deliberate tradeoff in selecting Jiff: `time` with
`large-dates` could cover Chrono's wider years, but would require a larger
custom-format compatibility layer. Neither replacement provides Chrono's
leap-second representation and arithmetic; retaining that would require a
bespoke representation and arithmetic layer. Rejecting leap seconds explicitly
avoids Jiff's silent clamping. RFC 3339 bases retain their original four-digit
year syntax; the wider signed-year range applies to custom formats and results.

The host normal dependency tree is Jiff -> jiff-core; the equivalent `time`
configuration adds deranged, num-conv, powerfmt, and time-core. Both candidates
passed advisories and bans against the same current advisory database (the
`time` check used an isolated manifest). The production all-target tree and
lockfile contain no Chrono. Existing duplicate-version warnings remain for
getrandom, syn, and windows-sys.

A second executable comparison ran 44 custom-format cases through a baseline
CLI built from HEAD and the replacement CLI. It covered composites (including
`%+`), raw and decimal fractions, fixed widths, offsets, epoch values,
week/ordinal dates, partial inputs, repeated-field conflicts, and invalid input.
The checked-in CLI regressions retain these compatibility cases, including
Chrono's surprising fallback from inconsistent date fields to time-only values.

Primary references: [Jiff strtime](https://docs.rs/jiff/1.0.0/jiff/fmt/strtime/),
[time strftime parser](https://docs.rs/time/0.3.55/time/format_description/fn.parse_strftime_owned.html).

## Initial Verification (Superseded By Compatibility Review)

- Baseline `cargo test --test cli_datetime` passed against Chrono before the
  dependency switch (the initial five regression tests).
- The initial suite contained 13 datetime integration tests covering compatibility
  matrices and errors, alongside the existing anchor, capture, format, and
  midnight-crossing tests. Help/explain tests cover the intentional limits.
- `cargo deny check advisories bans`: passed with the current database and no
  advisory exception; existing duplicate-version warnings remain non-fatal.
- `cargo tree --locked --target all` and a parsed lockfile assertion: no Chrono.
- `./tools/verify.sh`: passed (510 tests, formatting, all-target/all-feature
  Clippy with warnings denied, release build, and Homebrew formula rendering).
- `git diff --check`: passed. Unrelated TASK-205/spec/index changes preserved.

SPEC-003's offset wording is clarified to describe the existing tested use of
the base offset, rather than imply preservation of a differing input offset.
The implementation also preserves the historical custom time-only fallback;
strict date and weekday validation remains specific to the built-in formats.

## Compatibility Review Follow-up

Reopened after differential CLI review found four regressions missed by the
initial verification: century/full-year reconciliation with `%y`, literal `s`
misclassified as `%s`, abbreviated month input for `%B`, and empty trailing
whitespace for `%n`/`%t`. These fixes restore SPEC-003's existing contract; they
do not introduce new datetime behavior or require another specification change.

Add failing CLI regressions before implementation, including elapsed intervals
across leap days/centuries, year directive order and conflicts, literal text,
month input variants versus output style, and zero-or-more whitespace. Replace
string-based token classification with explicit literal/directive tokens. Then
broaden differential comparisons and repeat advisory and full verification.

### Follow-up Implementation

- Literal characters and directives now have separate token variants. Timestamp,
  offset, and fractional guards run only for their directive kinds. `%s` and
  its padding variants retain support for leading whitespace.
- Gregorian `%C`/`%y`/`%Y` and ISO `%g`/`%G` components are reconciled after
  parsing, independent of order. Only a standalone two-digit year uses the
  Chrono 69/70 pivot (Jiff natively uses 68/69). Inconsistent different year
  components preserve the historical time-only fallback; conflicting repeated
  components still fail during parsing.
- `%B` accepts full and abbreviated month names without changing full-name
  output. The equivalent `%A` weekday compatibility gap found during expansion
  of the checks is fixed too. Matching remains ASCII case-insensitive.
- `%n`, `%t`, and literal whitespace consume zero or more whitespace characters,
  including empty input at the end. Formatting still prints the authored
  newline/tab/whitespace style.
- Seven new integration tests exercise multi-value distances across leap days
  and centuries, year order/conflicts/pivots, ISO years, literal prefixes,
  padding, name variants, and empty/nonempty whitespace. These restore the
  existing contract without further spec or CLI help changes.

### Follow-up Verification

- The initial five new regression groups failed against the pre-fix adapter.
  After refining year-conflict expectations against Chrono, all eleven
  baseline-compatible custom-format test groups pass on both CLI versions.
- `cargo test --offline --test cli_datetime`: all 20 tests pass, including seven
  new regression groups. The baseline checks exclude intentional migration
  changes such as leap-second rejection.
- Expanded differential CLI probes: **217 cases, zero mismatches**, including
  1900/2000/2100/2400 and boundary-year intervals, directive orders and conflicts,
  literals, full/abbreviated/case-varied names, and empty/nonempty Unicode
  whitespace. Input status and complete rendered Markdown were compared.
- `cargo deny check advisories bans`: passed against the current database, with
  no advisory exception; existing duplicate-version warnings remain non-fatal.
- `cargo tree --locked --offline --target all` and parsed lockfile assertions:
  Chrono remains absent, and `deny.toml` has no RUSTSEC-2026-0543 exception.
- Final `./tools/verify.sh`: passed (517 tests, formatting, Clippy, release build,
  and Homebrew formula rendering).
- `git diff --check`: passed. Unrelated local TASK-205 and SPEC-008 edits were
  preserved, including the existing TASK-205 index entry. No commit or push.

## Semantic Field Reconciliation Follow-up

Reopened after review reproduced two remaining contract regressions: `%s`
unconditionally overriding inconsistent calendar/time fields, and contradictory
12-hour/24-hour/meridiem fields being accepted. Preserve the existing full-
datetime-then-time-only fallback and operational errors; no spec change is
intended. Add failing multi-value/order/consistency regressions first, then
centralize semantic reconciliation before converting parsed fields into a
civil datetime. Verify against the Chrono baseline, current advisories, and
`./tools/verify.sh` before restoring completion status.

### Semantic Reconciliation Implementation

- Added `src/runbook/datetime_fields.rs` to retain year, hour, calendar, quarter,
  and epoch constraints through conversion. The format adapter handles syntax;
  the field model decides whether a full datetime, epoch completion, or the
  historical time-only fallback is valid.
- A complete date/time is checked against `%s` before use. Epoch completion
  fills only missing information and checks every supplied constraint. It does
  not compare parsed numeric offsets, matching the previous naive conversion.
- Hours are represented as half-day and remainder constraints: `%H`/`%k`
  provide both, `%I`/`%l` provide the remainder, and `%p`/`%P` provide the
  half-day. Contradictions fail during parsing, independent of directive order.
  A missing meridiem can be supplied by a consistent 24-hour or epoch value.
- Seven new integration groups reproduce the two findings and cover both
  directive orders, partial and complete fields, invalid combinations, all six
  hour-directive permutations, midnight/noon, ordinal/ISO/Sunday/Monday dates,
  quarters, fractional seconds, and ignored custom offsets.
- No further user-visible contract or CLI help change is introduced; these
  corrections restore the existing SPEC-003 behavior.

### Semantic Reconciliation Verification

- Reproduced epoch/fallback and hour/meridiem failures before the corrections.
- `cargo test --offline --test cli_datetime`: all 27 integration tests pass.
- Rebuilt the original Chrono CLI from tracked HEAD in an isolated ignored
  directory. Its 18 baseline-compatible custom-format test groups also pass;
  the deliberate migration limits remain excluded from baseline comparisons.
- A new differential matrix compared exit status and complete rendered Markdown
  for **375 distinct cases, zero mismatches**. It crosses complete/partial/
  conflicting calendar and time fields, epoch position, all 24 hours, 12-hour
  and meridiem contradictions, quarters, fractions, and custom offsets.
- `cargo deny check advisories bans`: passed against the current database with
  no advisory exception (existing duplicate-version warnings remain allowed).
- All-target dependency-tree and parsed-lockfile checks confirm Chrono is absent.
- `./tools/verify.sh`: passed (524 tests, formatting, all-target/all-feature
  Clippy with warnings denied, release build, and Homebrew formula rendering).
- `git diff --check`: passed. Unrelated TASK-205/SPEC-008 changes and the existing
  TASK-205 index entry were preserved. No commit or push was performed.

## ISO Year Compatibility Follow-up

Reopened to restore `%g` validation against a complete Gregorian date or epoch.
The pivot applies only when a two-digit ISO year establishes the date itself.

The adapter now checks `%g` against the ISO year supplied by a complete
Gregorian date, ordinal date, or epoch. It retains the 69/70 pivot when ISO
fields establish the date. Regression cases for both directive orders, an
inconsistent ISO year, an ordinal date, and epoch completion pass against both
Chrono and the replacement. `cargo deny check advisories bans` and
`./tools/verify.sh` passed; the latter ran 525 tests. Chrono remains absent from
the all-target graph and lockfile. Unrelated local changes were preserved.

## Gregorian Year and ISO Week Follow-up

Reopened to preserve two-digit Gregorian year validation against an ISO week date.

A complete ISO week date now establishes the Gregorian date before `%y` is
validated against its two-digit year. Gregorian calendar and non-ISO week dates
retain their original precedence. Regression tests cover both directive orders,
conflicting components, and the ISO/Gregorian year boundary; these cases pass
against the original Chrono CLI and the replacement. A 60-case differential
matrix over years 1969, 1999, 2000, 2015, 2069, and 2169 found no mismatches.
`cargo deny check advisories bans` and `./tools/verify.sh` passed; the latter ran
526 tests. Chrono and the exception remain absent. Unrelated changes were kept.
