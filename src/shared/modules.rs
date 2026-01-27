//! Kernel module definitions for initramfs.
//!
//! SINGLE SOURCE OF TRUTH for which kernel modules are needed in each initramfs type.
//!
//! Consumers:
//! - `recinit` - builds initramfs, constructs full paths from these names
//! - `fsdbg` - verifies initramfs contents against these lists

// =============================================================================
// LIVE INITRAMFS MODULES
// =============================================================================

/// Kernel modules for live ISO boot (busybox initramfs).
///
/// These modules enable booting from CD/USB and mounting the EROFS rootfs.
/// Order matters for dependencies when using insmod.
///
/// NOTE: EROFS is built-in to kernel (CONFIG_EROFS_FS=y), no module needed.
pub const LIVE_MODULES: &[&str] = &[
    // === Virtio core (QEMU support - must load first) ===
    "virtio",
    "virtio_ring",
    "virtio_pci",
    // === SCSI core (dependency for sr_mod, sd_mod, virtio_scsi) ===
    "scsi_mod",
    // === CDROM/SCSI for ISO mount ===
    "cdrom",
    "sr_mod",
    "sd_mod",
    "virtio_scsi",
    "isofs",
    // === NVMe (modern SSDs - for USB boot sticks on NVMe systems) ===
    "nvme-core",
    "nvme",
    // === SATA/AHCI (traditional drives) ===
    "libata",
    "libahci",
    "ahci",
    // === Virtio block (QEMU virtual disks) ===
    "virtio_blk",
    // === Loop device and overlay (for EROFS + overlay boot) ===
    "loop",
    "overlay",
];

/// Modules typically built-in to LevitateOS kernel (won't exist as .ko files).
///
/// LevitateOS uses a custom kernel config that builds essential boot
/// modules into the kernel for faster boot times.
pub const LIVE_MODULES_BUILTIN: &[&str] = &[
    "erofs",       // CONFIG_EROFS_FS=y
    "loop",        // CONFIG_BLK_DEV_LOOP=y
    "overlay",     // CONFIG_OVERLAY_FS=y
    // Virtio core (QEMU support)
    "virtio",
    "virtio_ring",
    "virtio_pci",
    "virtio_scsi",
    "virtio_blk",
    // SCSI core
    "scsi_mod",
    "sd_mod",
    // CDROM
    "cdrom",
    "sr_mod",
    "isofs",
    // NVMe
    "nvme-core",
    "nvme",
    // SATA/AHCI
    "libata",
    "libahci",
    "ahci",
];

// =============================================================================
// INSTALL INITRAMFS MODULES
// =============================================================================

/// Kernel modules for installed system boot (systemd initramfs).
///
/// Superset of storage drivers plus filesystem support for booting
/// from any hardware configuration (NVMe, SATA, USB, etc.).
pub const INSTALL_MODULES: &[&str] = &[
    // === Virtio core (QEMU support) ===
    "virtio",
    "virtio_ring",
    "virtio_pci",
    "virtio_scsi",
    "virtio_blk",
    // === SCSI core ===
    "scsi_mod",
    "sd_mod",
    // === NVMe (modern SSDs) ===
    "nvme-core",
    "nvme",
    // === SATA/AHCI (traditional drives) ===
    "libata",
    "libahci",
    "ahci",
    "ata_piix",
    // === USB Storage ===
    "usb-common",
    "usbcore",
    "xhci-hcd",
    "xhci-pci",
    "ehci-hcd",
    "ehci-pci",
    "usb-storage",
    // === HID (keyboards for LUKS prompts) ===
    "hid",
    "hid-generic",
    "usbhid",
    // === Filesystems ===
    "ext4",
    "xfs",
    "btrfs",
    "fat",
    "vfat",
    "nls_cp437",
    "nls_iso8859-1",
    "nls_utf8",
    // === Device Mapper (LUKS/LVM) ===
    "dm-mod",
    "dm-crypt",
];

/// Modules typically built-in to LevitateOS kernel (won't exist as .ko files).
///
/// LevitateOS uses a custom kernel config that builds essential boot
/// modules into the kernel for faster boot times. These modules are
/// =y (built-in) instead of =m (module) in the kernel config.
pub const INSTALL_MODULES_BUILTIN: &[&str] = &[
    // === Virtio core (QEMU support) ===
    "virtio",
    "virtio_ring",
    "virtio_pci",
    "virtio_scsi",
    "virtio_blk",
    // === SCSI core ===
    "scsi_mod",
    "sd_mod",
    // === NVMe (modern SSDs) ===
    "nvme-core",
    "nvme",
    // === SATA/AHCI (traditional drives) ===
    "libata",
    "libahci",
    "ahci",
    "ata_piix",
    // === USB Storage ===
    "usb-common",
    "usbcore",
    "xhci-hcd",
    "xhci-pci",
    "ehci-hcd",
    "ehci-pci",
    "usb-storage",
    // === HID (keyboards for LUKS prompts) ===
    "hid",
    "hid-generic",
    "usbhid",
    // === Filesystems ===
    "ext4",
    "xfs",
    "btrfs",
    "fat",
    "vfat",
    "nls_cp437",
    "nls_iso8859-1",
    "nls_utf8",
    // === Device Mapper (LUKS/LVM) ===
    "dm-mod",
    "dm-crypt",
];

