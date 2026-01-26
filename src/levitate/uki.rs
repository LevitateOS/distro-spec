//! LevitateOS UKI configuration.
//!
//! Defines UKI entries for both live ISO and installed systems.
//!
//! Live UKIs use the tiny initramfs and mount the ISO's EROFS rootfs.
//! Installed UKIs use the full dracut initramfs and boot from disk.

pub use crate::shared::uki::*;

/// A UKI boot entry definition.
#[derive(Debug, Clone)]
pub struct UkiEntry {
    /// Display name shown in boot menu.
    pub name: &'static str,
    /// Filename for the UKI (e.g., "levitateos-live.efi").
    pub filename: &'static str,
    /// Extra kernel cmdline parameters appended to base cmdline.
    pub extra_cmdline: &'static str,
}

/// UKI boot entries for live ISO.
///
/// These define the boot menu entries created by systemd-boot.
pub const UKI_ENTRIES: &[UkiEntry] = &[
    UkiEntry {
        name: "LevitateOS",
        filename: UKI_LIVE_FILENAME,
        extra_cmdline: "",
    },
    UkiEntry {
        name: "LevitateOS (Emergency)",
        filename: UKI_EMERGENCY_FILENAME,
        extra_cmdline: "emergency",
    },
    UkiEntry {
        name: "LevitateOS (Debug)",
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
        name: "LevitateOS",
        filename: UKI_INSTALLED_FILENAME,
        extra_cmdline: "",
    },
    UkiEntry {
        name: "LevitateOS (Recovery)",
        filename: UKI_INSTALLED_RECOVERY_FILENAME,
        extra_cmdline: "single",
    },
];
