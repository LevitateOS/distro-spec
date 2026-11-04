//! Udev subsystem configuration.
//!
//! Single source of truth for udev-related constants and configuration.
//! Used by:
//! - `recinit` - builds udev into initramfs
//! - `leviso` - builds udev into rootfs
//!
//! # Defense in Depth
//!
//! The `/run/udev` directory is created in 3 places intentionally:
//! 1. **Init wrapper** - Shell script runs earliest, most reliable
//! 2. **udev-dirs.service** - Systemd-managed, before socket activation
//! 3. **tmpfiles.d/udev-initrd.conf** - Declarative fallback
//!
//! This redundancy is intentional - udev socket activation is boot-critical.
//! If any one mechanism fails, the others provide backup.

/// Udev helper programs needed for device identification.
///
/// These binaries are invoked by udev rules to probe device attributes.
/// Located in `/usr/lib/udev/`.
pub const UDEV_HELPERS: &[&str] = &[
    "ata_id",        // ATA device identification (SATA drives)
    "scsi_id",       // SCSI device identification
    "cdrom_id",      // CD/DVD detection
    "v4l_id",        // Video4Linux identification
    "dmi_memory_id", // DMI memory identification
    "mtd_probe",     // MTD (Memory Technology Device) probe
];

/// Systemd units that need patching for initramfs operation.
///
/// These units have `ConditionPathIsReadWrite=/sys` which fails during
/// initramfs boot even though sysfs is properly mounted. The condition
/// must be removed for udevd to start.
///
/// Additionally, socket units need dependency on udev-dirs.service so
/// /run/udev exists before they try to bind.
pub const UDEV_UNITS_TO_PATCH: &[&str] = &[
    "systemd-udevd-control.socket",
    "systemd-udevd-kernel.socket",
    "systemd-udevd.service",
    "systemd-udev-trigger.service",
    "systemd-udev-settle.service",
];

/// tmpfiles.d entries for creating /run/udev.
///
/// Format: "type path mode user group age"
/// - d = directory
/// - 0755 = rwxr-xr-x
/// - root root = owner/group
/// - - = no cleanup age
pub const UDEV_TMPFILES_ENTRIES: &[&str] = &[
    "d /run/udev 0755 root root -",
    "d /run/udev/rules.d 0755 root root -",
];

/// Content for the udev-initrd.conf tmpfiles configuration.
///
/// This file is written to /usr/lib/tmpfiles.d/udev-initrd.conf in the initramfs.
pub const UDEV_TMPFILES_CONF: &str = "\
# Create /run/udev for udev socket activation in initrd
# Required before systemd-udevd-control.socket can bind
#
# Defense in depth: /run/udev is created in 3 places:
# 1. Init wrapper (earliest, most reliable)
# 2. udev-dirs.service (systemd-managed)
# 3. This tmpfiles.d config (declarative)
# Redundancy is intentional - udev socket activation is boot-critical.
d /run/udev 0755 root root -
d /run/udev/rules.d 0755 root root -
";

/// Content for the udev-dirs.service unit.
///
/// This service creates /run/udev before udev sockets try to bind.
/// It runs before sockets.target and the specific udev sockets.
pub const UDEV_DIRS_SERVICE: &str = "\
# LevitateOS: Create /run/udev before socket activation
#
# Defense in depth: /run/udev is created in 3 places:
# 1. Init wrapper (earliest, most reliable)
# 2. This service (systemd-managed)
# 3. tmpfiles.d/udev-initrd.conf (declarative)
# Redundancy is intentional - udev socket activation is boot-critical.
[Unit]
Description=Create udev runtime directories
Documentation=man:udev(7)
DefaultDependencies=no
# Run before both sockets.target AND the specific udev sockets
Before=sockets.target systemd-udevd-control.socket systemd-udevd-kernel.socket
ConditionPathIsDirectory=!/run/udev

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/bin/mkdir -p /run/udev /run/udev/rules.d
";
