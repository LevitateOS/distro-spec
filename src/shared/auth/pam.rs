//! PAM configuration file contents.
//!
//! This module contains the static content of all PAM configuration files
//! used in the authentication subsystem. These are embedded as strings and
//! written to `/etc/pam.d/` during system build.
//!
//! # Design Note
//!
//! PAM configurations are static and reusable across different build scenarios
//! (ISO, rootfs, testing). Rather than embedding them in leviso, they're defined
//! here in distro-spec as the single source of truth.
//!
//! Build logic (writing files, directory creation, etc.) stays in leviso.
//! Configuration content lives here.

/// Core authentication stack used by multiple PAM services.
///
/// Implements:
/// - Environment setup (pam_env)
/// - Failure delay (pam_faildelay) to prevent brute-force attacks
/// - Unix password authentication (pam_unix) with YESCRYPT hashing
/// - Account lockout (pam_deny fallback)
/// - User validation (pam_nologin)
/// - Resource limits (pam_limits)
/// - Session tracking (pam_systemd)
pub const PAM_SYSTEM_AUTH: &str = include_str!("files/system-auth");

/// Postlogin processing.
///
/// Runs after successful authentication. Typically includes:
/// - Session-specific initialization
/// - User environment setup
pub const PAM_POSTLOGIN: &str = include_str!("files/postlogin");

/// Local console login configuration.
///
/// Used by agetty on /dev/tty1 and other local terminals.
/// Includes additional login-specific PAM modules:
/// - pam_loginuid: Set audit UID for session tracking
/// - pam_namespace: Polyinstantiation for /tmp per-user isolation
pub const PAM_LOGIN: &str = include_str!("files/login");

/// SSH login configuration.
///
/// Used by sshd for remote authentication. Similar to login but
/// typically omits some interactive elements like MOTD.
pub const PAM_SSHD: &str = include_str!("files/sshd");

/// Remote login configuration.
///
/// Used for other remote login services beyond SSH.
pub const PAM_REMOTE: &str = include_str!("files/remote");

/// sudo configuration.
///
/// Privilege escalation for users in wheel group.
pub const PAM_SUDO: &str = include_str!("files/sudo");

/// su (switch user) configuration.
///
/// Allows users to switch to other accounts with target's password.
pub const PAM_SU: &str = include_str!("files/su");

/// su with login shell configuration.
///
/// Like su but starts a login shell.
pub const PAM_SU_L: &str = include_str!("files/su-l");

/// runuser (root-only user switching) configuration.
///
/// Allows root to switch to other users without password.
pub const PAM_RUNUSER: &str = include_str!("files/runuser");

/// runuser with login shell configuration.
///
/// Like runuser but starts a login shell.
pub const PAM_RUNUSER_L: &str = include_str!("files/runuser-l");

/// cron job execution configuration.
///
/// Used by cron daemon to authenticate job execution.
pub const PAM_CROND: &str = include_str!("files/crond");

/// passwd command configuration.
///
/// Used when users change their own password.
pub const PAM_PASSWD: &str = include_str!("files/passwd");

/// chpasswd command configuration.
///
/// Used for batch password setting (installation scripts).
pub const PAM_CHPASSWD: &str = include_str!("files/chpasswd");

/// chfn command configuration (change full name).
///
/// Used to change user's GECOS field.
pub const PAM_CHFN: &str = include_str!("files/chfn");

/// chsh command configuration (change shell).
///
/// Used to change default shell.
pub const PAM_CHSH: &str = include_str!("files/chsh");

/// Fallback configuration for unconfigured services.
///
/// Default policy is to DENY all access. If a service doesn't have
/// its own PAM config file, this is used and access is denied.
pub const PAM_OTHER: &str = include_str!("files/other");

/// systemd user session configuration.
///
/// Used by systemd --user to authenticate user services.
pub const PAM_SYSTEMD_USER: &str = include_str!("files/systemd-user");

/// Security configuration: resource limits.
///
/// Controls ulimit settings: max file descriptors, core dumps, stack size, etc.
/// Prevents denial of service via resource exhaustion.
pub const LIMITS_CONF: &str = include_str!("files/limits.conf");

/// Security configuration: access control by user/group/host.
///
/// Controls who can login based on user, group, and host matching rules.
pub const ACCESS_CONF: &str = include_str!("files/access.conf");

/// Security configuration: polyinstantiation (per-user /tmp isolation).
///
/// Enables per-user isolated /tmp directories (if configured by pam_namespace).
/// Left mostly empty by default - administrators can configure as needed.
pub const NAMESPACE_CONF: &str = include_str!("files/namespace.conf");

/// Security configuration: PAM environment variables.
///
/// Sets environment variables for PAM sessions.
/// Left mostly empty by default - applications configure via pam.conf.
pub const PAM_ENV_CONF: &str = include_str!("files/pam_env.conf");

/// Security configuration: password quality requirements.
///
/// Enforces password strength via pam_pwquality.so.
/// Requirements: 12+ chars, 3+ classes, numbers, no repeats.
pub const PWQUALITY_CONF: &str = include_str!("files/pwquality.conf");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pam_configs_have_content() {
        // Verify all PAM configs are non-empty
        assert!(!PAM_SYSTEM_AUTH.is_empty());
        assert!(!PAM_LOGIN.is_empty());
        assert!(!PAM_SSHD.is_empty());
        assert!(!PAM_SUDO.is_empty());
    }

    #[test]
    fn test_critical_pam_modules() {
        // Verify critical modules are in system-auth
        assert!(PAM_SYSTEM_AUTH.contains("pam_unix.so"), "pam_unix.so required");
        assert!(PAM_SYSTEM_AUTH.contains("yescrypt"), "yescrypt hashing required");
        assert!(PAM_SYSTEM_AUTH.contains("pam_deny.so"), "pam_deny fallback required");
    }

    #[test]
    fn test_login_includes_namespace() {
        // Verify login includes namespace for /tmp isolation
        assert!(PAM_LOGIN.contains("pam_namespace.so"));
    }

    #[test]
    fn test_other_denies_all() {
        // Verify fallback denies access to unknown services
        assert!(PAM_OTHER.contains("pam_deny.so"), "Unknown services must be denied");
    }
}
