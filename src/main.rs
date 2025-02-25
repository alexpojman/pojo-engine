mod audio;
mod graphics;
mod gui;
mod input;
mod physics;
mod renderer;

use renderer::window::WindowedApp;

fn main() {
    env_logger::init();
    WindowedApp::new().run();
}
