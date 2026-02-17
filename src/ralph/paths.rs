//! RalphOS file paths and naming conventions.
//!
//! These constants define the exact names and locations of files produced by
//! RalphOS tooling and expected during installation.
//!
//! RalphOS uses a UsrMerge layout (same as LevitateOS).

// Re-export shared constants that are identical across distros.
pub use crate::shared::{ROOTFS_NAME, ROOTFS_TYPE};

/// ISO volume label used for boot-device detection.
pub const ISO_LABEL: &str = "RALPHOS";

/// ISO output filename (x86_64 architecture).
pub const ISO_FILENAME: &str = "ralphos-x86_64-s00_build.iso";

/// Module installation path (UsrMerge compliant).
///
/// This is where `make modules_install INSTALL_MOD_PATH=...` should place modules.
pub const MODULE_INSTALL_PATH: &str = "/usr/lib/modules";

/// Default hostname for fresh installations.
pub const DEFAULT_HOSTNAME: &str = "ralphos";

/// OS identification.
pub const OS_NAME: &str = "RalphOS";
pub const OS_ID: &str = "ralphos";
