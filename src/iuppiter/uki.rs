//! IuppiterOS UKI configuration.
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
/// IuppiterOS boots headless — serial console is the primary interface.
pub const UKI_ENTRIES: &[UkiEntry] = &[
    UkiEntry {
        name: "IuppiterOS",
        filename: UKI_LIVE_FILENAME,
        extra_cmdline: "console=ttyS0,115200n8 console=tty0",
    },
    UkiEntry {
        name: "IuppiterOS (Emergency)",
        filename: UKI_EMERGENCY_FILENAME,
        extra_cmdline: "emergency console=ttyS0,115200n8 console=tty0",
    },
    UkiEntry {
        name: "IuppiterOS (Debug)",
        filename: UKI_DEBUG_FILENAME,
        extra_cmdline: "debug console=ttyS0,115200n8 console=tty0",
    },
];

/// UKI boot entries for installed systems.
pub const UKI_INSTALLED_ENTRIES: &[UkiEntry] = &[
    UkiEntry {
        name: "IuppiterOS",
        filename: UKI_INSTALLED_FILENAME,
        extra_cmdline: "console=ttyS0,115200n8 console=tty0",
    },
    UkiEntry {
        name: "IuppiterOS (Recovery)",
        filename: UKI_INSTALLED_RECOVERY_FILENAME,
        extra_cmdline: "single console=ttyS0,115200n8 console=tty0",
    },
];
