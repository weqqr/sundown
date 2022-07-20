use pollster::FutureExt;
use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Renderer {
    window: Window,

    instance: Instance,
    surface: Surface,
    adapter: Adapter,
    preferred_surface_format: TextureFormat,
    device: Device,
    queue: Queue,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .unwrap();

        let preferred_surface_format = surface.get_supported_formats(&adapter)[0];

        let features = Default::default();
        let limits = Default::default();
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features,
                    limits,
                },
                None,
            )
            .block_on()
            .unwrap();

        let mut renderer = Self {
            window,
            instance,
            surface,
            adapter,
            preferred_surface_format,
            device,
            queue,
        };

        renderer.resize(renderer.window.inner_size());

        renderer
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface.configure(
            &self.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: self.preferred_surface_format,
                width: size.width,
                height: size.height,
                present_mode: PresentMode::Fifo,
            },
        )
    }

    pub fn render(&mut self) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let surface_texture_view = surface_texture.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let _rp = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

        }

        self.queue.submit([encoder.finish()]);

        surface_texture.present();
    }
}
