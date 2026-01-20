//! User account configuration.
//!
//! Shared types and constants for user creation during installation.
//! Variant-specific defaults (shell, groups) are in variant modules.

/// Root account configuration.
pub const ROOT_HOME: &str = "/root";

/// Minimum UID for regular users.
pub const MIN_UID: u32 = 1000;

/// Minimum GID for regular user groups.
pub const MIN_GID: u32 = 1000;

/// User creation specification.
#[derive(Debug, Clone)]
pub struct UserSpec {
    /// Username
    pub username: String,
    /// Full name (GECOS field)
    pub full_name: Option<String>,
    /// Login shell
    pub shell: String,
    /// Additional groups
    pub groups: Vec<String>,
    /// Whether to create home directory
    pub create_home: bool,
}

impl UserSpec {
    /// Create a new user spec with custom shell and groups.
    pub fn new(
        username: impl Into<String>,
        shell: impl Into<String>,
        groups: &[&str],
    ) -> Self {
        Self {
            username: username.into(),
            full_name: None,
            shell: shell.into(),
            groups: groups.iter().map(|s| s.to_string()).collect(),
            create_home: true,
        }
    }

    /// Set the full name (GECOS field).
    pub fn with_full_name(mut self, name: impl Into<String>) -> Self {
        self.full_name = Some(name.into());
        self
    }

    /// Generate the useradd command for this user.
    pub fn useradd_command(&self) -> String {
        let mut cmd = format!("useradd -m -s {}", self.shell);

        if !self.groups.is_empty() {
            cmd.push_str(" -G ");
            cmd.push_str(&self.groups.join(","));
        }

        if let Some(ref name) = self.full_name {
            cmd.push_str(&format!(" -c \"{}\"", name));
        }

        cmd.push(' ');
        cmd.push_str(&self.username);
        cmd
    }
}

/// Sudoers configuration.
pub const SUDOERS_WHEEL_LINE: &str = "%wheel ALL=(ALL:ALL) ALL";

/// Path to sudoers.d directory.
pub const SUDOERS_D_PATH: &str = "/etc/sudoers.d";

/// Filename for wheel group sudoers config.
pub const SUDOERS_WHEEL_FILENAME: &str = "wheel";
