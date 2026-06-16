use rotex_window::Error as WindowError;

pub fn event_loop_error(error: winit::error::EventLoopError) -> WindowError {
    WindowError::Backend(format!("event loop error: {error}"))
}

pub fn os_error(error: winit::error::OsError) -> WindowError {
    WindowError::Backend(format!("window os error: {error}"))
}

pub fn backend_error(message: impl Into<String>) -> WindowError {
    WindowError::Backend(message.into())
}
