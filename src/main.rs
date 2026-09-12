#![forbid(unsafe_code)]

use ores_otel_desktop_core::{app::DesktopApp, config::DesktopConfig, telemetry};
use serde_json::json;

fn main() {
    const ROUTINE_ID: &str = "ores-routine-EhqcmdIb2-t-r8rDnyvlQ";
    let log = telemetry::logger();
    // Telemetry never changes what the app does, so a failed send is ignored.
    let _ = log
        .info(vec![json!("ores-otel desktop starting")])
        .add_trace("ores-trace-5CZZW8fPD_f72qDntHHQS", false)
        .add_routine_id(ROUTINE_ID)
        .send();
    let cfg = DesktopConfig::from_env();
    DesktopApp::new(cfg).run(&log);
    let _ = log
        .info(vec![json!("ores-otel desktop exiting")])
        .add_trace("ores-trace-0CoYVCOr2Bp9Gya4LXmHo", false)
        .add_routine_id(ROUTINE_ID)
        .send();
}
