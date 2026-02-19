//! Overlayfs payload image constants.
//!
//! Live boot composes a writable root from:
//! - base rootfs image (`live/filesystem.erofs`)
//! - optional live overlay image (`live/overlayfs.erofs`)
//! - tmpfs upper/work dirs
//!
//! The live overlay payload is modeled as a first-class, read-only EROFS image.

use crate::shared::rootfs::{EROFS_CHUNK_SIZE, EROFS_COMPRESSION, EROFS_COMPRESSION_LEVEL};

/// Overlay payload filesystem type.
pub const OVERLAYFS_TYPE: &str = "erofs";

/// Overlay payload image filename.
pub const OVERLAYFS_NAME: &str = "overlayfs.erofs";

/// Overlay payload path inside the ISO (relative to ISO root).
pub const OVERLAYFS_ISO_PATH: &str = "live/overlayfs.erofs";

/// Overlay payload path on mounted CDROM in live runtime.
pub const OVERLAYFS_CDROM_PATH: &str = "/run/live-overlay.erofs";

/// Compression algorithm for overlay payload image creation.
pub const OVERLAYFS_COMPRESSION: &str = EROFS_COMPRESSION;

/// Compression level for overlay payload image creation.
pub const OVERLAYFS_COMPRESSION_LEVEL: u8 = EROFS_COMPRESSION_LEVEL;

/// Chunk size for overlay payload image creation.
pub const OVERLAYFS_CHUNK_SIZE: u32 = EROFS_CHUNK_SIZE;
