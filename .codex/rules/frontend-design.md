# Frontend Design Rule

Use this rule when changing user-visible UI, CSS, layout, component composition, route presentation, loading states, toasts, empty/error states, or visual assets in this Dioxus token-list workspace.

Inspired by Koomook's `claude-frontend-skills` distinctive frontend guidance, but adapted for this app. Do not copy its generic landing-page patterns directly; translate the intent into a dense, readable crypto market interface that works in both web and desktop targets.

## Design Intent

- Give the app a specific visual point of view before changing styles. Prefer a concise reference such as "exchange terminal", "technical market dashboard", "editorial crypto index", or "hardware-wallet utility" over vague words like modern or clean.
- Apply the point of view through a system: typography, color, spacing, motion, and background should support the same idea.
- Keep the token list scannable. Visual personality must not reduce price, symbol, rank, source, loading, or error readability.
- Preserve the existing application structure unless the requested behavior needs a broader change. Shared UI belongs in `packages/ui`; target-specific styling and entry behavior stay in `packages/web` or `packages/desktop`.

## Dioxus Implementation

- Use Dioxus 0.7 RSX and hooks only: `#[component] fn Name(...) -> Element`, `use_signal`, `use_memo`, `use_resource`, `Router::<Route> {}`, `Outlet<Route> {}`, and `asset!`.
- Keep component props owned, `Clone`, and `PartialEq`. Use `String`, `Vec<T>`, and `ReadOnlySignal<T>` where appropriate.
- Use loops and conditionals directly in RSX when rendering token rows, state branches, or repeated UI.
- Link local visual assets with `asset!("/assets/...")`; do not assume remote image or font access for core UI.

## Visual System

- Define or reuse CSS custom properties for color, spacing, radius, shadow, type, and motion before adding one-off declarations.
- Favor strong but restrained hierarchy: clear display styles for route titles and compact, readable body styles for token rows and controls.
- Avoid default-looking AI UI choices: generic purple-blue gradients, flat white/gray backgrounds with no hierarchy, low-contrast pastels, and arbitrary accent colors.
- Avoid over-decorating operational surfaces. This is a token-list app, so repeated cards, tables, and controls should stay compact and predictable.
- Do not make the palette one-note. Crypto tokens already provide varied imagery; surrounding UI should frame them instead of competing with them.
- Keep cards at 8px radius or less unless an existing local style requires otherwise.
- Use icons or compact symbols for repeated tool controls when available; keep text buttons for clear commands.
- Make all text fit on mobile and desktop without overlap. Use stable sizing, wrapping, and responsive constraints for rows, cards, toolbars, and status regions.

## Motion And State Feedback

- Use motion to clarify state changes: load, refresh, row entrance, toast appearance, cache-source changes, and database repopulation.
- Keep motion short and purposeful. Respect `prefers-reduced-motion`.
- Preserve visible loading and toast-style status feedback during online fetches, browser snapshot reads, cache writes, SQLite reads, errors, and database repopulation.
- Browser SQLite/OPFS startup must not block rendering when snapshot or online token data can show first.

## Backgrounds And Assets

- Prefer subtle depth: bands, borders, grids, quiet texture, and local token imagery. Avoid decorative blobs, vague stock-like backgrounds, and full-page gradients that reduce data readability.
- If a visual asset is central to the page, keep it local under the appropriate assets folder and reference it with `asset!`.
- For this app, visuals should reveal the real product state: token rows, market data, status, navigation, and developer tools.

## Verification

- For UI, routing, asset, browser SQLite, or cache-visible changes, serve the real web app and verify it in the browser when practical.
- If the web server is already running on the target port, stop it and restart it before trusting the browser result.
- Check both narrow compile behavior and runtime behavior when a change can affect both web and desktop.
- Use the smallest useful check, usually one of:

```powershell
cargo check -p ui --target wasm32-unknown-unknown
cargo check -p web --target wasm32-unknown-unknown
cargo check -p desktop
.\Scripts\Other\RunTests.ps1
.\Scripts\Common\RunWeb.ps1
.\Scripts\Common\RunDesktop.ps1
```

