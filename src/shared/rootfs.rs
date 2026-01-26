//! Rootfs image constants (EROFS primary, squashfs legacy).
//!
//! EROFS (Enhanced Read-Only File System) is the primary format for live boot.
//! Used by Fedora 42+, RHEL 10, Android. Better than squashfs because:
//! - Random-access directory lookups (squashfs requires linear search)
//! - Fixed 4KB output blocks (better disk I/O alignment)
//! - Lower memory amplification during decompression
//! - More actively developed than squashfs
//!
//! For ISO paths (where rootfs lives on the ISO), see `iso.rs`.

// =============================================================================
// EROFS Build Parameters (Primary)
// =============================================================================

/// Filesystem type for the rootfs image.
///
/// EROFS: Modern, fast random reads, used by Fedora 42+/RHEL 10/Android.
pub const ROOTFS_TYPE: &str = "erofs";

/// Compression algorithm for mkfs.erofs -z flag.
///
/// zstd provides excellent compression with fast decompression.
/// Requires kernel 6.10+ (CONFIG_EROFS_FS_ZIP_ZSTD=y).
pub const EROFS_COMPRESSION: &str = "zstd";

/// Compression level for zstd (1-22, default 3).
///
/// Level 6 matches Fedora's choice - good balance of ratio and speed.
pub const EROFS_COMPRESSION_LEVEL: u8 = 6;

/// Chunk size for mkfs.erofs -C flag (in bytes).
///
/// Fedora uses 1MB (1048576) for live media.
/// Larger chunks = better compression, slightly more memory.
pub const EROFS_CHUNK_SIZE: u32 = 1048576;

/// Name of the EROFS rootfs image file.
pub const EROFS_NAME: &str = "filesystem.erofs";

/// Path to EROFS rootfs on mounted CDROM at runtime.
///
/// The tiny initramfs mounts the ISO at /media/cdrom before switch_root.
/// This is where the initramfs looks for the rootfs to mount.
pub const EROFS_CDROM_PATH: &str = "/media/cdrom/live/filesystem.erofs";

// =============================================================================
// Unified Rootfs Constants (for code that doesn't care about format)
// =============================================================================

/// Name of the rootfs image file (currently EROFS).
///
/// Use this in code that shouldn't care about the underlying format.
pub const ROOTFS_NAME: &str = EROFS_NAME;

/// Path to rootfs on mounted CDROM at runtime (currently EROFS).
///
/// Use this in code that shouldn't care about the underlying format.
pub const ROOTFS_CDROM_PATH: &str = EROFS_CDROM_PATH;

// =============================================================================
// Squashfs Build Parameters (Legacy/Fallback)
// =============================================================================

/// Squashfs compression algorithm for mksquashfs -comp flag.
///
/// Using zstd for better compression than legacy gzip.
/// NOTE: This is only used if EROFS is not available.
pub const SQUASHFS_COMPRESSION: &str = "zstd";

/// Squashfs block size for mksquashfs -b flag.
///
/// 1MB blocks provide good compression ratio for the base system.
pub const SQUASHFS_BLOCK_SIZE: &str = "1M";

/// Name of the squashfs image file (legacy).
pub const SQUASHFS_NAME: &str = "filesystem.squashfs";

/// Path to squashfs on mounted CDROM at runtime (legacy).
pub const SQUASHFS_CDROM_PATH: &str = "/media/cdrom/live/filesystem.squashfs";
