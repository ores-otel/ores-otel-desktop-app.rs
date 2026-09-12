#![forbid(unsafe_code)]

//! `next-loggers/v1` diagnostics for the ores-otel desktop app.
//!
//! stdout is the rendered UI, so the SDK console printer is disabled and every record is written
//! as one JSON line to stderr. Records carry lifecycle and outcome metadata only: never the API
//! endpoint (it can embed credentials or private hosts), user data, or file paths.

use std::io::Write;
use std::sync::Arc;

use next_loggers::{JsonObject, LogLevel, LogRecord, Logger, LoggerError, Options, Transport};
use serde_json::json;

use crate::state::DesktopState;

/// `appName` stamped on every record.
pub const APP_NAME: &str = "ores-otel-desktop-app";

/// Writes each record as a single JSON line to stderr.
#[derive(Clone, Copy, Debug, Default)]
pub struct StderrJsonTransport;

impl Transport for StderrJsonTransport {
    fn write(&self, record: &LogRecord) -> Result<(), LoggerError> {
        let line = record.to_json()?;
        writeln!(std::io::stderr().lock(), "{line}").map_err(|error| LoggerError(error.to_string()))
    }
}

/// Logger options for the desktop app: no console printer, records at or above `max_level` only.
pub fn options(max_level: LogLevel) -> Options {
    Options {
        app_name: APP_NAME.into(),
        max_level,
        console: false,
        ..Options::default()
    }
}

/// The process logger, writing `next-loggers/v1` JSON lines to stderr at `Info` and above.
pub fn logger() -> Logger {
    Logger::new(options(LogLevel::Info).with_transport(Arc::new(StderrJsonTransport)))
}

/// Fields describing an API probe outcome. The endpoint is deliberately omitted.
pub fn probe_fields(state: &DesktopState) -> JsonObject {
    [("connected".to_owned(), json!(state.connected))]
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn probe_fields_report_connectivity_without_the_endpoint() {
        let state = DesktopState {
            connected: false,
            endpoint: "https://api.example.invalid/private-path".into(),
        };
        let fields = probe_fields(&state);
        assert_eq!(fields.get("connected"), Some(&json!(false)));
        assert_eq!(fields.len(), 1);
        assert!(!Value::Object(fields).to_string().contains("private-path"));
    }
}
