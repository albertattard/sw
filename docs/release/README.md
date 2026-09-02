# Release Distribution

Official releases are published only from `v*` tags. Normal pushes and pull
requests run baseline quality checks and publish transient diagnostic artifacts;
they do not create downloadable releases.

## Delivering A Change

This is the canonical maintainer workflow for taking a completed change to
users. The root [`AGENTS.md`](../../AGENTS.md) defines agent-specific
authorization and branch-protection rules.

### 1. Prepare The Change

Before committing, make sure the relevant specification and task record match
the implemented behavior. Inspect the complete working-tree diff and confirm
that it contains only the intended change. Never commit directly on `main`:
create a short feature branch first, for example:

```shell
git switch -c codex/describe-the-change
```

### 2. Verify And Commit

Run the complete local verification suite on the branch that will receive the
commit:

```shell
./tools/verify.sh
```

It formats Rust files, runs Clippy and the test suite, builds the release
binary, and checks Homebrew formula rendering. If it fails, fix or investigate
the failure before committing.

Stage the intended paths, review the staged diff, and create one focused commit
with an imperative subject and a short product-oriented body:

```shell
git add <intended-paths>
git diff --cached
git commit
```

### 3. Push And Open A Pull Request

Push the feature branch, then open a pull request targeting `main`:

```shell
git push -u origin codex/describe-the-change
gh pr create --base main
```

Do not push directly to `main`. The pull request is the review and CI boundary;
wait for its required `Quality` check to pass and review the final diff before
merging.

### 4. Merge And Clean Up

Merge the approved pull request into `main` using GitHub. A merge makes the
source change part of the product's main branch, but does not publish a new
installable version.

After the pull request has merged, run the conservative cleanup tool from the
local repository:

```shell
./tools/cleanup-merged-branch.sh codex/describe-the-change
```

The tool fetches and fast-forwards `main`, proves that the branch has no unique
work before deleting it, and rebuilds the local release binary. Keep the branch
when the tool reports that it cannot delete it safely.

### One-Flow Agent Release

When an agent should deliver a completed change through a published Homebrew
update, use this explicit request:

```text
release changes as v<version>
```

For example, `release changes as v0.1.3` authorizes the following guarded
workflow:

1. Create a feature branch and update `Cargo.toml` to version `0.1.3` as part
   of the release change; the `v` prefix belongs only to the Git tag.
2. Run `./tools/verify.sh`, commit the complete intended change, push the
   branch, and open a pull request to `main`.
3. Wait for the required `Quality` check and review to pass, then merge the
   pull request.
4. Run `./tools/cleanup-merged-branch.sh <branch>` and confirm that local
   `main` contains the merged version-bump commit.
5. Create and push the exact requested tag from merged `main`:

   ```shell
   git tag v0.1.3
   git push origin v0.1.3
   ```

6. Wait for the tagged workflow to publish the GitHub Release assets and
   synchronize the Homebrew tap.
7. Verify the release assets and tap formula, then run `brew update`,
   `brew upgrade albertattard/tap/sw`, and `sw version`.

The agent must stop rather than publish a partial release if a local check,
required pull-request check, merge, tag push, release workflow, tap update, or
Homebrew upgrade fails. The phrase always requires an exact `v<version>`;
the agent must never infer a version number.

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

## Publishing A Release

Publishing is optional and separate from merging. Release only when an
intentional version-bump change has been merged and the version in `Cargo.toml`
matches the tag you intend to create.

After the target commit is merged and verified, create and push a version tag:

```shell
git tag v<version>
git push origin v<version>
```

The tag triggers the release workflow, which publishes the GitHub Release
archives, `SHA256SUMS`, and release README before updating the Homebrew tap.
Wait for that workflow to succeed, then verify the release assets and the tap
formula. Homebrew users receive the same verified release through:

```shell
brew update
brew upgrade albertattard/tap/sw
```

Confirm that `sw version` reports the released version. If the workflow or tap
synchronization fails, do not treat the release as published; correct the
reported problem and rerun the release workflow.

The stable formula remains archive-based and does not install Rust or compile
Sociable Weaver on users' machines.
