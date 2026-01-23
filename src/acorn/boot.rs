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
/// Alpine kernel modules may differ from Rocky. These paths are relative
/// to `/lib/modules/<kernel-version>/`.
///
/// TODO: Verify Alpine-specific module paths and names.
pub const BOOT_MODULES: &[&str] = &[
    // CDROM/SCSI support (for mounting the ISO)
    // Note: Alpine may use different compression (.ko.gz vs .ko.xz)
    "kernel/drivers/cdrom/cdrom.ko.gz",
    "kernel/drivers/scsi/sr_mod.ko.gz",
    "kernel/drivers/scsi/virtio_scsi.ko.gz",
    "kernel/fs/isofs/isofs.ko.gz",
    // Virtio block device
    "kernel/drivers/block/virtio_blk.ko.gz",
    // Loop device and filesystems for squashfs+overlay boot
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
