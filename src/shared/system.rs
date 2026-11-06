//! System utilities shared between tools.
//!
//! Common system-level checks and operations used by recstrap, recchroot,
//! and other installer tools.

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
