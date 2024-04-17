use std::ptr::NonNull;

use jni::objects::JObject;
use jni::JNIEnv;
use ndk_sys::ANativeWindow;
use wgpu::rwh::*;

pub struct NativeHandle {
    native_window: NonNull<ANativeWindow>,
}

unsafe impl Send for NativeHandle {}

unsafe impl Sync for NativeHandle {}

impl NativeHandle {
    pub fn new(env: JNIEnv, surface: JObject) -> Option<Self> {
        let native_window = unsafe { ndk_sys::ANativeWindow_fromSurface(env.get_raw(), *surface) };
        log::debug!("{:?}", native_window);
        let native_window = NonNull::new(native_window)?;
        dbg!(native_window);
        Some(Self { native_window })
    }

    pub fn size(&self) -> (u32, u32) {
        unsafe {
            (
                ndk_sys::ANativeWindow_getWidth(self.native_window.as_ptr()) as u32,
                ndk_sys::ANativeWindow_getHeight(self.native_window.as_ptr()) as u32,
            )
        }
    }
}

impl Drop for NativeHandle {
    fn drop(&mut self) {
        unsafe { ndk_sys::ANativeWindow_release(self.native_window.as_ptr()) };
    }
}

impl HasWindowHandle for NativeHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Ok(unsafe {
            WindowHandle::borrow_raw(RawWindowHandle::AndroidNdk(AndroidNdkWindowHandle::new(
                NonNull::new(self.native_window.as_ptr() as *mut _)
                    .expect("could not create an ANativeWindow instance"),
            )))
        })
    }
}

impl HasDisplayHandle for NativeHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(unsafe {
            DisplayHandle::borrow_raw(RawDisplayHandle::Android(AndroidDisplayHandle::new()))
        })
    }
}
