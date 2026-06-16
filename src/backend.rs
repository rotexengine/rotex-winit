use rotex_window::{EngineApp, Error as WindowError, WindowBackend, WindowDescriptor};

use crate::runner;

#[derive(Default)]
pub struct WinitBackend;

impl WindowBackend for WinitBackend {
    fn run(
        self: Box<Self>,
        descriptor: WindowDescriptor,
        app: Box<dyn EngineApp>,
    ) -> Result<(), WindowError> {
        let _ = self;
        runner::run(descriptor, app)
    }
}
