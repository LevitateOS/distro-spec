//! QEMU testing defaults shared between LevitateOS and AcornOS.

// =============================================================================
// VM Resources
// =============================================================================

/// QEMU memory allocation (GB) - Match real desktop hardware.
///
/// 8GB is the minimum for a modern desktop. This ensures tests
/// run in realistic conditions rather than artificial constraints.
pub const QEMU_MEMORY_GB: u32 = 8;

/// QEMU virtual disk size (GB) - Room for packages and user data.
///
/// 256GB matches typical NVMe sizes and gives room for package
/// installation during tests without hitting space issues.
pub const QEMU_DISK_GB: u32 = 256;

// =============================================================================
// File Paths
// =============================================================================

/// Virtual disk filename
pub const QEMU_DISK_FILENAME: &str = "virtual-disk.qcow2";

/// Serial log path
pub const QEMU_SERIAL_LOG: &str = "/tmp/levitateos-serial.log";

// =============================================================================
// CPU Configuration
// =============================================================================

/// CPU emulation mode for TCG fallback (when KVM unavailable).
///
/// Uses qemu64 to avoid TCG warnings about unsupported features.
pub const QEMU_CPU_MODE: &str = "qemu64";

// =============================================================================
// qcow2 VM Image Constants
// =============================================================================

/// Output filename for qcow2 VM images.
pub const QCOW2_IMAGE_FILENAME: &str = "levitateos.qcow2";

/// Temporary raw disk filename (converted to qcow2 after building).
pub const RAW_DISK_FILENAME: &str = "levitateos.raw";
