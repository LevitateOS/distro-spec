//! Shared service management trait.
//!
//! Defines an abstract interface for service management across init systems.
//! This enables polymorphic handling of services regardless of whether
//! the system uses systemd, OpenRC, or another init system.

/// Abstract interface for service management across init systems.
///
/// Both systemd (LevitateOS) and OpenRC (AcornOS) services implement this trait,
/// enabling code to work with services without knowing the init system.
pub trait ServiceManager {
    /// Service name (without unit suffix).
    fn name(&self) -> &str;

    /// Human-readable description.
    fn description(&self) -> &str;

    /// Whether failure to enable should abort installation.
    fn required(&self) -> bool;

    /// Command to enable this service.
    ///
    /// For systemd: `systemctl enable <service>`
    /// For OpenRC: `rc-update add <service> <runlevel>`
    fn enable_command(&self) -> String;

    /// Command to disable this service.
    ///
    /// For systemd: `systemctl disable <service>`
    /// For OpenRC: `rc-update del <service> <runlevel>`
    fn disable_command(&self) -> String;

    /// Command to start this service immediately.
    ///
    /// For systemd: `systemctl start <service>`
    /// For OpenRC: `rc-service <service> start`
    fn start_command(&self) -> String;

    /// Command to stop this service immediately.
    ///
    /// For systemd: `systemctl stop <service>`
    /// For OpenRC: `rc-service <service> stop`
    fn stop_command(&self) -> String;
}
