# Dioxus Token List Constitution

## Core Principles

### I. Dioxus 0.7 Is The UI Contract
All UI work MUST use Dioxus 0.7 patterns. Components use `#[component] fn Name(...) -> Element`, signals use `use_signal`, and routes use the `Routable` enum with `Router::<Route> {}` and `Outlet<Route> {}` where layouts require it. Removed APIs such as `cx`, `Scope`, and `use_state` are not acceptable in this project.

### II. Shared UI, Platform Entry Points
Product behavior belongs in `packages/ui` unless it is truly platform-specific. `packages/web` and `packages/desktop` remain thin launch and asset surfaces. Changes MUST preserve both browser and desktop support unless a spec explicitly narrows the platform scope.

### III. Visible Loading And Cache Feedback
Token loading, cache reads, cache writes, refreshes, and database repopulation MUST remain visible to users through the app's loading and toast-style status feedback. Browser SQLite startup MUST NOT block the UI when snapshot or online data can render first.

### IV. Local Preferences Stay Local
Theme, language, and browser token snapshots remain local user preferences. UI preferences MUST be persisted through the storage service rather than the token database, and online token data MUST remain unlocalized market data.

### V. Verify Real Behavior
Browser-visible changes SHOULD be checked against the served web app when practical. Use repository scripts for repeat workflows, especially `.\Scripts\Common\RunWeb.ps1`, `.\Scripts\Common\RunDesktop.ps1`, and `.\Scripts\Other\RunTests.ps1`. Compile success alone is not enough for browser runtime or visible UI issues.

## Project Constraints

- Keep the workspace split between `packages/ui`, `packages/web`, and `packages/desktop`.
- Keep static app chrome localizable through the Fluent locale assets in `packages/ui/assets/i18n/`.
- Preserve source labels visible to users as `Database` and `Online`; browser snapshot status is treated as database-sourced copy in the UI.
- Preserve the top navigation order: Home, Video, About, Contact.
- Keep local token icons, profile imagery, tech imagery, and route CSS under the existing asset boundaries.
- Do not introduce broad app redesigns or unrelated refactors while implementing a feature spec.

## Development Workflow

1. Inspect current files before editing and keep changes scoped to the requested behavior.
2. Prefer project scripts over ad hoc commands for setup, web serving, desktop serving, and tests.
3. For Dioxus or dependency guidance, use current Dioxus 0.7 documentation and the project-local Codex rules before changing code.
4. If a port or build artifact is stale or locked, diagnose the actual process or path instead of assuming a clean environment.
5. Treat `target/`, `node_modules/`, runtime data, and browser-test output as generated artifacts unless a spec explicitly says otherwise.

## Governance

This constitution applies to all future Spec Kit specifications, plans, and task lists for this repository. Specs may add narrower acceptance criteria, but they must not contradict these principles without explicitly updating this constitution and documenting the reason.

**Version**: 1.0.0 | **Ratified**: 2026-04-30 | **Last Amended**: 2026-04-30
