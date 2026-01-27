//! System utilities shared between tools.
//!
//! Common system-level checks and operations used by recstrap, recchroot,
//! and other installer tools.

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

/// Check if the current process is running as root (euid == 0).
///
/// This is a common pre-flight check for tools that require root privileges
/// to perform operations like mounting filesystems, chrooting, or extracting
/// system images.
///
/// # Example
///
/// ```rust,ignore
/// use distro_spec::shared::is_root;
///
/// if !is_root() {
///     eprintln!("This tool must be run as root");
///     std::process::exit(1);
/// }
/// ```
pub fn is_root() -> bool {
    // SAFETY: geteuid() is a simple syscall with no safety requirements
    unsafe { libc::geteuid() == 0 }
}

/// Check if a path is a mount point by comparing device IDs with its parent.
///
/// A path is considered a mount point if its device ID differs from its parent's,
/// indicating that a filesystem is mounted there. The root path "/" is always
/// considered a mount point.
///
/// # Example
///
/// ```rust,ignore
/// use distro_spec::shared::is_mount_point;
/// use std::path::Path;
///
/// // Check if /mnt is a mount point before extracting
/// if !is_mount_point(Path::new("/mnt"))? {
///     eprintln!("Warning: /mnt is not a mount point");
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the path or its parent cannot be accessed (e.g., doesn't
/// exist or permission denied).
pub fn is_mount_point(path: &Path) -> std::io::Result<bool> {
    let path_meta = fs::metadata(path)?;
    let path_dev = path_meta.dev();

    // Get parent directory
    let parent = match path.parent() {
        Some(p) if p.as_os_str().is_empty() => Path::new("/"),
        Some(p) => p,
        None => return Ok(true), // Root is always a mount point
    };

    let parent_meta = fs::metadata(parent)?;
    let parent_dev = parent_meta.dev();

    // If device IDs differ, it's a mount point
    Ok(path_dev != parent_dev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_root_returns_bool() {
        // Just verify it doesn't panic and returns a bool
        let _ = is_root();
    }

    #[test]
    fn test_is_mount_point_root() {
        // Root should always be a mount point
        assert!(is_mount_point(Path::new("/")).unwrap());
    }

    #[test]
    fn test_is_mount_point_nonexistent() {
        // Nonexistent path should return error
        let result = is_mount_point(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_is_mount_point_tmp() {
        // /tmp typically exists; whether it's a mount point varies by system
        // Just verify it doesn't panic
        if Path::new("/tmp").exists() {
            let _ = is_mount_point(Path::new("/tmp"));
        }
    }
}
