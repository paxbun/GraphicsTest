#[cfg(target_os = "ios")]
#[macro_use]
extern crate objc;

mod native;

use std::mem;
use std::sync::{Arc, Mutex};

use futures::executor::block_on;
use wgpu::util::*;
use wgpu::*;

use crate::native::NativeHandle;

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_MyNativeLibrary_nativeWindowFromSurface(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    surface: jni::objects::JObject,
) -> i64 {
    unsafe { ndk_sys::ANativeWindow_fromSurface(env.get_raw(), *surface) as usize as _ }
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_RustNativeViewLib_nativeViewAsNativeHandle(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    surface: jni::objects::JObject,
) -> i64 {
    let Some(native_handle) = NativeHandle::new(env, surface) else {
        return 0;
    };
    Arc::into_raw(Arc::new(native_handle)) as usize as i64
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn RustNativeViewLib_nativeViewAsNativeHandle(
    ui_view: Option<std::ptr::NonNull<objc::runtime::Object>>,
) -> i64 {
    let Some(ui_view) = ui_view else {
        return 0;
    };
    Arc::into_raw(Arc::new(NativeHandle::new(ui_view))) as usize as i64
}

#[derive(uniffi::Object, Debug)]
pub struct RustNativeViewContext {
    surface: Surface<'static>,
    surface_config: Mutex<SurfaceConfiguration>,
    device: Device,
    queue: Queue,
    vertex_buffer: Buffer,
    pipeline: RenderPipeline,
    _density: f32,
}

#[uniffi::export]
impl RustNativeViewContext {
    #[uniffi::constructor]
    fn new(native_handle: i64, density: f32) -> Arc<RustNativeViewContext> {
        let native_handle = unsafe { Arc::from_raw(native_handle as usize as *mut NativeHandle) };
        let instance = Instance::new(InstanceDescriptor::default());
        let surface = instance
            .create_surface(Arc::clone(&native_handle))
            .expect("could not create surface");
        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("could not retrieve a adapter");
        let (device, queue) = block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::default(),
                required_limits: Limits::default(),
            },
            None,
        ))
        .expect("could not retrieve a device");
        let (width, height) = native_handle.size();
        log::debug!("width: {width}, height: {height}");
        let surface_config = surface
            .get_default_config(&adapter, width, height)
            .expect("could not retrieve the surface configuration");
        surface.configure(&device, &surface_config);
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let shader_module = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vertex",
                buffers: &[VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        offset: 0,
                        format: wgpu::VertexFormat::Float32x2,
                        shader_location: 0,
                    }],
                }],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                front_face: FrontFace::Ccw,
                strip_index_format: None,
                cull_mode: None,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fragment",
                targets: &[Some(ColorTargetState {
                    format: surface_config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::all(),
                })],
            }),
            multiview: None,
        });
        Arc::new(RustNativeViewContext {
            surface,
            surface_config: Mutex::new(surface_config),
            device,
            queue,
            vertex_buffer,
            pipeline,
            _density: density,
        })
    }

    pub fn change_size(self: Arc<Self>, width: i32, height: i32, scale: f32) {
        let _ = scale;
        let (Ok(width), Ok(height)) = (width.try_into(), height.try_into()) else {
            return;
        };

        let mut surface_config = self.surface_config.lock().unwrap();
        if surface_config.width == width || surface_config.height == height {
            return;
        }

        surface_config.width = width;
        surface_config.height = height;
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn render(self: Arc<Self>) {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                log::debug!("b");
                log::error!("Swap-chain error: {e:?}");
                return;
            }
        };
        let frame_view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::GREEN),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..VERTICES.len() as u32, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

#[ctor::ctor]
fn setup_logger() {
    use log::LevelFilter;

    let level_filter = if cfg!(debug_assertions) {
        LevelFilter::Trace
    } else {
        LevelFilter::Warn
    };

    #[cfg(target_os = "android")]
    {
        android_logger::init_once(android_logger::Config::default().with_max_level(level_filter));
    }
    #[cfg(not(target_os = "android"))]
    {
        env_logger::Builder::from_default_env()
            .filter_level(level_filter)
            .init();
    }
}

static VERTICES: &[Vertex] = &[
    Vertex { x: 0.1, y: 0.1 },
    Vertex { x: 0.1, y: 0.9 },
    Vertex { x: 0.9, y: 0.9 },
];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Vertex {
    x: f32,
    y: f32,
}

unsafe impl bytemuck::Pod for Vertex {}

unsafe impl bytemuck::Zeroable for Vertex {}

uniffi::setup_scaffolding!();
