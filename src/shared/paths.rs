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
/// This is the tiny initramfs that mounts the squashfs for the live environment.
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
// Version
// =============================================================================

/// OS version number.
///
/// Both distros share the same version for coordinated releases.
pub const OS_VERSION: &str = "1.0";
