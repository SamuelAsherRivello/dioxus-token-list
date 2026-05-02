# Codex Project Boilerplate

Use this folder for Codex-specific project context that should travel with this repository. Spec Kit workflow skills live in `.agents/skills`; keep `.codex` focused on project-specific rules and the Dioxus token-list skill.

## Project

`dioxus-token-list` is a Rust workspace for a Dioxus 0.7 token list UI with shared UI code and separate web and desktop entrypoints.

| Path | Purpose |
| ---- | ------- |
| `packages/ui` | Shared Dioxus code split between `src/client` UI/client services and `src/server` optional server functions. |
| `packages/web` | Web app entrypoint, web assets, and optional fullstack server feature wiring. |
| `packages/desktop` | Desktop app entrypoint and desktop assets. |
| `Scripts` | Windows PowerShell workflows for setup, web run, and desktop run. |
| `Documentation/Images` | README screenshots and infographic assets. |

## Project Rules

Use `.codex/rules/dioxus-0.7-workflow.md` for Dioxus implementation, debugging, routing, state, asset, async loading, cache, or cross-platform work.

Use `.codex/rules/frontend-design.md` when changing user-visible UI, CSS, layout, component composition, route presentation, loading states, toasts, empty/error states, or visual assets.

## Default Commands

Prefer the repository scripts when possible:

```powershell
.\Scripts\Common\InstallDependencies.ps1
.\Scripts\Common\RunWeb.ps1
.\Scripts\Common\RunDesktop.ps1
.\Scripts\Other\RunTests.ps1
```

For direct Dioxus web work, use:

```powershell
dx serve --platform web --addr 127.0.0.1 --port 8080
```

The web script stops an older `dx serve` process on the requested port before starting a new one. If you run `dx serve` directly and port `8080` is already occupied, stop the old server and restart it so browser testing uses the latest build for this checkout.

## Dioxus Rules

- Use Dioxus `0.7.1` patterns from `AGENTS.md` and `.codex/rules/dioxus-0.7-workflow.md`.
- Do not use removed APIs such as `cx`, `Scope`, or `use_state`.
- Use `use_signal`, `use_memo`, `use_resource`, `Router::<Route> {}`, `Outlet<Route> {}`, and `asset!` with Dioxus 0.7 syntax.
- Keep props owned, cloneable, and `PartialEq`; use `String` and `Vec<T>` instead of borrowed props.

## Runtime Notes

- The app supports web and desktop paths.
- Token data can come from online data, browser snapshot data, and client-side native SQLite caches.
- Server functions under `packages/ui/src/server/services` should stay optional so static hosting can fail gracefully.
- Preserve visible loading or toast-style status feedback during cache refreshes and repopulation.
- Browser behavior should be validated in a real served app when UI, routing, cache loading, or asset behavior changes.

## Skill Work

Use `.codex/skills/dioxus-token-list-project/SKILL.md` as the starting point for a project-specific custom skill.

If the skill should be auto-discovered by Codex, install or copy the finalized skill folder into your Codex skills directory, commonly:

```powershell
$env:USERPROFILE\.codex\skills\dioxus-token-list-project
```

After installing a new skill, restart Codex so it can discover the new metadata.