// =============================================================================
// MODULE PATH CONSTRUCTION
// =============================================================================

/// Known kernel source paths for modules.
///
/// Maps module name to kernel source tree path (without .ko extension).
/// Used by recinit to construct full paths for copying.
pub const MODULE_PATHS: &[(&str, &str)] = &[
    // Virtio
    ("virtio", "kernel/drivers/virtio/virtio"),
    ("virtio_ring", "kernel/drivers/virtio/virtio_ring"),
    ("virtio_pci", "kernel/drivers/virtio/virtio_pci"),
    ("virtio_scsi", "kernel/drivers/scsi/virtio_scsi"),
    ("virtio_blk", "kernel/drivers/block/virtio_blk"),
    // SCSI
    ("scsi_mod", "kernel/drivers/scsi/scsi_mod"),
    ("sd_mod", "kernel/drivers/scsi/sd_mod"),
    ("sr_mod", "kernel/drivers/scsi/sr_mod"),
    // CDROM
    ("cdrom", "kernel/drivers/cdrom/cdrom"),
    ("isofs", "kernel/fs/isofs/isofs"),
    // NVMe
    ("nvme-core", "kernel/drivers/nvme/host/nvme-core"),
    ("nvme", "kernel/drivers/nvme/host/nvme"),
    // SATA/AHCI
    ("libata", "kernel/drivers/ata/libata"),
    ("libahci", "kernel/drivers/ata/libahci"),
    ("ahci", "kernel/drivers/ata/ahci"),
    ("ata_piix", "kernel/drivers/ata/ata_piix"),
    // Loop/Overlay
    ("loop", "kernel/drivers/block/loop"),
    ("overlay", "kernel/fs/overlayfs/overlay"),
    // USB
    ("usb-common", "kernel/drivers/usb/common/usb-common"),
    ("usbcore", "kernel/drivers/usb/core/usbcore"),
    ("xhci-hcd", "kernel/drivers/usb/host/xhci-hcd"),
    ("xhci-pci", "kernel/drivers/usb/host/xhci-pci"),
    ("ehci-hcd", "kernel/drivers/usb/host/ehci-hcd"),
    ("ehci-pci", "kernel/drivers/usb/host/ehci-pci"),
    ("usb-storage", "kernel/drivers/usb/storage/usb-storage"),
    // HID
    ("hid", "kernel/drivers/hid/hid"),
    ("hid-generic", "kernel/drivers/hid/hid-generic"),
    ("usbhid", "kernel/drivers/hid/usbhid/usbhid"),
    // Filesystems
    ("ext4", "kernel/fs/ext4/ext4"),
    ("xfs", "kernel/fs/xfs/xfs"),
    ("btrfs", "kernel/fs/btrfs/btrfs"),
    ("fat", "kernel/fs/fat/fat"),
    ("vfat", "kernel/fs/vfat/vfat"),
    ("nls_cp437", "kernel/fs/nls/nls_cp437"),
    ("nls_iso8859-1", "kernel/fs/nls/nls_iso8859-1"),
    ("nls_utf8", "kernel/fs/nls/nls_utf8"),
    // Device Mapper
    ("dm-mod", "kernel/drivers/md/dm-mod"),
    ("dm-crypt", "kernel/drivers/md/dm-crypt"),
];

/// Get the kernel path for a module name.
///
/// Returns the path relative to `/lib/modules/<version>/` without extension.
pub fn module_path(name: &str) -> Option<&'static str> {
    MODULE_PATHS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, p)| *p)
}

/// Get full paths for a list of module names.
///
/// Returns paths suitable for copying from kernel modules directory.
/// Modules not in MODULE_PATHS are skipped with a warning.
#[cfg(feature = "std")]
pub fn module_paths_for(modules: &[&str]) -> Vec<&'static str> {
    modules
        .iter()
        .filter_map(|name| module_path(name))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_modules_have_paths() {
        for module in LIVE_MODULES {
            if !LIVE_MODULES_BUILTIN.contains(module) {
                assert!(
                    module_path(module).is_some(),
                    "Missing path for live module: {}",
                    module
                );
            }
        }
    }

    #[test]
    fn test_install_modules_have_paths() {
        for module in INSTALL_MODULES {
            assert!(
                module_path(module).is_some(),
                "Missing path for install module: {}",
                module
            );
        }
    }

    #[test]
    fn test_module_path_lookup() {
        assert_eq!(
            module_path("virtio"),
            Some("kernel/drivers/virtio/virtio")
        );
        assert_eq!(
            module_path("ext4"),
            Some("kernel/fs/ext4/ext4")
        );
        assert_eq!(module_path("nonexistent"), None);
    }
}
