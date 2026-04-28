---
name: dioxus-token-list-project
description: Work on the dioxus-token-list Rust workspace. Use when Codex is asked to modify, debug, verify, or explain this repository, especially Dioxus 0.7 UI, routing, assets, token loading, browser or desktop cache behavior, SQLite WASM, OPFS, or project scripts.
---

# dioxus-token-list Project

Use this skill for repository-specific execution context. Follow `AGENTS.md` and `.codex/rules/dioxus-0.7-workflow.md` first for Dioxus 0.7 rules.

## Start Here

1. Confirm the current directory is `D:\Documents\Projects\VC\Rust\dioxus-token-list`.
2. Read `.codex/rules/dioxus-0.7-workflow.md` for Dioxus implementation work.
3. Read the files directly involved in the request before editing.
4. Keep web and desktop support intact unless the request is explicitly platform-specific.
5. Prefer the existing `packages/ui`, `packages/web`, and `packages/desktop` boundaries.
6. Use the project scripts before inventing new commands.

## Workspace Map

| Path | Use |
| ---- | --- |
| `packages/ui/src/app.rs` | Shared app shell and routing. |
| `packages/ui/src/components` | Shared Dioxus components. |
| `packages/ui/src/pages` | Routed pages. |
| `packages/ui/src/services` | Token, online, browser snapshot, and database services. |
| `packages/ui/assets` | Shared CSS and token images. |
| `packages/web/src/main.rs` | Web entrypoint. |
| `packages/web/public/assets/sqlite.org` | Browser SQLite WASM and worker assets. |
| `packages/desktop/src/main.rs` | Desktop entrypoint. |
| `Scripts` | Windows PowerShell setup and run workflows. |

## Dioxus 0.7 Constraints

- Use `dioxus = "0.7.1"` patterns.
- Do not use `cx`, `Scope`, or `use_state`.
- Use `#[component] fn Name(...) -> Element`.
- Use `use_signal`, `use_memo`, `use_resource`, and signal `.read()`, `.write()`, `.with_mut()`, or call syntax.
- Use `Router::<Route> {}` and `Outlet<Route> {}` for routing.
- Use `asset!("/path/from/project/root")` for local assets.
- Keep props owned, `Clone`, and `PartialEq`.

## Cache And Loading Behavior

- Preserve visible loading affordances during online fetches, cache reads, database writes, and repopulation.
- Avoid feedback loops from reactive reads inside cache-write paths; use non-subscribing reads such as `peek()` when the codebase already follows that pattern.
- Browser SQLite worker startup can be sensitive. Prefer rendering from snapshot or online data first, then persisting SQLite in the background when appropriate.
- Treat stale dev servers as a common source of false browser results.

## Verification

Use the narrowest check that proves the change:

```powershell
cargo check -p ui --target wasm32-unknown-unknown
cargo check -p web --target wasm32-unknown-unknown
cargo check -p desktop
.\Scripts\Common\RunWeb.ps1
.\Scripts\Common\RunDesktop.ps1
.\Scripts\Other\RunTests.ps1
```

For browser UI, routing, asset, or cache changes, serve the web app and inspect the actual page when practical. The default web target is:

```powershell
dx serve --platform web --addr 0.0.0.0 --port 8080
```

If `8080` is occupied by a stale server, stop that server and restart the project server.

## Response Style

Report the changed files, the verification command or browser check used, and any remaining risk. Keep the answer concise and concrete.
