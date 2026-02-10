//! Component definitions - SINGLE SOURCE OF TRUTH for rootfs contents.
//!
//! These lists define what a complete LevitateOS rootfs should contain.
//! Used by:
//! - `leviso` - builds the rootfs from these definitions
//! - `fsdbg` - verifies the rootfs contains these items
//!
//! # Adding New Items
//!
//! 1. Add to the appropriate list in the relevant sub-module
//! 2. Both leviso and fsdbg will automatically pick up the change
//! 3. Run `cargo build --workspace` to verify

mod bins;
mod etc;
mod filesystem;
pub mod systemd;
mod units;
mod users;

pub use bins::*;
pub use etc::*;
pub use filesystem::*;
pub use systemd::*;
pub use units::*;
pub use users::*;
