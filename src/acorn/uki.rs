//! AcornOS UKI configuration.
//!
//! Defines UKI entries for both live ISO and installed systems.
//!
//! UKIs are created with objcopy (Alpine-native, no ukify dependency).
//! systemd-boot auto-discovers UKIs in EFI/Linux/ directory.

use super::paths::{
    UKI_DEBUG_FILENAME, UKI_EMERGENCY_FILENAME, UKI_INSTALLED_FILENAME,
    UKI_INSTALLED_RECOVERY_FILENAME, UKI_LIVE_FILENAME,
};
pub use crate::shared::UkiEntry;

/// UKI boot entries for live ISO.
///
/// These define the boot menu entries created by systemd-boot.
/// Console parameters are in extra_cmdline so base cmdline stays clean.
pub const UKI_ENTRIES: &[UkiEntry] = &[
    UkiEntry {
        name: "AcornOS",
        filename: UKI_LIVE_FILENAME,
        extra_cmdline: "console=tty0 console=ttyS0,115200n8",
    },
    UkiEntry {
        name: "AcornOS (Emergency)",
        filename: UKI_EMERGENCY_FILENAME,
        extra_cmdline: "emergency console=tty0 console=ttyS0,115200n8",
    },
    UkiEntry {
        name: "AcornOS (Debug)",
        filename: UKI_DEBUG_FILENAME,
        extra_cmdline: "debug console=tty0 console=ttyS0,115200n8",
    },
];

/// UKI boot entries for installed systems.
///
/// These are pre-built during ISO creation and placed in boot/uki/.
/// Users copy them to /boot/EFI/Linux/ during installation.
/// systemd-boot auto-discovers UKIs in that directory.
pub const UKI_INSTALLED_ENTRIES: &[UkiEntry] = &[
    UkiEntry {
        name: "AcornOS",
        filename: UKI_INSTALLED_FILENAME,
        extra_cmdline: "console=tty0 console=ttyS0,115200n8",
    },
    UkiEntry {
        name: "AcornOS (Recovery)",
        filename: UKI_INSTALLED_RECOVERY_FILENAME,
        extra_cmdline: "single console=tty0 console=ttyS0,115200n8",
    },
];
