#![forbid(unsafe_code)]

use next_loggers::Logger;
use serde_json::json;

use crate::config::DesktopConfig;
use crate::net;
use crate::telemetry;
use crate::ui;

pub struct DesktopApp {
    config: DesktopConfig,
}

impl DesktopApp {
    pub fn new(config: DesktopConfig) -> Self {
        Self { config }
    }

    pub fn run(&self, log: &Logger) {
        const ROUTINE_ID: &str = "ores-routine-5VOYNXbHEFzIgXji__e-E";
        let state = net::probe(&self.config.api_base);
        // Outcome only: the endpoint is never logged.
        let _ = log
            .info(vec![json!("desktop api probe completed")])
            .add_fields(telemetry::probe_fields(&state))
            .add_trace("ores-trace-xKTfnYk9rzCgM4HDYxls9", false)
            .add_routine_id(ROUTINE_ID)
            .send();
        print!("{}", ui::render(&state));
    }
}
