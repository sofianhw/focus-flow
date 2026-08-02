# Security policy (draft)

This document is a draft and is not a published vulnerability-reporting
commitment. Before publication, a maintainer must confirm the supported
versions and configure a private reporting channel in this file and in the
repository settings.

## Reporting a vulnerability

Please do **not** report suspected vulnerabilities in a public GitHub issue,
pull request, or discussion. No private reporting contact is configured yet;
maintainers must add one before this policy can be treated as active. Until
then, avoid sharing sensitive details publicly and use ordinary issues only
for non-sensitive bugs.

## Scope and support

The supported product scope and versions are intentionally unlisted until the
maintainer approves a release/support policy. The desktop application is
local-first and should not require a remote product service at runtime; report
security-relevant behavior in the native wrapper, embedded frontend, build
pipeline, or documented dependencies once a private channel is available.

## Maintainer checklist before publication

- Name the supported release lines and end-of-support expectations.
- Add a private reporting path and response/credit policy.
- Confirm who owns triage and disclosure decisions.
- Add the approved link to [SUPPORT.md](SUPPORT.md) and the repository issue
  configuration where appropriate.
