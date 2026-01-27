//! Kernel modules required for boot, shared between LevitateOS and AcornOS.
//!
//! **ORDER MATTERS**: Dependencies must be listed before modules that use them.
//! The initramfs uses `insmod` which requires dependencies to be loaded first
//! (unlike `modprobe` which resolves them automatically).
//!
//! Paths are relative to `/lib/modules/<kernel-version>/`.

// =============================================================================
// Core Boot Modules (required by both distros)
// =============================================================================

/// Virtio core modules - must be loaded first, other virtio modules depend on these.
pub const VIRTIO_CORE: &[&str] = &[
    "kernel/drivers/virtio/virtio",       // Base virtio bus
    "kernel/drivers/virtio/virtio_ring",  // Virtqueue implementation
    "kernel/drivers/virtio/virtio_pci",   // PCI transport (needs virtio + virtio_ring)
];

/// SCSI core - needed by sr_mod, sd_mod, virtio_scsi, usb-storage.
pub const SCSI_CORE: &[&str] = &[
    "kernel/drivers/scsi/scsi_mod",
];

/// CDROM and SCSI support for mounting the ISO.
pub const CDROM_SCSI: &[&str] = &[
    "kernel/drivers/cdrom/cdrom",
    "kernel/drivers/scsi/sr_mod",      // Needs scsi_mod
    "kernel/drivers/scsi/sd_mod",      // Needs scsi_mod
    "kernel/drivers/scsi/virtio_scsi", // Needs virtio_pci, scsi_mod
    "kernel/fs/isofs/isofs",
];

/// NVMe storage for modern SSDs.
pub const NVME_STORAGE: &[&str] = &[
    "kernel/drivers/nvme/host/nvme-core", // NVMe core (dependency)
    "kernel/drivers/nvme/host/nvme",      // NVMe driver
];

/// SATA/AHCI storage for traditional drives.
pub const SATA_STORAGE: &[&str] = &[
    "kernel/drivers/ata/libata",  // ATA core
    "kernel/drivers/ata/libahci", // AHCI library
    "kernel/drivers/ata/ahci",    // SATA support (needs libata, libahci)
];

/// Virtio block device for QEMU virtual disks.
pub const VIRTIO_BLK: &[&str] = &[
    "kernel/drivers/block/virtio_blk", // QEMU -drive if=virtio -> /dev/vda
];

/// Loop device and filesystems for squashfs+overlay live boot.
pub const SQUASHFS_OVERLAY: &[&str] = &[
    "kernel/drivers/block/loop",
    "kernel/fs/squashfs/squashfs",
    "kernel/fs/overlayfs/overlay",
];

// =============================================================================
// USB Modules (optional - for USB boot media and input devices)
// =============================================================================

/// USB core modules - needed by USB storage and HID.
pub const USB_CORE: &[&str] = &[
    "kernel/drivers/usb/common/usb-common", // USB common utilities
    "kernel/drivers/usb/core/usbcore",      // USB core
];

/// USB host controller drivers (xHCI for USB 3.0, EHCI for USB 2.0).
pub const USB_HOST_CONTROLLERS: &[&str] = &[
    "kernel/drivers/usb/host/xhci-hcd", // USB 3.0 host controller
    "kernel/drivers/usb/host/xhci-pci", // xHCI PCI driver
    "kernel/drivers/usb/host/ehci-hcd", // USB 2.0 host controller
    "kernel/drivers/usb/host/ehci-pci", // EHCI PCI driver
];

/// USB mass storage for USB boot media.
pub const USB_STORAGE: &[&str] = &[
    "kernel/drivers/usb/storage/usb-storage",
];

/// HID (Human Interface Devices) for keyboards and mice.
pub const USB_HID: &[&str] = &[
    "kernel/drivers/hid/hid",             // HID core
    "kernel/drivers/hid/hid-generic",     // Generic HID driver
    "kernel/drivers/hid/usbhid/usbhid",   // USB HID (keyboards)
];

// =============================================================================
// Composed Module Lists
// =============================================================================

/// Core boot modules needed by all distros (no USB).
///
/// This is the minimum set for:
/// - QEMU/KVM boot (virtio)
/// - CDROM/ISO boot (scsi, cdrom, isofs)
/// - Real hardware boot (nvme, sata)
/// - Live squashfs boot (loop, squashfs, overlay)
pub const CORE_BOOT_MODULES: &[&str] = &[
    // Virtio core (must be first)
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
];

/// USB boot modules - add these to CORE_BOOT_MODULES for USB boot support.
///
/// Required for:
/// - Booting from USB drives
/// - USB keyboards during early boot (disk encryption, boot menu)
pub const USB_BOOT_MODULES: &[&str] = &[
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
// Install Boot Modules (for installed systems)
// =============================================================================

/// Modules for installed system boot - includes ALL common storage/fs drivers.
///
/// This is the "universal" set that boots on any hardware. Unlike the live ISO
/// which only needs CDROM modules, installed systems need modules for:
/// - NVMe, SATA, USB storage
/// - ext4, xfs, btrfs, vfat filesystems
/// - Device mapper (for future LUKS/LVM support)
/// - HID (keyboards for LUKS prompts)
///
/// **ORDER MATTERS**: Dependencies must be listed before modules that use them.
pub const INSTALL_BOOT_MODULES: &[&str] = &[
    // === Virtio core (must be first for QEMU) ===
    "kernel/drivers/virtio/virtio",
    "kernel/drivers/virtio/virtio_ring",
    "kernel/drivers/virtio/virtio_pci",

    // === SCSI core (needed by sd_mod, virtio_scsi, usb-storage) ===
    "kernel/drivers/scsi/scsi_mod",
    "kernel/drivers/scsi/sd_mod",
    "kernel/drivers/scsi/virtio_scsi",

    // === NVMe (modern SSDs) ===
    "kernel/drivers/nvme/host/nvme-core",
    "kernel/drivers/nvme/host/nvme",

    // === SATA/AHCI ===
    "kernel/drivers/ata/libata",
    "kernel/drivers/ata/libahci",
    "kernel/drivers/ata/ahci",
    "kernel/drivers/ata/ata_piix",

    // === Virtio block (QEMU virtual disks) ===
    "kernel/drivers/block/virtio_blk",

    // === USB Storage ===
    "kernel/drivers/usb/common/usb-common",
    "kernel/drivers/usb/core/usbcore",
    "kernel/drivers/usb/host/xhci-hcd",
    "kernel/drivers/usb/host/xhci-pci",
    "kernel/drivers/usb/host/ehci-hcd",
    "kernel/drivers/usb/host/ehci-pci",
    "kernel/drivers/usb/storage/usb-storage",

    // === HID (keyboards for LUKS prompts) ===
    "kernel/drivers/hid/hid",
    "kernel/drivers/hid/hid-generic",
    "kernel/drivers/hid/usbhid/usbhid",

    // === Filesystems ===
    "kernel/fs/ext4/ext4",
    "kernel/fs/xfs/xfs",
    "kernel/fs/btrfs/btrfs",
    "kernel/fs/fat/fat",
    "kernel/fs/vfat/vfat",
    "kernel/fs/nls/nls_cp437",
    "kernel/fs/nls/nls_iso8859-1",
    "kernel/fs/nls/nls_utf8",

    // === Device Mapper (for future LUKS/LVM) ===
    "kernel/drivers/md/dm-mod",
    "kernel/drivers/md/dm-crypt",
];
