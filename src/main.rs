mod app;

use crate::app::Application;

fn main() -> Result<(), winit::error::EventLoopError> {
    Application::new().run()
}
