# UI

Shared Dioxus UI, routes, models, assets, client services, and optional server-function probes for the web and desktop apps.

Put shared app behavior here when both targets should use the same UI. Browser-only behavior is guarded with `wasm32` cfgs, and native-only persistence uses local SQLite behind non-wasm cfgs.

```text
ui/
├─ assets/
│  ├─ images/
│  └─ styling/
└─ src/
   ├─ client/
   │  ├─ components/
   │  ├─ pages/
   │  └─ services/
   ├─ lib.rs
   └─ server/
      └─ services/
```
