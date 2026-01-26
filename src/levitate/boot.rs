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
/// LevitateOS uses the core boot modules (no USB support in initramfs).
/// USB devices work after the full system boots.
///
/// See `shared::boot_modules` for module group definitions and documentation.
///
/// Paths are relative to `/lib/modules/<kernel-version>/`.
/// Rocky kernel uses uncompressed `.ko` files (unlike Alpine's `.ko.gz`).
pub use crate::shared::boot_modules::CORE_BOOT_MODULES as BOOT_MODULES;

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
