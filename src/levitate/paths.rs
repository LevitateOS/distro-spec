//! LevitateOS file paths and naming conventions.
//!
//! These constants define the exact names and locations of files
//! produced by leviso and expected during installation.
//!
//! LevitateOS: Rocky Linux base, systemd, glibc, GNU coreutils

/// Name of the base system tarball.
///
/// This is the "stage3" tarball containing the complete base system.
/// Located on the ISO at the root or in a media mount.
pub const TARBALL_NAME: &str = "levitateos-base.tar.xz";

/// Kernel filename in /boot after installation.
pub const KERNEL_FILENAME: &str = "vmlinuz";

/// Initramfs filename in /boot after installation.
pub const INITRAMFS_FILENAME: &str = "initramfs.img";

/// Intel microcode filename (optional).
pub const INTEL_UCODE_FILENAME: &str = "intel-ucode.img";

/// AMD microcode filename (optional).
pub const AMD_UCODE_FILENAME: &str = "amd-ucode.img";

/// Boot entry configuration filename.
pub const BOOT_ENTRY_FILENAME: &str = "levitateos.conf";

/// Loader configuration filename.
pub const LOADER_CONF_FILENAME: &str = "loader.conf";

/// Default hostname for fresh installations.
pub const DEFAULT_HOSTNAME: &str = "levitateos";

/// OS identification.
pub const OS_NAME: &str = "LevitateOS";
pub const OS_ID: &str = "levitateos";
pub const OS_VERSION: &str = "1.0";

/// Possible locations where the tarball might be found during installation.
///
/// install-tests and installers should check these in order.
pub const TARBALL_SEARCH_PATHS: &[&str] = &[
    // Direct path (copied to live system)
    "/levitateos-base.tar.xz",
    // Mounted ISO/USB
    "/run/media/levitateos-base.tar.xz",
    "/mnt/cdrom/levitateos-base.tar.xz",
    // Legacy/alternative names for compatibility
    "/stage3.tar.xz",
    "/mnt/cdrom/stage3.tar.xz",
];

/// Find the tarball path from the search paths.
///
/// Returns the first path that exists, or None if not found.
#[cfg(feature = "std")]
pub fn find_tarball() -> Option<&'static str> {
    use std::path::Path;
    TARBALL_SEARCH_PATHS
        .iter()
        .find(|p| Path::new(p).exists())
        .copied()
}

// User defaults for LevitateOS

/// Default shell for new users (GNU bash).
pub const DEFAULT_SHELL: &str = "/bin/bash";

/// Root shell (GNU bash).
pub const ROOT_SHELL: &str = "/bin/bash";

/// Groups that new users should be added to by default.
pub const DEFAULT_USER_GROUPS: &[&str] = &[
    "wheel",  // sudo access
    "audio",  // audio device access
    "video",  // video device access
    "input",  // input device access
];

/// Create a UserSpec with LevitateOS defaults.
pub fn default_user(username: impl Into<String>) -> crate::shared::UserSpec {
    crate::shared::UserSpec::new(username, DEFAULT_SHELL, DEFAULT_USER_GROUPS)
}
