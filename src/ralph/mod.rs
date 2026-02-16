//! RalphOS installation specification.
//!
//! RalphOS is "based on LevitateOS" in the sense that it reuses the build engine
//! and the Rocky Linux (glibc/systemd) base, but it targets a different runtime
//! contract: headless, agents-only, sandbox-host.

pub mod paths;

pub use paths::{
    DEFAULT_HOSTNAME, ISO_FILENAME, ISO_LABEL, MODULE_INSTALL_PATH, OS_ID, OS_NAME, ROOTFS_NAME,
    ROOTFS_TYPE,
};

// Kernel source specification
pub use crate::shared::RALPH_KERNEL as KERNEL_SOURCE;

// Re-export shared constants that are identical across distros.
pub use crate::shared::{
    BOOT_DEVICE_PROBE_ORDER, CPIO_GZIP_LEVEL, INITRAMFS_LIVE_OUTPUT, KERNEL_FILENAME,
    LIVE_OVERLAY_ISO_PATH, OS_VERSION, ROOTFS_ISO_PATH,
};
