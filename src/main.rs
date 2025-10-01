mod audio;
mod graphics;
mod gui;
mod input;
mod physics;
mod renderer;

fn main() {
    env_logger::init();

    let _ = pollster::block_on(renderer::window::create());
    println!("Exiting...");
}
