mod backend;
mod bridge;
mod error_map;
mod event_map;
mod runner;
#[cfg(target_arch = "wasm32")]
mod wasm_frame_scheduler;
#[cfg(target_arch = "wasm32")]
mod wasm_pre_present;

#[cfg(target_arch = "wasm32")]
pub use wasm_frame_scheduler::last_rafgap_us;

#[cfg(target_arch = "wasm32")]
pub fn wasm_pre_present_notify() {
    wasm_pre_present::notify();
}

pub use backend::WinitBackend;
pub use bridge::WinitBridge;

pub fn backend() -> WinitBackend {
    WinitBackend::default()
}
