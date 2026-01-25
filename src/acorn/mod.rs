//! AcornOS installation specification.
//!
//! AcornOS is built on:
//! - Alpine Linux (lightweight base)
//! - OpenRC (init system)
//! - musl (C library)
//! - busybox (coreutils)
//! - systemd-boot (bootloader, despite using OpenRC)

pub mod boot;
pub mod packages;
pub mod paths;
pub mod services;

pub use boot::{
    boot_entry_with_label, boot_entry_with_partuuid, boot_entry_with_root, bootctl_install_command,
    default_boot_entry, default_loader_config, BootEntry, LoaderConfig, BOOT_MODULES,
    DEFAULT_TIMEOUT, ENTRIES_DIR, ESP_MOUNT_POINT, LOADER_CONF_PATH,
};
pub use paths::{
    // Helper functions
    alpine_community_repo, alpine_iso_filename, alpine_iso_sha256_url, alpine_iso_url,
    apk_tools_static_filename, apk_tools_static_url, default_user,
    // Alpine Version Constants
    ALPINE_PATCH_VERSION,
    ALPINE_VERSION,
    APK_TOOLS_VERSION,
    TARGET_ARCH,
    // Alpine Extended ISO Download
    ALPINE_EXTENDED_ISO_FILENAME,
    ALPINE_EXTENDED_ISO_SHA256_URL,
    ALPINE_EXTENDED_ISO_SIZE,
    ALPINE_EXTENDED_ISO_URL,
    ALPINE_ISO_PATH_ENV,
    APK_TOOLS_PATH_ENV,
    APK_TOOLS_STATIC_FILENAME,
    APK_TOOLS_STATIC_SHA256,
    APK_TOOLS_STATIC_URL,
    // File names
    BOOT_ENTRY_FILENAME,
    // Initramfs Build
    BUSYBOX_SHA256,
    BUSYBOX_URL,
    BUSYBOX_URL_ENV,
    BUSYBOX_VERSION,
    DEFAULT_HOSTNAME,
    // User defaults
    DEFAULT_SHELL,
    DEFAULT_USER_GROUPS,
    INITRAMFS_BUILD_DIR,
    INITRAMFS_FILENAME,
    INITRAMFS_LIVE_OUTPUT,
    // ISO Output
    ISO_FILENAME,
    // ISO constants
    ISO_LABEL,
    KERNEL_FILENAME,
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
    // Squashfs constants
    SQUASHFS_BLOCK_SIZE,
    SQUASHFS_CDROM_PATH,
    SQUASHFS_COMPRESSION,
    SQUASHFS_NAME,
    TARBALL_NAME,
};
pub use services::{optional_services, required_services, ServiceSpec, ENABLED_SERVICES};
pub use packages::{
    all_live_packages, bootable_packages, core_packages, daily_driver_packages,
    ALPINE_KEYS, BOOTABLE_PACKAGES, CORE_PACKAGES, DAILY_DRIVER_PACKAGES, LIVE_ISO_PACKAGES,
};

// Re-export shared constants
pub use crate::shared::{
    BOOT_DEVICE_PROBE_ORDER, CPIO_GZIP_LEVEL, EFIBOOT_FILENAME, EFIBOOT_SIZE_MB, EFI_BOOTLOADER,
    EFI_GRUB, INITRAMFS_DIRS, INITRAMFS_LIVE_ISO_PATH, ISO_BOOT_DIR, ISO_CHECKSUM_SUFFIX, ISO_EFI_DIR,
    ISO_LIVE_DIR, KERNEL_ISO_PATH, LIVE_OVERLAY_ISO_PATH, MOUNT_LIVE_OVERLAY, MOUNT_NEWROOT,
    MOUNT_OVERLAY, MOUNT_SQUASHFS, QEMU_CPU_MODE, QEMU_DISK_FILENAME, QEMU_SERIAL_LOG,
    SELINUX_DISABLE, SERIAL_BAUD_RATE, SERIAL_CONSOLE, SHA512_SEPARATOR, SQUASHFS_ISO_PATH,
    VGA_CONSOLE, XORRISO_FS_FLAGS, XORRISO_PARTITION_OFFSET,
};
