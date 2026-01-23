//! Shared installation specifications.
//!
//! These modules contain specifications that are common across all distro variants.

pub mod boot;
pub mod chroot;
pub mod partitions;
pub mod services;
pub mod users;

pub use boot::{BootEntry, LoaderConfig, ESP_MOUNT_POINT, LOADER_CONF_PATH, ENTRIES_DIR, DEFAULT_TIMEOUT, bootctl_install_command};
pub use chroot::{BindMount, CHROOT_BIND_MOUNTS};
pub use partitions::{PartitionLayout, PartitionSpec, EFI_PARTITION_SIZE_MB};
pub use services::ServiceManager;
pub use users::{UserSpec, MIN_UID, MIN_GID, SUDOERS_WHEEL_LINE};
