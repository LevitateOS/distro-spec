//! AcornOS file paths and naming conventions.
//!
//! These constants define the exact names and locations of files
//! for AcornOS installation.
//!
//! AcornOS: Alpine Linux base, openrc, musl, busybox

/// Name of the base system tarball.
pub const TARBALL_NAME: &str = "acornos-base.tar.xz";

/// Kernel filename in /boot after installation.
pub const KERNEL_FILENAME: &str = "vmlinuz";

/// Initramfs filename in /boot after installation.
pub const INITRAMFS_FILENAME: &str = "initramfs.img";

/// Intel microcode filename (optional).
pub const INTEL_UCODE_FILENAME: &str = "intel-ucode.img";

/// AMD microcode filename (optional).
pub const AMD_UCODE_FILENAME: &str = "amd-ucode.img";

/// Boot entry configuration filename.
pub const BOOT_ENTRY_FILENAME: &str = "acornos.conf";

/// Loader configuration filename.
pub const LOADER_CONF_FILENAME: &str = "loader.conf";

/// Default hostname for fresh installations.
pub const DEFAULT_HOSTNAME: &str = "acornos";

/// OS identification.
pub const OS_NAME: &str = "AcornOS";
pub const OS_ID: &str = "acornos";
pub const OS_VERSION: &str = "1.0";

/// Possible locations where the tarball might be found during installation.
pub const TARBALL_SEARCH_PATHS: &[&str] = &[
    "/acornos-base.tar.xz",
    "/run/media/acornos-base.tar.xz",
    "/mnt/cdrom/acornos-base.tar.xz",
];

/// Find the tarball path from the search paths.
#[cfg(feature = "std")]
pub fn find_tarball() -> Option<&'static str> {
    use std::path::Path;
    TARBALL_SEARCH_PATHS
        .iter()
        .find(|p| Path::new(p).exists())
        .copied()
}

// User defaults for AcornOS

/// Default shell for new users (busybox ash).
pub const DEFAULT_SHELL: &str = "/bin/ash";

/// Root shell (busybox ash).
pub const ROOT_SHELL: &str = "/bin/ash";

/// Groups that new users should be added to by default.
pub const DEFAULT_USER_GROUPS: &[&str] = &[
    "wheel",  // sudo/doas access
    "audio",  // audio device access
    "video",  // video device access
    "input",  // input device access
];

/// Create a UserSpec with AcornOS defaults.
pub fn default_user(username: impl Into<String>) -> crate::shared::UserSpec {
    crate::shared::UserSpec::new(username, DEFAULT_SHELL, DEFAULT_USER_GROUPS)
}
