//! AcornOS file paths and naming conventions.
//!
//! These constants define the exact names and locations of files
//! for AcornOS installation.
//!
//! AcornOS: Alpine Linux base, openrc, musl, busybox

// =============================================================================
// ISO Constants
// =============================================================================

/// ISO volume label - used for boot device detection (root=LABEL=X in kernel params).
///
/// This MUST match what xorriso uses with the -V flag when creating the ISO.
/// The initramfs uses this label to find and mount the boot device.
pub const ISO_LABEL: &str = "ACORNOS";

// =============================================================================
// Squashfs Build Constants
// =============================================================================

/// Squashfs compression algorithm for mksquashfs -comp flag.
///
/// Using gzip for universal kernel compatibility.
/// Alpine kernels typically support both gzip and zstd.
pub const SQUASHFS_COMPRESSION: &str = "gzip";

/// Squashfs block size for mksquashfs -b flag.
///
/// 1MB blocks provide good compression ratio for the base system.
pub const SQUASHFS_BLOCK_SIZE: &str = "1M";

// =============================================================================
// File Names
// =============================================================================

/// Name of the base system tarball.
pub const TARBALL_NAME: &str = "acornos-base.tar.xz";

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
    "wheel", // sudo/doas access
    "audio", // audio device access
    "video", // video device access
    "input", // input device access
];

// =============================================================================
// ISO Output
// =============================================================================

/// ISO output filename
pub const ISO_FILENAME: &str = "acornos.iso";

// =============================================================================
// QEMU Testing Defaults
// =============================================================================

/// QEMU memory allocation (GB) - Match real desktop hardware
pub const QEMU_MEMORY_GB: u32 = 8;

/// QEMU virtual disk size (GB) - Room for packages and user data
pub const QEMU_DISK_GB: u32 = 256;

// =============================================================================
// Alpine Extended ISO Download
// =============================================================================
// Alpine Extended ISO is like Rocky DVD - includes microcode + ~200 packages.
// This allows offline installation with hardware microcode updates.

/// Alpine Linux version for package downloads.
pub const ALPINE_VERSION: &str = "3.21";

/// Alpine Extended ISO URL (includes microcode + ~200 packages).
///
/// Extended ISO is the Alpine equivalent of Rocky DVD - has offline packages.
/// Size: ~1GB (includes apks/ folder with local package repository).
pub const ALPINE_EXTENDED_ISO_URL: &str =
    "https://dl-cdn.alpinelinux.org/alpine/v3.21/releases/x86_64/alpine-extended-3.21.3-x86_64.iso";

/// SHA256 checksum URL for Alpine Extended ISO.
pub const ALPINE_EXTENDED_ISO_SHA256_URL: &str =
    "https://dl-cdn.alpinelinux.org/alpine/v3.21/releases/x86_64/alpine-extended-3.21.3-x86_64.iso.sha256";

/// Alpine Extended ISO filename (for downloads directory).
pub const ALPINE_EXTENDED_ISO_FILENAME: &str = "alpine-extended-3.21.3-x86_64.iso";

/// Expected size of the Alpine Extended ISO in bytes (~994MB).
/// Used for download progress calculation and verification.
/// alpine-extended-3.21.3-x86_64.iso = 1042284544 bytes
pub const ALPINE_EXTENDED_ISO_SIZE: u64 = 1_042_284_544;

/// apk-tools-static URL for package management without Alpine host.
///
/// This allows running `apk` to install packages without needing an Alpine system.
/// The static binary can bootstrap an Alpine rootfs from any Linux host.
pub const APK_TOOLS_STATIC_URL: &str =
    "https://dl-cdn.alpinelinux.org/alpine/v3.21/main/x86_64/apk-tools-static-2.14.6-r3.apk";

/// apk-tools-static filename.
pub const APK_TOOLS_STATIC_FILENAME: &str = "apk-tools-static-2.14.6-r3.apk";

/// Environment variable to override Alpine ISO path.
pub const ALPINE_ISO_PATH_ENV: &str = "ALPINE_ISO_PATH";

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
