//! Shared installation specifications.
//!
//! These modules contain specifications that are common across all distro variants.

pub mod boot;
pub mod chroot;
pub mod devices;
pub mod initramfs;
pub mod iso;
pub mod partitions;
pub mod qemu;
pub mod services;
pub mod users;

pub use boot::{
    bootctl_install_command, BootEntry, LoaderConfig, DEFAULT_TIMEOUT, ENTRIES_DIR,
    ESP_MOUNT_POINT, LOADER_CONF_PATH,
};
pub use chroot::{BindMount, CHROOT_BIND_MOUNTS};
pub use devices::BOOT_DEVICE_PROBE_ORDER;
pub use initramfs::{
    CPIO_GZIP_LEVEL, INITRAMFS_DIRS, MOUNT_LIVE_OVERLAY, MOUNT_NEWROOT, MOUNT_OVERLAY,
    MOUNT_SQUASHFS,
};
pub use iso::{
    EFIBOOT_FILENAME, EFIBOOT_SIZE_MB, EFI_BOOTLOADER, EFI_GRUB, INITRAMFS_ISO_PATH, ISO_BOOT_DIR,
    ISO_CHECKSUM_SUFFIX, ISO_EFI_DIR, ISO_LIVE_DIR, KERNEL_ISO_PATH, LIVE_OVERLAY_ISO_PATH,
    SELINUX_DISABLE, SERIAL_BAUD_RATE, SERIAL_CONSOLE, SHA512_SEPARATOR, SQUASHFS_ISO_PATH,
    VGA_CONSOLE, XORRISO_FS_FLAGS, XORRISO_PARTITION_OFFSET,
};
pub use partitions::{PartitionLayout, PartitionSpec, EFI_PARTITION_SIZE_MB};
pub use qemu::{QEMU_CPU_MODE, QEMU_DISK_FILENAME, QEMU_SERIAL_LOG};
pub use services::ServiceManager;
pub use users::{UserSpec, MIN_GID, MIN_UID, SUDOERS_WHEEL_LINE};
