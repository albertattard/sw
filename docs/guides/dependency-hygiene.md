# Dependency Hygiene

Run the same advisory and duplicate-version checks as CI with:

```shell
cargo deny check advisories bans
```

## Datetime Dependency

[TASK-208](../tasks/repo-process/TASK-208-replace-chrono.md) replaces Chrono with
Jiff 1.0, enabling only `std`: no host timezone lookup or bundled timezone data
is needed for fixed-offset runbook rewriting. The replacement was compared
with `time` 0.3.55 and the original Chrono implementation, including executable
CLI comparisons. The task records the decision and compatibility evidence.

`src/runbook/datetime.rs` retains the established custom-format semantics through
an explicit adapter. Similar strftime spelling is insufficient: raw nanoseconds,
fixed fractional widths, fractional output grouping, composite formats, numeric
offsets, weekday checks, and time-only fallback need individual treatment.
`datetime_format.rs` handles syntax; `datetime_fields.rs` reconciles calendar,
epoch, and clock constraints before conversion and time-only fallback. Parsing
each directive successfully does not establish consistency between fields.
SPEC-003 documents the deliberately bounded year range, rejection of leap
seconds, and operational errors for arithmetic overflow and invalid formats.

The temporary RUSTSEC-2026-0543 exception has been removed from `deny.toml`.
No advisory exceptions are active. Keep
[TASK-207](../tasks/repo-process/TASK-207-temporarily-except-chrono-advisory.md)
as the historical risk-acceptance record; its manual 2026-12-17 deadline no
longer applies to the resolved dependency graph.

To check removal across all target graphs, run:

```shell
cargo tree --locked --target all
```

The output must contain no `chrono` package. `Cargo.lock` must likewise contain
no Chrono package, even for other targets. Always run the advisory check against
the current database before delivery; replacing one library does not establish
that its entire dependency graph remains free of new advisories.

Upstream format references:
[Jiff strtime](https://docs.rs/jiff/1.0.0/jiff/fmt/strtime/) and
[time strftime](https://docs.rs/time/0.3.55/time/format_description/fn.parse_strftime_owned.html).
