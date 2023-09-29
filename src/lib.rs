use core::ops::Range;
use image::{buffer, Pixel};
use imageproc::drawing::{Blend, Canvas};
use std::borrow::Cow;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
    window::Window,
};
mod gpu;
mod input;
mod sprite;
use sprite::SpriteRender;
pub use sprite::{GPUCamera, GPUSprite};

pub use gpu::WGPU;
mod engine;
pub use engine::Engine;

#[async_trait::async_trait]
pub trait Game {
    async fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);
}
// In WGPU, we define an async function whose operation can be suspended and resumed.
// This is because on web, we can't take over the main event loop and must leave it to
// the browser.  On desktop, we'll just be running this function to completion.
// async fn run(event_loop: EventLoop<()>, window: Window) {
//     // Load the shaders from disk.  Remember, shader programs are things we compile for
//     // our GPU so that it can compute vertices and colorize fragments.

//     // let mut sprites: Vec<GPUSprite> = vec![
//     //     //It's the 2 different sprites for king.png at 2 different locations
//     //     GPUSprite {
//     //         screen_region: [32.0, 32.0, 64.0, 64.0],
//     //         sheet_region: [0.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
//     //     },
//     //     GPUSprite {
//     //         screen_region: [32.0, 128.0, 64.0, 64.0],
//     //         sheet_region: [16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
//     //     },
//     //     GPUSprite {
//     //         screen_region: [128.0, 32.0, 64.0, 64.0],
//     //         sheet_region: [0.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
//     //     },
//     //     GPUSprite {
//     //         screen_region: [128.0, 128.0, 64.0, 64.0],
//     //         sheet_region: [16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
//     //     },
//     // ];

//     use std::path::Path;
//     let img = image::open(Path::new(
//         "/Users/calebtheperson/RustProjects/sprites/src/king.png",
//     ))
//     .expect("Bruh where ur picture'");
//     let img = img.to_rgba8();
//     let (img_w, img_h) = img.dimensions();
//     // How big is the texture in GPU memory?
//     let size = wgpu::Extent3d {
//         width: img_w,
//         height: img_h,
//         depth_or_array_layers: 1,
//     };

//     let texture = wgpu.device.create_texture(
//         // Parameters for the texture
//         &wgpu::TextureDescriptor {
//             // An optional label
//             label: Some("King image"),
//             // Its dimensions. This line is equivalent to size:size
//             size,
//             // Number of mipmapping levels (to show different pictures at different distances)
//             mip_level_count: 1,
//             // Number of samples per pixel in the texture. It'll be one for our whole class.
//             sample_count: 1,
//             // Is it a 1D, 2D, or 3D texture?
//             dimension: gpu::TextureDimension::D2,
//             // 8 bits per component, four components per pixel, unsigned, normalized in 0..255, SRGB
//             format: gpu::TextureFormat::Rgba8UnormSrgb,
//             // This texture will be bound for shaders and have stuff copied to it
//             usage: gpu::TextureUsages::TEXTURE_BINDING | gpu::TextureUsages::COPY_DST,
//             // What formats are allowed as views on this texture besides the native format
//             view_formats: &[],
//         },
//     );

//     // Now that we have a texture, we need to copy its data to the GPU:
//     gpu.queue.write_texture(
//         // A description of where to write the image data.
//         // We'll use this helper to say "the whole texture"
//         texture.as_image_copy(),
//         // The image data to write
//         &img,
//         // What portion of the image data to copy from the CPU
//         gpu::ImageDataLayout {
//             // Where in img do the bytes to copy start?
//             offset: 0,
//             // How many bytes in each row of the image?
//             bytes_per_row: Some(4 * img_w),
//             // We could pass None here and it would be alright,
//             // since we're only uploading one image
//             rows_per_image: Some(img_h),
//         },
//         // What portion of the texture we're writing into
//         size,
//     );

//     // AsRef means we can take as parameters anything that cheaply converts into a Path,

//     //Creating Background texture
//     let img = image::open(Path::new(
//         "/Users/calebtheperson/RustProjects/triangle/src/47.png",
//     ))
//     .expect("Bruh where ur picture'");
//     let img = img.to_rgba8();
//     let (img_w, img_h) = img.dimensions();
//     // How big is the texture in GPU memory?
//     let size = wgpu::Extent3d {
//         width: img_w,
//         height: img_h,
//         depth_or_array_layers: 1,
//     };

//     // Now our setup is all done and we can kick off the windowing event loop.
//     // This closure is a "move closure" that claims ownership over variables used within its scope.
//     // It is called once per iteration of the event loop.

//     //CPU Side stuff

//     // let mut background: Vec<GPUSprite> = vec![GPUSprite {
//     //     screen_region: [0.0, 0.0, 1024.0, 768.0],
//     //     sheet_region: [0.0, 0.0, 1.0, 1.0],
//     // }];

//     // let buffer_background = gpu.device.create_buffer(&wgpu::BufferDescriptor {
//     //     label: None,
//     //     size: bytemuck::cast_slice::<_, u8>(&background).len() as u64,
//     //     usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
//     //     mapped_at_creation: false,
//     // });

//     let mut input = input::Input::default();
// }

// Main is just going to configure an event loop, open a window, set up logging,
// and kick off our `run` function.


