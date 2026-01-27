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
//!
//! # Installer Constants
//!
//! This module also contains constants used by recstrap and other installer tools:
//! - `ROOTFS_SEARCH_PATHS` - where to look for rootfs during installation
//! - `ESSENTIAL_DIRS` - directories that must exist after extraction
//! - `MIN_REQUIRED_BYTES` - minimum disk space for installation
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

// =============================================================================
// Installer Constants (used by recstrap, fsdbg, etc.)
// =============================================================================

/// Common rootfs locations to search during installation (in order of preference).
///
/// EROFS paths are listed first as it's the modern format (Fedora 42+, LevitateOS).
/// Used by recstrap to auto-detect the rootfs when running from the live ISO.
pub const ROOTFS_SEARCH_PATHS: &[&str] = &[
    // EROFS (modern - LevitateOS default)
    "/media/cdrom/live/filesystem.erofs",
    "/run/initramfs/live/filesystem.erofs",
    "/run/archiso/bootmnt/live/filesystem.erofs",
    "/mnt/cdrom/live/filesystem.erofs",
    // Squashfs (legacy fallback)
    "/media/cdrom/live/filesystem.squashfs",
    "/run/initramfs/live/filesystem.squashfs",
    "/run/archiso/bootmnt/live/filesystem.squashfs",
    "/mnt/cdrom/live/filesystem.squashfs",
];

/// Essential directories that must exist after rootfs extraction.
///
/// Used by recstrap to verify that extraction completed successfully.
/// A missing directory indicates a corrupt or incomplete rootfs.
pub const ESSENTIAL_DIRS: &[&str] = &["bin", "etc", "lib", "sbin", "usr", "var"];

/// Minimum required disk space in bytes for rootfs extraction.
///
/// 2GB is typical - a compressed rootfs expands to roughly this size.
/// recstrap checks available space before extraction to fail fast.
pub const MIN_REQUIRED_BYTES: u64 = 2 * 1024 * 1024 * 1024;

// =============================================================================
// Format Detection (Magic Bytes)
// =============================================================================

/// EROFS magic number (little-endian at offset 1024).
///
/// Used to validate that a file is actually an EROFS image before mounting.
/// ```text
/// Offset 1024: 0xe2 0xe1 0xf5 0xe0 (little-endian: 0xe0f5e1e2)
/// ```
pub const EROFS_MAGIC: u32 = 0xe0f5e1e2;

/// EROFS magic byte offset from start of file.
pub const EROFS_MAGIC_OFFSET: u64 = 1024;

/// Squashfs magic bytes at offset 0.
///
/// Used to validate that a file is actually a squashfs image before extraction.
/// The magic is "hsqs" (little-endian "sqsh").
pub const SQUASHFS_MAGIC: &[u8; 4] = b"hsqs";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_essential_dirs_list() {
        assert!(ESSENTIAL_DIRS.contains(&"bin"));
        assert!(ESSENTIAL_DIRS.contains(&"etc"));
        assert!(ESSENTIAL_DIRS.contains(&"usr"));
        assert!(ESSENTIAL_DIRS.contains(&"lib"));
        assert!(ESSENTIAL_DIRS.contains(&"var"));
    }

    #[test]
    fn test_rootfs_search_paths_not_empty() {
        assert!(!ROOTFS_SEARCH_PATHS.is_empty());
    }

    #[test]
    fn test_rootfs_search_paths_valid_extensions() {
        for path in ROOTFS_SEARCH_PATHS {
            assert!(
                path.ends_with(".erofs") || path.ends_with(".squashfs"),
                "Path {} should end with .erofs or .squashfs",
                path
            );
        }
    }

    #[test]
    fn test_rootfs_search_paths_erofs_first() {
        // EROFS should be preferred (listed first)
        assert!(
            ROOTFS_SEARCH_PATHS[0].ends_with(".erofs"),
            "First search path should be EROFS"
        );
    }

    #[test]
    fn test_min_required_bytes_is_reasonable() {
        // Should be at least 1GB, at most 10GB
        assert!(MIN_REQUIRED_BYTES >= 1024 * 1024 * 1024);
        assert!(MIN_REQUIRED_BYTES <= 10 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_erofs_magic_constant() {
        // EROFS magic is 0xe0f5e1e2 (little-endian)
        assert_eq!(EROFS_MAGIC, 0xe0f5e1e2);
    }

    #[test]
    fn test_erofs_magic_offset() {
        // EROFS superblock is at offset 1024
        assert_eq!(EROFS_MAGIC_OFFSET, 1024);
    }

    #[test]
    fn test_squashfs_magic_constant() {
        // Squashfs magic is "hsqs"
        assert_eq!(SQUASHFS_MAGIC, b"hsqs");
    }

    #[test]
    fn test_cdrom_paths_match_search_paths() {
        // The CDROM paths should be in the search paths
        assert!(ROOTFS_SEARCH_PATHS.contains(&EROFS_CDROM_PATH));
        assert!(ROOTFS_SEARCH_PATHS.contains(&SQUASHFS_CDROM_PATH));
    }
}
