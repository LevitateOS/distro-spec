//! Authentication component definitions.
//!
//! Lists of binaries, PAM modules, and configuration files required for the
//! authentication subsystem. These are extracted from `components.rs` for
//! centralized authentication configuration.

/// Authentication binaries for /usr/bin.
///
/// Implements privilege escalation and user management.
pub const AUTH_BIN: &[&str] = &["su", "sudo", "sudoedit", "sudoreplay"];

/// Authentication binaries for /usr/sbin.
///
/// **unix_chkpwd is CRITICAL**: pam_unix.so has a hardcoded path to /usr/sbin/unix_chkpwd.
/// Without it, chpasswd/passwd silently fail (PAM returns success but password unchanged).
/// This is the most common cause of silent authentication failures.
pub const AUTH_SBIN: &[&str] = &["visudo", "unix_chkpwd"];

/// Additional shadow-utils binaries for /usr/sbin.
///
/// These provide password expiry, account lockout, and batch user management.
pub const SHADOW_SBIN: &[&str] = &[
    "faillock",     // Account lockout management (pam_faillock)
    "chage",        // Password expiry management
    "newusers",     // Batch user creation
    "chgpasswd",    // Batch group password changes
    "pwck",         // Verify passwd file integrity
    "grpck",        // Verify group file integrity
    "vipw",         // Safe passwd/shadow/group editing
    "vigr",         // Safe group editing
    "pwconv",       // Convert to shadow format
    "pwunconv",     // Unconvert from shadow format
    "grpconv",      // Convert to shadow group format
    "grpunconv",    // Unconvert from shadow group format
];

/// SSH binaries in /usr/sbin.
///
/// Implements remote login capability for the system.
pub const SSH_SBIN: &[&str] = &["sshd"];

/// SSH binaries in /usr/bin.
///
/// Client-side SSH utilities for users.
pub const SSH_BIN: &[&str] = &["ssh", "scp", "sftp", "ssh-keygen", "ssh-add", "ssh-agent"];

/// PAM modules in /usr/lib64/security/.
///
/// Comprehensive list for Arch-comparable security. These modules implement
/// the pluggable authentication framework used by all login services.
pub const PAM_MODULES: &[&str] = &[
    // === CORE AUTH ===
    "pam_unix.so",           // Traditional Unix password auth (CRITICAL)
    "pam_permit.so",         // Always permit (for stacking)
    "pam_deny.so",           // Always deny (for fallback/other)
    // === PASSWORD QUALITY ===
    "pam_pwquality.so",      // Password strength checking
    "pam_pwhistory.so",      // Prevent password reuse
    // === ACCOUNT LOCKOUT ===
    "pam_faillock.so",       // Lock account after failed attempts
    "pam_faildelay.so",      // Delay after auth failure
    "pam_tally2.so",         // Legacy tally (some configs use it)
    // === ACCESS CONTROL ===
    "pam_access.so",         // /etc/security/access.conf
    "pam_time.so",           // /etc/security/time.conf
    "pam_group.so",          // /etc/security/group.conf
    "pam_wheel.so",          // Restrict su to wheel group
    "pam_nologin.so",        // Deny login when /etc/nologin exists
    "pam_securetty.so",      // Restrict root to secure ttys
    "pam_shells.so",         // Require valid shell in /etc/shells
    // === RESOURCE LIMITS ===
    "pam_limits.so",         // /etc/security/limits.conf (ulimit)
    "pam_umask.so",          // Set default umask
    // === SESSION SETUP ===
    "pam_env.so",            // Set environment from pam_env.conf
    "pam_systemd.so",        // Register session with systemd-logind
    "pam_keyinit.so",        // Initialize kernel keyring
    "pam_loginuid.so",       // Set loginuid for auditing
    "pam_namespace.so",      // Polyinstantiated directories (/tmp per-user)
    // === CONDITIONAL ===
    "pam_succeed_if.so",     // Conditional success based on user attributes
    "pam_listfile.so",       // Check user against file list
    "pam_rootok.so",         // Skip auth for root
    "pam_localuser.so",      // Only allow local users
    // === SELINUX ===
    "pam_selinux.so",        // SELinux context setup
    "pam_sepermit.so",       // SELinux permit mapping
    // === INFO/LOGGING ===
    "pam_lastlog.so",        // Record/display last login
    "pam_motd.so",           // Display message of the day
    "pam_mail.so",           // Check for new mail
    "pam_warn.so",           // Log warnings to syslog
    "pam_echo.so",           // Display messages
    // === X11 ===
    "pam_xauth.so",          // Forward X11 credentials on su
    // === MISC ===
    "pam_exec.so",           // Execute external command
    "pam_mkhomedir.so",      // Create home directory on first login
    "pam_ftp.so",            // Anonymous FTP-style login
];

