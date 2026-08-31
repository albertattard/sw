# Release Distribution

Official releases are published only from `v*` tags. Normal pushes and pull
requests run baseline quality checks and publish transient diagnostic artifacts;
they do not create downloadable releases.

## Maintainer setup

Configure the `HOMEBREW_TAP_TOKEN` Actions secret in `albertattard/sw`. It must
be a narrowly scoped credential with contents write access to
`albertattard/homebrew-tap` and no broader repository permissions.

The tagged-release workflow builds the supported Apple Silicon macOS and
portable x86_64 Linux archives, publishes them with `SHA256SUMS` and a generated
release `README.md`, then updates `Formula/sw.rb` in the tap from that exact
checksum file. Formula synchronization intentionally fails when the secret is
missing or the tap cannot be updated; correct the credential or access issue
and rerun the release workflow.

## Releasing

After the target commit is merged and verified, create and push a version tag:

```shell
git tag v<version>
git push origin v<version>
```

The resulting GitHub Release offers version-pinned archive URLs and appears at
GitHub's latest-release page. Homebrew users then receive the same verified
release through:

```shell
brew update
brew upgrade albertattard/tap/sw
```

The stable formula remains archive-based and does not install Rust or compile
Sociable Weaver on users' machines.
