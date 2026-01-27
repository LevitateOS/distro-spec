//! AcornOS file paths and naming conventions.
//!
//! These constants define the exact names and locations of files
//! for AcornOS installation.
//!
//! AcornOS: Alpine Linux base, openrc, musl, busybox

// Re-export shared constants that are identical across distros
pub use crate::shared::{
    // Paths
    AMD_UCODE_FILENAME, DEFAULT_USER_GROUPS, INITRAMFS_BUILD_DIR, INITRAMFS_FILENAME,
    INITRAMFS_LIVE_OUTPUT, INTEL_UCODE_FILENAME, KERNEL_FILENAME, LOADER_CONF_FILENAME,
    OS_VERSION,
    // QEMU
    QEMU_DISK_GB, QEMU_MEMORY_GB,
};

// =============================================================================
// EROFS Constants (AcornOS-specific compression)
// =============================================================================

/// Filesystem type for the rootfs image.
pub const ROOTFS_TYPE: &str = "erofs";

/// Name of the EROFS rootfs image file.
pub const EROFS_NAME: &str = "filesystem.erofs";

/// Path to EROFS rootfs on mounted CDROM at runtime.
pub const EROFS_CDROM_PATH: &str = "/media/cdrom/live/filesystem.erofs";

/// Compression algorithm for mkfs.erofs.
///
/// AcornOS uses lz4hc for faster decompression and wider compatibility.
/// (LevitateOS uses zstd which requires erofs-utils 1.8+)
pub const EROFS_COMPRESSION: &str = "lz4hc";

/// Compression level for lz4hc (1-12, default 9).
pub const EROFS_COMPRESSION_LEVEL: u8 = 9;

/// Chunk size for mkfs.erofs (1MB).
pub const EROFS_CHUNK_SIZE: u32 = 1048576;

/// Unified rootfs name (currently EROFS).
pub const ROOTFS_NAME: &str = EROFS_NAME;

/// Unified rootfs CDROM path (currently EROFS).
pub const ROOTFS_CDROM_PATH: &str = EROFS_CDROM_PATH;

// NOTE: No squashfs constants exported. AcornOS uses EROFS only.
// Squashfs is only for reading Alpine's modloop, handled by shared::rootfs.

// =============================================================================
// ISO Constants
// =============================================================================

/// ISO volume label - used for boot device detection (root=LABEL=X in kernel params).
///
/// This MUST match what xorriso uses with the -V flag when creating the ISO.
/// The initramfs uses this label to find and mount the boot device.
pub const ISO_LABEL: &str = "ACORNOS";

// =============================================================================
// File Names
// =============================================================================

/// Name of the base system tarball.
pub const TARBALL_NAME: &str = "acornos-base.tar.xz";

/// Module installation path.
///
/// Alpine/AcornOS uses /lib/modules (traditional FHS).
/// This is where `make modules_install INSTALL_MOD_PATH=...` should place modules.
pub const MODULE_INSTALL_PATH: &str = "/lib/modules";

/// Boot entry configuration filename.
pub const BOOT_ENTRY_FILENAME: &str = "acornos.conf";

/// Default hostname for fresh installations.
pub const DEFAULT_HOSTNAME: &str = "acornos";

/// OS identification.
pub const OS_NAME: &str = "AcornOS";
pub const OS_ID: &str = "acornos";

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

// =============================================================================
// ISO Output
// =============================================================================

/// ISO output filename
pub const ISO_FILENAME: &str = "acornos.iso";

// =============================================================================
// Alpine Version Constants
// =============================================================================

/// Alpine Linux major.minor version for repository URLs.
///
/// This is used to configure /etc/apk/repositories in the final rootfs.
/// Update this when upgrading to a new Alpine release series.
pub const ALPINE_VERSION: &str = "3.23";

/// Target architecture.
pub const TARGET_ARCH: &str = "x86_64";

// =============================================================================
// Live System
// =============================================================================

/// /etc/issue message for live boot
pub const LIVE_ISSUE_MESSAGE: &str = "\nAcornOS Live - \\l\n\n";

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a UserSpec with AcornOS defaults.
pub fn default_user(username: impl Into<String>) -> crate::shared::UserSpec {
    crate::shared::UserSpec::new(username, DEFAULT_SHELL, DEFAULT_USER_GROUPS)
}

// =============================================================================
// Repository URL Functions
// =============================================================================

/// Construct Alpine main repository URL.
pub fn alpine_main_repo() -> String {
    format!(
        "https://dl-cdn.alpinelinux.org/alpine/v{}/main",
        ALPINE_VERSION
    )
}

/// Construct Alpine community repository URL.
pub fn alpine_community_repo() -> String {
    format!(
        "https://dl-cdn.alpinelinux.org/alpine/v{}/community",
        ALPINE_VERSION
    )
}
