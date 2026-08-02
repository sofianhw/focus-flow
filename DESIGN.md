# Focus Flow UI design guide

This document is the visual and interaction source of truth for Focus Flow. Keep new screens consistent with these rules unless a deliberate product decision changes them.

Architecture context is defined in [`ARCHITECTURE.md`](ARCHITECTURE.md) and
the accepted runtime and boundary decisions in [`docs/adr`](docs/adr). This
guide describes presentation behavior only; visual components must not become
the place where domain invariants or persistence rules are defined.

## Product character

Focus Flow should feel calm, clear, and lightweight: a small personal workspace for starting a focused session and seeing the day's progress. The current build is a local-first desktop prototype with two views, a three-priority workspace, and a deliberately small feature surface. It intentionally avoids account, login, team, and operational-dashboard patterns.

Prefer:

- One clear primary action per surface.
- Comfortable whitespace and short labels.
- Quiet borders and surfaces with violet used for focus and progress.
- Plain language that describes the user's next action.

Avoid:

- Login or account screens.
- Dense tables, admin-dashboard chrome, or unnecessary badges.
- Unicode emoji as navigation icons; use the shared SVG icon components instead.

## Application frame

```text
Native macOS/Windows title bar
┌──────────────────────────────────────────────────────────────┐
│ Focus Flow header · brand · theme toggle                     │
├───────────────┬──────────────────────────────────────────────┤
│ Workspace     │ Main content                                 │
│ Home          │ Home dashboard or Settings                   │
│ Settings      │                                              │
└───────────────┴──────────────────────────────────────────────┘
```

- The Tauri title bar is visible so native window controls do not overlap the brand.
- The custom header is 56px high and carries the `data-tauri-drag-region` attribute.
- The sidebar is 256px wide when expanded and 80px wide when collapsed.
- The sidebar collapse control sits beside “Workspace”. In the collapsed state, icons and the control are centered.
- Desktop content is centered with a `max-w-5xl` dashboard width and `max-w-3xl` settings width.
- Keep the main region scrollable while the header and app frame remain stable.

The header and sidebar are rendered once by `app/src/app.rs`; Home and Settings
swap inside the main region without recreating the workspace controller. Do not
move the controller into an individual page or a page navigation will reset the
timer and priorities.

## Visual language

The design tokens live in [`style/tailwind.css`](style/tailwind.css).

| Role | Current treatment |
| --- | --- |
| Page background | `background` token |
| Primary text | `foreground` token |
| Secondary text | `muted-foreground` token |
| Cards | `card` surface, subtle border, rounded corners |
| Quiet controls | `secondary` or `accent` surface |
| Focus/progress | Violet-to-indigo gradient or violet fill |
| Positive trend | Emerald text/fill |
| Warning | Amber text/fill |
| Destructive action | `destructive` token |

Use the existing semantic tokens (`bg-background`, `text-muted-foreground`, `bg-secondary`, `bg-accent`, and related classes) instead of introducing one-off colors.

Base geometry:

- Default radius: `0.625rem`.
- Small controls: 32–40px height.
- Primary content gaps: `gap-6` between sections and `gap-3` within control groups.
- Content padding: `px-5 py-8`, increasing to `sm:px-8` on wider screens.
- Use tabular numerals for timer and metric values.

## Typography

- Use the existing sans-serif stack from the Tailwind/browser defaults.
- Page titles use `text-3xl font-bold tracking-tight`, growing to `sm:text-4xl` where appropriate.
- Section labels are short uppercase labels with `text-sm font-medium` and `tracking-wider`.
- Supporting copy uses `text-muted-foreground` and a relaxed line height.
- Timer values should be large, high-contrast, and `tabular-nums`.

## Component rules

### Navigation

- Use the shared `icons` crate for navigation symbols.
- Place each icon in a fixed `size-5` wrapper with `place-items-center` and `aria-hidden="true"`.
- Keep navigation rows `items-center`, with a consistent `gap-3`.
- Active navigation uses the accent surface and accent foreground; inactive navigation uses muted text and a quiet hover surface.
- Every icon-only control needs an accessible `aria-label` and a `title`.

