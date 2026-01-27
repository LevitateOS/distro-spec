//! LevitateOS installation specification.
//!
//! LevitateOS is built on:
//! - Rocky Linux (RHEL-compatible base)
//! - systemd (init, networking, boot)
//! - glibc (C library)
//! - GNU coreutils
//! - systemd-boot (bootloader)

pub mod boot;
pub mod paths;
pub mod services;
pub mod uki;

pub use boot::{
    boot_entry_with_label, boot_entry_with_partuuid, boot_entry_with_root, bootctl_install_command,
    default_boot_entry, default_loader_config, BootEntry, LoaderConfig, BOOT_MODULES,
    DEFAULT_TIMEOUT, ENTRIES_DIR, ESP_MOUNT_POINT, LOADER_CONF_PATH,
};
pub use paths::{
    // Helper functions
    default_user,
    // File names
    BOOT_ENTRY_FILENAME,
    // Initramfs Build
    BUSYBOX_URL,
    BUSYBOX_URL_ENV,
    DEFAULT_HOSTNAME,
    // User defaults
    DEFAULT_SHELL,
    DEFAULT_USER_GROUPS,
    INITRAMFS_BUILD_DIR,
    INITRAMFS_FILENAME,
    INITRAMFS_LIVE_OUTPUT,
    INITRAMFS_INSTALLED_OUTPUT,
    INITRAMFS_INSTALLED_ISO_PATH,
    // ISO Output
    ISO_FILENAME,
    // ISO constants
    ISO_LABEL,
    KERNEL_FILENAME,
    // Module installation
    MODULE_INSTALL_PATH,
    // Live System
    LIVE_ISSUE_MESSAGE,
    LOADER_CONF_FILENAME,
    // OS identity
    OS_ID,
    OS_NAME,
    OS_VERSION,
    // QEMU Testing Defaults
    QEMU_DISK_GB,
    QEMU_MEMORY_GB,
    ROOT_SHELL,
    // Rootfs constants (EROFS)
    EROFS_CDROM_PATH,
    EROFS_CHUNK_SIZE,
    EROFS_COMPRESSION,
    EROFS_COMPRESSION_LEVEL,
    EROFS_NAME,
    ROOTFS_CDROM_PATH,
    ROOTFS_NAME,
    ROOTFS_TYPE,
    TARBALL_NAME,
    // Installed UKI paths on ISO
    UKI_INSTALLED_ISO_DIR,
    UKI_INSTALLED_ISO_PATH,
    UKI_INSTALLED_RECOVERY_ISO_PATH,
};
pub use services::{optional_services, required_services, ServiceSpec, ENABLED_SERVICES};
pub use uki::{UkiEntry, UKI_ENTRIES, UKI_INSTALLED_ENTRIES};

// Re-export shared constants
pub use crate::shared::{
    BOOT_DEVICE_PROBE_ORDER, CPIO_GZIP_LEVEL, EFIBOOT_FILENAME, EFIBOOT_SIZE_MB, EFI_BOOTLOADER,
    EFI_GRUB, INITRAMFS_DIRS, INITRAMFS_LIVE_ISO_PATH, ISO_BOOT_DIR, ISO_CHECKSUM_SUFFIX, ISO_EFI_DIR,
    ISO_LIVE_DIR, KERNEL_ISO_PATH, LIVE_OVERLAY_ISO_PATH, MOUNT_LIVE_OVERLAY, MOUNT_NEWROOT,
    MOUNT_OVERLAY, MOUNT_SQUASHFS, QEMU_CPU_MODE, QEMU_DISK_FILENAME, QEMU_SERIAL_LOG,
    ROOTFS_ISO_PATH, SELINUX_DISABLE, SERIAL_BAUD_RATE, SERIAL_CONSOLE, SHA512_SEPARATOR,
    SQUASHFS_ISO_PATH, VGA_CONSOLE, XORRISO_FS_FLAGS, XORRISO_PARTITION_OFFSET,
    // UKI constants
    LOADER_ENTRIES_DIR, SYSTEMD_BOOT_EFI, SYSTEMD_BOOT_STUB, UKI_DEBUG_FILENAME,
    UKI_EFI_DIR, UKI_EMERGENCY_FILENAME, UKI_LIVE_FILENAME,
    // Installed UKI constants
    UKI_INSTALLED_FILENAME, UKI_INSTALLED_RECOVERY_FILENAME,
};
