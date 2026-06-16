use std::cell::Cell;

use crate::bridge::WinitBridge;

thread_local! {
    static PRE_PRESENT_TARGET: Cell<*const WinitBridge> = const { Cell::new(std::ptr::null()) };
}

pub fn set_target(bridge: &WinitBridge) {
    PRE_PRESENT_TARGET.set(bridge as *const WinitBridge);
}

pub fn notify() {
    let target = PRE_PRESENT_TARGET.get();
    if target.is_null() {
        return;
    }
    // SAFETY: target is set each frame from the live WinitBridge in the frame handler.
    unsafe { (*target).pre_present_notify() };
}
