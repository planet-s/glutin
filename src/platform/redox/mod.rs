use {ContextError, CreationError, GlAttributes, PixelFormatRequirements};

use api::osmesa;

pub struct Context{
    osmesa: osmesa::OsMesaContext,
}

impl Context {
    #[inline]
    pub unsafe fn new(
        window_builder: winit::WindowBuilder,
        events_loop: &winit::EventsLoop,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
    ) -> Result<(winit::Window, Self), CreationError> {
        Err(CreationError::NotSupported("Context::new is not supported on Redox"))
    }

    #[inline]
    pub unsafe fn new_context(
        el: &winit::EventsLoop,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
        shareable_with_windowed_contexts: bool,
    ) -> Result<Self, CreationError>
    {
        Err(CreationError::NotSupported("Context::new_context is not supported on Redox"))
    }

    #[inline]
    pub fn resize(&self, width: u32, height: u32) {
        unreachable!();
    }

    #[inline]
    pub unsafe fn make_current(&self) -> Result<(), ContextError> {
        self.osmesa.make_current()
    }

    #[inline]
    pub fn is_current(&self) -> bool {
        self.osmesa.is_current()
    }

    #[inline]
    pub fn get_proc_address(&self, addr: &str) -> *const () {
        self.osmesa.get_proc_address(addr)
    }

    #[inline]
    pub fn swap_buffers(&self) -> Result<(), ContextError> {
        unreachable!();
    }

    #[inline]
    pub fn get_api(&self) -> ::Api {
        self.osmesa.get_api()
    }

    #[inline]
    pub fn get_pixel_format(&self) -> PixelFormat {
        unreachable!();
    }

    #[inline]
    pub unsafe fn raw_handle(&self) -> RawHandle {
        RawHandle::Egl(self.osmesa.raw_handle())
    }
}
