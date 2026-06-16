use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_time::Instant;

thread_local! {
    static LAST_RAF_AT: RefCell<Option<Instant>> = RefCell::new(None);
    static LAST_RAF_GAP_US: Cell<f64> = const { Cell::new(0.0) };
}

pub fn last_rafgap_us() -> f64 {
    LAST_RAF_GAP_US.with(|gap| gap.get())
}

type FrameHandler = unsafe fn();

static mut FRAME_HANDLER: Option<FrameHandler> = None;

/// Register the single WASM app frame handler. Must be called before [`start`].
///
/// # Safety
/// The handler must remain valid for the lifetime of the page.
pub unsafe fn register_frame_handler(handler: FrameHandler) {
    unsafe {
        FRAME_HANDLER = Some(handler);
    }
}

pub fn start() {
    let closure = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));

    *closure.borrow_mut() = Some(Closure::new({
        let closure = closure.clone();
        move || {
            record_rafgap();
            schedule_next(&closure);
            dispatch_frame();
        }
    }));

    schedule_next(&closure);
}

fn record_rafgap() {
    let now = Instant::now();
    LAST_RAF_AT.with(|slot| {
        let mut last = slot.borrow_mut();
        if let Some(previous) = *last {
            LAST_RAF_GAP_US.with(|gap| {
                gap.set(now.duration_since(previous).as_secs_f64() * 1_000_000.0);
            });
        }
        *last = Some(now);
    });
}

fn dispatch_frame() {
    let handler = unsafe { FRAME_HANDLER };
    if let Some(handler) = handler {
        unsafe { handler() };
    }
}

fn schedule_next(closure: &Rc<RefCell<Option<Closure<dyn FnMut()>>>>) {
    let window = web_sys::window().expect("wasm runner requires a browser window");
    let borrowed = closure.borrow();
    let callback = borrowed
        .as_ref()
        .expect("wasm frame closure must be initialized")
        .as_ref()
        .unchecked_ref();
    let _ = window
        .request_animation_frame(callback)
        .expect("request_animation_frame failed");
}
