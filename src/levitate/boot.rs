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
/// must copy these from the rootfs to enable ISO boot. Order matters for dependencies.
///
/// These enable:
/// - CDROM/SCSI: mounting the ISO (`cdrom`, `sr_mod`, `isofs`)
/// - Virtio: QEMU virtual devices (`virtio_scsi`, `virtio_blk`)
/// - Squashfs boot: mounting the root filesystem (`loop`, `squashfs`, `overlay`)
///
/// Paths are relative to `/lib/modules/<kernel-version>/`.
pub const BOOT_MODULES: &[&str] = &[
    // CDROM/SCSI support (for mounting the ISO)
    "kernel/drivers/cdrom/cdrom.ko.xz",
    "kernel/drivers/scsi/sr_mod.ko.xz",
    "kernel/drivers/scsi/virtio_scsi.ko.xz",
    "kernel/fs/isofs/isofs.ko.xz",
    // Virtio block device (QEMU -drive if=virtio -> /dev/vda)
    "kernel/drivers/block/virtio_blk.ko.xz",
    // Loop device and filesystems for squashfs+overlay boot
    "kernel/drivers/block/loop.ko.xz",
    "kernel/fs/squashfs/squashfs.ko.xz",
    "kernel/fs/overlayfs/overlay.ko.xz",
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
