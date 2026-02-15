//! RalphOS file paths and naming conventions.
//!
//! These constants define the exact names and locations of files produced by
//! RalphOS tooling and expected during installation.
//!
//! RalphOS uses a UsrMerge layout (same as LevitateOS).

/// Module installation path (UsrMerge compliant).
///
/// This is where `make modules_install INSTALL_MOD_PATH=...` should place modules.
pub const MODULE_INSTALL_PATH: &str = "/usr/lib/modules";

/// Default hostname for fresh installations.
pub const DEFAULT_HOSTNAME: &str = "ralphos";

/// OS identification.
pub const OS_NAME: &str = "RalphOS";
pub const OS_ID: &str = "ralphos";
