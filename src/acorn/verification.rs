//! Rootfs staging verification constants for AcornOS.
//!
//! Used by the rootfs builder to verify the staging directory contains
//! all required files before creating the EROFS image.

/// Essential binaries that must exist in the staging directory.
/// Paths are relative to the staging root.
pub const REQUIRED_BINARIES: &[&str] = &[
    "sbin/openrc-init",
    "sbin/openrc",
    "bin/busybox",
    "usr/bin/openrc-run",
];

/// FHS directories that must exist (or be symlinks).
pub const REQUIRED_DIRS: &[&str] = &[
    "bin", "sbin", "etc", "usr", "var", "tmp", "dev", "proc", "sys",
];

/// Config files that must exist for a bootable system.
pub const REQUIRED_CONFIGS: &[&str] = &["etc/inittab", "etc/passwd", "etc/group", "etc/os-release"];

/// Directories that must exist and be non-empty.
pub const REQUIRED_SERVICE_DIR: &str = "etc/init.d";

/// Kernel modules directory prefix (must contain at least one version dir).
pub const KERNEL_MODULES_DIR: &str = "lib/modules";
