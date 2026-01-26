//! Squashfs build constants shared between LevitateOS and AcornOS.
//!
//! These constants define how the squashfs rootfs image is built.
//! For ISO paths (where squashfs lives on the ISO), see `iso.rs`.

// =============================================================================
// Squashfs Build Parameters
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
// Squashfs File Names and Paths
// =============================================================================

/// Name of the squashfs image file.
///
/// This is the filename used when creating the squashfs.
/// Located at /live/filesystem.squashfs on the ISO.
pub const SQUASHFS_NAME: &str = "filesystem.squashfs";

/// Path to squashfs on mounted CDROM at runtime.
///
/// The tiny initramfs mounts the ISO at /media/cdrom before switch_root.
/// This is where the initramfs looks for the rootfs to mount.
pub const SQUASHFS_CDROM_PATH: &str = "/media/cdrom/live/filesystem.squashfs";
