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
