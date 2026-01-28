//! Authentication/login subsystem.
//!
//! This module consolidates all authentication-related configuration and specifications
//! for LevitateOS and AcornOS. It serves as the **single source of truth** for:
//!
//! - PAM (Pluggable Authentication Modules) configuration files
//! - Authentication binaries and libraries (login, passwd, sudo, ssh, etc.)
//! - Security policies (password quality, account lockout, resource limits)
//! - SSH server configuration
//! - Console login (getty) configuration
//! - User and group database specifications
//! - Live ISO autologin behavior
//!
//! # Architecture
//!
//! The authentication subsystem is organized into layers:
//!
//! ## 1. Authentication Mechanisms
//!
//! **PAM (Pluggable Authentication Modules)**:
//! - 17 PAM configuration files in `/etc/pam.d/`
//! - 40+ PAM security modules in `/usr/lib64/security/`
//! - Implements: YESCRYPT password hashing, account lockout, password quality, resource limits
//! - See `pam::*` for configuration file contents
//!
//! **User Database**:
//! - `/etc/passwd`: User accounts (world-readable)
//! - `/etc/shadow`: Password hashes (root-only, 0600 permissions)
//! - `/etc/group`: Group definitions (world-readable)
//! - `/etc/gshadow`: Group passwords (root-only, 0600 permissions)
//! - See `distro_spec::shared::users` for user specifications
//!
//! ## 2. Console Login
//!
//! **Getty Services**:
//! - `getty@tty1.service` on console
//! - `serial-getty@ttyS0.service` on serial port (with critical `-L` flag for QEMU)
//! - See `getty::*` for getty configuration
//!
//! **Login Binary**:
//! - `/usr/sbin/login` - actual login command
//! - `/usr/bin/login` - symlink (agetty searches PATH for this)
//! - Both in `components::AUTH_SBIN`
//!
//! ## 3. Remote Login (SSH)
//!
//! **OpenSSH Server**:
//! - `sshd` daemon with three host key types (RSA, ECDSA, Ed25519)
//! - Allows password and public key authentication
//! - Pre-generated host keys for immediate startup
//! - See `ssh::*` for SSH configuration
//!
//! ## 4. Privilege Escalation
//!
//! **sudo**: Users in wheel group get NOPASSWD access
//! **su**: User-to-user switching with password
//! **runuser**: Root-only user switching
//!
//! ## 5. OverlayFS for Live ISO
//!
//! Live ISO uses three-layer OverlayFS mount:
//! ```
//! Layer 3 (top):    tmpfs /overlay/upper           [read-write, ephemeral]
//! Layer 2 (middle): /live/overlay from ISO          [read-only, live configs]
//! Layer 1 (bottom): EROFS /rootfs                   [read-only, base system]
//! ```
//!
//! Live-specific files (applied ONLY during live boot):
//! - Empty root password in `/live/overlay/etc/shadow`
//! - Autologin services in `/live/overlay/etc/systemd/system/`
//!
//! **Key Insight**: Installed systems don't use OverlayFS. They boot with standard
//! initramfs that mounts EROFS only (no live-overlay). Therefore:
//! - Live ISO: Root has empty password (can login without password)
//! - Installed system: Root is locked (`root:!:...`)
//!
//! See `requirements.md` for full OverlayFS architecture documentation.
//!
//! # Usage
//!
//! ## For Verification (fsdbg)
//!
//! Check that all authentication components are present:
//! ```ignore
//! use distro_spec::shared::auth::components::*;
//!
//! // Verify binaries
//! assert!(AUTH_BIN.contains(&"sudo"));
//! assert!(AUTH_SBIN.contains(&"unix_chkpwd"));  // Critical for PAM!
//!
//! // Verify PAM modules
//! assert!(PAM_MODULES.contains(&"pam_unix.so"));
//!
//! // Verify PAM configs
//! assert!(PAM_CONFIGS.contains(&"etc/pam.d/login"));
//! ```
//!
//! ## For Build (leviso)
//!
//! Import PAM configuration contents:
//! ```ignore
//! use distro_spec::shared::auth::pam::*;
//!
//! // Write PAM config file
//! fs::write("/etc/pam.d/login", PAM_LOGIN)?;
//! ```
//!
//! ## For Component Lists
//!
//! Use component lists for rootfs verification:
//! ```ignore
//! use distro_spec::shared::auth::components::*;
//! use distro_spec::shared::components::*;  // Re-exported for backwards compatibility
//!
//! // All component lists are available
//! for binary in AUTH_BIN {
//!     verify_binary_exists(binary)?;
//! }
//! ```
//!
//! # Key Design Decisions
//!
//! ## 1. YESCRYPT Password Hashing
//!
//! Modern memory-hard algorithm resistant to GPU attacks. Configured in PAM as:
//! ```
//! password sufficient pam_unix.so yescrypt shadow use_authtok
//! ```
//!
//! ## 2. Serial getty with `-L` Flag for QEMU
//!
//! QEMU's emulated serial port doesn't generate proper modem signals (CD).
//! Without the `-L` flag, agetty waits forever for carrier signal.
//! Override in `definitions.rs`: replace `-` with `-L` in baud rate string.
//! See `getty::SERIAL_GETTY_OVERRIDE` for details.
//!
//! ## 3. unix_chkpwd is Critical
//!
//! pam_unix.so has a hardcoded path to `/usr/sbin/unix_chkpwd`. Without it,
//! password verification silently fails. This is documented in `components::AUTH_SBIN`.
//!
//! ## 4. Live Overlay Doesn't Affect Installed Systems
//!
//! Live overlay (with empty root password) exists on the ISO filesystem, not in EROFS.
//! When recstrap extracts the rootfs, it extracts EROFS only (not the overlay).
//! Therefore: live systems have passwordless root, installed systems have locked root.
//!
//! ## 5. No Circular Dependencies
//!
//! distro-spec is a leaf crate. It imports only from:
//! - std library
//! - Other distro-spec modules (users.rs)
//!
//! Build logic that depends on PAM configs lives in `leviso`, not here.
//!
//! # Critical Single Points of Failure
//!
//! These components are required for login to work:
//!
//! 1. **`/usr/bin/login` symlink**
//!    - agetty searches PATH for login
//!    - Without it: no login prompt appears, login fails silently
//!    - See TEAM_108 for root cause analysis
//!
//! 2. **`/usr/sbin/unix_chkpwd`**
//!    - pam_unix.so calls it with hardcoded path
//!    - Without it: password verification fails silently
//!
//! 3. **`/etc/shadow` with 0600 permissions**
//!    - Contains password hashes (secret)
//!    - If world-readable: security hole
//!    - If writable: users can corrupt password database
//!
//! 4. **PAM `login` service**
//!    - If configuration is malformed: login hangs or fails
//!    - Contains critical imports (system-auth, postlogin)
//!
//! # Verification Strategy
//!
//! **Static checks (fsdbg)**:
//! - All binaries present in rootfs
//! - All PAM modules present
//! - All PAM config files present
//! - File permissions correct
//! - Critical symlinks exist
//!
//! **Runtime checks (rootfs-tests)**:
//! - System boots to login prompt
//! - Root login works on live ISO
//! - SSH server starts
//! - PAM authentication completes
//! - No errors in systemd-logind journal
//!
//! # References
//!
//! - `requirements.md` - Complete requirements documentation
//! - TEAM_108 - Review of login architecture
//! - TEAM_143 - Udev subsystem consolidation (model for this work)
//! - init_tiny.template - OverlayFS mount code
//! - leviso/src/component/custom/pam.rs - PAM file creation logic
//! - leviso/src/component/custom/live.rs - Live overlay creation

