mod app;
mod state;

use crate::app::Application;

fn main() -> Result<(), winit::error::EventLoopError> {
    Application::new().run()
}
