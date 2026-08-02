# Plan 005: Publish the contributor contract and community files

> **Executor instructions**: Do not document unfinished behavior as complete. Re-run every command shown to contributors. Stop on missing maintainer contact/governance decisions rather than inventing them. Update `plans/README.md` when complete.
>
> **Drift check (run first)**: `git diff --stat 13268db..HEAD -- README.md AGENTS.md DESIGN.md ARCHITECTURE.md PREREQUISITES.md LICENSE CONTRIBUTING.md SECURITY.md CODE_OF_CONDUCT.md SUPPORT.md .github`
> Reconcile completed Plans 001–004 before editing claims.

## Status

- **Priority**: P1
- **Effort**: M
- **Risk**: LOW
- **Depends on**: Plans 001–004
- **Category**: docs, dx
- **Planned at**: commit `13268db`, 2026-08-02

## Why this matters

The current README is enough for the original author to run the prototype, but not enough for a stranger to judge maturity, install safely, understand the architecture, or submit a compatible change. GitHub also has no contribution, security, conduct, support, issue, or pull-request guidance. Public documentation must match the actual runtime and verification gates established by earlier plans.

## Current state

- `README.md:1-29` contains a description, two run commands, a packaging claim, and links to AGENTS/DESIGN; it lacks status, features, screenshot, architecture, tests, contribution workflow, privacy, support/security, and license sections.
- `AGENTS.md:14-35` describes current files/signals but not enforced hexagonal dependency rules; `AGENTS.md:46` contains a machine-specific port-conflict story; `AGENTS.md:94-107` makes Graphify appear central even though its output is untracked and optional.
- `DESIGN.md:1-126` is a useful visual guide but does not distinguish implemented versus planned responsive behavior or include a contributor checklist for component states and reduced motion.
- `ARCHITECTURE.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `SUPPORT.md`, PR template, and issue forms were absent at audit time.
- `LICENSE:1-20` is MIT, but the copyright holder must be confirmed by the maintainer before publication.
- GitHub's community profile recognizes CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, SUPPORT, issue/PR templates, governance, and LICENSE as standard project health files.

## Commands you will need

| Purpose | Command | Expected on success |
| --- | --- | --- |
| Canonical checks | `just check` | Exit 0 |
| Docs whitespace | `git diff --check -- README.md AGENTS.md DESIGN.md ARCHITECTURE.md CONTRIBUTING.md SECURITY.md CODE_OF_CONDUCT.md SUPPORT.md .github` | Exit 0 |
| Link/path inventory | `rg --files | sort` | Every relative link target exists |
| Required files | `test -f CONTRIBUTING.md && test -f SECURITY.md && test -f CODE_OF_CONDUCT.md && test -f .github/pull_request_template.md` | Exit 0 |
| GitHub forms | Parse `.github/ISSUE_TEMPLATE/*.yml` with a YAML validator | Exit 0 |

## Scope

**In scope**:

- `README.md`
- `CONTRIBUTING.md` (create)
- `SECURITY.md` (create)
- `CODE_OF_CONDUCT.md` (create from a recognized template after maintainer approval)
- `SUPPORT.md` (create)
- `AGENTS.md`
- `DESIGN.md`
- `ARCHITECTURE.md` link/accuracy corrections only
- `PREREQUISITES.md`
- `.github/pull_request_template.md` (create)
- `.github/ISSUE_TEMPLATE/bug.yml`, `feature.yml`, `config.yml` (create)
- Optional `CHANGELOG.md` and `ROADMAP.md` only if the maintainer accepts their maintenance policy
- `LICENSE` only after copyright ownership is confirmed
- `plans/README.md`

**Out of scope**:

- Product code, build configuration, or workflow behavior
- Publishing the repository, release, package, or installer
- Inventing support/security email addresses, governance roles, or release promises
- Marketing claims not supported by tests/releases

## Git workflow

- Branch: `codex/005-open-source-docs`
- Suggested commit: `docs: prepare public contribution guide`.
- Do not push, enable discussions, or publish a release unless instructed.

## Steps

### Step 1: Rewrite README as the public landing page

Lead with the product outcome and mark maturity honestly (for example, early alpha until signed installers and persistence exist). Add:

1. Screenshot/GIF using a committed asset with alt text.
2. Implemented features and explicit current limitations.
3. Supported platforms and installer status.
4. Quick start with exact pinned prerequisites or link to PREREQUISITES.
5. One-command verification (`just check`).
6. Small architecture diagram and links to ARCHITECTURE/ADRs.
7. Documentation index: CONTRIBUTING, DESIGN, AGENTS, SECURITY, SUPPORT, LICENSE.
8. Privacy/runtime wording: no remote/product backend; Node is build-time only; installed desktop UI is embedded and offline after Plan 003.
9. Contribution invitation and a short “good first contribution” path.
10. License section.

README should serve users and new contributors; detailed agent rules stay in AGENTS.

**Verify**: run every command copied into README on a clean clone/environment; all exit as documented.

### Step 2: Add CONTRIBUTING and PR expectations

Create `CONTRIBUTING.md` with:

- reading order: README → ARCHITECTURE → DESIGN → relevant ADR → AGENTS;
- prerequisites and first-run setup;
- issue-first guidance for behavior/architecture changes;
- branch naming and commit style based on an explicitly chosen convention;
- exact `just check` gate and platform packaging expectations;
- where domain, application, UI, Tauri, web, and adapter changes belong;
- test requirements by layer;
- documentation synchronization table (which docs change when runtime, ports, UI tokens, commands, or persistence change);
- PR checklist, review expectations, and scope discipline.

Create a concise PR template that asks for problem, approach, tests, screenshots for UI changes, architecture/doc impact, and checklist completion. Issue forms should collect reproduction/version/platform for bugs and user value/trade-offs for features.

**Verify**: a contributor with no prior context can locate the file for a new timer rule, storage adapter, and sidebar change using only CONTRIBUTING/ARCHITECTURE.

### Step 3: Add security, conduct, and support policies

- SECURITY: supported versions and a private vulnerability-reporting path. Prefer GitHub private vulnerability reporting if enabled; otherwise use a maintainer-provided address. Explicitly tell reporters not to file public security issues.
- CODE_OF_CONDUCT: adopt Contributor Covenant or another recognized template only after the maintainer confirms enforcement responsibility and contact details.
- SUPPORT: route usage questions, bug reports, feature discussions, and security reports to distinct channels that actually exist.
- Confirm the copyright holder in LICENSE; do not silently preserve or replace `Max Wells` without owner confirmation.

**Verify**: every contact/channel is real, private where promised, and linked from README and issue configuration.

### Step 4: Tighten AGENTS and DESIGN for contributors

AGENTS should be an operational guide, not a second README:

- concise project purpose and non-goals;
- enforced dependency rules and composition roots;
- exact canonical commands and targeted checks;
- safe file ownership/change map;
- doc-update matrix and STOP conditions;
- optional Graphify instructions clearly labeled optional, or remove them if outputs will not be maintained.

DESIGN should preserve its current visual language and add:

- implemented/planned labels for responsive/mobile behavior;
- component state checklist (default, hover, active, focus, disabled, loading, error, empty);
- reduced-motion and keyboard checks;
- screenshot requirement for visual PRs;
- rule that product/domain behavior belongs in core, not view components.

**Verify**: no contradiction remains among README, AGENTS, DESIGN, ARCHITECTURE, and ADRs for runtime, supported platforms, state persistence, or commands.

### Step 5: Run a public-readiness review

From a fresh clone, follow README and CONTRIBUTING without using machine-global knowledge. Run `just check`. Review GitHub's community profile after pushing to a private staging repository or after publication approval. Confirm secrets, local IPs, machine paths, generated artifacts, and personal troubleshooting notes are absent from tracked files.

**Verify**: `rg -n '/Users/|192\.168\.|Langfuse|OrbStack|assume-unchanged' --glob '!plans/**' .` → no tracked public-document/config matches except deliberately ignored local examples.

## Test plan

- Fresh-clone onboarding on macOS and Windows.
- All README/CONTRIBUTING commands execute exactly as written.
- Markdown links resolve and issue-form YAML parses.
- UI PR template requires screenshots and accessibility confirmation.
- Security reporting path is private and tested by the maintainer.
- Cross-document assertion table confirms runtime, architecture, persistence, supported targets, and verification claims agree.

## Done criteria

- [ ] README is a truthful public landing page with status, screenshot, features, quick start, architecture, contribution, privacy, security/support, and license sections.
- [ ] CONTRIBUTING gives a zero-context contributor an exact path and quality gate.
- [ ] SECURITY, CODE_OF_CONDUCT, SUPPORT, PR template, and issue forms exist with real maintainer-approved contacts/channels.
- [ ] AGENTS is concise, operational, standalone, and consistent with ARCHITECTURE.
- [ ] DESIGN covers states, accessibility/reduced motion, responsive status, and visual PR evidence.
- [ ] No local paths/IPs or machine-specific troubleshooting remain in tracked public material.
- [ ] Fresh-clone macOS and Windows onboarding succeeds.
- [ ] `just check` passes.
- [ ] `plans/README.md` marks Plan 005 DONE.

## STOP conditions

- No maintainer-approved private security channel exists.
- No person/team accepts responsibility for enforcing the chosen code of conduct.
- LICENSE copyright ownership is uncertain.
- Plan 003 has not proven offline packaged behavior; keep installer wording explicitly experimental and do not claim release readiness.
- A documented command fails twice in a fresh clone.

## Maintenance notes

Community files create promises. Assign an owner and review cadence for dependency updates, security reports, issue triage, and release notes. Keep README short enough to scan; detailed design and architecture belong in their dedicated documents.

