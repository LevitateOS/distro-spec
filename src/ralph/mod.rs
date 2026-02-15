//! RalphOS installation specification.
//!
//! RalphOS is "based on LevitateOS" in the sense that it reuses the build engine
//! and the Rocky Linux (glibc/systemd) base, but it targets a different runtime
//! contract: headless, agents-only, sandbox-host.

pub mod paths;

pub use paths::{DEFAULT_HOSTNAME, MODULE_INSTALL_PATH, OS_ID, OS_NAME};

// Kernel source specification
pub use crate::shared::RALPH_KERNEL as KERNEL_SOURCE;

// Re-export shared constants that are identical across distros.
pub use crate::shared::{KERNEL_FILENAME, OS_VERSION};
