//! Device paths for boot media detection.

/// Devices to probe for ISO/boot media (in order)
pub const BOOT_DEVICE_PROBE_ORDER: &[&str] = &[
    "/dev/sr0",      // CD/DVD drive
    "/dev/sda",      // First SATA/SCSI disk
    "/dev/sdb",      // Second SATA/SCSI disk
    "/dev/vda",      // VirtIO disk (QEMU)
    "/dev/nvme0n1",  // NVMe drive
];
