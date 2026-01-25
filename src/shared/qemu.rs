//! QEMU testing defaults.

/// Virtual disk filename
pub const QEMU_DISK_FILENAME: &str = "virtual-disk.qcow2";

/// Serial log path
pub const QEMU_SERIAL_LOG: &str = "/tmp/levitateos-serial.log";

/// CPU emulation mode for TCG fallback (when KVM unavailable)
/// Uses qemu64 to avoid TCG warnings about unsupported features
pub const QEMU_CPU_MODE: &str = "qemu64";
