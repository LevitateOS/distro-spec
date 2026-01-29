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
pub const PAM_SYSTEM_AUTH: &str = "\
# LevitateOS system-auth - PAM configuration for system authentication
# Based on Fedora/Rocky authselect defaults

auth        required                     pam_env.so
auth        required                     pam_faildelay.so delay=2000000
auth        sufficient                   pam_unix.so nullok
auth        required                     pam_deny.so

account     required                     pam_unix.so
account     required                     pam_nologin.so

password    requisite                    pam_pwquality.so
password    sufficient                   pam_unix.so yescrypt shadow use_authtok
password    required                     pam_deny.so

session     optional                     pam_keyinit.so revoke
session     required                     pam_limits.so
-session    optional                     pam_systemd.so
session     [success=1 default=ignore]   pam_succeed_if.so service in crond quiet use_uid
session     required                     pam_unix.so
";

/// Postlogin processing.
///
/// Runs after successful authentication. Typically includes:
/// - Session-specific initialization
/// - User environment setup
pub const PAM_POSTLOGIN: &str = "\
# Postlogin processing (included by login services)
session     optional                     pam_lastlog.so showfailed silent
session     optional                     pam_motd.so motd=/etc/motd
session     optional                     pam_motd.so noupdate
";

/// Local console login configuration.
///
/// Used by agetty on /dev/tty1 and other local terminals.
/// Includes additional login-specific PAM modules:
/// - pam_loginuid: Set audit UID for session tracking
/// - pam_namespace: Polyinstantiation for /tmp per-user isolation
pub const PAM_LOGIN: &str = "\
# LevitateOS login - PAM configuration for local login
auth       substack     system-auth
auth       include      postlogin
account    required     pam_nologin.so
account    include      system-auth
password   include      system-auth
session    required     pam_loginuid.so
session    required     pam_namespace.so
session    optional     pam_keyinit.so force revoke
session    include      system-auth
session    include      postlogin
";

/// SSH login configuration.
///
/// Used by sshd for remote authentication. Similar to login but
/// typically omits some interactive elements like MOTD.
pub const PAM_SSHD: &str = "\
# SSH login - PAM configuration for SSH remote access
auth       substack     system-auth
auth       include      postlogin
account    required     pam_nologin.so
account    include      system-auth
password   include      system-auth
session    required     pam_loginuid.so
session    required     pam_namespace.so
session    optional     pam_keyinit.so force revoke
session    include      system-auth
session    include      postlogin
";

/// Remote login configuration.
///
/// Used for other remote login services beyond SSH.
pub const PAM_REMOTE: &str = "\
# Remote login - PAM configuration for remote login services
auth       substack     system-auth
auth       include      postlogin
account    required     pam_nologin.so
account    include      system-auth
password   include      system-auth
session    required     pam_loginuid.so
session    include      system-auth
session    include      postlogin
";

/// sudo configuration.
///
/// Privilege escalation for users in wheel group.
pub const PAM_SUDO: &str = "\
# sudo - PAM configuration for privilege escalation
auth       include      system-auth
account    include      system-auth
password   include      system-auth
session    include      system-auth
";

/// su (switch user) configuration.
///
/// Allows users to switch to other accounts with target's password.
pub const PAM_SU: &str = "\
# su - PAM configuration for user switching
auth       sufficient   pam_rootok.so
auth       include      system-auth
account    required     pam_nologin.so
account    include      system-auth
password   include      system-auth
session    required     pam_unix.so
session    include      system-auth
";

/// su with login shell configuration.
///
/// Like su but starts a login shell.
pub const PAM_SU_L: &str = "\
# su-l - PAM configuration for su with login shell
auth       sufficient   pam_rootok.so
auth       include      system-auth
account    required     pam_nologin.so
account    include      system-auth
password   include      system-auth
session    required     pam_loginuid.so
session    required     pam_namespace.so
session    optional     pam_keyinit.so force revoke
session    required     pam_unix.so
session    include      system-auth
";

/// runuser (root-only user switching) configuration.
///
/// Allows root to switch to other users without password.
pub const PAM_RUNUSER: &str = "\
# runuser - PAM configuration for root-only user switching
auth       sufficient   pam_rootok.so
account    include      system-auth
session    include      system-auth
";