### Cards

Use the shared card components in [`app/src/components/ui/card.rs`](app/src/components/ui/card.rs). Cards should contain a clear heading, optional description, and one focused content group. Avoid nesting cards unless the hierarchy is genuinely necessary.

### Buttons and inputs

Use [`button.rs`](app/src/components/ui/button.rs) and [`input.rs`](app/src/components/ui/input.rs) for new controls. Keep the primary action visually distinct, provide a visible hover state, and preserve a readable focus ring.

### Charts and progress

Charts should explain one simple relationship, use the same violet/semantic palette, and include text labels or titles so the data is not color-only. Progress bars need a visible label and percentage/value context. The current weekly chart uses intentionally static sample values; do not describe it as persisted history until a typed history model and repository are implemented.

## Contributor checklist

For every new or changed component, review the states that apply and record
any intentional exceptions:

- default and hover;
- active/pressed and selected;
- keyboard focus-visible;
- disabled and loading;
- error and empty.

Keep state changes legible in both themes and do not rely on color alone.

Visual pull requests should include before/after screenshots of the affected
surface, with relevant light/dark or responsive states when those change. Use
non-sensitive sample data and mention any state that cannot be shown in a
static capture.

## Interaction states

- Timer: ready → running → paused or complete. The button label must reflect the current state.
- Sidebar: expanded and collapsed states must preserve the same navigation order and accessible labels.
- Theme: light/dark mode is controlled by the theme hook, uses semantic tokens,
  and stores its boolean in browser `localStorage`.
- Settings: values are parsed and dispatched by an effect as the inputs change;
  they update the next session immediately and stay within the core's bounds
  (5–90 minutes per session, 15–600 minutes per daily goal).
- Priorities: the prototype keeps three priorities, replaces the first one from
  the Home input, and marks the first one complete from the action button.
- Save/complete actions should provide an immediate visual result; avoid silent
  no-op clicks.

Use short transitions for layout and color changes. Do not animate essential content in a way that blocks starting a focus session.

Presentation state such as the selected view, sidebar collapse, theme, focus
ring, loading state, and responsive layout belongs in the UI adapter. Timer
rules, value limits, completion semantics, and persistence belong behind the
application/core boundary described in `ARCHITECTURE.md`.

## Accessibility and platform behavior

- Prefer semantic `header`, `nav`, `main`, `section`, `button`, and `label` elements.
- Keep timer status text inside an `aria-live="polite"` region.
- Decorative icons are hidden from assistive technology; icon-only buttons retain text alternatives.
- Maintain keyboard focus visibility and sufficient contrast in both themes.
- Verify keyboard-only operation, logical tab order, and Enter/Space behavior
  for controls; do not make an essential action pointer-only.
- Honor `prefers-reduced-motion`: keep transitions short and decorative, and
  never make essential content or a focus-session action depend on animation.
- Do not place custom branding beneath macOS traffic lights when the native title bar is visible.
- Keep the header drag region separate from interactive controls so buttons remain clickable.

Responsive status: the desktop frame and collapsed sidebar are implemented;
mobile navigation parity remains planned and is not a supported product
surface yet.

## Source map

- App shell and view state: [`app/src/app.rs`](app/src/app.rs)
- Header and title-bar drag region: [`app/src/components/layout/header.rs`](app/src/components/layout/header.rs)
- Sidebar and navigation: [`app/src/components/layout/sidebar.rs`](app/src/components/layout/sidebar.rs)
- Dashboard: [`app/src/domain/home/page_home.rs`](app/src/domain/home/page_home.rs)
- Settings: [`app/src/domain/settings/page.rs`](app/src/domain/settings/page.rs)
- Core rules and commands: [`crates/focus-flow-core/src`](crates/focus-flow-core/src)
- Shared primitives: [`app/src/components/ui`](app/src/components/ui)
- Theme tokens and global styles: [`style/tailwind.css`](style/tailwind.css)
