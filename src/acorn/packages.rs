//! AcornOS package tier definitions.
//!
//! This module defines package tiers for building AcornOS images with different
//! levels of functionality. Packages are organized into tiers based on their
//! purpose:
//!
//! - **Tier 0 (Bootable)**: Absolute minimum for kernel + init to run
//! - **Tier 1 (Core)**: Adds storage, encryption, device management
//! - **Tier 2 (Daily Driver)**: Networking, diagnostics, certificates
//! - **Tier 3 (Live ISO)**: Installer tools, partitioning
//!
//! # Usage
//!
//! ```rust
//! use distro_spec::acorn::packages::{BOOTABLE_PACKAGES, all_live_packages};
//!
//! // Get all packages for live ISO build
//! let packages = all_live_packages();
//! println!("Installing {} packages", packages.len());
//! ```

// =============================================================================
// Tier 0: Bootable Minimum (~15 packages)
// =============================================================================
// These are the absolute minimum packages needed for the kernel to boot and
// hand off to init. Without these, the system won't POST.

/// Tier 0: Absolute minimum for boot.
///
/// These packages provide:
/// - Base Alpine system (alpine-base includes musl, busybox, apk-tools)
/// - Kernel and initramfs generation
/// - Basic init system (OpenRC)
pub const BOOTABLE_PACKAGES: &[&str] = &[
    // Alpine base metapackage (includes musl, busybox, apk-tools)
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
    "e2fsprogs",     // ext4 - most common root filesystem
    "dosfstools",    // FAT - required for EFI System Partition
    "util-linux",    // mount, fdisk, blkid
];

// =============================================================================
// Tier 1: Core System - P0 Requirements
// =============================================================================
// Adds hardware support, device management, and storage capabilities.
// These are P0 requirements - the system is broken without them.

/// Tier 1: Core system packages (P0 requirements).
///
/// Adds to Tier 0:
/// - Device management (eudev)
/// - Hardware firmware (linux-firmware, microcode)
/// - Storage capabilities (LUKS, LVM, btrfs)
/// - Privilege escalation (doas)
/// - Login support
pub const CORE_PACKAGES: &[&str] = &[
    // Device management (P0 - hardware won't work without it)
    "eudev",
    "eudev-openrc",
    // Hardware firmware (P0 - many devices won't initialize)
    "linux-firmware",
    "intel-ucode",
    "amd-ucode",
    // Storage & Encryption (P0)
    "cryptsetup",       // LUKS disk encryption
    "lvm2",             // Logical Volume Manager
    "btrfs-progs",      // Btrfs filesystem tools
    "device-mapper",    // Required by cryptsetup/lvm2
    // Login support
    "util-linux-login", // login binary for agetty
    // Shell
    "bash",             // Bash shell (in addition to busybox ash)
    "coreutils",        // GNU coreutils
    // Privilege escalation (P0 - users cannot become root without this)
    "doas",             // sudo alternative (simpler, more secure)
    // Text processing (required by OpenRC scripts)
    "grep",
    "sed",
    "gawk",
    "findutils",
];

// =============================================================================
// Tier 2: Daily Driver - P1 Requirements
// =============================================================================
// Everything needed for a functional desktop system. Without these packages,
// common tasks fail.

/// Tier 2: Daily driver packages (P1 requirements).
///
/// Adds to Tier 1:
/// - Networking (dhcpcd, iproute2, WiFi)
/// - System certificates (HTTPS validation)
/// - Timezone support
/// - Text editor and pager
/// - Hardware diagnostics
/// - Audio firmware
pub const DAILY_DRIVER_PACKAGES: &[&str] = &[
    // Networking (P1)
    "dhcpcd",
    "iproute2",
    "iputils",          // ping
    "iwd",              // WiFi daemon
    "wireless-regdb",   // WiFi regulatory database
    // Certificates (P1 - HTTPS won't validate without this)
    "ca-certificates",
    // Timezone support (P1 - no timezone = wrong time everywhere)
    "tzdata",
    // HTTP client (P1 - can't download files)
    "curl",
    // Pager (P1 - can't read man pages or long output)
    "less",
    // Text editor
    "vim",
    // Hardware diagnostics
    "pciutils",         // lspci
    "usbutils",         // lsusb
    "dmidecode",        // BIOS/DMI info
    "ethtool",          // NIC diagnostics
    "smartmontools",    // SMART disk health
    "hdparm",           // Disk parameters
    "nvme-cli",         // NVMe management
    // Audio firmware (P1 - no audio on many Intel laptops)
    "sof-firmware",
    // SSH
    "openssh",
];

// =============================================================================
// Tier 3: Live ISO Specific
// =============================================================================
// Packages only needed for live ISO functionality (installer tools).

/// Tier 3: Live ISO installer packages.
///
/// These are only needed on the live ISO for installation:
/// - Partitioning tools
/// - Additional filesystem support
pub const LIVE_ISO_PACKAGES: &[&str] = &[
    // Partitioning
    "parted",           // GPT partitioning
    // Filesystems
    "xfsprogs",         // XFS filesystem
];

// =============================================================================
// Alpine Signing Keys
// =============================================================================
// These keys are embedded in distro-spec to enable signature verification
// during bootstrap (before alpine-keys package is installed).

