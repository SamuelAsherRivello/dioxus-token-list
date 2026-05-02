# Dioxus 0.7 Workflow Rule

Use this rule for any Dioxus implementation, debugging, routing, state, asset, async loading, cache, or cross-platform work in this repository.

Primary docs: https://dioxuslabs.com/learn/0.7/

## First Pass

- Read `AGENTS.md`, this file, and the files directly involved in the requested behavior before editing.
- Keep shared UI and business logic in `packages/ui`; keep target entrypoints in `packages/web` and `packages/desktop`.
- Preserve both web and desktop unless the request is explicitly platform-specific.
- Prefer existing app patterns before adding new abstractions. This app already uses context signals for theme, token load requests, cached token load results, and toast state.
- Treat Dioxus 0.7 docs as authoritative. Do not use older Dioxus APIs such as `cx`, `Scope`, `use_state`, old router setup, or borrowed component props.

## Components And Props

- Components return `Element` and use `#[component]` when they are rendered from RSX.
- Component names must start with a capital letter or contain an underscore.
- Props must be owned, `Clone`, and `PartialEq`. Use `String`, `Vec<T>`, `Option<T>`, `ReadOnlySignal<T>`, or local model types instead of `&str` or borrowed slices.
- Use component syntax inside RSX, for example `TokenList { tokens }`; do not call component functions directly.
- For repeated UI, prefer direct `for` loops in RSX when the output is straightforward and easier to read.
- Keep target-specific platform code behind modules or `cfg` gates. Shared components should compile for both `wasm32-unknown-unknown` and desktop.

## State And Reactivity

- Use `use_signal` for local mutable state and context signals for shared app state.
- Read signal values with call syntax for cheap clones, `.read()` for borrowed reads, `.peek()` for non-subscribing reads, and write with `.set()`, `.write()`, or `.with_mut()`.
- Use `.peek()` inside cache-write, toast-deduplication, or background persistence logic when a reactive subscription would create a feedback loop.
- Use `use_memo` for derived values that should recalculate only when their dependencies change.
- Use `use_context_provider` in layout/root components and `use_context::<Signal<T>>()` in descendants for shared state. This repo provides app-level context in `packages/ui/src/client/mod.rs`.
- In effects and async closures, clone or copy only the values needed for the closure. Avoid keeping long borrows across `await`.

## Async Loading

- Use `use_resource` for async data that should rerun when the signals it reads change.
- `Resource` reads return `None` while loading and `Some(value)` after completion. Preserve a visible loading or toast state for `None`.
- Use async event handlers or `spawn` for user-triggered work and timers.
- Avoid overlapping user-triggered loads unless the existing flow supports them. Gate refresh behavior with existing request signals or explicit loading state.
- Never block the first meaningful render on optional cache work when snapshot or online token data can render first.

## Routing And Layout

- Keep routes in the single `Route` enum in `packages/ui/src/client/mod.rs`.
- Use `#[derive(Routable, Clone, PartialEq)]`, `#[route("/path")]`, `#[layout(AppLayout)]`, `Router::<Route> {}`, and `Outlet::<Route> {}`.
- Dynamic route fields must match component props and be owned values.
- Render router-aware navigation only under `Router::<Route> {}`. If links fail at runtime, inspect layout and router context before changing unrelated assets.

## Assets And Styles

- Use `asset!("/assets/...")` for local files relative to the package root. Do not use absolute machine paths.
- Keep shared CSS and images under `packages/ui/assets` when used by shared UI.
- Inject styles with Dioxus document components already used in the repo, such as `document::Link { rel: "stylesheet", href: TOKEN_LIST_CSS }`.
- Keep normal token services under `packages/ui/src/client/services`; server functions under `packages/ui/src/server/services` should be optional probes or features that fail gracefully on static hosting.
- For browser-visible styling or asset changes, run the real web app and inspect it instead of trusting compile success alone.

## Token Cache Behavior

- Token data may come from online data, browser snapshot data, or native SQLite.
- Preserve status feedback for token loading, cache reads, cache writes, SQLite operations, errors, and database repopulation.
- Prefer rendering from already available data first, then persisting or refreshing the native SQLite cache in the background when available.
- Do not remove the current toast-style feedback path unless replacing it with an equivalent visible status mechanism.

## Verification

Use the smallest check that proves the change, then broaden when the edit crosses packages or runtime surfaces:

```powershell
cargo check -p ui --target wasm32-unknown-unknown
cargo check -p web --target wasm32-unknown-unknown
cargo check -p desktop
.\Scripts\Other\RunTests.ps1
.\Scripts\Common\RunWeb.ps1
.\Scripts\Common\RunDesktop.ps1
```

For browser UI, routing, asset, or cache-visible changes:

```powershell
dx serve --platform web --addr <this-laptop-ipv4> --port 8080
```

Use a concrete local IPv4 address instead of `0.0.0.0` for fullstack web testing on Windows; the wildcard address can make backend readiness fail with `os error 10049`. If port `8080` is already serving an older build, stop that server and restart this project before trusting browser results.

## Useful Official Doc Pages

- Main Dioxus 0.7 docs: https://dioxuslabs.com/learn/0.7/
- Components and props: https://dioxuslabs.com/learn/0.7/essentials/ui/components/
- Assets: https://dioxuslabs.com/learn/0.7/essentials/ui/assets/
- Signals: https://dioxuslabs.com/learn/0.7/essentials/basics/signals/
- Data fetching: https://dioxuslabs.com/learn/0.7/essentials/basics/resources/
- Defining routes: https://dioxuslabs.com/learn/0.7/essentials/router/routes/
- Navigation: https://dioxuslabs.com/learn/0.7/essentials/router/navigation/
- Layouts: https://dioxuslabs.com/learn/0.7/essentials/router/layouts/
- Anti-patterns: https://dioxuslabs.com/learn/0.7/guides/tips/antipatterns/
