//! Shared observability feature-parity contract with `ores-otel/ores-otel-flutter`.
//! Native notification, filesystem, lifecycle, and secure-storage behavior
//! belongs only in [`AppPlatformAdapter`].
pub const CROSS_PLATFORM_PARITY_CONTRACT_VERSION: u32 = 1;
pub const FLUTTER_COUNTERPART: &str = "ores-otel/ores-otel-flutter";
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppSurface { Mobile, FlutterDesktop, RustDesktop }
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppCapability {
    Authentication, Traces, Metrics, Logs, LiveTail, Dashboards, Filters,
    SavedViews, AlertInbox, Notifications, FileExport, OfflineCache,
    BackgroundSync, RedactionControls, Telemetry, Accessibility,
    ApplicationUpdates,
}
pub const REQUIRED_PARITY_CAPABILITIES: &[AppCapability] = &[
    AppCapability::Authentication, AppCapability::Traces,
    AppCapability::Metrics, AppCapability::Logs, AppCapability::LiveTail,
    AppCapability::Dashboards, AppCapability::Filters,
    AppCapability::SavedViews, AppCapability::AlertInbox,
    AppCapability::Notifications, AppCapability::FileExport,
    AppCapability::OfflineCache, AppCapability::BackgroundSync,
    AppCapability::RedactionControls, AppCapability::Telemetry,
    AppCapability::Accessibility, AppCapability::ApplicationUpdates,
];
pub trait AppPlatformAdapter {
    fn surface(&self) -> AppSurface;
    fn supports(&self, capability: AppCapability) -> bool;
}
pub fn verify_required_parity_capabilities(
    adapter: &impl AppPlatformAdapter,
) -> Result<(), Vec<AppCapability>> {
    let missing = REQUIRED_PARITY_CAPABILITIES.iter().copied()
        .filter(|capability| !adapter.supports(*capability)).collect::<Vec<_>>();
    if missing.is_empty() { Ok(()) } else { Err(missing) }
}
