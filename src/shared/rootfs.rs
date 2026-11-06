//! Rootfs image constants (EROFS only).
//!
//! EROFS (Enhanced Read-Only File System) is the primary format for live boot.
//! Used by Fedora 42+, RHEL 10, Android:
//! - Random-access directory lookups
//! - Fixed 4KB output blocks (better disk I/O alignment)
//! - Lower memory amplification during decompression
//! - Actively developed
//!
//! For ISO paths (where rootfs lives on the ISO), see `iso.rs`.
//!
//! # Installer Constants
//!
//! This module also contains constants used by recstrap and other installer tools:
//! - `ROOTFS_SEARCH_PATHS` - where to look for rootfs during installation
//! - Magic byte constants for format detection

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

/// Path to EROFS rootfs payload device in live runtime.
///
/// Stage 01+ live initramfs resolves the rootfs payload partition and
/// exposes it as a stable symlink for installer tooling.
pub const EROFS_CDROM_PATH: &str = "/run/live-rootfs.erofs";

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
// Installer Constants (used by recstrap, fsdbg, etc.)
// =============================================================================

/// Common rootfs locations to search during installation (in order of preference).
///
/// EROFS is the canonical format.
/// Used by recstrap to auto-detect the rootfs when running from the live ISO.
pub const ROOTFS_SEARCH_PATHS: &[&str] = &[
    "/run/live-rootfs.erofs",
    "/run/live-media/live/filesystem.erofs",
    "/run/initramfs/live/filesystem.erofs",
    "/run/archiso/bootmnt/live/filesystem.erofs",
    "/mnt/cdrom/live/filesystem.erofs",
    "/media/cdrom/live/filesystem.erofs",
];
