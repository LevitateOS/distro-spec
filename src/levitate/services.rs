//! Systemd service configuration.
//!
//! Defines which services should be enabled by default on a fresh installation.

use crate::shared::services::ServiceManager;

/// Services that must be enabled during installation.
///
/// These are enabled via `systemctl enable <service>` in the chroot.
/// Note: Rocky 10 uses NetworkManager (not systemd-networkd) and chronyd (not timesyncd).
pub const ENABLED_SERVICES: &[ServiceSpec] = &[
    ServiceSpec {
        name: "NetworkManager",
        description: "Network configuration",
        required: true,
    },
    ServiceSpec {
        name: "chronyd",
        description: "Time synchronization (NTP)",
        required: true,
    },
    ServiceSpec {
        name: "sshd",
        description: "SSH server",
        required: false,
    },
];

/// Specification for a systemd service.
#[derive(Debug, Clone, Copy)]
pub struct ServiceSpec {
    /// Service unit name (without .service suffix)
    pub name: &'static str,
    /// Human-readable description
    pub description: &'static str,
    /// Whether failure to enable should abort installation
    pub required: bool,
}

impl ServiceSpec {
    /// Get the full unit name with .service suffix.
    pub fn unit_name(&self) -> String {
        format!("{}.service", self.name)
    }
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
        format!("systemctl enable {}", self.name)
    }

    fn disable_command(&self) -> String {
        format!("systemctl disable {}", self.name)
    }

    fn start_command(&self) -> String {
        format!("systemctl start {}", self.name)
    }

    fn stop_command(&self) -> String {
        format!("systemctl stop {}", self.name)
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