/// Essential PAM configuration files in /etc/pam.d/.
///
/// These files define the authentication policy for all services.
/// Each file contains stacks of PAM modules that are called in sequence.
pub const PAM_CONFIGS: &[&str] = &[
    // === CORE AUTH STACKS ===
    "etc/pam.d/system-auth",      // Main auth stack (password, session, account)
    "etc/pam.d/password-auth",    // Password-based auth (remote services)
    // === LOGIN SERVICES ===
    "etc/pam.d/login",            // Console login (agetty)
    "etc/pam.d/sshd",             // SSH login
    "etc/pam.d/remote",           // Remote login services
    // === PRIVILEGE ESCALATION ===
    "etc/pam.d/sudo",             // sudo command
    "etc/pam.d/su",               // su command
    "etc/pam.d/su-l",             // su - (login shell)
    "etc/pam.d/runuser",          // runuser (root-only su)
    "etc/pam.d/runuser-l",        // runuser - (login shell)
    // === PASSWORD MANAGEMENT ===
    "etc/pam.d/passwd",           // passwd command
    "etc/pam.d/chpasswd",         // chpasswd command (batch password setting)
    // === USER/GROUP MANAGEMENT ===
    "etc/pam.d/useradd",          // useradd command
    "etc/pam.d/usermod",          // usermod command
    "etc/pam.d/userdel",          // userdel command
    "etc/pam.d/groupadd",         // groupadd command
    "etc/pam.d/groupmod",         // groupmod command
    "etc/pam.d/groupdel",         // groupdel command
    "etc/pam.d/chage",            // chage (password expiry)
    "etc/pam.d/chgpasswd",        // chgpasswd (batch group password)
    "etc/pam.d/groupmems",        // groupmems (group membership)
    "etc/pam.d/newusers",         // newusers (batch user creation)
    // === FALLBACK ===
    "etc/pam.d/other",            // Fallback for unconfigured services (should deny)
    // === SYSTEMD ===
    "etc/pam.d/systemd-user",     // systemd user sessions
];

/// Security configuration files in /etc/security/.
///
/// These files configure PAM policies, resource limits, and access control.
pub const SECURITY_FILES: &[&str] = &[
    "etc/security/limits.conf",       // Resource limits (ulimit)
    "etc/security/pam_env.conf",      // PAM environment variables
    "etc/security/faillock.conf",     // Account lockout configuration
    "etc/security/access.conf",       // Access control by user/group/host
    "etc/security/group.conf",        // Group-based access control
    "etc/security/namespace.conf",    // Namespace/polyinstantiation
    "etc/security/time.conf",         // Time-based access control
    "etc/security/pwquality.conf",    // Password quality requirements
];

/// Libraries required for sudo.
///
/// The sudo binary depends on several shared libraries from sudo-libs.
pub const SUDO_LIBS: &[&str] = &[
    "libsudo_util.so.0.0.0",
    "libsudo_util.so.0",
    "libsudo_util.so",
    "sudoers.so",
    "group_file.so",
    "system_group.so",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_components_exist() {
        // Verify critical components for login to work
        assert!(AUTH_BIN.contains(&"sudo"));
        assert!(AUTH_BIN.contains(&"su"));
        assert!(AUTH_SBIN.contains(&"unix_chkpwd"));
        assert!(PAM_MODULES.contains(&"pam_unix.so"));
        assert!(PAM_MODULES.contains(&"pam_permit.so"));
        assert!(PAM_MODULES.contains(&"pam_deny.so"));
        assert!(SSH_BIN.contains(&"ssh"));
        assert!(SSH_SBIN.contains(&"sshd"));
    }

    #[test]
    fn test_pam_configs_complete() {
        // Verify all essential PAM configs are defined
        assert!(PAM_CONFIGS.contains(&"etc/pam.d/login"));
        assert!(PAM_CONFIGS.contains(&"etc/pam.d/sshd"));
        assert!(PAM_CONFIGS.contains(&"etc/pam.d/sudo"));
        assert!(PAM_CONFIGS.contains(&"etc/pam.d/system-auth"));
    }

    #[test]
    fn test_security_files_complete() {
        // Verify security configuration files are defined
        assert!(SECURITY_FILES.contains(&"etc/security/limits.conf"));
        assert!(SECURITY_FILES.contains(&"etc/security/faillock.conf"));
        assert!(SECURITY_FILES.contains(&"etc/security/pwquality.conf"));
    }
}
