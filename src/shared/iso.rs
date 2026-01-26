//! ISO structure constants shared between LevitateOS and AcornOS.

// =============================================================================
// ISO Directory Structure
// =============================================================================

/// Boot directory on ISO (contains kernel, initramfs)
pub const ISO_BOOT_DIR: &str = "boot";

/// Live directory on ISO (contains squashfs, overlay)
pub const ISO_LIVE_DIR: &str = "live";

/// EFI boot directory on ISO
pub const ISO_EFI_DIR: &str = "EFI/BOOT";

/// Path to rootfs image inside ISO (relative to ISO root)
///
/// This is the EROFS filesystem image that contains the complete system.
/// Legacy squashfs format used "live/filesystem.squashfs".
pub const ROOTFS_ISO_PATH: &str = "live/filesystem.erofs";

/// Path to squashfs inside ISO (relative to ISO root) - LEGACY
///
/// Kept for backward compatibility. New builds use ROOTFS_ISO_PATH.
pub const SQUASHFS_ISO_PATH: &str = "live/filesystem.squashfs";

/// Path to kernel inside ISO (relative to ISO root)
pub const KERNEL_ISO_PATH: &str = "boot/vmlinuz";

/// Path to live initramfs inside ISO (relative to ISO root)
/// This is the tiny initramfs that mounts squashfs for the live environment.
pub const INITRAMFS_LIVE_ISO_PATH: &str = "boot/initramfs-live.img";

/// Path to live overlay inside ISO (relative to ISO root)
pub const LIVE_OVERLAY_ISO_PATH: &str = "live/overlay";

// =============================================================================
// EFI Boot Files
// =============================================================================

/// EFI boot image filename
pub const EFIBOOT_FILENAME: &str = "efiboot.img";

/// EFI boot image size in MB.
///
/// UKIs require ~50MB each (kernel + initramfs + cmdline).
/// With 3 UKIs + systemd-boot + loader.conf, we need ~200MB.
pub const EFIBOOT_SIZE_MB: u32 = 200;

/// Primary EFI bootloader filename
pub const EFI_BOOTLOADER: &str = "BOOTX64.EFI";

/// GRUB EFI binary filename
pub const EFI_GRUB: &str = "grubx64.efi";

// =============================================================================
// Console Configuration
// =============================================================================

/// Serial console kernel parameter
pub const SERIAL_CONSOLE: &str = "console=ttyS0,115200n8";

/// VGA console kernel parameter
pub const VGA_CONSOLE: &str = "console=tty0";

/// Serial console baud rate (for documentation/validation)
pub const SERIAL_BAUD_RATE: u32 = 115200;

// =============================================================================
// Kernel Boot Parameters
// =============================================================================

/// SELinux disable parameter (Rocky/RHEL based need this for live boot)
pub const SELINUX_DISABLE: &str = "selinux=0";

// =============================================================================
// Checksum
// =============================================================================

/// ISO checksum file suffix
pub const ISO_CHECKSUM_SUFFIX: &str = ".sha512";

/// SHA512 checksum format separator (two spaces per sha512sum standard)
pub const SHA512_SEPARATOR: &str = "  ";

// =============================================================================
// xorriso Parameters
// =============================================================================

/// MBR partition offset for hybrid ISO
pub const XORRISO_PARTITION_OFFSET: u32 = 16;

/// ISO filesystem flags for xorriso
pub const XORRISO_FS_FLAGS: &[&str] = &[
    "-full-iso9660-filenames",
    "-joliet",
    "-rational-rock",
];
