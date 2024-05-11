use crate::state::State;
use winit::{
    error::EventLoopError,
    event::*,
    event_loop::{EventLoop, EventLoopBuilder},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

// Application is a interface between event_loop and state
pub struct Application {
    rt: tokio::runtime::Runtime,
    event_loop: EventLoop<()>,
    window: Window,
    state: State,
}

impl Application {
    pub fn new() -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let event_loop = EventLoopBuilder::new().build().unwrap();
        let window = Window::new(&event_loop).unwrap();
        let state = rt.block_on(State::new(&window));

        Self {
            rt,
            event_loop,
            window,
            state,
        }
    }

    pub fn run(self) -> Result<(), EventLoopError> {
        self.event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { window_id, event } => {
                if window_id == self.window.id() {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => elwt.exit(),
                        WindowEvent::RedrawRequested => self.state.render(),
                        _ => (),
                    }
                }
            }
            Event::AboutToWait => {
                self.window.request_redraw();
            }
            _ => (),
        })
    }
}
