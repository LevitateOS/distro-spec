//! Shared installation specifications.
//!
//! These modules contain specifications that are common across all distro variants.

pub mod auth;
pub mod boot;
pub mod boot_modules;
pub mod busybox;
pub mod chroot;
pub mod components;
pub mod devices;
pub mod error;
pub mod firmware;
pub mod initramfs;
pub mod iso;
pub mod kernel;
pub mod modules;
pub mod partitions;
pub mod paths;
pub mod qemu;
pub mod requirements;
pub mod rootfs;
pub mod services;
pub mod system;
pub mod udev;
pub mod uki;
pub mod users;

pub use auth::{
    ACCESS_CONF,
    // Component lists (auth subsystem)
    AUTH_BIN,
    AUTH_SBIN,
    // All security configuration files (SINGLE SOURCE OF TRUTH)
    LIMITS_CONF,
    NAMESPACE_CONF,
    PAM_CHFN,
    PAM_CHPASSWD,
    PAM_CHSH,
    PAM_CONFIGS,
    PAM_CROND,
    PAM_ENV_CONF,
    PAM_LOGIN,
    PAM_MODULES,
    PAM_OTHER,
    PAM_PASSWD,
    PAM_POSTLOGIN,
    PAM_REMOTE,
    PAM_RUNUSER,
    PAM_RUNUSER_L,
    PAM_SSHD,
    PAM_SU,
    PAM_SUDO,
    PAM_SU_L,
    PAM_SYSTEMD_USER,
    // All PAM configuration files (SINGLE SOURCE OF TRUTH)
    PAM_SYSTEM_AUTH,
    PWQUALITY_CONF,
    SECURITY_FILES,
    SHADOW_SBIN,
    SSH_BIN,
    SSH_SBIN,
    SUDO_LIBS,
};
pub use boot::{
    bootctl_install_command, BootEntry, LoaderConfig, DEFAULT_TIMEOUT, ENTRIES_DIR,
    ESP_MOUNT_POINT, LOADER_CONF_PATH,
};
pub use boot_modules::{CORE_BOOT_MODULES, INSTALL_BOOT_MODULES, USB_BOOT_MODULES};
pub use busybox::{COMMON_APPLETS, SBIN_APPLETS};
pub use chroot::{BindMount, CHROOT_BIND_MOUNTS};
pub use components::{
    all_systemd_units,
    // Binaries - /usr/bin
    BIN_UTILS,
    BLUETOOTH_SBIN,
    BLUETOOTH_UNITS,
    // Libraries
    CRITICAL_LIBS,
    DBUS_ACTIVATION_SYMLINKS,
    // Systemd units
    ESSENTIAL_UNITS,
    // /etc files
    ETC_FILES,
    // Filesystem hierarchy
    FHS_DIRS,
    FHS_SYMLINKS,
    LEVITATE_CARGO_TOOLS,
    // LevitateOS installation tools
    LEVITATE_TOOLS,
    NM_BIN,
    NM_SBIN,
    NM_UNITS,
    PIPEWIRE_SBIN,
    PIPEWIRE_UNITS,
    POLKIT_SBIN,
    POLKIT_UNITS,
    // Binaries - /usr/sbin
    SBIN_UTILS,
    SSH_UNITS,
    SYSTEMD_BINARIES,
    SYSTEM_GROUPS,
    // Users/groups
    SYSTEM_USERS,
    // Udev (UDEV_HELPERS kept here for backwards compatibility, canonical source in udev.rs)
    UDEV_HELPERS,
    UDISKS_SBIN,
    UDISKS_UNITS,
    UPOWER_SBIN,
    UPOWER_UNITS,
    VAR_SYMLINKS,
    WPA_SBIN,
    WPA_UNITS,
};
pub use devices::BOOT_DEVICE_PROBE_ORDER;
pub use error::{ToolError, ToolErrorCode};
pub use firmware::WIFI_FIRMWARE_DIRS;
pub use initramfs::{
    CPIO_GZIP_LEVEL, INITRAMFS_DIRS, MOUNT_LIVE_OVERLAY, MOUNT_NEWROOT, MOUNT_OVERLAY, MOUNT_ROOTFS,
};
pub use iso::{
    EFIBOOT_FILENAME, EFIBOOT_SIZE_MB, EFI_BOOTLOADER, EFI_DEBUG, EFI_GRUB,
    INITRAMFS_LIVE_ISO_PATH, ISO_BOOT_DIR, ISO_CHECKSUM_SUFFIX, ISO_EFI_DIR, ISO_LIVE_DIR,
    KERNEL_ISO_PATH, LIVE_OVERLAY_ISO_PATH, ROOTFS_ISO_PATH, SELINUX_DISABLE, SERIAL_BAUD_RATE,
    SERIAL_CONSOLE, SHA512_SEPARATOR, SQUASHFS_ISO_PATH, VGA_CONSOLE, XORRISO_FS_FLAGS,
    XORRISO_PARTITION_OFFSET,
};
pub use kernel::{KernelSource, ACORN_KERNEL, IUPPITER_KERNEL, LEVITATE_KERNEL, RALPH_KERNEL};
pub use modules::{
    module_path, INSTALL_MODULES, INSTALL_MODULES_BUILTIN, LIVE_MODULES, LIVE_MODULES_BUILTIN,
    MODULE_METADATA_FILES, MODULE_PATHS,
};
pub use partitions::{PartitionLayout, PartitionSpec, EFI_PARTITION_SIZE_MB};
pub use paths::{
    is_protected_path, AMD_UCODE_FILENAME, DEFAULT_USER_GROUPS, INITRAMFS_BUILD_DIR,
    INITRAMFS_FILENAME, INITRAMFS_LIVE_OUTPUT, INTEL_UCODE_FILENAME, KERNEL_FILENAME, LIBRARY_DIRS,
    LOADER_CONF_FILENAME, OS_VERSION, PROTECTED_PATHS,
};
pub use qemu::{
    QCOW2_IMAGE_FILENAME, QEMU_CPU_MODE, QEMU_DISK_FILENAME, QEMU_DISK_GB, QEMU_MEMORY_GB,
    QEMU_SERIAL_LOG, RAW_DISK_FILENAME,
};
pub use requirements::{
    SystemRequirements, ACORN_REQUIREMENTS, IUPPITER_REQUIREMENTS, LEVITATE_REQUIREMENTS,
};
pub use rootfs::{
    // EROFS (primary)
    EROFS_CDROM_PATH,
    EROFS_CHUNK_SIZE,
    EROFS_COMPRESSION,
    EROFS_COMPRESSION_LEVEL,
    EROFS_MAGIC,
    EROFS_MAGIC_OFFSET,
    EROFS_NAME,
    // Installer constants
    ESSENTIAL_DIRS,
    MIN_REQUIRED_BYTES,
    ROOTFS_CDROM_PATH,
    ROOTFS_NAME,
    ROOTFS_SEARCH_PATHS,
    ROOTFS_TYPE,
    // Squashfs (legacy)
    SQUASHFS_BLOCK_SIZE,
    SQUASHFS_CDROM_PATH,
    SQUASHFS_COMPRESSION,
    SQUASHFS_MAGIC,
    SQUASHFS_NAME,
};
pub use services::ServiceManager;
pub use system::{is_mount_point, is_root};
pub use udev::{
    UDEV_DIRS_SERVICE, UDEV_HELPERS as UDEV_HELPER_BINARIES, UDEV_TMPFILES_CONF,
    UDEV_TMPFILES_ENTRIES, UDEV_UNITS_TO_PATCH,
};
pub use uki::{
    // Shared type
    UkiEntry,
    LOADER_ENTRIES_DIR,
    SYSTEMD_BOOT_EFI,
    SYSTEMD_BOOT_STUB,
    UKI_DEBUG_FILENAME,
    UKI_EFI_DIR,
    UKI_EMERGENCY_FILENAME,
    // Installed UKI constants
    UKI_INSTALLED_FILENAME,
    UKI_INSTALLED_RECOVERY_FILENAME,
    UKI_LIVE_FILENAME,
};
pub use users::{UserSpec, MIN_GID, MIN_UID, SUDOERS_WHEEL_LINE};
