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

// Re-export shared boot module constants for direct access
pub use crate::shared::boot_modules::{CORE_BOOT_MODULES, USB_BOOT_MODULES};

// =============================================================================
// Initramfs Kernel Modules
// =============================================================================

/// Kernel modules required in the initramfs for boot.
///
/// AcornOS includes both core modules AND USB modules for:
/// - USB boot media support (boot from USB drives)
/// - USB keyboard support during early boot (disk encryption prompts)
///
/// This combines `CORE_BOOT_MODULES` + `USB_BOOT_MODULES` from shared.
/// See `shared::boot_modules` for module group definitions.
///
/// Paths are relative to `/lib/modules/<kernel-version>/`.
/// Extensions are omitted - the initramfs builder will find the correct
/// compressed version (.ko.zst for custom-built, .ko.gz for Alpine packages).
pub const BOOT_MODULES: &[&str] = &[
    // =========================================================================
    // CORE_BOOT_MODULES (from shared::boot_modules)
    // =========================================================================
    // Virtio core
    "kernel/drivers/virtio/virtio",
    "kernel/drivers/virtio/virtio_ring",
    "kernel/drivers/virtio/virtio_pci",
    // SCSI core
    "kernel/drivers/scsi/scsi_mod",
    // CDROM/SCSI
    "kernel/drivers/cdrom/cdrom",
    "kernel/drivers/scsi/sr_mod",
    "kernel/drivers/scsi/sd_mod",
    "kernel/drivers/scsi/virtio_scsi",
    "kernel/fs/isofs/isofs",
    // NVMe
    "kernel/drivers/nvme/host/nvme-core",
    "kernel/drivers/nvme/host/nvme",
    // SATA
    "kernel/drivers/ata/libata",
    "kernel/drivers/ata/libahci",
    "kernel/drivers/ata/ahci",
    // Virtio block
    "kernel/drivers/block/virtio_blk",
    // Squashfs/overlay
    "kernel/drivers/block/loop",
    "kernel/fs/squashfs/squashfs",
    "kernel/fs/overlayfs/overlay",

    // =========================================================================
    // USB_BOOT_MODULES (from shared::boot_modules)
    // =========================================================================
    // USB core
    "kernel/drivers/usb/common/usb-common",
    "kernel/drivers/usb/core/usbcore",
    // Host controllers
    "kernel/drivers/usb/host/xhci-hcd",
    "kernel/drivers/usb/host/xhci-pci",
    "kernel/drivers/usb/host/ehci-hcd",
    "kernel/drivers/usb/host/ehci-pci",
    // Storage
    "kernel/drivers/usb/storage/usb-storage",
    // HID
    "kernel/drivers/hid/hid",
    "kernel/drivers/hid/hid-generic",
    "kernel/drivers/hid/usbhid/usbhid",
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
