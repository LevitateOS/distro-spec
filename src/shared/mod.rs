//! Shared installation specifications.
//!
//! These modules contain specifications that are common across all distro variants.

pub mod auth;
pub mod boot;
pub mod boot_modules;
pub mod chroot;
pub mod components;
pub mod devices;
pub mod error;
pub mod initramfs;
pub mod iso;
pub mod licenses;
pub mod modules;
pub mod partitions;
pub mod paths;
pub mod qemu;
pub mod requirements;
pub mod services;
pub mod rootfs;
pub mod system;
pub mod udev;
pub mod uki;
pub mod users;

pub use boot::{
    bootctl_install_command, BootEntry, LoaderConfig, DEFAULT_TIMEOUT, ENTRIES_DIR,
    ESP_MOUNT_POINT, LOADER_CONF_PATH,
};
pub use chroot::{BindMount, CHROOT_BIND_MOUNTS};
pub use devices::BOOT_DEVICE_PROBE_ORDER;
pub use error::{ToolError, ToolErrorCode};
pub use initramfs::{
    CPIO_GZIP_LEVEL, INITRAMFS_DIRS, MOUNT_LIVE_OVERLAY, MOUNT_NEWROOT, MOUNT_OVERLAY,
    MOUNT_ROOTFS,
};
pub use iso::{
    EFI_DEBUG, EFIBOOT_FILENAME, EFIBOOT_SIZE_MB, EFI_BOOTLOADER, EFI_GRUB, INITRAMFS_LIVE_ISO_PATH, ISO_BOOT_DIR,
    ISO_CHECKSUM_SUFFIX, ISO_EFI_DIR, ISO_LIVE_DIR, KERNEL_ISO_PATH, LIVE_OVERLAY_ISO_PATH,
    ROOTFS_ISO_PATH, SELINUX_DISABLE, SERIAL_BAUD_RATE, SERIAL_CONSOLE, SHA512_SEPARATOR,
    SQUASHFS_ISO_PATH, VGA_CONSOLE, XORRISO_FS_FLAGS, XORRISO_PARTITION_OFFSET,
};
pub use partitions::{PartitionLayout, PartitionSpec, EFI_PARTITION_SIZE_MB};
pub use qemu::{QEMU_CPU_MODE, QEMU_DISK_FILENAME, QEMU_DISK_GB, QEMU_MEMORY_GB, QEMU_SERIAL_LOG, QCOW2_IMAGE_FILENAME, RAW_DISK_FILENAME};
pub use rootfs::{
    // EROFS (primary)
    EROFS_CDROM_PATH, EROFS_CHUNK_SIZE, EROFS_COMPRESSION, EROFS_COMPRESSION_LEVEL,
    EROFS_MAGIC, EROFS_MAGIC_OFFSET, EROFS_NAME, ROOTFS_CDROM_PATH, ROOTFS_NAME, ROOTFS_TYPE,
    // Squashfs (legacy)
    SQUASHFS_BLOCK_SIZE, SQUASHFS_CDROM_PATH, SQUASHFS_COMPRESSION, SQUASHFS_MAGIC, SQUASHFS_NAME,
    // Installer constants
    ESSENTIAL_DIRS, MIN_REQUIRED_BYTES, ROOTFS_SEARCH_PATHS,
};
pub use uki::{
    LOADER_ENTRIES_DIR, SYSTEMD_BOOT_EFI, SYSTEMD_BOOT_STUB, UKI_DEBUG_FILENAME,
    UKI_EFI_DIR, UKI_EMERGENCY_FILENAME, UKI_LIVE_FILENAME,
    // Installed UKI constants
    UKI_INSTALLED_FILENAME, UKI_INSTALLED_RECOVERY_FILENAME,
};
pub use boot_modules::{CORE_BOOT_MODULES, INSTALL_BOOT_MODULES, USB_BOOT_MODULES};
pub use modules::{
    module_path, INSTALL_MODULES, INSTALL_MODULES_BUILTIN, LIVE_MODULES, LIVE_MODULES_BUILTIN,
    MODULE_PATHS,
};
pub use paths::{
    is_protected_path, AMD_UCODE_FILENAME, DEFAULT_USER_GROUPS, INITRAMFS_BUILD_DIR,
    INITRAMFS_FILENAME, INITRAMFS_LIVE_OUTPUT, INTEL_UCODE_FILENAME, KERNEL_FILENAME,
    LOADER_CONF_FILENAME, OS_VERSION, PROTECTED_PATHS,
};
pub use requirements::{SystemRequirements, ACORN_REQUIREMENTS, LEVITATE_REQUIREMENTS};
pub use services::ServiceManager;
pub use system::{is_mount_point, is_root};
pub use users::{UserSpec, MIN_GID, MIN_UID, SUDOERS_WHEEL_LINE};
pub use auth::{
    // All PAM configuration files (SINGLE SOURCE OF TRUTH)
    PAM_SYSTEM_AUTH, PAM_POSTLOGIN, PAM_LOGIN, PAM_SSHD, PAM_REMOTE,
    PAM_SUDO, PAM_SU, PAM_SU_L, PAM_RUNUSER, PAM_RUNUSER_L,
    PAM_CROND, PAM_PASSWD, PAM_CHPASSWD, PAM_CHFN, PAM_CHSH,
    PAM_OTHER, PAM_SYSTEMD_USER,
    // All security configuration files (SINGLE SOURCE OF TRUTH)
    LIMITS_CONF, ACCESS_CONF, NAMESPACE_CONF, PAM_ENV_CONF, PWQUALITY_CONF,
    // Component lists (auth subsystem)
    AUTH_BIN, AUTH_SBIN, SHADOW_SBIN, SSH_BIN, SSH_SBIN, SUDO_LIBS,
    PAM_MODULES, PAM_CONFIGS, SECURITY_FILES,
};
pub use components::{
    // LevitateOS installation tools
    LEVITATE_TOOLS, LEVITATE_CARGO_TOOLS,
    // Filesystem hierarchy
    FHS_DIRS, FHS_SYMLINKS,
    // Binaries - /usr/bin
    BIN_UTILS, NM_BIN,
    // Binaries - /usr/sbin
    SBIN_UTILS, NM_SBIN, WPA_SBIN,
    BLUETOOTH_SBIN, PIPEWIRE_SBIN, POLKIT_SBIN, UDISKS_SBIN, UPOWER_SBIN,
    SYSTEMD_BINARIES,
    // Systemd units
    ESSENTIAL_UNITS, NM_UNITS, WPA_UNITS, SSH_UNITS, DBUS_ACTIVATION_SYMLINKS,
    BLUETOOTH_UNITS, PIPEWIRE_UNITS, POLKIT_UNITS, UDISKS_UNITS, UPOWER_UNITS,
    ALL_SYSTEMD_UNITS,
    // Udev (UDEV_HELPERS kept here for backwards compatibility, canonical source in udev.rs)
    UDEV_HELPERS,
    // /etc files
    ETC_FILES,
    // Libraries
    CRITICAL_LIBS,
    // Users/groups
    SYSTEM_USERS, SYSTEM_GROUPS,
};
pub use udev::{
    UDEV_HELPERS as UDEV_HELPER_BINARIES,
    UDEV_UNITS_TO_PATCH, UDEV_TMPFILES_ENTRIES, UDEV_TMPFILES_CONF, UDEV_DIRS_SERVICE,
};