pub mod components;
pub mod getty;
pub mod pam;
pub mod ssh;

// Re-export from users.rs for convenience (users module is in shared, not under auth)
pub use super::users;

// Public API convenience re-exports
pub use self::components::{
    AUTH_BIN, AUTH_SBIN, PAM_CONFIGS, PAM_MODULES, SECURITY_FILES, SHADOW_SBIN,
    SSH_BIN, SSH_SBIN, SUDO_LIBS,
};
pub use self::getty::{GETTY_TERM_TYPE, LIVE_CONSOLE_AUTOLOGIN_SERVICE,
    LIVE_SERIAL_CONSOLE_SERVICE, SERIAL_BAUD_RATES, SERIAL_GETTY_OVERRIDE};
pub use self::pam::{
    PAM_CHFN, PAM_CHPASSWD, PAM_CROND, PAM_LOGIN, PAM_OTHER, PAM_PASSWD,
    PAM_POSTLOGIN, PAM_REMOTE, PAM_RUNUSER, PAM_RUNUSER_L, PAM_SSHD, PAM_SU,
    PAM_SU_L, PAM_SUDO, PAM_SYSTEM_AUTH, PAM_SYSTEMD_USER,
};
pub use self::ssh::{SSHD_CONFIG_SETTINGS, SSHD_TMPFILES_CONFIG};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_module_structure() {
        // Verify the module structure is correct
        assert!(!PAM_SYSTEM_AUTH.is_empty(), "PAM system-auth config required");
        assert!(!SERIAL_GETTY_OVERRIDE.is_empty(), "Serial getty config required");
        assert!(!SSHD_TMPFILES_CONFIG.is_empty(), "SSH tmpfiles config required");
    }
}
