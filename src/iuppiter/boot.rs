//! Bootloader configuration for IuppiterOS.
//!
//! Defines systemd-boot configuration for UEFI systems.
//! IuppiterOS uses systemd-boot despite using OpenRC for init.
//!
//! Boot modules include SAS HBA drivers for 64+ slot refurbishment servers.

use super::paths::{INITRAMFS_FILENAME, KERNEL_FILENAME, OS_ID, OS_NAME};

// Re-export shared boot types
pub use crate::shared::boot::{
    bootctl_install_command, BootEntry, LoaderConfig, DEFAULT_TIMEOUT, ENTRIES_DIR,
    ESP_MOUNT_POINT, LOADER_CONF_PATH,
};

// Re-export shared boot module constants for direct access
pub use crate::shared::boot_modules::CORE_BOOT_MODULES;

// =============================================================================
// Initramfs Kernel Modules
// =============================================================================

/// Kernel modules required in the initramfs for boot.
///
/// IuppiterOS is a headless refurbishment server with 64+ SATA/SAS drive slots.
/// Boot modules include:
/// - Core storage (SATA/AHCI, NVMe, virtio for QEMU testing)
/// - SAS HBA drivers (server hardware uses SAS backplanes)
/// - SCSI enclosure services (slot mapping, LED control)
/// - No USB modules (server boots from internal disk, not USB)
///
/// Paths are relative to `/lib/modules/<kernel-version>/`.
pub const BOOT_MODULES: &[&str] = &[
    // =========================================================================
    // CORE BOOT MODULES (from shared::boot_modules)
    // =========================================================================
    // Virtio core (QEMU testing)
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
    // SATA/AHCI
    "kernel/drivers/ata/libata",
    "kernel/drivers/ata/libahci",
    "kernel/drivers/ata/ahci",
    // Virtio block (QEMU)
    "kernel/drivers/block/virtio_blk",
    // Rootfs
    "kernel/drivers/block/loop",
    "kernel/fs/erofs/erofs",
    "kernel/fs/overlayfs/overlay",

    // =========================================================================
    // SAS / HBA MODULES (server-specific)
    // =========================================================================
    // SAS transport layer
    "kernel/drivers/scsi/scsi_transport_sas",
    // LSI/Broadcom/Avago — most common server HBAs (9200, 9300, 9400 series)
    "kernel/drivers/scsi/mpt3sas/mpt3sas",
    // Broadcom MegaRAID SAS — Dell PERC, Lenovo, etc.
    "kernel/drivers/scsi/megaraid/megaraid_sas",

    // =========================================================================
    // SCSI ENCLOSURE SERVICES (slot-to-device mapping)
    // =========================================================================
    // SES — read enclosure status, slot numbers, LED control, temperature
    "kernel/drivers/misc/enclosure",
    "kernel/drivers/scsi/ses",

    // =========================================================================
    // SCSI GENERIC (SG_IO passthrough for smartctl/hdparm)
    // =========================================================================
    "kernel/drivers/scsi/sg",
];

// =============================================================================
// IuppiterOS-Specific Constructors
// =============================================================================

/// Create a default boot entry for IuppiterOS.
pub fn default_boot_entry() -> BootEntry {
    BootEntry::with_defaults(OS_ID, OS_NAME, KERNEL_FILENAME, INITRAMFS_FILENAME)
}

/// Create a boot entry with the given root device.
pub fn boot_entry_with_root(root_device: impl Into<String>) -> BootEntry {
    BootEntry::with_root(
        OS_ID,
        OS_NAME,
        KERNEL_FILENAME,
        INITRAMFS_FILENAME,
        root_device,
    )
}

/// Create a boot entry using PARTUUID.
pub fn boot_entry_with_partuuid(partuuid: impl Into<String>) -> BootEntry {
    BootEntry::with_partuuid(
        OS_ID,
        OS_NAME,
        KERNEL_FILENAME,
        INITRAMFS_FILENAME,
        partuuid,
    )
}

/// Create a boot entry using LABEL.
pub fn boot_entry_with_label(label: impl Into<String>) -> BootEntry {
    BootEntry::with_label(
        OS_ID,
        OS_NAME,
        KERNEL_FILENAME,
        INITRAMFS_FILENAME,
        label,
    )
}

/// Create a default loader config for IuppiterOS.
pub fn default_loader_config() -> LoaderConfig {
    LoaderConfig::with_defaults(OS_ID)
}
