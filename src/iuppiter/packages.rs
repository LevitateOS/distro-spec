//! IuppiterOS package tier definitions.
//!
//! This module defines package tiers for building IuppiterOS images.
//! IuppiterOS is a headless HDD refurbishment server appliance.
//!
//! - **Tier 0 (Bootable)**: Kernel + init + minimal filesystem
//! - **Tier 1 (Server Core)**: Networking, SSH, device management
//! - **Tier 2 (Refurbishment)**: Disk diagnostics, SAS/SCSI tools
//! - **Tier 3 (Live ISO)**: Installer tools (only on install media)
//!
//! # NOT included (vs AcornOS daily-driver)
//!
//! - No WiFi (iwd, wireless-regdb)
//! - No audio (sof-firmware, pipewire)
//! - No desktop (no DE, no Wayland compositor, no fonts)
//! - No LUKS/LVM (appliance rootfs is EROFS, data partition is plain ext4)
//! - No Btrfs (not needed for appliance)
//!
//! # Usage
//!
//! ```rust
//! use distro_spec::iuppiter::packages::{BOOTABLE_PACKAGES, all_live_packages};
//!
//! // Get all packages for live ISO build
//! let packages = all_live_packages();
//! println!("Installing {} packages", packages.len());
//! ```

// =============================================================================
// Tier 0: Bootable Minimum
// =============================================================================
// Absolute minimum for kernel to boot and hand off to init.

/// Tier 0: Absolute minimum for boot.
///
/// These packages provide:
/// - Base Alpine system (alpine-base includes musl, busybox, apk-tools)
/// - Kernel and initramfs generation
/// - OpenRC init system
/// - EFI boot support
pub const BOOTABLE_PACKAGES: &[&str] = &[
    // Alpine base metapackage (musl, busybox, apk-tools)
    "alpine-base",
    // Init system
    "openrc",
    "openrc-init",
    // Kernel
    "linux-lts",
    // Bootloader
    "grub",
    "grub-efi",
    "efibootmgr",
    // Filesystem essentials
    "e2fsprogs",  // ext4 — root and data partitions
    "dosfstools", // FAT — EFI System Partition
    "util-linux", // mount, fdisk, blkid, lsblk
];

// =============================================================================
// Tier 1: Server Core
// =============================================================================
// Networking, SSH, device management. The system is broken without these.

/// Tier 1: Server core packages.
///
/// Adds to Tier 0:
/// - Device management (eudev for udev rules on /dev/sd*)
/// - Hardware firmware and microcode
/// - Network (wired only — no WiFi)
/// - SSH (remote management)
/// - Certificates (HTTPS for API)
/// - Time synchronization
pub const SERVER_CORE_PACKAGES: &[&str] = &[
    // Device management (udev rules for /dev/sd* enumeration)
    "eudev",
    "eudev-openrc",
    // Hardware firmware
    "linux-firmware",
    "intel-ucode",
    "amd-ucode",
    // Login support
    "util-linux-login",
    // Shell
    "bash",
    "coreutils",
    // Privilege escalation
    "doas",
    // Text processing (required by OpenRC scripts)
    "grep",
    "sed",
    "gawk",
    "findutils",
    // Networking (wired only — no WiFi on a racked server)
    "dhcpcd",
    "iproute2",
    "iputils",
    // Certificates (API server needs TLS)
    "ca-certificates",
    // Time synchronization
    "tzdata",
    "chrony",
    // SSH (remote management — required for headless server)
    "openssh",
    // HTTP client
    "curl",
];

// =============================================================================
// Tier 2: Refurbishment Tools
// =============================================================================
// Disk diagnostics, SAS/SCSI utilities, monitoring.

