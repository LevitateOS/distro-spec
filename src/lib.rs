//! Distro Installation Specification
//!
//! Single source of truth for installation constants, paths, and configuration.
//! Supports multiple distro variants: LevitateOS and AcornOS.
//!
//! # STOP. READ. THEN ACT.
//!
//! This is the source of truth for installation specs. Before modifying:
//! 1. Read existing modules (levitate/, acorn/, shared/)
//! 2. Understand what's already defined
//! 3. Changes here affect multiple crates
//!
//! # Usage
//!
//! ```rust
//! // Use LevitateOS spec
//! use distro_spec::levitate;
//! let user = levitate::default_user("alice");
//! println!("Shell: {}", levitate::DEFAULT_SHELL);
//!
//! // Use AcornOS spec
//! use distro_spec::acorn;
//! let user = acorn::default_user("bob");
//! println!("Shell: {}", acorn::DEFAULT_SHELL);
//!
//! // Use shared types
//! use distro_spec::shared::{PartitionLayout, UserSpec};
//! ```
//!
//! # Variants
//!
//! - `levitate` - LevitateOS: Rocky Linux, systemd, glibc, GNU coreutils
//! - `acorn` - AcornOS: Alpine Linux, OpenRC, musl, busybox
//!
//! # Philosophy
//!
//! If something is defined here, it MUST be:
//! 1. Produced by the builder (leviso or equivalent)
//! 2. Tested by install-tests
//! 3. Documented in docs-content
//!
//! Any mismatch between these three is a bug.

pub mod acorn;
pub mod levitate;
pub mod shared;

// Re-export shared items at crate root for convenience
pub use shared::{
    chroot::{BindMount, CHROOT_BIND_MOUNTS, mounts_in_order, mounts_in_unmount_order},
    partitions::{PartitionLayout, PartitionSpec, EFI_PARTITION_SIZE_MB},
    users::{UserSpec, MIN_UID, MIN_GID, SUDOERS_WHEEL_LINE},
};
