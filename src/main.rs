use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct App {
    window: Window,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();

        Self { window }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> Option<ControlFlow> {
        match event {
            WindowEvent::CloseRequested => Some(ControlFlow::Exit),
            _ => None,
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let mut app = App::new(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => {
                if let Some(cf) = app.handle_event(&event) {
                    *control_flow = cf;
                }
            }
            Event::MainEventsCleared => {
                // update
            }
            Event::RedrawRequested(_) => {
                // redraw
            }
            _ => (),
        }
    });
}
