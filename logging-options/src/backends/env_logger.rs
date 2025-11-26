use env_logger::{Builder, Logger};

use crate::Backend;
use crate::backend::BackendBuilder;

impl Backend for Logger {
    type Builder = Builder;

    fn builder() -> Self::Builder {
        let mut b = env_logger::builder();
        b.format_level(true)
            .format_file(true)
            .format_line_number(true)
            .format_timestamp_millis();
        b
    }
}

impl BackendBuilder for Builder {
    fn init(mut self) {
        Builder::init(&mut self)
    }
}
