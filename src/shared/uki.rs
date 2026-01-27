//! Unified Kernel Image (UKI) constants.
//!
//! UKIs combine kernel + initramfs + cmdline into a single signed PE binary.
//! This simplifies boot and enables Secure Boot with a single file to sign.

/// Directory for UKIs on the EFI system partition.
pub const UKI_EFI_DIR: &str = "EFI/Linux";

/// systemd-boot EFI stub path (from systemd package).
/// Used by ukify to create UKI binaries.
pub const SYSTEMD_BOOT_STUB: &str = "/usr/lib/systemd/boot/efi/linuxx64.efi.stub";

/// systemd-boot binary path.
/// This is copied to EFI/BOOT/BOOTX64.EFI to serve as the bootloader.
pub const SYSTEMD_BOOT_EFI: &str = "/usr/lib/systemd/boot/efi/systemd-bootx64.efi";

/// Default UKI filename for live boot.
pub const UKI_LIVE_FILENAME: &str = "levitateos-live.efi";

/// UKI filename for emergency shell.
pub const UKI_EMERGENCY_FILENAME: &str = "levitateos-emergency.efi";

/// UKI filename for debug mode.
pub const UKI_DEBUG_FILENAME: &str = "levitateos-debug.efi";

// =============================================================================
// Installed System UKIs
// =============================================================================
// These UKIs are for installed systems (daily driver boot).
// They use the full initramfs and root=LABEL=root cmdline.

/// UKI filename for installed system normal boot.
pub const UKI_INSTALLED_FILENAME: &str = "levitateos.efi";

/// UKI filename for installed system recovery mode.
pub const UKI_INSTALLED_RECOVERY_FILENAME: &str = "levitateos-recovery.efi";

/// loader.conf directory on EFI system partition.
pub const LOADER_ENTRIES_DIR: &str = "loader";
