//! OpenRC service configuration for IuppiterOS.
//!
//! Defines which services should be enabled on a fresh IuppiterOS installation.
//! IuppiterOS is a headless refurbishment server — services are minimal.

use crate::shared::services::ServiceManager;

/// Services that must be enabled during installation.
///
/// These are enabled via `rc-update add <service> <runlevel>` in the chroot.
pub const ENABLED_SERVICES: &[ServiceSpec] = &[
    // --- Boot runlevel (early boot, before networking) ---
    ServiceSpec {
        name: "networking",
        runlevel: "boot",
        description: "Network configuration",
        required: true,
    },
    ServiceSpec {
        name: "eudev",
        runlevel: "sysinit",
        description: "Device manager (udev rules for /dev/sd*)",
        required: true,
    },
    // --- Default runlevel (normal operation) ---
    ServiceSpec {
        name: "chronyd",
        runlevel: "default",
        description: "Time synchronization (NTP)",
        required: true,
    },
    ServiceSpec {
        name: "sshd",
        runlevel: "default",
        description: "SSH server (remote management)",
        required: true,
    },
    ServiceSpec {
        name: "iuppiter-engine",
        runlevel: "default",
        description: "iuppiter HDD refurbishment engine",
        required: true,
    },
];

/// Specification for an OpenRC service.
#[derive(Debug, Clone, Copy)]
pub struct ServiceSpec {
    /// Service name.
    pub name: &'static str,
    /// Runlevel to add service to (boot, sysinit, default, shutdown).
    pub runlevel: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Whether failure to enable should abort installation.
    pub required: bool,
}

impl ServiceManager for ServiceSpec {
    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        self.description
    }

    fn required(&self) -> bool {
        self.required
    }

    fn enable_command(&self) -> String {
        format!("rc-update add {} {}", self.name, self.runlevel)
    }

    fn disable_command(&self) -> String {
        format!("rc-update del {} {}", self.name, self.runlevel)
    }

    fn start_command(&self) -> String {
        format!("rc-service {} start", self.name)
    }

    fn stop_command(&self) -> String {
        format!("rc-service {} stop", self.name)
    }
}

/// Get only the required services.
pub fn required_services() -> impl Iterator<Item = &'static ServiceSpec> {
    ENABLED_SERVICES.iter().filter(|s| s.required)
}

/// Get optional services.
pub fn optional_services() -> impl Iterator<Item = &'static ServiceSpec> {
    ENABLED_SERVICES.iter().filter(|s| !s.required)
}
