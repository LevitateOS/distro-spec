//! LevitateOS file paths and naming conventions.
//!
//! These constants define the exact names and locations of files
//! produced by leviso and expected during installation.
//!
//! LevitateOS: Rocky Linux base, systemd, glibc, GNU coreutils

// =============================================================================
// ISO Constants
// =============================================================================

/// ISO volume label - used for boot device detection (root=LABEL=X in kernel params).
///
/// This MUST match what xorriso uses with the -V flag when creating the ISO.
/// The initramfs uses this label to find and mount the boot device.
pub const ISO_LABEL: &str = "LEVITATEOS";

// =============================================================================
// Squashfs Build Constants
// =============================================================================

/// Squashfs compression algorithm for mksquashfs -comp flag.
///
/// Using gzip for universal kernel compatibility.
/// (zstd requires CONFIG_SQUASHFS_ZSTD=y which not all kernels have)
pub const SQUASHFS_COMPRESSION: &str = "gzip";

/// Squashfs block size for mksquashfs -b flag.
///
/// 1MB blocks provide good compression ratio for the base system.
pub const SQUASHFS_BLOCK_SIZE: &str = "1M";

// =============================================================================
// File Names
// =============================================================================

/// Name of the base system tarball.
///
/// Used by rootfs-tests for container-based testing.
/// Not used for installation (recstrap uses squashfs).
pub const TARBALL_NAME: &str = "levitateos-base.tar.xz";

/// Name of the squashfs image (preferred for installation).
///
/// Faster than tarball extraction - uses unsquashfs directly.
/// Located at /live/filesystem.squashfs on the ISO.
pub const SQUASHFS_NAME: &str = "filesystem.squashfs";

/// Path to squashfs on mounted CDROM.
/// The tiny initramfs mounts ISO at /media/cdrom before switch_root.
pub const SQUASHFS_CDROM_PATH: &str = "/media/cdrom/live/filesystem.squashfs";

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

/// Groups that new users should be added to by default.
pub const DEFAULT_USER_GROUPS: &[&str] = &[
    "wheel",  // sudo access
    "audio",  // audio device access
    "video",  // video device access
    "input",  // input device access
];

// =============================================================================
// ISO Output
// =============================================================================

/// ISO output filename
pub const ISO_FILENAME: &str = "levitateos.iso";

// =============================================================================
// QEMU Testing Defaults
// =============================================================================

/// QEMU memory allocation (GB) - LevitateOS needs more for glibc + systemd
pub const QEMU_MEMORY_GB: u32 = 4;

/// QEMU virtual disk size (GB)
pub const QEMU_DISK_GB: u32 = 20;

// =============================================================================
// Initramfs Build
// =============================================================================

/// Busybox binary URL (default, can be overridden via BUSYBOX_URL env)
pub const BUSYBOX_URL: &str =
    "https://busybox.net/downloads/binaries/1.35.0-x86_64-linux-musl/busybox";

/// Environment variable name for busybox URL override
pub const BUSYBOX_URL_ENV: &str = "BUSYBOX_URL";

/// Initramfs build directory name
pub const INITRAMFS_BUILD_DIR: &str = "initramfs-live-root";

/// Live initramfs output filename (tiny - mounts squashfs for live environment)
pub const INITRAMFS_LIVE_OUTPUT: &str = "initramfs-live.cpio.gz";

/// Installed initramfs output filename (full dracut - boots the daily driver OS)
pub const INITRAMFS_INSTALLED_OUTPUT: &str = "initramfs-installed.img";

/// Installed initramfs path on ISO (copied to /boot/initramfs.img during installation)
pub const INITRAMFS_INSTALLED_ISO_PATH: &str = "boot/initramfs-installed.img";

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
