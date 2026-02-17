//! IuppiterOS installation specification.
//!
//! # Deprecation Notice
//!
//! This module is deprecated and will move to `distro-variants/iuppiter`.
//!
//! IuppiterOS is built on:
//! - Alpine Linux (lightweight base)
//! - OpenRC (init system)
//! - musl (C library)
//! - busybox (coreutils)
//! - systemd-boot (bootloader, despite using OpenRC)
//! - UKI boot (kernel+initramfs+cmdline in single PE binary)
//!
//! Purpose: Headless HDD refurbishment server appliance (64+ drive slots).
//! NOT a daily-driver desktop — minimal packages, no GUI, serial console.

pub mod boot;
pub mod packages;
pub mod paths;
pub mod services;
pub mod uki;
pub mod verification;

pub use boot::{
    boot_entry_with_label, boot_entry_with_partuuid, boot_entry_with_root, bootctl_install_command,
    default_boot_entry, default_loader_config, BootEntry, LoaderConfig, BOOT_MODULES,
    DEFAULT_TIMEOUT, ENTRIES_DIR, ESP_MOUNT_POINT, LOADER_CONF_PATH,
};
pub use packages::{
    all_live_packages, bootable_packages, refurbishment_packages, server_core_packages,
    ALPINE_KEYS, BOOTABLE_PACKAGES, LIVE_ISO_PACKAGES, REFURBISHMENT_PACKAGES,
    SERVER_CORE_PACKAGES,
};
pub use paths::{
    // Helper functions
    alpine_community_repo,
    alpine_main_repo,
    default_user,
    operator_user,
    // Alpine Version Constants
    ALPINE_VERSION,
    // File names
    BOOT_ENTRY_FILENAME,
    // Appliance-specific
    DATA_MOUNT_POINT,
    DEFAULT_HOSTNAME,
    // User defaults
    DEFAULT_SHELL,
    DEFAULT_USER_GROUPS,
    // Disk Image
    DISK_IMAGE_FILENAME,
    DISK_IMAGE_SIZE_GB,
    ENGINE_BINARY,
    ENGINE_CONFIG,
    ENGINE_CONFIG_DIR,
    // Rootfs constants (EROFS only)
    EROFS_CDROM_PATH,
    EROFS_CHUNK_SIZE,
    EROFS_COMPRESSION,
    EROFS_COMPRESSION_LEVEL,
    EROFS_NAME,
    // Initramfs Build
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
    // UKI constants
    LOADER_ENTRIES_DIR,
    // Module installation
    MODULE_INSTALL_PATH,
    OPERATOR_GROUPS,
    // OS identity
    OS_ID,
    OS_NAME,
    OS_VERSION,
    // QEMU Testing Defaults
    QEMU_DISK_GB,
    QEMU_MEMORY_GB,
    ROOTFS_CDROM_PATH,
    ROOTFS_NAME,
    ROOTFS_TYPE,
    ROOT_SHELL,
    SYSTEMD_BOOT_EFI,
    SYSTEMD_BOOT_STUB,
    TARBALL_NAME,
    TARGET_ARCH,
    UKI_DEBUG_FILENAME,
    UKI_EFI_DIR,
    UKI_EMERGENCY_FILENAME,
    UKI_INSTALLED_FILENAME,
    UKI_INSTALLED_RECOVERY_FILENAME,
    UKI_LIVE_FILENAME,
};
pub use services::{optional_services, required_services, ServiceSpec, ENABLED_SERVICES};
pub use uki::{UkiEntry, UKI_ENTRIES, UKI_INSTALLED_ENTRIES};

// Kernel source specification
pub use crate::shared::IUPPITER_KERNEL as KERNEL_SOURCE;

// Re-export shared constants (no squashfs - EROFS only)
pub use crate::shared::{
    BOOT_DEVICE_PROBE_ORDER, CPIO_GZIP_LEVEL, EFIBOOT_FILENAME, EFIBOOT_SIZE_MB, EFI_BOOTLOADER,
    EFI_GRUB, INITRAMFS_DIRS, INITRAMFS_LIVE_ISO_PATH, ISO_BOOT_DIR, ISO_CHECKSUM_SUFFIX,
    ISO_EFI_DIR, ISO_LIVE_DIR, KERNEL_ISO_PATH, LIVE_OVERLAYFS_ISO_PATH, LIVE_OVERLAY_ISO_PATH,
    MOUNT_LIVE_OVERLAY, MOUNT_NEWROOT, MOUNT_OVERLAY, QEMU_CPU_MODE, QEMU_DISK_FILENAME,
    QEMU_SERIAL_LOG, ROOTFS_ISO_PATH, SELINUX_DISABLE, SERIAL_BAUD_RATE, SERIAL_CONSOLE,
    SHA512_SEPARATOR, VGA_CONSOLE, XORRISO_FS_FLAGS, XORRISO_PARTITION_OFFSET,
};
