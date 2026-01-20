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

pub use boot::{BootEntry, LoaderConfig, ESP_MOUNT_POINT, LOADER_CONF_PATH, ENTRIES_DIR};
pub use paths::{
    TARBALL_NAME, KERNEL_FILENAME, INITRAMFS_FILENAME,
    BOOT_ENTRY_FILENAME, LOADER_CONF_FILENAME,
    DEFAULT_HOSTNAME, OS_NAME, OS_ID, OS_VERSION,
    DEFAULT_SHELL, ROOT_SHELL, DEFAULT_USER_GROUPS,
    default_user,
};
pub use services::{ServiceSpec, ENABLED_SERVICES, required_services, optional_services};
