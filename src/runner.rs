#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

use rotex_window::{
    EngineApp, EngineEvent, Error as WindowError, WindowBridge, WindowDescriptor, WindowMode,
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Fullscreen, WindowAttributes, WindowId},
};

use crate::{
    bridge::WinitBridge,
    error_map::{backend_error, event_loop_error, os_error},
    event_map::physical_key_from_winit,
};

struct WinitApplication {
    descriptor: WindowDescriptor,
    app: Box<dyn EngineApp>,
    bridge: Option<WinitBridge>,
    last_update: Option<Instant>,
    startup_error: Option<WindowError>,
    #[cfg(target_arch = "wasm32")]
    frame_loop_started: bool,
}

impl WinitApplication {
    fn new(descriptor: WindowDescriptor, app: Box<dyn EngineApp>) -> Self {
        Self {
            descriptor,
            app,
            bridge: None,
            last_update: None,
            startup_error: None,
            #[cfg(target_arch = "wasm32")]
            frame_loop_started: false,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn on_frame(&mut self) {
        let Some(bridge) = self.bridge.as_ref() else {
            return;
        };
        crate::wasm_pre_present::set_target(bridge);
        let now = Instant::now();
        let delta_seconds = self
            .last_update
            .map(|last| now.duration_since(last).as_secs_f32())
            .unwrap_or(0.0);
        self.last_update = Some(now);
        self.app
            .on_event(EngineEvent::Update(delta_seconds), bridge);
        self.app.on_event(EngineEvent::Render, bridge);
    }
}

#[cfg(target_arch = "wasm32")]
unsafe fn dispatch_wasm_frame() {
    // SAFETY: pointer registered from the live WinitApplication in resumed().
    unsafe {
        let app = WASM_APP_PTR;
        if app.is_null() {
            return;
        }
        (*app).on_frame();
    }
}

#[cfg(target_arch = "wasm32")]
static mut WASM_APP_PTR: *mut WinitApplication = std::ptr::null_mut();

#[cfg(not(target_arch = "wasm32"))]
impl ApplicationHandler for WinitApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.bridge.is_some() {
            return;
        }

        let mut attrs = WindowAttributes::default()
            .with_title(self.descriptor.title.clone())
            .with_inner_size(LogicalSize::new(
                self.descriptor.width as f64,
                self.descriptor.height as f64,
            ));
        attrs = with_window_mode(attrs, &self.descriptor, event_loop);

        let window = match event_loop.create_window(attrs) {
            Ok(window) => window,
            Err(error) => {
                self.startup_error = Some(os_error(error));
                event_loop.exit();
                return;
            }
        };

        let bridge = WinitBridge { window };
        bridge.set_cursor_visible(self.descriptor.cursor_visible);
        if let Err(error) = bridge.set_cursor_grab(self.descriptor.cursor_grab) {
            self.startup_error = Some(error);
            event_loop.exit();
            return;
        }

        self.bridge = Some(bridge);
        self.last_update = Some(Instant::now());
        if let Some(bridge) = self.bridge.as_ref() {
            self.app.on_event(EngineEvent::Init, bridge);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(bridge) = self.bridge.as_ref() else {
            return;
        };
        if bridge.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.app.on_event(EngineEvent::CloseRequested, bridge);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.app.on_event(
                    EngineEvent::Resized(size.width.max(1), size.height.max(1)),
                    bridge,
                );
            }
            WindowEvent::RedrawRequested => {
                self.app.on_event(EngineEvent::Render, bridge);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.app.on_event(
                    EngineEvent::CursorMoved {
                        x: position.x,
                        y: position.y,
                    },
                    bridge,
                );
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.app.on_event(
                    EngineEvent::KeyboardInput {
                        key: physical_key_from_winit(event.physical_key),
                        pressed: event.state == ElementState::Pressed,
                        repeat: event.repeat,
                    },
                    bridge,
                );
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.app.on_event(
                    EngineEvent::ModifiersChanged {
                        shift: modifiers.state().shift_key(),
                        control: modifiers.state().control_key(),
                        alt: modifiers.state().alt_key(),
                        super_key: modifiers.state().super_key(),
                    },
                    bridge,
                );
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(bridge) = self.bridge.as_ref() else {
            return;
        };
        let now = Instant::now();
        let delta_seconds = self
            .last_update
            .map(|last| now.duration_since(last).as_secs_f32())
            .unwrap_or(0.0);
        self.last_update = Some(now);
        self.app
            .on_event(EngineEvent::Update(delta_seconds), bridge);
        bridge.request_redraw();
    }
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler for WinitApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.bridge.is_some() {
            return;
        }

        let mut attrs = WindowAttributes::default()
            .with_title(self.descriptor.title.clone())
            .with_inner_size(LogicalSize::new(
                self.descriptor.width as f64,
                self.descriptor.height as f64,
            ));
        attrs = with_window_mode(attrs, &self.descriptor, event_loop);
        use winit::platform::web::WindowAttributesExtWebSys;
        attrs = attrs.with_append(true);

        let window = match event_loop.create_window(attrs) {
            Ok(window) => window,
            Err(error) => {
                self.startup_error = Some(os_error(error));
                event_loop.exit();
                return;
            }
        };

        let bridge = WinitBridge { window };
        bridge.set_cursor_visible(self.descriptor.cursor_visible);
        if let Err(error) = bridge.set_cursor_grab(self.descriptor.cursor_grab) {
            self.startup_error = Some(error);
            event_loop.exit();
            return;
        }

        self.bridge = Some(bridge);
        self.last_update = Some(Instant::now());
        if let Some(bridge) = self.bridge.as_ref() {
            self.app.on_event(EngineEvent::Init, bridge);
        }

        if !self.frame_loop_started {
            self.frame_loop_started = true;
            unsafe {
                WASM_APP_PTR = self as *mut WinitApplication;
                crate::wasm_frame_scheduler::register_frame_handler(dispatch_wasm_frame);
            }
            crate::wasm_frame_scheduler::start();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(bridge) = self.bridge.as_ref() else {
            return;
        };
        if bridge.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.app.on_event(EngineEvent::CloseRequested, bridge);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.app.on_event(
                    EngineEvent::Resized(size.width.max(1), size.height.max(1)),
                    bridge,
                );
            }
            // Frames are driven by the chained rAF scheduler.
            WindowEvent::RedrawRequested => {}
            WindowEvent::CursorMoved { position, .. } => {
                self.app.on_event(
                    EngineEvent::CursorMoved {
                        x: position.x,
                        y: position.y,
                    },
                    bridge,
                );
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.app.on_event(
                    EngineEvent::KeyboardInput {
                        key: physical_key_from_winit(event.physical_key),
                        pressed: event.state == ElementState::Pressed,
                        repeat: event.repeat,
                    },
                    bridge,
                );
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.app.on_event(
                    EngineEvent::ModifiersChanged {
                        shift: modifiers.state().shift_key(),
                        control: modifiers.state().control_key(),
                        alt: modifiers.state().alt_key(),
                        super_key: modifiers.state().super_key(),
                    },
                    bridge,
                );
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {}
}

fn with_window_mode(
    attrs: WindowAttributes,
    descriptor: &WindowDescriptor,
    event_loop: &ActiveEventLoop,
) -> WindowAttributes {
    match descriptor.mode {
        WindowMode::Windowed => attrs,
        WindowMode::BorderlessFullscreen => {
            attrs.with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
        }
        WindowMode::ExclusiveFullscreen => {
            if let Some(monitor) = event_loop.primary_monitor() {
                if let Some(video_mode) = monitor.video_modes().next() {
                    return attrs.with_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
                }
            }
            attrs.with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
        }
    }
}

pub(crate) fn run(
    descriptor: WindowDescriptor,
    app: Box<dyn EngineApp>,
) -> Result<(), WindowError> {
    if descriptor.width == 0 || descriptor.height == 0 {
        return Err(WindowError::InvalidDescriptor(
            "width and height must be greater than zero",
        ));
    }

    let event_loop = EventLoop::new().map_err(event_loop_error)?;
    event_loop.set_control_flow(ControlFlow::Wait);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut application = WinitApplication::new(descriptor, app);
        event_loop
            .run_app(&mut application)
            .map_err(event_loop_error)?;
        if let Some(error) = application.startup_error {
            return Err(error);
        }
        return Ok(());
    }

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::EventLoopExtWebSys;
        event_loop.spawn_app(WinitApplication::new(descriptor, app));
        return Ok(());
    }

    #[allow(unreachable_code)]
    Err(backend_error("unreachable winit runner state"))
}
