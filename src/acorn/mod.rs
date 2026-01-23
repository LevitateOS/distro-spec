//! AcornOS installation specification.
//!
//! AcornOS is built on:
//! - Alpine Linux (lightweight base)
//! - OpenRC (init system)
//! - musl (C library)
//! - busybox (coreutils)
//! - systemd-boot (bootloader, despite using OpenRC)

pub mod boot;
pub mod paths;
pub mod services;

pub use boot::{
    BootEntry, LoaderConfig, ESP_MOUNT_POINT, LOADER_CONF_PATH, ENTRIES_DIR, DEFAULT_TIMEOUT,
    BOOT_MODULES, bootctl_install_command,
    default_boot_entry, default_loader_config,
    boot_entry_with_root, boot_entry_with_partuuid, boot_entry_with_label,
};
pub use paths::{
    // ISO constants
    ISO_LABEL,
    // Squashfs constants
    SQUASHFS_NAME, SQUASHFS_CDROM_PATH, SQUASHFS_COMPRESSION, SQUASHFS_BLOCK_SIZE,
    // File names
    TARBALL_NAME, KERNEL_FILENAME, INITRAMFS_FILENAME,
    BOOT_ENTRY_FILENAME, LOADER_CONF_FILENAME,
    // OS identity
    DEFAULT_HOSTNAME, OS_NAME, OS_ID, OS_VERSION,
    // User defaults
    DEFAULT_SHELL, ROOT_SHELL, DEFAULT_USER_GROUPS,
    default_user,
};
pub use services::{ServiceSpec, ENABLED_SERVICES, required_services, optional_services};
