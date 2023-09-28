struct GPUState {
    instance : wgpu::Instance,
    surface: wgpu::Surface,
    size: PhysicalSize<u32>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration
}

impl GPUState {
    fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // An Instance is an instance of the graphics API.  It's the context in which other
        // WGPU values and operations take place, and there can be only one.
        // Its implementation of the Default trait automatically selects a driver backend.
        let instance = wgpu::Instance::default();
    
        // From the OS window (or web canvas) the graphics API can obtain a surface onto which
        // we can draw.  This operation is unsafe (it depends on the window not outliving the surface)
        // and it could fail (if the window can't provide a rendering destination).
        // The unsafe {} block allows us to call unsafe functions, and the unwrap will abort the program
        // if the operation fails.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
    
        // Next, we need to get a graphics adapter from the instance---this represents a physical
        // graphics card (GPU) or compute device.  Here we ask for a GPU that will be able to draw to the
        // surface we just obtained.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            // This operation can take some time, so we await the result. We can only await like this
            // in an async function.
            .await
            // And it can fail, so we panic with an error message if we can't get a GPU.
            .expect("Failed to find an appropriate adapter");
    
        // Create the logical device and command queue.  A logical device is like a connection to a GPU, and
        // we'll be issuing instructions to the GPU over the command queue.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Bump up the limits to require the availability of storage buffers.
                    limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");
    
        // The swapchain is how we obtain images from the surface we're drawing onto.
        // This is so we can draw onto one image while a different one is being presented
        // to the user on-screen.
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        // We'll just use the first supported format, we don't have any reason here to use
        // one format or another.
        let swapchain_format = swapchain_capabilities.formats[0];
    
        // Our surface config lets us set up our surface for drawing with the device
        // we're actually using.  It's mutable in case the window's size changes later on.
        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);


    
    }
    

    surface.configure(&device, &config);
}