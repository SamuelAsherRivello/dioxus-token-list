# UI

Shared Dioxus components for the web and desktop apps.

Put components here when both targets should use the same UI. Keep platform-specific dependencies out of this crate; add those to `packages/web` or `packages/desktop` instead.

```text
ui/
├─ assets/
│  ├─ images/
│  └─ styling/
└─ src/
   ├─ body.rs
   ├─ lib.rs
   └─ navbar.rs
```
