pub mod renderer;
pub mod serialization;
pub mod world;

use crate::renderer::Renderer;
use crate::world::{BlockPos3, World};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub struct App {
    renderer: Renderer,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let mut world = World::open(std::env::args().nth(1).unwrap()).unwrap();
        let block = world.get_block(BlockPos3::new(0, 0, 0));
        let window = WindowBuilder::new().build(event_loop).unwrap();
        let renderer = Renderer::new(window);

        Self { renderer }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> Option<ControlFlow> {
        match event {
            WindowEvent::CloseRequested => return Some(ControlFlow::Exit),
            WindowEvent::Resized(size) => self.renderer.resize(*size),
            _ => {}
        }

        None
    }

    pub fn render(&mut self) {
        self.renderer.render();
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
                app.render();
            }
            _ => (),
        }
    });
}
