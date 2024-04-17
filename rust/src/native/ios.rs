use std::ptr::NonNull;

use core_graphics::base::CGFloat;
use core_graphics::geometry::CGRect;
use objc::runtime::Object;
use wgpu::rwh::*;

pub struct NativeHandle {
    ui_view: NonNull<Object>,
}

impl NativeHandle {
    pub fn new(ui_view: NonNull<Object>) -> Self {
        Self { ui_view }
    }
}

unsafe impl Send for NativeHandle {}

unsafe impl Sync for NativeHandle {}

impl NativeHandle {
    pub fn size(&self) -> (u32, u32) {
        let layer: *mut Object = unsafe { msg_send![self.ui_view.as_ptr(), layer] };
        let bounds: CGRect = unsafe { msg_send![layer, bounds] };
        let contents_scale: CGFloat = unsafe { msg_send![layer, contentsScale] };
        (
            (bounds.size.width * contents_scale) as u32,
            (bounds.size.height * contents_scale) as u32,
        )
    }
}

impl HasWindowHandle for NativeHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Ok(unsafe {
            WindowHandle::borrow_raw(RawWindowHandle::UiKit(UiKitWindowHandle::new(
                self.ui_view.cast(),
            )))
        })
    }
}

impl HasDisplayHandle for NativeHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(
            unsafe {
                DisplayHandle::borrow_raw(RawDisplayHandle::UiKit(UiKitDisplayHandle::new()))
            },
        )
    }
}
