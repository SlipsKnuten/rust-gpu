// A simple runner to display your learning-shader

use std::borrow::Cow;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

// This will be set by the build script to the compiled SPIR-V shader
const SHADER: &[u8] = include_bytes!(env!("learning_shader.spv"));

struct App {
    window: Option<Arc<Window>>,
    state: Option<RenderState>,
}

struct RenderState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    size: winit::dpi::PhysicalSize<u32>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = WindowAttributes::default()
                .with_title("Learning Project - My First Rust GPU Shader!")
                .with_inner_size(winit::dpi::PhysicalSize::new(800, 600));

            let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
            self.window = Some(window.clone());

            // Initialize wgpu
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                ..Default::default()
            });

            let surface = instance.create_surface(window.clone()).unwrap();

            let adapter = futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .unwrap();

            let (device, queue) = futures::executor::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::default(),
                },
            ))
            .unwrap();

            // Load the shader
            let shader_spirv = wgpu::util::make_spirv_raw(SHADER);
            let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Learning Shader"),
                source: wgpu::ShaderSource::SpirV(Cow::Borrowed(bytemuck::cast_slice(&shader_spirv))),
            });

            // Configure the surface
            let size = window.inner_size();
            let config = surface
                .get_default_config(&adapter, size.width, size.height)
                .unwrap();
            surface.configure(&device, &config);

            // Create the render pipeline
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Learning Pipeline"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: Some("main_vs"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: Some("main_fs"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            self.state = Some(RenderState {
                surface,
                device,
                queue,
                config,
                pipeline,
                size,
            });
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(state) = &mut self.state {
                    state.size = new_size;
                    state.config.width = new_size.width.max(1);
                    state.config.height = new_size.height.max(1);
                    state.surface.configure(&state.device, &state.config);
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &self.state {
                    let frame = state.surface.get_current_texture().unwrap();
                    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                    {
                        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: wgpu::StoreOp::Store,
                                },
                                depth_slice: None,
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });

                        render_pass.set_pipeline(&state.pipeline);
                        render_pass.draw(0..3, 0..1); // Draw 3 vertices (our triangle)
                    }

                    state.queue.submit(Some(encoder.finish()));
                    frame.present();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        state: None,
    };

    event_loop.run_app(&mut app).unwrap();
}
