# Pojo Engine — Current State

_Last updated: 2026-02-26_

## Snapshot

This repository is an early-stage Rust game engine prototype centered around a working `wgpu` render loop.

- **Project type:** Binary crate (`game_engine`)
- **Entry point:** `src/main.rs`
- **Current runtime behavior:** opens a window, renders a colored triangle, handles resize/redraw/exit events.
- **Maturity:** foundational scaffolding with one implemented vertical slice (renderer/graphics).

## What is implemented

### 1) App bootstrap and event loop
- `src/main.rs`
  - Initializes logging (`env_logger::init()`)
  - Runs async window creation with `pollster::block_on(renderer::window::create())`
- `src/renderer/window.rs`
  - Creates `winit` event loop and window
  - Runs event loop in `ControlFlow::Poll`
  - Handles:
    - close window
    - `Esc` to exit
    - resize (`state.resize`)
    - redraw (`state.update` + `state.render`)
  - Handles common `wgpu::SurfaceError` cases

### 2) Graphics pipeline
- `src/graphics/state.rs`
  - Initializes `wgpu` instance/surface/adapter/device/queue
  - Chooses surface format and configures swapchain
  - Creates shader module from `src/graphics/shaders/shader.wgsl`
  - Creates render pipeline (no bind groups yet)
  - Uploads static vertex buffer from `VERTICES`
  - Renders one triangle each frame
- `src/graphics/vertex.rs`
  - `Vertex` POD layout (`position`, `color`)
  - Vertex buffer layout descriptor
  - Static triangle vertex data
  - Unit tests for memory layout + vertex data
- `src/graphics/shaders/shader.wgsl`
  - Minimal vertex/fragment shaders
  - Pass-through position and color

### 3) CI pipeline
- `.github/workflows/CI.yml`
  - Build and test jobs on push/PR to `main`
  - Installs Linux audio build deps (`libasound2-dev`, `pkg-config`)
  - Uses cache for cargo registry/git + `target`
  - Includes separate `update-formatting` job that commits `test-final-con-2.json` on non-PR events

## What is present but not wired into runtime

- `src/audio/manager.rs`
  - `AudioManager` built on `rodio` with `play_audio`
  - Module export currently commented out in `src/audio/mod.rs`
- `src/gui/manager.rs`
  - Minimal `egui` window with one button
  - Module export currently commented out in `src/gui/mod.rs`
- `src/input/mod.rs`
  - Empty module file
- `src/physics/mod.rs`
  - Empty module file

## Build and test status (local environment)

Observed when running `cargo test` in this environment:
- ❌ Fails during `alsa-sys` build (missing system ALSA development package / `alsa.pc` in this runtime)
- Note: CI workflow explicitly installs ALSA deps, so CI may pass where local sandbox fails.

## Documentation vs actual code

The root `README.md` describes a broad engine architecture (rendering, scripting, asset management, etc.).
Current codebase does **not** yet implement most of those areas. Current reality is:

- Implemented: window/event loop + basic rendering triangle + vertex tests
- Scaffolding only: audio/gui/input/physics modules
- Not present yet: scripting system, asset pipeline, scene/entity systems, resource management, gameplay framework

## Risks / technical debt

1. **Runtime errors handled via `unwrap()`**
   - Device/surface/audio setup includes hard unwraps.
2. **No separation of engine core abstractions yet**
   - Rendering state is monolithic in `State`.
3. **No integration tests / smoke test harness**
   - Only vertex layout unit tests currently.
4. **CI workflow oddity**
   - `update-formatting` job mutates repository state by committing a test JSON file.
   - Commit message/file mismatch suggests this job may be a temporary experiment.

## Suggested next milestones

1. **Stabilize project hygiene**
   - Remove/replace experimental CI auto-commit behavior
   - Add `cargo fmt --check`, `clippy`, and docs lint steps
2. **Wire module boundaries**
   - Re-enable `audio` and `gui` module exports when integration is ready
   - Add minimal input API in `src/input/mod.rs`
3. **Add render extensibility**
   - Uniforms/camera, index buffer, texture pipeline
4. **Add a small engine loop contract**
   - `init/update/render` trait(s) for game/app layer
5. **Align README with implementation status**
   - Mark roadmap vs implemented features clearly
