//! Common file paths and naming conventions shared between LevitateOS and AcornOS.
//!
//! These constants are identical across both distros. Distro-specific paths
//! (like ISO_LABEL, TARBALL_NAME, MODULE_INSTALL_PATH) remain in their
//! respective modules.

// =============================================================================
// Kernel and Boot Files
// =============================================================================

/// Kernel filename in /boot after installation.
///
/// Both distros use the standard "vmlinuz" name for the compressed kernel.
pub const KERNEL_FILENAME: &str = "vmlinuz";

/// Initramfs filename in /boot after installation.
///
/// Both distros use "initramfs.img" for the installed system's initramfs.
pub const INITRAMFS_FILENAME: &str = "initramfs.img";

/// Loader configuration filename for systemd-boot.
///
/// Both distros use systemd-boot with the standard loader.conf name.
pub const LOADER_CONF_FILENAME: &str = "loader.conf";

// =============================================================================
// Microcode Files
// =============================================================================

/// Intel microcode filename (optional).
///
/// Loaded by the bootloader before the kernel for CPU microcode updates.
pub const INTEL_UCODE_FILENAME: &str = "intel-ucode.img";

/// AMD microcode filename (optional).
///
/// Loaded by the bootloader before the kernel for CPU microcode updates.
pub const AMD_UCODE_FILENAME: &str = "amd-ucode.img";

// =============================================================================
// Initramfs Build
// =============================================================================

/// Initramfs build directory name.
///
/// Temporary directory where the initramfs contents are assembled before
/// compression into a cpio archive.
pub const INITRAMFS_BUILD_DIR: &str = "initramfs-live-root";

/// Live initramfs output filename.
///
/// This is the tiny initramfs that mounts the EROFS rootfs for the live environment.
/// Compressed with gzip for universal bootloader compatibility.
pub const INITRAMFS_LIVE_OUTPUT: &str = "initramfs-live.cpio.gz";

// =============================================================================
// User Defaults
// =============================================================================

/// Groups that new users should be added to by default.
///
/// These groups provide access to hardware devices needed for a desktop system:
/// - wheel: sudo/doas access for administrative tasks
/// - audio: audio device access (speakers, microphone)
/// - video: video device access (GPU, webcam)
/// - input: input device access (keyboard, mouse, gamepad)
pub const DEFAULT_USER_GROUPS: &[&str] = &[
    "wheel", // sudo/doas access
    "audio", // audio device access
    "video", // video device access
    "input", // input device access
];

// =============================================================================
// Protected Paths
// =============================================================================

/// Protected system paths that should never be extraction/chroot targets.
///
/// These are critical system directories that would be destroyed if used
/// as installation targets. Tools like recstrap and recchroot use this list
/// to prevent catastrophic user errors.
pub const PROTECTED_PATHS: &[&str] = &[
    "/",
    "/bin",
    "/boot",
    "/boot/efi",
    "/dev",
    "/efi",
    "/etc",
    "/home",
    "/lib",
    "/lib32",
    "/lib64",
    "/opt",
    "/proc",
    "/root",
    "/run",
    "/sbin",
    "/srv",
    "/sys",
    "/tmp",
    "/usr",
    "/usr/local",
    "/var",
];

/// Check if a path is a protected system path.
///
/// The path is resolved first (symlinks via `canonicalize` when it exists,
/// otherwise `.`/`..` components lexically), so `/mnt/../usr` or a symlink
/// pointing at `/etc` is caught. Only the resolved path itself is compared;
/// children such as `/mnt/usr` are NOT protected.
pub fn is_protected_path(path: &std::path::Path) -> bool {
    let resolved = resolve_for_protection(path);
    PROTECTED_PATHS
        .iter()
        .any(|protected| resolved == std::path::Path::new(protected))
}

fn resolve_for_protection(path: &std::path::Path) -> std::path::PathBuf {
    use std::path::{Component, PathBuf};

    if let Ok(canonical) = std::fs::canonicalize(path) {
        return canonical;
    }

    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    if out.as_os_str().is_empty() {
        out.push("/");
    }
    out
}

// =============================================================================
// Version
// =============================================================================

// =============================================================================
// Library Directories
// =============================================================================

/// Library directories to copy shared libraries from.
///
/// Alpine uses musl (no multilib), so we use /usr/lib (not /usr/lib64).
pub const LIBRARY_DIRS: &[&str] = &["lib", "usr/lib"];

// =============================================================================
// Version
// =============================================================================

/// OS version number.
///
/// Both distros share the same version for coordinated releases.
pub const OS_VERSION: &str = "1.0";
