# ores-otel-desktop-app.rs

Native Rust desktop app. No webviews, no React. UI rendering is isolated in `src/ui.rs`.

## Telemetry

The app logs through `oresoftware-next-loggers` (`src/telemetry.rs`). stdout stays the rendered UI;
records go to stderr as `next-loggers/v1` JSON lines. It logs startup and exit, plus the API probe
outcome (`connected` only; the endpoint is never logged). Every call site carries an inline
`ores-trace-…` literal, and each function declares one `ores-routine-…` ID at its top (see
[ores.otel.log docs](https://github.com/ores-otel/ores.otel.log/blob/main/docs/ores-trace-and-routine-ids.md)).
