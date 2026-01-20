//! Systemd service configuration.
//!
//! Defines which services should be enabled by default on a fresh installation.

/// Services that must be enabled during installation.
///
/// These are enabled via `systemctl enable <service>` in the chroot.
pub const ENABLED_SERVICES: &[ServiceSpec] = &[
    ServiceSpec {
        name: "systemd-networkd",
        description: "Network configuration",
        required: true,
    },
    ServiceSpec {
        name: "systemd-resolved",
        description: "DNS resolution",
        required: true,
    },
    ServiceSpec {
        name: "systemd-timesyncd",
        description: "Time synchronization",
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

    /// Generate the systemctl enable command.
    pub fn enable_command(&self) -> String {
        format!("systemctl enable {}", self.name)
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
