struct Sprites {
    pipeline: wgpu::render_pipeline,
    groups: (wgpu::Texture, wgpu::buffer, Vec<GPUSprite>)
}

impl sprites {
    pub fn set_sprite_position(&mut self, group_id: usize, sprite_num:usize, pos: Vec2) {
        self.groups[group_id].2[sprite_num]
    }
    
    pub fn new(gpi: &GPUState) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            // Cow is a "copy on write" wrapper that abstracts over owned or borrowed memory.
            // Here we just need to use it since wgpu wants "some text" to compile a shader from.
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let sprite_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&sprite_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Our specific "function" is going to be a draw call using our shaders. That's what we
        // set up here, calling the result a render pipeline.  It's not only what shaders to use,
        // but also how to interpret streams of vertices (e.g. as separate triangles or as a list of lines),
        // whether to draw both the fronts and backs of triangles, and how many times to run the pipeline for
        // things like multisampling antialiasing.
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
    }

    pub fn render(
        gpu: &GPUState,
        camera: GPUCamera,
        sprite_tex: wgpu::Texture,
        sprite_buf: &[GPUSprite],
    ) {
    }
}
