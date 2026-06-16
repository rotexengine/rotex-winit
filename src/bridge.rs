use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use rotex_window::{Error as WindowError, WindowBridge};
use winit::window::{CursorGrabMode, Window};

use crate::error_map::backend_error;

pub struct WinitBridge {
    pub window: Window,
}

impl WindowBridge for WinitBridge {
    fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    fn set_cursor_visible(&self, visible: bool) {
        self.window.set_cursor_visible(visible);
    }

    fn set_cursor_grab(&self, grab: bool) -> Result<(), WindowError> {
        if grab {
            self.window
                .set_cursor_grab(CursorGrabMode::Locked)
                .or_else(|_| self.window.set_cursor_grab(CursorGrabMode::Confined))
                .map_err(|error| backend_error(format!("set_cursor_grab failed: {error}")))
        } else {
            self.window
                .set_cursor_grab(CursorGrabMode::None)
                .map_err(|error| backend_error(format!("set_cursor_grab failed: {error}")))
        }
    }

    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn extent(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width.max(1), size.height.max(1))
    }
}

impl HasDisplayHandle for WinitBridge {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.window.display_handle()
    }
}

impl HasWindowHandle for WinitBridge {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.window.window_handle()
    }
}

#[cfg(target_arch = "wasm32")]
impl WinitBridge {
    pub(crate) fn pre_present_notify(&self) {
        self.window.pre_present_notify();
    }
}
