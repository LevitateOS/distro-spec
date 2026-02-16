//! Distro Installation Specification
//!
//! # Deprecation Notice
//!
//! This crate is deprecated as the long-term home for distro declarations.
//! - `src/{levitate,acorn,iuppiter,ralph}` is migrating to `distro-variants/*`
//! - shared policy is migrating to `distro-builder`
//! - test/checklist concerns are migrating to `testing/*`
//! - Stage 00 conformance contracts are now authored in `distro-variants/*/stage-00.toml`
//!
//! Single source of truth for installation constants, paths, and configuration.
//! Supports multiple distro variants: LevitateOS, AcornOS, IuppiterOS, and RalphOS.
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
//! - `iuppiter` - IuppiterOS: Alpine Linux, OpenRC, musl, busybox (headless refurbishment server)
//! - `ralph` - RalphOS: LevitateOS base (Rocky Linux, systemd, glibc), headless sandbox host
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
pub mod conformance;
pub mod iuppiter;
pub mod levitate;
pub mod ralph;
pub mod shared;

// Re-export shared items at crate root for convenience
pub use shared::{
    boot::{
        bootctl_install_command, BootEntry, LoaderConfig, DEFAULT_TIMEOUT, ENTRIES_DIR,
        ESP_MOUNT_POINT, LOADER_CONF_PATH,
    },
    chroot::{mounts_in_order, mounts_in_unmount_order, BindMount, CHROOT_BIND_MOUNTS},
    partitions::{PartitionLayout, PartitionSpec, EFI_PARTITION_SIZE_MB},
    services::ServiceManager,
    users::{UserSpec, MIN_GID, MIN_UID, SUDOERS_WHEEL_LINE},
};