/// Alpine signing key: 4a6a0840 (used for older packages)
pub const ALPINE_KEY_4A6A0840: &str = include_str!("keys/alpine-devel@lists.alpinelinux.org-4a6a0840.rsa.pub");

/// Alpine signing key: 5243ef4b (used for v3.x main packages)
pub const ALPINE_KEY_5243EF4B: &str = include_str!("keys/alpine-devel@lists.alpinelinux.org-5243ef4b.rsa.pub");

/// Alpine signing key: 5261cecb (used for v3.x community packages)
pub const ALPINE_KEY_5261CECB: &str = include_str!("keys/alpine-devel@lists.alpinelinux.org-5261cecb.rsa.pub");

/// Alpine signing key: 6165ee59 (used for v3.15+ packages)
pub const ALPINE_KEY_6165EE59: &str = include_str!("keys/alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub");

/// Alpine signing key: 61666e3f (used for v3.17+ packages)
pub const ALPINE_KEY_61666E3F: &str = include_str!("keys/alpine-devel@lists.alpinelinux.org-61666e3f.rsa.pub");

/// All Alpine signing keys as (filename, content) tuples.
pub const ALPINE_KEYS: &[(&str, &str)] = &[
    ("alpine-devel@lists.alpinelinux.org-4a6a0840.rsa.pub", ALPINE_KEY_4A6A0840),
    ("alpine-devel@lists.alpinelinux.org-5243ef4b.rsa.pub", ALPINE_KEY_5243EF4B),
    ("alpine-devel@lists.alpinelinux.org-5261cecb.rsa.pub", ALPINE_KEY_5261CECB),
    ("alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub", ALPINE_KEY_6165EE59),
    ("alpine-devel@lists.alpinelinux.org-61666e3f.rsa.pub", ALPINE_KEY_61666E3F),
];

// =============================================================================
// Helper Functions
// =============================================================================

/// Returns all packages for a bootable system (Tier 0).
pub fn bootable_packages() -> Vec<&'static str> {
    BOOTABLE_PACKAGES.to_vec()
}

/// Returns all packages for a core system (Tiers 0-1).
pub fn core_packages() -> Vec<&'static str> {
    let mut packages = bootable_packages();
    packages.extend_from_slice(CORE_PACKAGES);
    packages
}

/// Returns all packages for a daily driver system (Tiers 0-2).
pub fn daily_driver_packages() -> Vec<&'static str> {
    let mut packages = core_packages();
    packages.extend_from_slice(DAILY_DRIVER_PACKAGES);
    packages
}

/// Returns all packages for a live ISO (Tiers 0-3).
///
/// This is the default package set for `acornos build`.
pub fn all_live_packages() -> Vec<&'static str> {
    let mut packages = daily_driver_packages();
    packages.extend_from_slice(LIVE_ISO_PACKAGES);
    packages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_tiers_not_empty() {
        assert!(!BOOTABLE_PACKAGES.is_empty(), "Tier 0 should have packages");
        assert!(!CORE_PACKAGES.is_empty(), "Tier 1 should have packages");
        assert!(!DAILY_DRIVER_PACKAGES.is_empty(), "Tier 2 should have packages");
        assert!(!LIVE_ISO_PACKAGES.is_empty(), "Tier 3 should have packages");
    }

    #[test]
    fn test_all_live_packages_includes_all_tiers() {
        let all = all_live_packages();

        // Check that all tiers are included
        for pkg in BOOTABLE_PACKAGES {
            assert!(all.contains(pkg), "Missing Tier 0 package: {}", pkg);
        }
        for pkg in CORE_PACKAGES {
            assert!(all.contains(pkg), "Missing Tier 1 package: {}", pkg);
        }
        for pkg in DAILY_DRIVER_PACKAGES {
            assert!(all.contains(pkg), "Missing Tier 2 package: {}", pkg);
        }
        for pkg in LIVE_ISO_PACKAGES {
            assert!(all.contains(pkg), "Missing Tier 3 package: {}", pkg);
        }
    }

    #[test]
    fn test_critical_packages_present() {
        let all = all_live_packages();

        // P0 critical packages
        assert!(all.contains(&"doas"), "doas is required for privilege escalation");
        assert!(all.contains(&"eudev"), "eudev is required for device management");
        assert!(all.contains(&"cryptsetup"), "cryptsetup is required for LUKS");

        // P1 daily driver packages
        assert!(all.contains(&"ca-certificates"), "ca-certificates required for HTTPS");
        assert!(all.contains(&"tzdata"), "tzdata required for timezone support");
        assert!(all.contains(&"less"), "less required for pager support");
        assert!(all.contains(&"curl"), "curl required for HTTP downloads");
    }

    #[test]
    fn test_alpine_keys_present() {
        assert_eq!(ALPINE_KEYS.len(), 5, "Should have 5 Alpine signing keys");

        for (filename, content) in ALPINE_KEYS {
            assert!(!filename.is_empty(), "Key filename should not be empty");
            assert!(content.contains("BEGIN PUBLIC KEY"), "Key {} should be PEM format", filename);
        }
    }
}
