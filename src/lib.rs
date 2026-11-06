//! Distro Installation Specification
//!
//! # Deprecation Notice
//!
//! This crate is deprecated as the long-term home for distro declarations.
//! - `src/{levitate,acorn}` is migrating to `distro/variants/*`
//! - shared policy is migrating to `distro-builder`
//! - test/checklist concerns are migrating to `testing/*`
//! - Conformance contracts load directly from `distro/variants/*` ring manifests
//!
//! Single source of truth for installation constants, paths, and configuration.
//! Supports multiple distro variants: LevitateOS and AcornOS.
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
//! use distro_spec::shared::UserSpec;
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
    boot::{
        bootctl_install_command, BootEntry, LoaderConfig, DEFAULT_TIMEOUT, ENTRIES_DIR,
        ESP_MOUNT_POINT, LOADER_CONF_PATH,
    },
    services::ServiceManager,
    users::{UserSpec, MIN_GID, MIN_UID, SUDOERS_WHEEL_LINE},
};
