//! Initramfs build constants shared between distros.

// =============================================================================
// Initramfs Directory Structure
// =============================================================================

/// Directories to create in initramfs root
pub const INITRAMFS_DIRS: &[&str] = &[
    "bin",
    "dev",
    "proc",
    "sys",
    "tmp",
    "mnt",
    "squashfs",
    "overlay",
    "newroot",
    "live-overlay",
];

// =============================================================================
// Mount Points (used by init script)
// =============================================================================

/// Squashfs mount point in initramfs
pub const MOUNT_SQUASHFS: &str = "/squashfs";

/// Overlay lower dir mount point
pub const MOUNT_OVERLAY: &str = "/overlay";

/// New root mount point (switch_root target)
pub const MOUNT_NEWROOT: &str = "/newroot";

/// Live overlay mount point
pub const MOUNT_LIVE_OVERLAY: &str = "/live-overlay";

// =============================================================================
// Compression
// =============================================================================

/// CPIO compression level for gzip
pub const CPIO_GZIP_LEVEL: u32 = 9;
