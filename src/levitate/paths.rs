//! LevitateOS file paths and naming conventions.
//!
//! These constants define the exact names and locations of files
//! produced by leviso and expected during installation.
//!
//! LevitateOS: Rocky Linux base, systemd, glibc, GNU coreutils

// Re-export shared constants that are identical across distros
pub use crate::shared::{
    // Paths
    AMD_UCODE_FILENAME, DEFAULT_USER_GROUPS, INITRAMFS_BUILD_DIR, INITRAMFS_FILENAME,
    INITRAMFS_LIVE_OUTPUT, INTEL_UCODE_FILENAME, KERNEL_FILENAME, LOADER_CONF_FILENAME,
    OS_VERSION,
    // QEMU
    QEMU_DISK_GB, QEMU_MEMORY_GB,
    // Rootfs (EROFS)
    EROFS_CDROM_PATH, EROFS_CHUNK_SIZE, EROFS_COMPRESSION, EROFS_COMPRESSION_LEVEL, EROFS_NAME,
    ROOTFS_CDROM_PATH, ROOTFS_NAME, ROOTFS_TYPE,
};

// =============================================================================
// ISO Constants
// =============================================================================

/// ISO volume label - used for boot device detection (root=LABEL=X in kernel params).
///
/// This MUST match what xorriso uses with the -V flag when creating the ISO.
/// The initramfs uses this label to find and mount the boot device.
pub const ISO_LABEL: &str = "LEVITATEOS";

// =============================================================================
// File Names
// =============================================================================

/// Name of the base system tarball.
///
/// Used by rootfs-tests for container-based testing.
/// Not used for installation (recstrap uses squashfs).
pub const TARBALL_NAME: &str = "levitateos-base.tar.xz";

/// Module installation path (UsrMerge compliant).
///
/// LevitateOS uses /usr/lib/modules per UsrMerge.
/// This is where `make modules_install INSTALL_MOD_PATH=...` should place modules.
pub const MODULE_INSTALL_PATH: &str = "/usr/lib/modules";

/// Boot entry configuration filename.
pub const BOOT_ENTRY_FILENAME: &str = "levitateos.conf";

/// Default hostname for fresh installations.
pub const DEFAULT_HOSTNAME: &str = "levitateos";

/// OS identification.
pub const OS_NAME: &str = "LevitateOS";
pub const OS_ID: &str = "levitateos";

/// Possible locations where the tarball might be found.
pub const TARBALL_SEARCH_PATHS: &[&str] = &[
    "/levitateos-base.tar.xz",
    "/run/media/levitateos-base.tar.xz",
    "/mnt/cdrom/levitateos-base.tar.xz",
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

// =============================================================================
// ISO Output
// =============================================================================

/// ISO output filename
pub const ISO_FILENAME: &str = "levitateos.iso";

// =============================================================================
// Initramfs Build
// =============================================================================

/// Busybox binary URL (default, can be overridden via BUSYBOX_URL env)
pub const BUSYBOX_URL: &str =
    "https://busybox.net/downloads/binaries/1.35.0-x86_64-linux-musl/busybox";

/// Environment variable name for busybox URL override
pub const BUSYBOX_URL_ENV: &str = "BUSYBOX_URL";

/// Installed initramfs output filename (full dracut - boots the daily driver OS)
pub const INITRAMFS_INSTALLED_OUTPUT: &str = "initramfs-installed.img";

/// Installed initramfs path on ISO (copied to /boot/initramfs.img during installation)
pub const INITRAMFS_INSTALLED_ISO_PATH: &str = "boot/initramfs-installed.img";

// =============================================================================
// Installed UKIs (for installed systems)
// =============================================================================

/// Directory on ISO containing pre-built UKIs for installed systems.
/// Users copy these to /boot/EFI/Linux/ during installation.
pub const UKI_INSTALLED_ISO_DIR: &str = "boot/uki";

/// Installed UKI path on ISO (normal boot).
pub const UKI_INSTALLED_ISO_PATH: &str = "boot/uki/levitateos.efi";

/// Installed UKI path on ISO (recovery mode).
pub const UKI_INSTALLED_RECOVERY_ISO_PATH: &str = "boot/uki/levitateos-recovery.efi";

// =============================================================================
// Live System
// =============================================================================

/// /etc/issue message for live boot
pub const LIVE_ISSUE_MESSAGE: &str = "\nLevitateOS Live - \\l\n\n";

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a UserSpec with LevitateOS defaults.
pub fn default_user(username: impl Into<String>) -> crate::shared::UserSpec {
    crate::shared::UserSpec::new(username, DEFAULT_SHELL, DEFAULT_USER_GROUPS)
}
