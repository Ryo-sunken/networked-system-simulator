mod app;
mod state;
mod texture;

use crate::app::Application;

fn main() -> Result<(), winit::error::EventLoopError> {
    Application::new().run()
}
