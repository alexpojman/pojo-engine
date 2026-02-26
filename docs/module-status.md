# Module Status Matrix

_Last updated: 2026-02-26_

## Core files

| Path | Status | Notes |
|---|---|---|
| `src/main.rs` | ✅ Implemented | Initializes logger and starts async renderer window flow. |
| `src/renderer/mod.rs` | ✅ Implemented | Exposes `window` module. |
| `src/renderer/window.rs` | ✅ Implemented | Winit event loop + redraw + resize + exit handling. |

## Graphics

| Path | Status | Notes |
|---|---|---|
| `src/graphics/mod.rs` | ✅ Implemented | Exposes `state` and `vertex`. |
| `src/graphics/state.rs` | ✅ Implemented | WGPU init, pipeline creation, vertex buffer, frame render. |
| `src/graphics/vertex.rs` | ✅ Implemented | Vertex struct/layout + triangle vertices + unit tests. |
| `src/graphics/shaders/shader.wgsl` | ✅ Implemented | Basic color pass-through shader pair. |

## Audio

| Path | Status | Notes |
|---|---|---|
| `src/audio/mod.rs` | ⚠️ Disabled export | `pub mod manager;` commented out. |
| `src/audio/manager.rs` | 🟡 Partial | Has `AudioManager` and playback call, not integrated. |

## GUI

| Path | Status | Notes |
|---|---|---|
| `src/gui/mod.rs` | ⚠️ Disabled export | `pub mod manager;` commented out. |
| `src/gui/manager.rs` | 🟡 Partial | Minimal egui window/button, not integrated. |

## Input + Physics

| Path | Status | Notes |
|---|---|---|
| `src/input/mod.rs` | ❌ Stub | Empty file. |
| `src/physics/mod.rs` | ❌ Stub | Empty file. |

## CI / Automation

| Path | Status | Notes |
|---|---|---|
| `.github/workflows/CI.yml` | 🟡 Active | Builds/tests; includes experimental auto-commit behavior in `update-formatting`. |

## Dependency usage reality check

| Dependency | In `Cargo.toml` | Actually used in runtime path? |
|---|---|---|
| `wgpu` | ✅ | ✅ |
| `winit` | ✅ | ✅ |
| `pollster` | ✅ | ✅ |
| `env_logger` | ✅ | ✅ |
| `log` | ✅ | ✅ |
| `bytemuck` | ✅ | ✅ |
| `rodio` | ✅ | ⚠️ only in unused audio manager |
| `egui` | ✅ | ⚠️ only in unused GUI manager |
| `rapier2d` | ✅ | ❌ not used yet |

## Overall completion estimate

- **Engine runtime skeleton:** ~35%
- **Renderable demo path:** ~70%
- **Feature set described in README:** ~15%

(Estimate based on implemented code paths, not planned architecture.)
