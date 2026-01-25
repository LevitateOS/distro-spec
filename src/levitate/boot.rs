//! Bootloader configuration for LevitateOS.
//!
//! Defines systemd-boot configuration for UEFI systems and kernel module
//! requirements for the initramfs.

use super::paths::{KERNEL_FILENAME, INITRAMFS_FILENAME, OS_NAME, OS_ID};

// Re-export shared boot types
pub use crate::shared::boot::{
    BootEntry, LoaderConfig,
    ESP_MOUNT_POINT, LOADER_CONF_PATH, ENTRIES_DIR, DEFAULT_TIMEOUT,
    bootctl_install_command,
};

// =============================================================================
// Initramfs Kernel Modules
// =============================================================================

/// Kernel modules required in the initramfs for boot.
///
/// The Rocky kernel has these as modules (not built-in). The initramfs builder
/// must copy these from the rootfs to enable ISO boot.
///
/// **ORDER MATTERS**: Dependencies must be listed before modules that use them.
/// The initramfs uses `insmod` which requires dependencies to be loaded first
/// (unlike `modprobe` which resolves them automatically).
///
/// These enable:
/// - CDROM/SCSI: mounting the ISO (`cdrom`, `sr_mod`, `isofs`)
/// - Storage: real hardware support (`sd_mod`, `nvme`, `ahci`)
/// - Virtio: QEMU virtual devices (`virtio_scsi`, `virtio_blk`, `virtio_pci`)
/// - Squashfs boot: mounting the root filesystem (`loop`, `squashfs`, `overlay`)
///
/// Paths are relative to `/lib/modules/<kernel-version>/`.
/// Rocky kernel uses uncompressed `.ko` files (unlike Alpine's `.ko.gz`).
pub const BOOT_MODULES: &[&str] = &[
    // === Virtio core (must be loaded first, other modules depend on these) ===
    "kernel/drivers/virtio/virtio",           // Base virtio bus
    "kernel/drivers/virtio/virtio_ring",      // Virtqueue implementation
    "kernel/drivers/virtio/virtio_pci",       // PCI transport (needs virtio + virtio_ring)

    // === SCSI core (needed by sr_mod, sd_mod, virtio_scsi) ===
    "kernel/drivers/scsi/scsi_mod",           // SCSI core

    // === CDROM/SCSI support ===
    "kernel/drivers/cdrom/cdrom",
    "kernel/drivers/scsi/sr_mod",             // Needs scsi_mod
    "kernel/drivers/scsi/sd_mod",             // Needs scsi_mod
    "kernel/drivers/scsi/virtio_scsi",        // Needs virtio_pci, scsi_mod
    "kernel/fs/isofs/isofs",

    // === Storage drivers (for real hardware) ===
    "kernel/drivers/nvme/host/nvme-core",     // NVMe core (dependency)
    "kernel/drivers/nvme/host/nvme",          // NVMe for modern SSDs
    "kernel/drivers/ata/libata",              // ATA core
    "kernel/drivers/ata/libahci",             // AHCI library
    "kernel/drivers/ata/ahci",                // SATA support (needs libata, libahci)

    // === Virtio block device ===
    "kernel/drivers/block/virtio_blk",        // QEMU -drive if=virtio -> /dev/vda

    // === Loop device and filesystems for squashfs+overlay boot ===
    "kernel/drivers/block/loop",
    "kernel/fs/squashfs/squashfs",
    "kernel/fs/overlayfs/overlay",
];

// =============================================================================
// LevitateOS-Specific Constructors
// =============================================================================

/// Create a default boot entry for LevitateOS.
pub fn default_boot_entry() -> BootEntry {
    BootEntry::with_defaults(OS_ID, OS_NAME, KERNEL_FILENAME, INITRAMFS_FILENAME)
}

/// Create a boot entry with the given root device.
pub fn boot_entry_with_root(root_device: impl Into<String>) -> BootEntry {
    BootEntry::with_root(OS_ID, OS_NAME, KERNEL_FILENAME, INITRAMFS_FILENAME, root_device)
}

/// Create a boot entry using PARTUUID.
pub fn boot_entry_with_partuuid(partuuid: impl Into<String>) -> BootEntry {
    BootEntry::with_partuuid(OS_ID, OS_NAME, KERNEL_FILENAME, INITRAMFS_FILENAME, partuuid)
}

/// Create a boot entry using LABEL.
pub fn boot_entry_with_label(label: impl Into<String>) -> BootEntry {
    BootEntry::with_label(OS_ID, OS_NAME, KERNEL_FILENAME, INITRAMFS_FILENAME, label)
}

/// Create a default loader config for LevitateOS.
pub fn default_loader_config() -> LoaderConfig {
    LoaderConfig::with_defaults(OS_ID)
}
