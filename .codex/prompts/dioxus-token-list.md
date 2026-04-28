# dioxus-token-list

Use this prompt as a slash-command draft for work in this repository.

## Prompt

Work in `D:\Documents\Projects\VC\Rust\dioxus-token-list`.

Follow `AGENTS.md` and use Dioxus 0.7.1 APIs only. Inspect the current code before changing files. Keep changes scoped to the requested behavior and preserve the workspace split between `packages/ui`, `packages/web`, and `packages/desktop`.

When making UI, routing, asset, cache, or loading changes:

- Prefer the existing component and service structure.
- Preserve browser and desktop support unless the request is explicitly platform-specific.
- Use visible loading/status feedback for cache refreshes or repopulation.
- Run the relevant script or Cargo/Dioxus check.
- For browser-visible behavior, serve the web app and verify the actual page when practical.

Useful commands:

```powershell
.\Scripts\Common\InstallDependencies.ps1
.\Scripts\Common\RunWeb.ps1
.\Scripts\Common\RunDesktop.ps1
.\Scripts\Other\RunTests.ps1
dx serve --platform web --addr 0.0.0.0 --port 8080
```
