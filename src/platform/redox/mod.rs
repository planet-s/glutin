use winit::os::redox::WindowExt;

use {ContextError, CreationError, GlAttributes, PixelFormat, PixelFormatRequirements};
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
        let window = window_builder.build(events_loop)?;

        let osmesa = {
            let (w, h) = window.get_inner_size().unwrap().into(); //TODO

            let arc = window.get_orbclient_window();
            let win = arc.borrow();

            let gl_attr = gl_attr.clone().map_sharing(|ctxt| &ctxt.osmesa);

            osmesa::OsMesaContext::new((w, h), pf_reqs, &gl_attr)?
        };

        Ok((window, Context {
            osmesa
        }))
    }

    #[inline]
    pub unsafe fn new_context(
        el: &winit::EventsLoop,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
        shareable_with_windowed_contexts: bool,
    ) -> Result<Self, CreationError> {
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
}
