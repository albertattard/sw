# `sw version`

Use `sw version` or `sw --version` to print the current build identity.

```shell
sw version
```

## Common Usage

Print the version:

```shell
sw version
```

Use the top-level version flag:

```shell
sw --version
```

## Notes

- Capture version output when debugging behavior differences between machines.
- Prefer checking the version before investigating reported CLI behavior that
  may depend on the installed build.

For the behavior contract, run `sw explain help` or read
[SPEC-001](../spec/SPEC-001-help-and-discovery.md).
