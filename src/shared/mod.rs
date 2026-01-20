//! Shared installation specifications.
//!
//! These modules contain specifications that are common across all distro variants.

pub mod chroot;
pub mod partitions;
pub mod users;

pub use chroot::{BindMount, CHROOT_BIND_MOUNTS};
pub use partitions::{PartitionLayout, PartitionSpec, EFI_PARTITION_SIZE_MB};
pub use users::{UserSpec, MIN_UID, MIN_GID, SUDOERS_WHEEL_LINE};