/// Tier 2: Refurbishment and disk diagnostic packages.
///
/// Adds to Tier 1:
/// - SMART monitoring (smartmontools)
/// - ATA/SCSI command passthrough (hdparm, sg3_utils)
/// - SAS drive parameters (sdparm)
/// - NVMe management (nvme-cli)
/// - Hardware enumeration (lsscsi, pciutils)
/// - System monitoring (htop, iotop)
pub const REFURBISHMENT_PACKAGES: &[&str] = &[
    // SMART data collection
    "smartmontools",
    // ATA command passthrough (identify, security erase, etc.)
    "hdparm",
    // SCSI/SAS generic utilities (sg_sat_identify, sg_readcap, etc.)
    "sg3_utils",
    // SAS drive parameters
    "sdparm",
    // NVMe management
    "nvme-cli",
    // SCSI device enumeration (maps /dev/sd* to HBA:channel:target:lun)
    "lsscsi",
    // Hardware enumeration
    "pciutils",  // lspci — identify HBAs
    "usbutils",  // lsusb
    "dmidecode", // BIOS/DMI — server model, serial number
    "ethtool",   // NIC diagnostics
    // System monitoring
    "htop",
    "iotop",
    // Pager
    "less",
    // Text editor (emergency maintenance)
    "vim",
];

// =============================================================================
// Tier 3: Live ISO Specific
// =============================================================================
// Packages only needed on the live ISO for installation.

/// Tier 3: Live ISO installer packages.
pub const LIVE_ISO_PACKAGES: &[&str] = &[
    // Partitioning (for initial appliance disk setup)
    "parted", // XFS (alternative data partition filesystem)
    "xfsprogs",
];

// =============================================================================
// Alpine Signing Keys
// =============================================================================
// Re-export from AcornOS — same Alpine base, same keys.

/// Alpine signing key: 4a6a0840 (used for older packages)
pub const ALPINE_KEY_4A6A0840: &str =
    include_str!("keys/alpine-devel@lists.alpinelinux.org-4a6a0840.rsa.pub");

/// Alpine signing key: 5243ef4b (used for v3.x main packages)
pub const ALPINE_KEY_5243EF4B: &str =
    include_str!("keys/alpine-devel@lists.alpinelinux.org-5243ef4b.rsa.pub");

/// Alpine signing key: 5261cecb (used for v3.x community packages)
pub const ALPINE_KEY_5261CECB: &str =
    include_str!("keys/alpine-devel@lists.alpinelinux.org-5261cecb.rsa.pub");

/// Alpine signing key: 6165ee59 (used for v3.15+ packages)
pub const ALPINE_KEY_6165EE59: &str =
    include_str!("keys/alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub");

/// Alpine signing key: 61666e3f (used for v3.17+ packages)
pub const ALPINE_KEY_61666E3F: &str =
    include_str!("keys/alpine-devel@lists.alpinelinux.org-61666e3f.rsa.pub");

/// All Alpine signing keys as (filename, content) tuples.
pub const ALPINE_KEYS: &[(&str, &str)] = &[
    (
        "alpine-devel@lists.alpinelinux.org-4a6a0840.rsa.pub",
        ALPINE_KEY_4A6A0840,
    ),
    (
        "alpine-devel@lists.alpinelinux.org-5243ef4b.rsa.pub",
        ALPINE_KEY_5243EF4B,
    ),
    (
        "alpine-devel@lists.alpinelinux.org-5261cecb.rsa.pub",
        ALPINE_KEY_5261CECB,
    ),
    (
        "alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub",
        ALPINE_KEY_6165EE59,
    ),
    (
        "alpine-devel@lists.alpinelinux.org-61666e3f.rsa.pub",
        ALPINE_KEY_61666E3F,
    ),
];

// =============================================================================
// Helper Functions
// =============================================================================

/// Returns all packages for a bootable system (Tier 0).
pub fn bootable_packages() -> Vec<&'static str> {
    BOOTABLE_PACKAGES.to_vec()
}

/// Returns all packages for a server core system (Tiers 0-1).
pub fn server_core_packages() -> Vec<&'static str> {
    let mut packages = bootable_packages();
    packages.extend_from_slice(SERVER_CORE_PACKAGES);
    packages
}

