//! Bootloader configuration for AcornOS.
//!
//! Defines systemd-boot configuration for UEFI systems.
//! AcornOS uses systemd-boot despite using OpenRC for init.

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
/// Alpine kernel modules use gzip compression (.ko.gz).
/// Paths are relative to `/lib/modules/<kernel-version>/`.
///
/// **ORDER MATTERS**: Dependencies must be listed before modules that use them.
///
/// These enable:
/// - CDROM/SCSI: mounting the ISO (`cdrom`, `sr_mod`, `isofs`)
/// - Storage: real hardware support (`sd_mod`, `nvme`, `ahci`)
/// - Virtio: QEMU virtual devices (`virtio_scsi`, `virtio_blk`, `virtio_pci`)
/// - USB: USB boot media and keyboards (`usb-storage`, `usbhid`, `xhci-hcd`)
/// - Squashfs boot: mounting the root filesystem (`loop`, `squashfs`, `overlay`)
pub const BOOT_MODULES: &[&str] = &[
    // === Virtio core (must be loaded first, other modules depend on these) ===
    "kernel/drivers/virtio/virtio.ko.gz",             // Base virtio bus
    "kernel/drivers/virtio/virtio_ring.ko.gz",        // Virtqueue implementation
    "kernel/drivers/virtio/virtio_pci_modern_dev.ko.gz", // Modern PCI helper
    "kernel/drivers/virtio/virtio_pci_legacy_dev.ko.gz", // Legacy PCI helper
    "kernel/drivers/virtio/virtio_pci.ko.gz",         // PCI transport (needs above)

    // === SCSI core (needed by sr_mod, sd_mod, virtio_scsi, usb-storage) ===
    "kernel/drivers/scsi/scsi_common.ko.gz",          // SCSI common utilities
    "kernel/drivers/scsi/scsi_mod.ko.gz",             // SCSI core (needs scsi_common)

    // === CDROM/SCSI support ===
    "kernel/drivers/cdrom/cdrom.ko.gz",
    "kernel/drivers/scsi/sr_mod.ko.gz",               // Needs scsi_mod
    "kernel/drivers/scsi/sd_mod.ko.gz",               // Needs scsi_mod
    "kernel/drivers/scsi/virtio_scsi.ko.gz",          // Needs virtio_pci, scsi_mod
    "kernel/fs/isofs/isofs.ko.gz",

    // === Storage drivers (for real hardware) ===
    "kernel/drivers/nvme/host/nvme-core.ko.gz",       // NVMe core (dependency)
    "kernel/drivers/nvme/host/nvme.ko.gz",            // NVMe for modern SSDs
    "kernel/drivers/ata/libata.ko.gz",                // ATA core
    "kernel/drivers/ata/libahci.ko.gz",               // AHCI library
    "kernel/drivers/ata/ahci.ko.gz",                  // SATA support (needs libata, libahci)

    // === USB core (needed by USB storage and HID) ===
    "kernel/drivers/usb/common/usb-common.ko.gz",     // USB common utilities
    "kernel/drivers/usb/core/usbcore.ko.gz",          // USB core

    // === USB host controllers ===
    "kernel/drivers/usb/host/xhci-hcd.ko.gz",         // USB 3.0 host controller
    "kernel/drivers/usb/host/xhci-pci.ko.gz",         // xHCI PCI driver
    "kernel/drivers/usb/host/ehci-hcd.ko.gz",         // USB 2.0 host controller
    "kernel/drivers/usb/host/ehci-pci.ko.gz",         // EHCI PCI driver

    // === USB storage (for USB boot media) ===
    "kernel/drivers/usb/storage/usb-storage.ko.gz",   // USB mass storage

    // === HID (Human Interface Devices - keyboards, mice) ===
    "kernel/drivers/hid/hid.ko.gz",                   // HID core
    "kernel/drivers/hid/hid-generic.ko.gz",           // Generic HID driver
    "kernel/drivers/hid/usbhid/usbhid.ko.gz",         // USB HID (keyboards)

    // === Virtio block device ===
    "kernel/drivers/block/virtio_blk.ko.gz",          // QEMU -drive if=virtio

    // === Loop device and filesystems for squashfs+overlay boot ===
    "kernel/drivers/block/loop.ko.gz",
    "kernel/fs/squashfs/squashfs.ko.gz",
    "kernel/fs/overlayfs/overlay.ko.gz",
];

// =============================================================================
// AcornOS-Specific Constructors
// =============================================================================

/// Create a default boot entry for AcornOS.
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

/// Create a default loader config for AcornOS.
pub fn default_loader_config() -> LoaderConfig {
    LoaderConfig::with_defaults(OS_ID)
}
