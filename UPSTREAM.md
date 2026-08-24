<!-- Added by the grok-build-hardened project. -->

# Upstream maintenance and release workflow

The repository keeps the official Git history intact and carries the fork as
a short commit stack on top. Use `origin` for this fork and `upstream` for the
official project:

```bash
git remote -v
# origin    https://github.com/ChillMagic/grok-build-hardened.git
# upstream  https://github.com/xai-org/grok-build.git
```

Configure or validate these local remotes idempotently with:

```bash
./scripts/hardening/setup-remotes.sh
```

## 1. Inspect upstream without changing the branch

```bash
git fetch upstream main --tags
./scripts/hardening/upstream-status.sh upstream/main
```

The report highlights files that overlap the hardening layer and upstream
changes whose paths contain upload, telemetry, remote, managed, storage,
feedback, share, update, policy, Sentry, Mixpanel, or workspace terms.

## 2. Rebase the fork commit stack

Start from a clean `main` branch:

```bash
./scripts/hardening/rebase-upstream.sh upstream/main
```

The script creates a local `backup/pre-rebase-*` tag and then runs:

```text
git rebase --onto <new-upstream> <approved-old-upstream> main
```

Resolve conflicts in favor of the privacy boundary, not merely in favor of
successful compilation. `git rerere` records recurring conflict resolutions.

## 3. Audit and approve the new base

Immediately after a rebase, `./scripts/hardening/check.sh` intentionally fails
because `.hardened/upstream.env` still names the old audited commit. Do not
weaken this check.

A maintainer must inspect:

- Every upstream change reported by `upstream-status.sh`.
- All new HTTP, WebSocket, gRPC, storage, subprocess, plugin, MCP, updater,
  telemetry, remote policy/config, workspace, feedback, share, trace, and
  archive paths.
- New configuration, environment variables, command-line flags, or server
  responses that could bypass compile-time privacy constants.
- Platform-specific branches for both macOS and Linux.
- Cargo dependency/build-script changes and newly bundled executables.

Only after the audit:

1. Update `UPSTREAM_COMMIT` and `UPSTREAM_VERSION` in
   `.hardened/upstream.env`.
2. Set `HARDENED_REVISION=1` for a new upstream version, or increment it for a
   hardening-only release on the same version.
3. Regenerate `.hardened/source-paths.tsv` from the reviewed diff:

   ```bash
   git diff --name-status "$UPSTREAM_COMMIT"..HEAD -- \
     Cargo.lock Cargo.toml crates >.hardened/source-paths.tsv
   ```

4. Ensure every modified upstream source file retains its first-line
   modification notice.
5. Run the verification and builds:

   ```bash
   ./scripts/hardening/check.sh
   cargo fmt --all -- --check
   cargo check --locked -p xai-grok-pager-bin
   ./scripts/hardening/build-offline.sh
   ```

6. Commit the audit approval separately so the reviewed base change is
   obvious in history.

## 4. Tag and publish

```bash
./scripts/hardening/tag-release.sh
git push origin main
git push origin v<upstream-version>-hardened.<revision>
```

The tag script refuses a dirty tree, a stale base, an unexpected tag name, or
a failed hardening check. It creates an annotated local tag; pushing remains
an explicit separate action.

Never force-push a published release tag. A correction on the same upstream
version receives a new hardening revision.