/// Returns all packages for a refurbishment appliance (Tiers 0-2).
pub fn refurbishment_packages() -> Vec<&'static str> {
    let mut packages = server_core_packages();
    packages.extend_from_slice(REFURBISHMENT_PACKAGES);
    packages
}

/// Returns all packages for a live ISO (Tiers 0-3).
///
/// This is the default package set for `iuppiter build`.
pub fn all_live_packages() -> Vec<&'static str> {
    let mut packages = refurbishment_packages();
    packages.extend_from_slice(LIVE_ISO_PACKAGES);
    packages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_tiers_not_empty() {
        assert!(!BOOTABLE_PACKAGES.is_empty(), "Tier 0 should have packages");
        assert!(
            !SERVER_CORE_PACKAGES.is_empty(),
            "Tier 1 should have packages"
        );
        assert!(
            !REFURBISHMENT_PACKAGES.is_empty(),
            "Tier 2 should have packages"
        );
        assert!(!LIVE_ISO_PACKAGES.is_empty(), "Tier 3 should have packages");
    }

    #[test]
    fn test_all_live_packages_includes_all_tiers() {
        let all = all_live_packages();

        for pkg in BOOTABLE_PACKAGES {
            assert!(all.contains(pkg), "Missing Tier 0 package: {}", pkg);
        }
        for pkg in SERVER_CORE_PACKAGES {
            assert!(all.contains(pkg), "Missing Tier 1 package: {}", pkg);
        }
        for pkg in REFURBISHMENT_PACKAGES {
            assert!(all.contains(pkg), "Missing Tier 2 package: {}", pkg);
        }
        for pkg in LIVE_ISO_PACKAGES {
            assert!(all.contains(pkg), "Missing Tier 3 package: {}", pkg);
        }
    }

    #[test]
    fn test_critical_refurbishment_packages_present() {
        let all = all_live_packages();

        // Disk diagnostic tools — these ARE the product
        assert!(
            all.contains(&"smartmontools"),
            "smartmontools required for SMART data"
        );
        assert!(all.contains(&"hdparm"), "hdparm required for ATA commands");
        assert!(
            all.contains(&"sg3_utils"),
            "sg3_utils required for SAS/SCSI passthrough"
        );
        assert!(
            all.contains(&"lsscsi"),
            "lsscsi required for device enumeration"
        );
        assert!(
            all.contains(&"nvme-cli"),
            "nvme-cli required for NVMe drives"
        );

        // Server essentials
        assert!(
            all.contains(&"openssh"),
            "openssh required for remote management"
        );
        assert!(
            all.contains(&"ca-certificates"),
            "ca-certificates required for TLS"
        );
        assert!(
            all.contains(&"eudev"),
            "eudev required for device management"
        );
    }

    #[test]
    fn test_no_desktop_packages() {
        let all = all_live_packages();

        // IuppiterOS is headless — none of these should be present
        assert!(
            !all.contains(&"iwd"),
            "WiFi daemon not needed on racked server"
        );
        assert!(
            !all.contains(&"wireless-regdb"),
            "WiFi regulatory DB not needed"
        );
        assert!(!all.contains(&"sof-firmware"), "Audio firmware not needed");
        assert!(
            !all.contains(&"cryptsetup"),
            "LUKS not needed for appliance"
        );
        assert!(!all.contains(&"lvm2"), "LVM not needed for appliance");
        assert!(
            !all.contains(&"btrfs-progs"),
            "Btrfs not needed for appliance"
        );
    }

    #[test]
    fn test_alpine_keys_present() {
        assert_eq!(ALPINE_KEYS.len(), 5, "Should have 5 Alpine signing keys");

        for (filename, content) in ALPINE_KEYS {
            assert!(!filename.is_empty(), "Key filename should not be empty");
            assert!(
                content.contains("BEGIN PUBLIC KEY"),
                "Key {} should be PEM format",
                filename
            );
        }
    }
}
