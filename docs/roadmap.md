# Pojo Engine — 30/60/90 Day Roadmap

_Last updated: 2026-02-26_

This roadmap assumes the project is still in an early prototype stage and optimizes for:
1) fast iteration,
2) a stable developer loop,
3) incremental architecture (avoid over-engineering too early).

---

## Guiding principles

- **Ship thin vertical slices** (input → update → render) instead of broad unfinished modules.
- **Keep core APIs small** until real use-cases force expansion.
- **Prefer compile-time confidence** (tests + CI checks) before adding major features.
- **Document reality, not aspiration** (keep README aligned with code).

---

## North Star (90-day target)

By day 90, the engine should support a **small playable 2D sample** with:
- sprite rendering (textured quads),
- deterministic update loop,
- keyboard input,
- basic audio playback,
- simple physics/collision,
- minimal asset loading,
- stable CI + quality gates.

---

## Day 0–30: Foundation hardening

### Goals
- Make the existing render prototype robust.
- Establish project hygiene and reliable CI.
- Define minimal core interfaces.

### Deliverables

#### 1) Project hygiene and CI stabilization
- Remove experimental CI auto-commit job behavior (or isolate into separate workflow).
- Add CI steps:
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings`
  - `cargo test`
- Add `CONTRIBUTING.md` with local setup and command checklist.

**Done when:** PRs fail on formatting/lints/tests; no workflow mutates repo unexpectedly.

#### 2) Error handling and startup resilience
- Replace critical `unwrap()` calls in renderer/window/audio setup with structured errors.
- Add user-friendly startup failure logs (GPU not available, audio init failure, etc.).

**Done when:** app exits cleanly with actionable messages on startup failures.

#### 3) Define minimal engine loop contract
Create basic trait(s), e.g.:
- `GameApp::init(...)`
- `GameApp::update(dt)`
- `GameApp::render(...)`
- `GameApp::on_event(...)`

This can be thin wrappers around existing `State` and `winit` handling.

**Done when:** sample app logic is not hard-coded directly in window loop internals.

#### 4) Input module v1
- Implement keyboard state tracking (`pressed`, `just_pressed`, `just_released`).
- Feed `winit` events into input manager.
- Add tests for state transitions.

**Done when:** sample app can query input from update loop.

### Suggested milestones for first month
- Week 1: CI cleanup + lint/format gates
- Week 2: renderer/window error handling cleanup
- Week 3: input module v1 + tests
- Week 4: minimal `GameApp` abstraction + README update

---

## Day 31–60: First gameplay-capable slice

### Goals
- Move from triangle demo to sprite-based rendering.
- Add baseline systems required for a tiny playable demo.

### Deliverables

#### 1) Graphics v2 (textured 2D)
- Add index buffer + quad mesh.
- Add texture loading and sampling.
- Add orthographic camera/uniforms.
- Render at least one moving textured sprite.

**Done when:** app renders textured sprite(s) with stable resize behavior.

#### 2) Asset loading v1
- Create lightweight asset module for textures/audio by path.
- Basic caching (avoid duplicate loads).
- Clear error reporting for missing/corrupt assets.

**Done when:** demo loads assets from `assets/` via one API.

#### 3) Audio integration v1
- Re-enable and integrate `audio::manager` into runtime.
- Implement simple SFX playback API and one background track path.

**Done when:** input action can trigger SFX; startup can optionally begin BGM.

#### 4) Physics/collision v1
- Introduce minimal AABB collision and velocity integration **or** wire lightweight Rapier usage for basics.
- Keep scope minimal: 2D dynamic body + static bounds.

**Done when:** sprite movement + collisions are visibly correct in demo.

### Suggested milestones for second month
- Week 5–6: textured rendering pipeline
- Week 7: assets + audio integration
- Week 8: simple collision + movement demo

---

## Day 61–90: Demo readiness + architecture consolidation

### Goals
- Produce a small playable sample project.
- Refine module boundaries based on real usage.
- Improve maintainability and developer experience.

### Deliverables

#### 1) Playable sample
- Build `examples/basic-game` or `examples/topdown-demo`.
- Include:
  - player movement,
  - collision with world bounds/obstacles,
  - one interactive element,
  - audio feedback.

**Done when:** new contributors can run one command and play the demo.

#### 2) ECS-like or scene organization (lightweight)
- Do not jump to full ECS unless needed.
- Introduce simple entity/component structs and scene update order.

**Done when:** demo logic is data-driven enough to add entities without duplicating boilerplate.

#### 3) Tooling and docs maturity
- Add architecture diagram in docs.
- Add module-level docs (`///`) for public APIs.
- Add a “current status vs roadmap” section in README.

**Done when:** docs match implementation and onboarding takes <15 minutes.

#### 4) Test strategy v2
- Unit tests for input, math, and core utilities.
- One smoke/integration test for core engine initialization (headless where possible).

**Done when:** regressions in loop/input/render setup are caught before merge.

---

## Backlog (defer until post-90 unless required)

- Full scripting language/runtime
- In-engine GUI tooling/editor flows
- Complex physics feature set
- Advanced renderer features (batching, post-processing, lighting)
- Multi-platform packaging/distribution automation

---

## Suggested issue labels and planning lanes

Use labels to keep early-stage planning clean:
- `foundation`
- `rendering`
- `input`
- `audio`
- `physics`
- `assets`
- `docs`
- `ci`
- `good-first-issue`

And track by 3 lanes:
- **Now** (active sprint)
- **Next** (next 2 weeks)
- **Later** (post current milestone)

---

## Concrete “start this week” task list

1. Remove/disable CI auto-commit mutation job.
2. Add fmt + clippy gates to CI.
3. Refactor `unwrap()` hot spots in `renderer/window.rs` and `graphics/state.rs`.
4. Implement `input` state manager and wire it into event loop.
5. Update README to separate:
   - implemented features,
   - in-progress,
   - planned roadmap.

If those five are done, the project will have a much stronger base for month 2 feature work.