/// runuser with login shell configuration.
///
/// Like runuser but starts a login shell.
pub const PAM_RUNUSER_L: &str = "\
# runuser-l - PAM configuration for runuser with login shell
auth       sufficient   pam_rootok.so
account    include      system-auth
session    required     pam_loginuid.so
session    required     pam_namespace.so
session    include      system-auth
";

/// cron job execution configuration.
///
/// Used by cron daemon to authenticate job execution.
pub const PAM_CROND: &str = "\
# crond - PAM configuration for cron job execution
auth       required     pam_env.so
auth       required     pam_unix.so
account    required     pam_unix.so
session    required     pam_limits.so
";

/// passwd command configuration.
///
/// Used when users change their own password.
pub const PAM_PASSWD: &str = "\
# passwd - PAM configuration for password changes
password   substack     system-auth
";

/// chpasswd command configuration.
///
/// Used for batch password setting (installation scripts).
pub const PAM_CHPASSWD: &str = "\
# chpasswd - PAM configuration for batch password changes
account    required     pam_unix.so
password   required     pam_unix.so yescrypt shadow
";

/// chfn command configuration (change full name).
///
/// Used to change user's GECOS field.
pub const PAM_CHFN: &str = "\
# chfn - PAM configuration for changing user info
auth       include      system-auth
account    include      system-auth
password   include      system-auth
session    include      system-auth
";

/// chsh command configuration (change shell).
///
/// Used to change default shell.
pub const PAM_CHSH: &str = "\
# chsh - PAM configuration for changing shell
auth       include      system-auth
account    include      system-auth
password   include      system-auth
session    include      system-auth
";

/// Fallback configuration for unconfigured services.
///
/// Default policy is to DENY all access. If a service doesn't have
/// its own PAM config file, this is used and access is denied.
pub const PAM_OTHER: &str = "\
# other - Fallback PAM configuration (deny all)
auth       required     pam_deny.so
account    required     pam_deny.so
password   required     pam_deny.so
session    required     pam_deny.so
";

/// systemd user session configuration.
///
/// Used by systemd --user to authenticate user services.
pub const PAM_SYSTEMD_USER: &str = "\
# systemd-user - PAM configuration for systemd user sessions
auth       required     pam_env.so
auth       required     pam_unix.so nullok
account    required     pam_unix.so
password   required     pam_unix.so
session    required     pam_loginuid.so
session    optional     pam_keyinit.so revoke
session    required     pam_limits.so
-session   optional     pam_systemd.so
session    required     pam_unix.so
";

/// Security configuration: resource limits.
///
/// Controls ulimit settings: max file descriptors, core dumps, stack size, etc.
/// Prevents denial of service via resource exhaustion.
pub const LIMITS_CONF: &str = "\
*               soft    core            0
*               hard    nofile          1048576
*               soft    nofile          1024
root            soft    nofile          1048576
";

/// Security configuration: access control by user/group/host.
///
/// Controls who can login based on user, group, and host matching rules.
pub const ACCESS_CONF: &str = "\
# Access control rules for pam_access.so
# Format: permission:users:origins
# Allow root from LOCAL or network
+:root:LOCAL
# Allow all other users
+:ALL:ALL
";

/// Security configuration: polyinstantiation (per-user /tmp isolation).
///
/// Enables per-user isolated /tmp directories (if configured by pam_namespace).
/// Left mostly empty by default - administrators can configure as needed.
pub const NAMESPACE_CONF: &str = "\
# Polyinstantiation configuration for pam_namespace.so
# Defines per-user isolated directories
# See pam_namespace(5) for syntax
";

/// Security configuration: PAM environment variables.
///
/// Sets environment variables for PAM sessions.
/// Left mostly empty by default - applications configure via pam.conf.
pub const PAM_ENV_CONF: &str = "\
# PAM environment configuration
# Format: variable DEFAULT|@{PAM_ITEM}
# See pam_env.conf(5) for syntax
";

/// Security configuration: password quality requirements.
///
/// Enforces password strength via pam_pwquality.so.
/// Requirements: 12+ chars, 3+ classes, numbers, no repeats.
pub const PWQUALITY_CONF: &str = "\
# Password quality requirements (pam_pwquality.so)
minlen = 12
minclass = 3
dcredit = -1
ucredit = -1
maxrepeat = 3
";

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
