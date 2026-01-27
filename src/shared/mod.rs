//! Shared installation specifications.
//!
//! These modules contain specifications that are common across all distro variants.

pub mod boot;
pub mod boot_modules;
pub mod chroot;
pub mod devices;
pub mod initramfs;
pub mod iso;
pub mod partitions;
pub mod paths;
pub mod qemu;
pub mod requirements;
pub mod services;
pub mod rootfs;
pub mod uki;
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
    EFI_DEBUG, EFIBOOT_FILENAME, EFIBOOT_SIZE_MB, EFI_BOOTLOADER, EFI_GRUB, INITRAMFS_LIVE_ISO_PATH, ISO_BOOT_DIR,
    ISO_CHECKSUM_SUFFIX, ISO_EFI_DIR, ISO_LIVE_DIR, KERNEL_ISO_PATH, LIVE_OVERLAY_ISO_PATH,
    ROOTFS_ISO_PATH, SELINUX_DISABLE, SERIAL_BAUD_RATE, SERIAL_CONSOLE, SHA512_SEPARATOR,
    SQUASHFS_ISO_PATH, VGA_CONSOLE, XORRISO_FS_FLAGS, XORRISO_PARTITION_OFFSET,
};
pub use partitions::{PartitionLayout, PartitionSpec, EFI_PARTITION_SIZE_MB};
pub use qemu::{QEMU_CPU_MODE, QEMU_DISK_FILENAME, QEMU_DISK_GB, QEMU_MEMORY_GB, QEMU_SERIAL_LOG};
pub use rootfs::{
    // EROFS (primary)
    EROFS_CDROM_PATH, EROFS_CHUNK_SIZE, EROFS_COMPRESSION, EROFS_COMPRESSION_LEVEL, EROFS_NAME,
    ROOTFS_CDROM_PATH, ROOTFS_NAME, ROOTFS_TYPE,
    // Squashfs (legacy)
    SQUASHFS_BLOCK_SIZE, SQUASHFS_CDROM_PATH, SQUASHFS_COMPRESSION, SQUASHFS_NAME,
};
pub use uki::{
    LOADER_ENTRIES_DIR, SYSTEMD_BOOT_EFI, SYSTEMD_BOOT_STUB, UKI_DEBUG_FILENAME,
    UKI_EFI_DIR, UKI_EMERGENCY_FILENAME, UKI_LIVE_FILENAME,
    // Installed UKI constants
    UKI_INSTALLED_FILENAME, UKI_INSTALLED_RECOVERY_FILENAME,
};
pub use boot_modules::{CORE_BOOT_MODULES, INSTALL_BOOT_MODULES, USB_BOOT_MODULES};
pub use paths::{
    is_protected_path, AMD_UCODE_FILENAME, DEFAULT_USER_GROUPS, INITRAMFS_BUILD_DIR,
    INITRAMFS_FILENAME, INITRAMFS_LIVE_OUTPUT, INTEL_UCODE_FILENAME, KERNEL_FILENAME,
    LOADER_CONF_FILENAME, OS_VERSION, PROTECTED_PATHS,
};
pub use requirements::{SystemRequirements, ACORN_REQUIREMENTS, LEVITATE_REQUIREMENTS};
pub use services::ServiceManager;
pub use users::{UserSpec, MIN_GID, MIN_UID, SUDOERS_WHEEL_LINE};
