<!-- Added by the grok-build-hardened project. -->

# Modifications from upstream

This repository is a derivative of
[`xai-org/grok-build`](https://github.com/xai-org/grok-build), licensed under
Apache License 2.0.

Approved upstream base:

- Repository: `https://github.com/xai-org/grok-build.git`
- Commit: `07b2f7144fd5c5c9d3dd1966937a87852d2dbdb8`
- Upstream package version: `1.0.8`
- First hardened release: `v1.0.8-hardened`

Files derived from upstream and changed by this project carry a prominent
notice in their first line. Deleted and added source paths are recorded by
`.hardened/source-paths.tsv`.

## Removed capabilities

- Passive/background repository, workspace, session, trace, feedback, share,
  heap-profile, and artifact uploads.
- GCS/S3 storage clients and upload queues used by those passive paths.
- Mixpanel, external OTLP, Sentry egress, and other network telemetry emitters.
- Remote managed configuration, signed remote policy, cloud workspace/session
  synchronization, remote skill/model catalogs, and rollout control.
- Automatic update checks, background downloads, manual `grok update`, setup
  reinstall paths, and remotely supplied version policy.
- Build-time downloading of bundled search tools.

Compatibility APIs that remain necessary for compilation are inert, local
facades returning a stable disabled/removed result. Their privacy markers are
compile-time constants and have no environment, CLI, config, or remote
override.

## Deliberately retained network behavior

This fork remains an online AI client. The following network behavior is
outside the “passive upload” removal claim:

- Model inference explicitly initiated by the user/agent, including the
  prompt and selected context needed for that request.
- Explicitly invoked image, image-edit, and video generation.
- User-installed or user-configured MCP servers and plugins.
- Authentication needed to access the chosen model provider.

Users must review provider terms and understand what context an agent selects
for inference. This project does not claim that no source text can ever leave
the machine.

## Update approval

No upstream commit, branch, tag, release, setting, or server response can make
the binary update itself. A new upstream revision must be rebased, manually
audited, checked, built, committed, and tagged. See [`UPSTREAM.md`](UPSTREAM.md).

## Trademark and project status

This is an unofficial community fork and is not affiliated with, endorsed by,
or supported by xAI or SpaceXAI. Upstream product names are used only for
identification and attribution. Official logos are intentionally not used in
the fork README or release branding.

## Repository operational hardening

The maintenance tools configure `origin` as the only push destination and set
the official `upstream` push URL to `DISABLED`. Public rebases use an exact
`--force-with-lease` bound to the previously fetched `origin/main` SHA. Release
tags are never moved or force-pushed. Source auditing, offline release builds,
runtime rejection tests, and native archive creation share one cross-platform
Python implementation used by Windows, macOS, and Linux CI runners. The
upstream proto dependency generator uses temporary files rather than Unix-only
`/dev/stdout` and `/dev/null` devices so the reviewed source builds on Windows.
