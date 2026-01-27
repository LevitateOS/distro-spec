//! User account configuration.
//!
//! Shared types and constants for user creation during installation.
//! Variant-specific defaults (shell, groups) are in variant modules.

use smallvec::SmallVec;
use std::borrow::Cow;

/// Root account configuration.
pub const ROOT_HOME: &str = "/root";

/// Minimum UID for regular users.
pub const MIN_UID: u32 = 1000;

/// Minimum GID for regular user groups.
pub const MIN_GID: u32 = 1000;

/// User creation specification.
///
/// Uses `Cow<'static, str>` for shell (usually a static default like "/bin/bash")
/// and `SmallVec<[String; 2]>` for groups (most users have 0-3 groups, avoiding heap allocation).
#[derive(Debug, Clone)]
pub struct UserSpec {
    /// Username
    pub username: String,
    /// Full name (GECOS field)
    pub full_name: Option<String>,
    /// Login shell (usually static, e.g., "/bin/bash")
    pub shell: Cow<'static, str>,
    /// Additional groups (inline up to 2, heap beyond)
    pub groups: SmallVec<[String; 2]>,
    /// Whether to create home directory
    pub create_home: bool,
}

impl UserSpec {
    /// Create a new user spec with custom shell and groups.
    ///
    /// Accepts static str for shell to enable zero-copy.
    pub fn new(
        username: impl Into<String>,
        shell: impl Into<Cow<'static, str>>,
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
            let groups: Vec<&str> = self.groups.iter().map(|s| s.as_str()).collect();
            cmd.push_str(&groups.join(","));
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure UserSpec doesn't grow unexpectedly.
    ///
    /// UserSpec uses:
    /// - username: String (24 bytes)
    /// - full_name: Option<String> (24 bytes)
    /// - shell: Cow<'static, str> (24 bytes)
    /// - groups: SmallVec<[String; 2]> (inline storage for 2 Strings)
    /// - create_home: bool (1 byte + padding)
    #[test]
    fn user_spec_size() {
        let size = std::mem::size_of::<UserSpec>();
        // With SmallVec<[String; 2]>, inline capacity is 2 Strings (48 bytes)
        // Total: 24 + 24 + 24 + ~72 (SmallVec overhead + 2 Strings inline) + 8 = ~152 bytes
        // This is still more efficient than Vec for common cases (0-2 groups)
        assert!(
            size <= 160,
            "UserSpec grew too large: {} bytes (max 160)",
            size
        );
        eprintln!("UserSpec size: {} bytes", size);
    }

    /// Test that SmallVec groups work correctly.
    #[test]
    fn user_spec_groups() {
        let user = UserSpec::new("alice", "/bin/bash", &["wheel", "video"]);
        assert_eq!(user.groups.len(), 2);
        assert_eq!(&user.groups[0], "wheel");
        assert_eq!(&user.groups[1], "video");

        // Test useradd command generation
        let cmd = user.useradd_command();
        assert!(cmd.contains("-G wheel,video"));
        assert!(cmd.contains("-s /bin/bash"));
        assert!(cmd.contains("alice"));
    }
}
