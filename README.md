# **Pojo Engine**

## **1. Overview**

### **1.1 Purpose**
The purpose of this game engine is to provide a lightweight, modular, and extensible framework for developing 2D games. It is designed to be easy to use while offering flexibility for advanced features like scripting, asset management, and cross-platform support.

### **1.2 Target Audience**
- **Indie Game Developers**: Developers looking for a simple yet powerful engine to create 2D games.
- **Hobbyists**: Beginners who want to learn game development with a clean and well-documented codebase.
- **Educators**: Teachers and students in game development courses.

### **1.3 Key Features**
- **Rendering**: 2D sprite rendering using `wgpu`.
- **Input Handling**: Keyboard and mouse input via `winit`.
- **Audio**: Sound effects and music playback using `rodio`.
- **Scripting**: Custom scripting language for game logic.
- **Asset Management**: GUI-based asset management using `egui`.
- **Cross-Platform**: Runs on Windows, macOS, and Linux.

---

## **2. Architecture**

### **2.1 High-Level Architecture**
The engine is divided into several modules, each responsible for a specific aspect of game development:

1. **Core**:
   - Manages the game loop, window, and event handling.
   - Coordinates interactions between other modules.

2. **Graphics**:
   - Handles 2D rendering using `wgpu`.
   - Supports sprite rendering, textures, and shaders.

3. **Input**:
   - Processes keyboard and mouse input using `winit`.

4. **Audio**:
   - Manages sound effects and music playback using `rodio`.

5. **Scripting**:
   - Provides a custom scripting language for game logic.

6. **Asset Management**:
   - Loads and manages assets (e.g., textures, sounds) using a GUI powered by `egui`.

7. **Physics**:
   - Basic collision detection and movement.
   - Uses [Rapier](https://crates.io/crates/rapier2d) for 2D physics.

---

### **2.2 Module Breakdown**

#### **2.2.1 Core Module**
- **Responsibilities**:
  - Initialize the game window and event loop.
  - Manage the game loop (update, render, input).
  - Coordinate interactions between modules.
- **Dependencies**: `winit`, `wgpu`.

#### **2.2.2 Graphics Module**
- **Responsibilities**:
  - Render 2D sprites and textures.
  - Manage shaders and rendering pipelines.
- **Dependencies**: `wgpu`, `image`.

#### **2.2.3 Input Module**
- **Responsibilities**:
  - Handle keyboard and mouse input.
  - Provide an API for querying input state.
- **Dependencies**: `winit`.

#### **2.2.4 Audio Module**
- **Responsibilities**:
  - Play sound effects and music.
  - Manage audio resources.
- **Dependencies**: `rodio`.

#### **2.2.5 Scripting Module**
- **Responsibilities**:
  - Interpret custom scripts for game logic.
  - Provide an API for interacting with the engine.
- **Dependencies**: Custom interpreter.

#### **2.2.6 Asset Management Module**
- **Responsibilities**:
  - Load and manage assets (e.g., textures, sounds).
  - Provide a GUI for asset management.
- **Dependencies**: `egui`, `wgpu`.

#### **2.2.7 Physics Module**
- **Responsibilities**:
  - Handle collision detection.
  - Manage basic movement and physics.
- **Dependencies**: None (custom implementation).

---

## **3. Design Decisions**

### **3.1 Rendering**
- **Why `wgpu`?**
  - `wgpu` is a modern, cross-platform graphics API that supports Vulkan, Metal, and DirectX.
  - It provides low-level control while being easier to use than raw Vulkan or Metal.

### **3.2 Input Handling**
- **Why `winit`?**
  - `winit` is a mature and widely-used library for window and input management.
  - It supports multiple platforms and provides a consistent API.

### **3.3 Audio**
- **Why `rodio`?**
  - `rodio` is a simple and lightweight audio library for Rust.
  - It supports common audio formats and provides basic playback features.

### **3.4 Scripting**
- **Why a Custom Language?**
  - A custom language allows for tight integration with the engine.
  - It simplifies game logic scripting for non-programmers.

### **3.5 Asset Management**
- **Why `egui`?**
  - `egui` is an immediate-mode GUI library that is easy to integrate and use.
  - It provides a clean and responsive interface for managing assets.

---

## **4. Data Flow**

### **4.1 Game Loop**
1. **Input**: Poll for input events (keyboard, mouse).
2. **Update**: Update game state (e.g., player movement, collisions).
3. **Render**: Draw sprites and textures to the screen.
4. **Audio**: Play sound effects and music.

### **4.2 Asset Pipeline**
1. **Load**: Load assets (e.g., textures, sounds) from disk.
2. **Process**: Convert assets into engine-specific formats (e.g., textures into GPU-compatible formats).
3. **Manage**: Store assets in a central repository for easy access.

---

## **5. Appendix**

### **5.1 Dependencies**
- `winit`: Window and input management.
- `wgpu`: Graphics rendering.
- `rodio`: Audio playback.
- `egui`: GUI for asset management.
- `image`: Image loading and processing.

### **5.2 References**
- [WGPU Documentation](https://docs.rs/wgpu)
- [Winit Documentation](https://docs.rs/winit)
- [Rodio Documentation](https://docs.rs/rodio)
- [Egui Documentation](https://docs.rs/egui)
- [Rapier Documentation](https://crates.io/crates/rapier2d)
