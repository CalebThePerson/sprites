use crate::{gpu, WGPU};
use bytemuck::bytes_of;
use core::ops::Range;
use std::borrow::Cow;
use wgpu;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct GPUSprite {
    pub screen_region: [f32; 4], // This is the area of the screen the sprite should take up, like a collision box
    // Textures with a bunch of sprites are often called "sprite sheets"
    pub sheet_region: [f32; 4], // Which part of the sheet to look at for the sprite ??
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct GPUCamera {
    pub screen_pos: [f32; 2],  // Position of the camera
    pub screen_size: [f32; 2], // The size of our screen???
}

pub struct SpriteRender {
    pipeline: wgpu::RenderPipeline,
    groups: Vec<SpriteGroup>,
    sprite_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}
impl SpriteRender {
    pub fn new(wgpu: &WGPU) -> Self {
        let shader = wgpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                // Cow is a "copy on write" wrapper that abstracts over owned or borrowed memory.
                // Here we just need to use it since wgpu wants "some text" to compile a shader from.
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            });
        let texture_bind_group_layout =
            wgpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    // This bind group's first entry is for the texture and the second is for the sampler.
                    entries: &[
                        // The texture binding
                        wgpu::BindGroupLayoutEntry {
                            // This matches the binding number in the shader
                            binding: 0,
                            // Only available in the fragment shader
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            // It's a texture binding
                            ty: wgpu::BindingType::Texture {
                                // We can use it with float samplers
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                // It's being used as a 2D texture
                                view_dimension: wgpu::TextureViewDimension::D2,
                                // This is not a multisampled texture
                                multisampled: false,
                            },
                            // This is not an array texture, so it has None for count
                            count: None,
                        },
                        // The sampler binding
                        wgpu::BindGroupLayoutEntry {
                            // This matches the binding number in the shader
                            binding: 1,
                            // Only available in the fragment shader
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            // It's a sampler
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            // No count
                            count: None,
                        },
                    ],
                });

        // Our specific "function" is going to be a draw call using our shaders. That's what we
        // set up here, calling the result a render pipeline.  It's not only what shaders to use,
        // but also how to interpret streams of vertices (e.g. as separate triangles or as a list of lines),
        // whether to draw both the fronts and backs of triangles, and how many times to run the pipeline for
        // things like multisampling antialiasing.

        let sprite_bind_group_layout =
            wgpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        // The camera binding
                        wgpu::BindGroupLayoutEntry {
                            // This matches the binding in the shader
                            binding: 0,
                            // Available in vertex shader
                            visibility: wgpu::ShaderStages::VERTEX,
                            // It's a buffer
                            ty: wgpu::BindingType::Buffer {
                                // Specifically, a uniform buffer
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            // No count, not a buffer array binding
                            count: None,
                        },
                        // The sprite buffer binding
                        wgpu::BindGroupLayoutEntry {
                            // This matches the binding in the shader
                            binding: 1,
                            // Available in vertex shader
                            visibility: wgpu::ShaderStages::VERTEX,
                            // It's a buffer
                            ty: wgpu::BindingType::Buffer {
                                // Specifically, a storage buffer
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            // No count, not a buffer array binding
                            count: None,
                        },
                    ],
                });

        // A graphics pipeline is sort of like the conventions for a function call: it defines
        // the shapes of arguments (bind groups and push constants) that will be used for
        // draw calls.
        // Now we'll create our pipeline layout, specifying the shape of the execution environment (the bind group)
        let pipeline_layout = wgpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&sprite_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = wgpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu.config.format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });
        //Converting that CPU stuff to GPU stuff

        Self {
            pipeline,
            groups: Vec::default(),
            sprite_bind_group_layout,
            texture_bind_group_layout,
        }
    }
    pub fn add_sprite_group(
        &mut self,
        gpu: &WGPU,
        tex: &wgpu::Texture,
        sprites: Vec<GPUSprite>,
        camera: GPUCamera,
    ) {
        let view_kingtex_king = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler_kingtex_king = gpu
            .device
            .create_sampler(&wgpu::SamplerDescriptor::default());
        let tex_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.texture_bind_group_layout,
            entries: &[
                // One for the texture, one for the sampler
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view_kingtex_king),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler_kingtex_king),
                },
            ],
        });

        let buffer_sprite = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: bytemuck::cast_slice::<_, u8>(&sprites).len() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer_camera = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<GPUCamera>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let sprite_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.sprite_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer_camera.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffer_sprite.as_entire_binding(),
                },
            ],
        });
        gpu.queue
            .write_buffer(&buffer_sprite, 0, bytemuck::cast_slice(&sprites));

        gpu.queue
            .write_buffer(&buffer_camera, 0, bytemuck::bytes_of(&camera));
        self.groups.push(SpriteGroup {
            sprite_buffer: buffer_sprite,
            sprites,
            tex_bind_group,
            sprite_bind_group,
            camera,
            buffer_camera,
        });

        // self.groups.len() - 1
    }

    pub fn print_group(&self, sprite: usize) {}
    pub fn set_camera(&mut self, gpu: &WGPU, index: usize, camera: GPUCamera) {
        let sg = &mut self.groups[index];
        sg.camera = camera;

        gpu.queue
            .write_buffer(&sg.buffer_camera, 0, bytemuck::bytes_of(&sg.camera));
    }
    pub fn set_camera_all(&mut self, gpu: &WGPU, camera: GPUCamera) {
        for sg_index in 0..self.groups.len() {
            self.set_camera(gpu, sg_index, camera);
        }
    }

    pub fn refresh_sprites(&mut self, gpu: &WGPU, which: usize, range: Range<usize>) {
        gpu.queue.write_buffer(
            &self.groups[which].sprite_buffer,
            range.start as u64,
            bytemuck::cast_slice(&self.groups[which].sprites[range]),
        )
    }

    pub fn get_sprite_mut(&mut self, which: usize, range: usize) -> &mut GPUSprite {
        &mut self.groups[which].sprites[range]
    }
    pub fn get_sprites(&self, which: usize) -> &[GPUSprite] {
        &self.groups[which].sprites
    }
    pub fn get_all_sprites_mut(&mut self, which: usize) -> &mut [GPUSprite] {
        &mut self.groups[which].sprites
    }
    pub fn group_size(&self, which: usize) -> &[GPUSprite] {
        &self.groups[which].sprites
    }

    pub fn render<'s, 'pass>(&'s self, rpass: &mut wgpu::RenderPass<'pass>)
    where
        's: 'pass,
    {
        rpass.set_pipeline(&self.pipeline);
        for group in self.groups.iter() {
            // rpass.set_vertex_buffer(0, group.sprite_buffer.slice(0..10));
            //maybe take out of loop idk

            rpass.set_bind_group(0, &group.sprite_bind_group, &[]);
            rpass.set_bind_group(1, &group.tex_bind_group, &[]);
            rpass.draw(0..6, 0..(group.sprites.len() as u32));
        }
    }

    pub fn update_position(&mut self, newRegion: [f32; 4], sprite: usize) {
        let theSprite = self.get_sprite_mut(sprite, 0);
        theSprite.screen_region = newRegion;
    }

    //Trying to make moving platforms that move back and foth
    pub fn platform_move(&mut self) {
        let allPlatforms = self.get_all_sprites_mut(2);
        for platform in allPlatforms.iter_mut() {
            platform.sheet_region[0] = platform.sheet_region[0] + 32.0;
        }
    }
}

pub struct SpriteGroup {
    sprite_buffer: wgpu::Buffer,
    sprites: Vec<GPUSprite>,
    tex_bind_group: wgpu::BindGroup,
    sprite_bind_group: wgpu::BindGroup,
    camera: GPUCamera,
    buffer_camera: wgpu::Buffer,
}
