//! AcornOS UKI configuration.
//!
//! Defines UKI entries for both live ISO and installed systems.
//!
//! UKIs are created with objcopy (Alpine-native, no ukify dependency).
//! systemd-boot auto-discovers UKIs in EFI/Linux/ directory.

use super::paths::{
    UKI_LIVE_FILENAME, UKI_EMERGENCY_FILENAME, UKI_DEBUG_FILENAME,
    UKI_INSTALLED_FILENAME, UKI_INSTALLED_RECOVERY_FILENAME,
};

/// A UKI boot entry definition.
#[derive(Debug, Clone)]
pub struct UkiEntry {
    /// Display name shown in boot menu.
    pub name: &'static str,
    /// Filename for the UKI (e.g., "acornos-live.efi").
    pub filename: &'static str,
    /// Extra kernel cmdline parameters appended to base cmdline.
    pub extra_cmdline: &'static str,
}

/// UKI boot entries for live ISO.
///
/// These define the boot menu entries created by systemd-boot.
pub const UKI_ENTRIES: &[UkiEntry] = &[
    UkiEntry {
        name: "AcornOS",
        filename: UKI_LIVE_FILENAME,
        extra_cmdline: "",
    },
    UkiEntry {
        name: "AcornOS (Emergency)",
        filename: UKI_EMERGENCY_FILENAME,
        extra_cmdline: "emergency",
    },
    UkiEntry {
        name: "AcornOS (Debug)",
        filename: UKI_DEBUG_FILENAME,
        extra_cmdline: "debug",
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
        extra_cmdline: "",
    },
    UkiEntry {
        name: "AcornOS (Recovery)",
        filename: UKI_INSTALLED_RECOVERY_FILENAME,
        extra_cmdline: "single",
    },
];
