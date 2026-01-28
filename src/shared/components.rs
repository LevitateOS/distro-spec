//! Component definitions - SINGLE SOURCE OF TRUTH for rootfs contents.
//!
//! These lists define what a complete LevitateOS rootfs should contain.
//! Used by:
//! - `leviso` - builds the rootfs from these definitions
//! - `fsdbg` - verifies the rootfs contains these items
//!
//! # Adding New Items
//!
//! 1. Add to the appropriate list here
//! 2. Both leviso and fsdbg will automatically pick up the change
//! 3. Run `cargo build --workspace` to verify

// =============================================================================
// FILESYSTEM HIERARCHY
// =============================================================================

/// FHS directory structure for a complete rootfs.
///
/// These directories are created during rootfs build and verified by fsdbg.
pub const FHS_DIRS: &[&str] = &[
    // /usr hierarchy (merged)
    "usr/bin",
    "usr/sbin",
    "usr/lib",
    "usr/lib64",
    "usr/share",
    "usr/share/man",
    "usr/share/doc",
    "usr/share/licenses",
    "usr/share/zoneinfo",
    "usr/local/bin",
    "usr/local/sbin",
    "usr/local/lib",
    "usr/local/share",
    // /etc configuration
    "etc",
    "etc/systemd/system",
    "etc/pam.d",
    "etc/security",
    "etc/profile.d",
    // XDG Base Directory spec
    "etc/xdg",
    "etc/xdg/autostart",
    // User skeleton with XDG structure
    "etc/skel",
    "etc/skel/.config",
    "etc/skel/.local",
    "etc/skel/.local/share",
    "etc/skel/.local/state",
    "etc/skel/.cache",
    // Volatile directories
    "proc",
    "sys",
    "dev",
    "dev/pts",
    "dev/shm",
    "run",
    "run/lock",
    "tmp",
    // Persistent data
    "var",
    "var/log",
    "var/log/journal",
    "var/tmp",
    "var/cache",
    "var/lib",
    "var/spool",
    // Mount points
    "mnt",
    "media",
    // User directories
    "root",
    "home",
    // Optional
    "opt",
    "srv",
    // Boot (for installed kernels)
    "boot",
    // Systemd
    "usr/lib/systemd/system",
    "usr/lib/systemd/system-generators",
    "usr/lib64/systemd",
    // Modules
    "usr/lib/modules",
    // PAM
    "usr/lib64/security",
    // D-Bus
    "usr/share/dbus-1/system.d",
    "usr/share/dbus-1/system-services",
    // Locale
    "usr/lib/locale",
];

/// Merged-usr symlinks that must exist.
pub const FHS_SYMLINKS: &[(&str, &str)] = &[
    ("bin", "usr/bin"),
    ("sbin", "usr/sbin"),
    ("lib", "usr/lib"),
    ("lib64", "usr/lib64"),
];

// =============================================================================
// BINARIES - /usr/bin
// =============================================================================

/// Binaries for /usr/bin.
///
/// These are the core utilities expected in a daily-driver Linux distribution.
/// Comparable to Arch Linux base + base-devel.
pub const BIN_UTILS: &[&str] = &[
    // === COREUTILS ===
    "ls", "cat", "cp", "mv", "rm", "mkdir", "rmdir", "touch",
    "chmod", "chown", "chgrp", "ln", "readlink", "realpath",
    "stat", "file", "mknod", "mkfifo",
    "timeout", "sleep", "true", "false", "test", "[",
    // Text processing
    "echo", "head", "tail", "wc", "sort", "cut", "tr", "tee",
    "sed", "awk", "gawk", "printf", "uniq", "seq",
    // Search
    "grep", "find", "xargs",
    // System info
    "pwd", "uname", "date", "env", "id", "hostname",
    "printenv", "whoami", "groups", "dmesg", "lsusb",
    // Process control
    "kill", "nice", "nohup", "setsid",
    // Compression
    "gzip", "gunzip", "xz", "unxz", "tar", "bzip2", "bunzip2", "cpio",
    // Shell utilities
    "expr", "yes", "mktemp",
    // Disk info
    "df", "du", "sync", "mount", "umount", "lsblk", "findmnt", "flock",
    // Path utilities
    "dirname", "basename",
    // Other
    "which",
    // === DIFFUTILS ===
    "diff", "cmp",
    // === PROCPS-NG ===
    "ps", "pgrep", "pkill", "top", "free", "uptime", "w", "vmstat", "watch",
    // === SYSTEMD ===
    "systemctl", "journalctl", "timedatectl", "hostnamectl", "localectl", "loginctl", "bootctl",
    "systemd-tmpfiles", // Note: this is in /usr/bin/, not /usr/lib/systemd/
    // === EDITORS ===
    "vi", "vim", "nano",
    // === NETWORK ===
    "ping", "curl", "wget",
    // === TERMINAL ===
    "clear", "stty", "tty",
    // === KEYBOARD ===
    "loadkeys",
    // === LOCALE ===
    "localedef",
    // === UDEV ===
    "udevadm",
    // === MISC ===
    "less", "more",
    // === UTIL-LINUX ===
    "getopt",
    // === GLIBC UTILITIES ===
    "getent", "ldd",
    // === CHECKSUMS ===
    "base64", "md5sum", "sha256sum", "sha512sum",
    // === TERMINAL MULTIPLEXER ===
    "tmux", "screen",
    // === NETWORK DIAGNOSTICS ===
    "dig", "nslookup", "tracepath",
    // NOTE: iwctl (iwd) is NOT in Rocky 10 repos - WiFi via NetworkManager-wifi instead
    // === BINARY INSPECTION ===
    "strings", "hexdump",
    // === FILE SYNC ===
    "rsync",
    // === DOCUMENTATION ===
    "man", "mandb", "apropos", "whatis",
    // === FILE MANAGERS ===
    "mc", "mcedit", "mcview",
    // === PIPE UTILITIES ===
    "pv",
    // === TEXT BROWSER ===
    "lynx",
    // === NETWORK TOOLS ===
    "nmap",
    // === AUDIO ===
    "alsamixer", "amixer", "aplay", "arecord", "speaker-test",
    // === GPG/CRYPTO ===
    "gpg", "gpg2", "gpgconf", "gpg-agent",
    // === NTFS (bin tools) ===
    "ntfsfix", "ntfscat", "ntfscluster", "ntfscmp", "ntfsfallocate",
    "ntfsinfo", "ntfsls", "ntfsmove", "ntfsrecover", "ntfssecaudit",
    "ntfstruncate", "ntfsusermap", "ntfswipe",
    // === VERSION CONTROL ===
    "git",
    // === SCRIPTING LANGUAGES ===
    "python3",  // Note: 'python' symlink not created by Rocky, use python3
    "perl",
    // === PROCESS MONITORING ===
    "htop",
    // === ARCHIVE TOOLS ===
    "zip", "unzip",
    "7za",  // Note: p7zip only provides 7za wrapper script, not 7z/7zr
    // === DIRECTORY TOOLS ===
    "tree",
    // === BLUETOOTH ===
    "bluetoothctl",
    // === PIPEWIRE AUDIO ===
    "pw-cli", "pw-dump", "pw-cat", "pw-play", "pw-record",
    "pw-top", "pw-metadata", "pw-mon", "pw-link",
    "wpctl",  // WirePlumber control
    // === PULSEAUDIO COMPAT (pipewire-pulse) ===
    "pactl", "paplay", "parecord",  // Note: pacmd not provided by pipewire-pulseaudio
    // === POLKIT ===
    "pkexec", "pkaction", "pkcheck",
    // === UDISKS2 ===
    "udisksctl",
    // === POWER MANAGEMENT ===
    "upower",
];

// Authentication and SSH binaries have been consolidated into the auth subsystem.
// See: distro-spec/src/shared/auth/components.rs
// For backwards compatibility, re-export from the auth module below.
pub use super::auth::components::{AUTH_BIN, SSH_BIN};

/// NetworkManager binaries for /usr/bin.
pub const NM_BIN: &[&str] = &["nmcli", "nm-online", "nmtui"];

// =============================================================================
// BINARIES - /usr/sbin
// =============================================================================

/// Binaries for /usr/sbin.
///
/// System administration utilities requiring elevated privileges.
pub const SBIN_UTILS: &[&str] = &[
    // === UTIL-LINUX ===
    "fsck", "blkid", "losetup", "mkswap", "swapon", "swapoff",
    "fdisk", "sfdisk", "wipefs", "blockdev", "pivot_root", "chroot",
    "switch_root", "parted",
    // === E2FSPROGS ===
    "fsck.ext4", "fsck.ext2", "fsck.ext3", "e2fsck", "mke2fs",
    "mkfs.ext4", "mkfs.ext2", "mkfs.ext3", "tune2fs", "resize2fs",
    // === DOSFSTOOLS ===
    "mkfs.fat", "mkfs.vfat", "fsck.fat", "fsck.vfat",
    // === BTRFS ===
    "btrfs", "btrfsck", "mkfs.btrfs", "btrfs-convert", "btrfs-find-root",
    "btrfs-image", "btrfs-map-logical", "btrfs-select-super",
    // === NTFS (sbin tools) ===
    "mkfs.ntfs", "ntfsresize", "ntfsclone", "ntfscp", "ntfslabel",
    // === KMOD ===
    "insmod", "rmmod", "modprobe", "lsmod", "depmod", "modinfo",
    // === SHADOW-UTILS ===
    "useradd", "userdel", "usermod", "groupadd", "groupdel", "groupmod",
    "chpasswd", "passwd",
    // === IPROUTE ===
    "ip", "ss", "bridge",
    // === PROCPS-NG ===
    "sysctl",
    // === SYSTEM CONTROL ===
    "reboot", "shutdown", "poweroff", "halt", "efibootmgr",
    // === OTHER ===
    "ldconfig", "hwclock", "lspci", "ifconfig", "route",
    "agetty", "login", "sulogin", "nologin", "chronyd",
    // === SQUASHFS-TOOLS ===
    "unsquashfs",
    // === CRYPTSETUP (LUKS) ===
    "cryptsetup",
    // === LVM ===
    "lvm",
    // === RAID ===
    "mdadm", "mdmon",
    // === HARDWARE DETECTION ===
    "dmidecode", "ethtool",
    // === XFS ===
    "mkfs.xfs", "xfs_repair", "xfs_admin", "xfs_copy", "xfs_db",
    "xfs_freeze", "xfs_growfs", "xfs_info", "xfs_io", "xfs_logprint",
    "xfs_mdrestore", "xfs_metadump", "xfs_ncheck", "xfs_quota",
    "xfs_rtcp", "xfs_spaceman",
    // === DISK HEALTH ===
    "smartctl", "hdparm", "nvme",
    // === RECOVERY TOOLS ===
    "ddrescue", "testdisk", "photorec",
];

// Authentication and shadow-utils binaries have been consolidated into the auth subsystem.
// See: distro-spec/src/shared/auth/components.rs
// For backwards compatibility, re-export from the auth module below.
pub use super::auth::components::{AUTH_SBIN, SHADOW_SBIN};

/// NetworkManager binaries for /usr/sbin.
pub const NM_SBIN: &[&str] = &["NetworkManager"];

/// wpa_supplicant binaries for /usr/sbin.
pub const WPA_SBIN: &[&str] = &["wpa_supplicant", "wpa_cli", "wpa_passphrase"];

// SSH server binaries have been consolidated into the auth subsystem.
// See: distro-spec/src/shared/auth/components.rs
pub use super::auth::components::SSH_SBIN;

/// Bluetooth binaries for /usr/sbin (from bluez).
/// Note: bluetoothd is in /usr/libexec/bluetooth/, not /usr/sbin - handled via CopyTree
pub const BLUETOOTH_SBIN: &[&str] = &[];

/// PipeWire binaries for /usr/sbin.
pub const PIPEWIRE_SBIN: &[&str] = &["pipewire", "pipewire-pulse", "wireplumber"];

/// Polkit binaries for /usr/sbin.
/// Note: polkitd is in /usr/lib/polkit-1/, not /usr/sbin - handled via config_trees
pub const POLKIT_SBIN: &[&str] = &[];

/// UDisks2 binaries for /usr/sbin.
/// Note: udisksd is in /usr/libexec/udisks2/, not /usr/sbin - handled via config_trees
pub const UDISKS_SBIN: &[&str] = &[];

/// UPower binaries for /usr/sbin.
/// Note: upowerd is in /usr/libexec/, not /usr/sbin - handled via config_trees
pub const UPOWER_SBIN: &[&str] = &[];

// =============================================================================
// SYSTEMD BINARIES
// =============================================================================

/// Systemd helper binaries in /usr/lib/systemd/.
/// Note: systemd-tmpfiles is in /usr/bin/, not here - see BIN_UTILS.
pub const SYSTEMD_BINARIES: &[&str] = &[
    "systemd-executor",
    "systemd-shutdown",
    "systemd-sulogin-shell",
    "systemd-cgroups-agent",
    "systemd-journald",
    "systemd-modules-load",
    "systemd-sysctl",
    // systemd-tmpfiles is in /usr/bin/, not /usr/lib/systemd/ - see BIN_UTILS
    "systemd-timedated",
    "systemd-hostnamed",
    "systemd-localed",
    "systemd-logind",
    "systemd-networkd",
    "systemd-resolved",
    "systemd-udevd",
    "systemd-fsck",
    "systemd-remount-fs",
    "systemd-makefs",  // For creating/formatting filesystems during boot
    "systemd-vconsole-setup",
    "systemd-random-seed",
];

// =============================================================================
// SYSTEMD UNITS
// =============================================================================

/// Essential systemd unit files.
///
/// These units are required for a bootable system.
pub const ESSENTIAL_UNITS: &[&str] = &[
    // Targets
    "basic.target", "sysinit.target", "multi-user.target", "default.target",
    "getty.target", "local-fs.target", "local-fs-pre.target",
    "remote-fs.target", "remote-fs-pre.target",
    "network.target", "network-pre.target", "network-online.target",
    "paths.target", "slices.target", "sockets.target", "timers.target",
    "swap.target", "shutdown.target", "rescue.target", "emergency.target",
    "reboot.target", "poweroff.target", "halt.target",
    "suspend.target", "sleep.target", "umount.target", "final.target",
    "graphical.target",
    // Initrd targets (required for install initramfs boot)
    "initrd.target", "initrd-root-fs.target", "initrd-root-device.target",
    "initrd-switch-root.target", "initrd-fs.target",
    // Services - core
    "systemd-journald.service", "systemd-journald@.service",
    "systemd-udevd.service", "systemd-udev-trigger.service",
    "systemd-modules-load.service", "systemd-sysctl.service",
    "systemd-tmpfiles-setup.service", "systemd-tmpfiles-setup-dev.service",
    "systemd-tmpfiles-clean.service",
    "systemd-random-seed.service", "systemd-vconsole-setup.service",
    // Services - disk
    "systemd-fsck-root.service", "systemd-fsck@.service",
    "systemd-remount-fs.service",
    // Note: systemd-fstab-generator is in system-generators/, not a unit file
    // Services - initrd (required for install initramfs boot)
    "initrd-switch-root.service", "initrd-cleanup.service",
    "initrd-udevadm-cleanup-db.service", "initrd-parse-etc.service",
    // Services - auth
    "systemd-logind.service",
    // Services - getty
    "getty@.service", "serial-getty@.service",
    "console-getty.service", "container-getty@.service",
    // Services - time/network
    "systemd-timedated.service", "systemd-hostnamed.service",
    "systemd-localed.service", "systemd-networkd.service",
    "systemd-resolved.service", "systemd-networkd-wait-online.service",
    // Services - misc
    "dbus.service", "dbus-broker.service", "chronyd.service",
    // Services - SSH
    "sshd.service", "sshd@.service", "sshd.socket",
    "sshd-keygen.target", "sshd-keygen@.service",
    // Sockets
    "systemd-journald.socket", "systemd-journald-dev-log.socket",
    "systemd-journald-audit.socket",
    "systemd-udevd-control.socket", "systemd-udevd-kernel.socket",
    "dbus.socket",
    // Paths
    "systemd-ask-password-console.path", "systemd-ask-password-wall.path",
    // Slices (note: -.slice, system.slice, machine.slice are built-in to systemd)
    "user.slice",
];

/// NetworkManager units.
pub const NM_UNITS: &[&str] = &["NetworkManager.service", "NetworkManager-dispatcher.service"];

/// wpa_supplicant units.
pub const WPA_UNITS: &[&str] = &["wpa_supplicant.service"];

/// Bluetooth units (bluez).
pub const BLUETOOTH_UNITS: &[&str] = &["bluetooth.service", "bluetooth.target"];

/// PipeWire units (user service - runs per-user, not system-wide).
/// Note: PipeWire runs as a user service, so these are in user/ not system/.
pub const PIPEWIRE_UNITS: &[&str] = &[
    "pipewire.service",
    "pipewire.socket",
    "pipewire-pulse.service",
    "pipewire-pulse.socket",
    "wireplumber.service",
];

/// Polkit units.
pub const POLKIT_UNITS: &[&str] = &["polkit.service"];

/// UDisks2 units.
pub const UDISKS_UNITS: &[&str] = &["udisks2.service"];

/// UPower units.
pub const UPOWER_UNITS: &[&str] = &["upower.service"];

/// SSH units (for Service definition).
pub const SSH_UNITS: &[&str] = &[
    "sshd.service",
    "sshd.socket",
    "sshd@.service",
    "sshd-keygen.target",
    "sshd-keygen@.service",
    "ssh-host-keys-migration.service",
];

/// D-Bus activation symlinks.
pub const DBUS_ACTIVATION_SYMLINKS: &[&str] = &[
    "dbus-org.freedesktop.timedate1.service",
    "dbus-org.freedesktop.hostname1.service",
    "dbus-org.freedesktop.locale1.service",
    "dbus-org.freedesktop.login1.service",
    "dbus-org.freedesktop.network1.service",
    "dbus-org.freedesktop.resolve1.service",
];

// =============================================================================
// UDEV
// =============================================================================

/// Udev helper binaries in /usr/lib/udev/.
pub const UDEV_HELPERS: &[&str] = &[
    "ata_id", "scsi_id", "cdrom_id", "v4l_id", "dmi_memory_id", "mtd_probe",
];

// =============================================================================
// SUDO
// =============================================================================

// Sudo libraries have been consolidated into the auth subsystem.
// See: distro-spec/src/shared/auth/components.rs
pub use super::auth::components::SUDO_LIBS;

// =============================================================================
// PAM
// =============================================================================

// PAM modules, configs, and security files have been consolidated into the auth subsystem.
// See: distro-spec/src/shared/auth/components.rs
pub use super::auth::components::{PAM_MODULES, PAM_CONFIGS, SECURITY_FILES};

// =============================================================================
// /etc FILES
// =============================================================================

/// Essential /etc files for a bootable system.
pub const ETC_FILES: &[&str] = &[
    // === USER DATABASE ===
    "etc/passwd",
    "etc/group",
    "etc/shadow",
    "etc/gshadow",
    // === NETWORK ===
    "etc/hostname",
    "etc/hosts",
    "etc/resolv.conf",
    "etc/nsswitch.conf",
    // === SYSTEM IDENTITY ===
    "etc/os-release",
    "etc/machine-id",
    // === FILESYSTEM ===
    "etc/fstab",
    // === LIBRARIES ===
    "etc/ld.so.conf",
    // === SHELLS ===
    "etc/shells",
    // === LOGIN/AUTH ===
    "etc/login.defs",
    // === SUDO ===
    "etc/sudoers",
    "etc/sudo.conf",
    // === SSH SERVER ===
    "etc/ssh/sshd_config",
    "etc/ssh/ssh_host_rsa_key",
    "etc/ssh/ssh_host_rsa_key.pub",
    "etc/ssh/ssh_host_ecdsa_key",
    "etc/ssh/ssh_host_ecdsa_key.pub",
    "etc/ssh/ssh_host_ed25519_key",
    "etc/ssh/ssh_host_ed25519_key.pub",
    // === SSH CLIENT ===
    "etc/ssh/ssh_config",
    // === TIMEZONE ===
    "etc/localtime",
    // === TIME SYNC ===
    "etc/chrony.conf",
    // === LOCALE ===
    "etc/locale.conf",
    "etc/vconsole.conf",
];

// =============================================================================
// LIBRARIES
// =============================================================================

/// Critical libraries that must exist for system to boot.
pub const CRITICAL_LIBS: &[&str] = &[
    "usr/lib64/libc.so.6",
    "usr/lib64/ld-linux-x86-64.so.2",
    "usr/lib64/libpam.so.0",
    "usr/lib64/libsystemd.so.0",
    "usr/lib64/libnss_files.so.2",
    "usr/lib64/libcrypt.so.2",
    "usr/lib64/libselinux.so.1",
];

// =============================================================================
// SYSTEM USERS/GROUPS
// =============================================================================

/// System users that must exist in /etc/passwd.
pub const SYSTEM_USERS: &[&str] = &[
    "root",
    "dbus",
    "sshd",
    "chrony",
    "polkitd",
    "pipewire",   // For PipeWire system mode (optional)
];

/// System groups that must exist in /etc/group.
pub const SYSTEM_GROUPS: &[&str] = &[
    "root",
    "wheel",
    "dbus",
    "sshd",
    "chrony",
    "polkitd",
    "pipewire",
    "bluetooth",  // Users in this group can use bluetooth
    "audio",      // Users in this group can use audio
    "video",      // Users in this group can use video devices
];

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_critical_binaries() {
        assert!(BIN_UTILS.contains(&"bash") || BIN_UTILS.contains(&"ls"));
        assert!(BIN_UTILS.contains(&"systemctl"));
        assert!(SBIN_UTILS.contains(&"passwd"));
        assert!(AUTH_SBIN.contains(&"unix_chkpwd"));
    }

    #[test]
    fn test_critical_units() {
        assert!(ESSENTIAL_UNITS.contains(&"multi-user.target"));
        assert!(ESSENTIAL_UNITS.contains(&"getty@.service"));
        assert!(ESSENTIAL_UNITS.contains(&"systemd-journald.service"));
    }

    // Regression test: TEAM_145 - initrd units must be in ESSENTIAL_UNITS for install initramfs
    #[test]
    fn test_initrd_units_present() {
        let initrd_targets = [
            "initrd.target",
            "initrd-root-fs.target",
            "initrd-root-device.target",
            "initrd-switch-root.target",
            "initrd-fs.target",
        ];
        let initrd_services = [
            "initrd-switch-root.service",
            "initrd-cleanup.service",
            "initrd-udevadm-cleanup-db.service",
            "initrd-parse-etc.service",
        ];

        for unit in initrd_targets.iter().chain(initrd_services.iter()) {
            assert!(
                ESSENTIAL_UNITS.contains(unit),
                "Initrd unit {} missing from ESSENTIAL_UNITS - required for install initramfs boot",
                unit
            );
        }
    }

    #[test]
    fn test_pam_critical_modules() {
        assert!(PAM_MODULES.contains(&"pam_unix.so"));
        assert!(PAM_MODULES.contains(&"pam_permit.so"));
        assert!(PAM_MODULES.contains(&"pam_deny.so"));
    }

    #[test]
    fn test_fhs_dirs_have_usr() {
        assert!(FHS_DIRS.contains(&"usr/bin"));
        assert!(FHS_DIRS.contains(&"usr/sbin"));
        assert!(FHS_DIRS.contains(&"etc"));
    }

    // Regression test: TEAM_145 - systemd-tmpfiles must be in BIN_UTILS, not SYSTEMD_BINARIES
    // systemd-tmpfiles is at /usr/bin/systemd-tmpfiles, not /usr/lib/systemd/
    // If it's in the wrong list, the copy will silently fail and cause boot failures
    #[test]
    fn test_systemd_tmpfiles_in_correct_location() {
        // systemd-tmpfiles must be in BIN_UTILS (for /usr/bin/)
        assert!(
            BIN_UTILS.contains(&"systemd-tmpfiles"),
            "systemd-tmpfiles must be in BIN_UTILS - it's at /usr/bin/, not /usr/lib/systemd/"
        );

        // systemd-tmpfiles must NOT be in SYSTEMD_BINARIES (for /usr/lib/systemd/)
        assert!(
            !SYSTEMD_BINARIES.contains(&"systemd-tmpfiles"),
            "systemd-tmpfiles must NOT be in SYSTEMD_BINARIES - wrong location causes silent copy failure"
        );
    }

    // Regression test: TEAM_145 - verify other /usr/bin systemd tools are in BIN_UTILS
    #[test]
    fn test_systemd_usr_bin_tools() {
        // These tools are in /usr/bin/, not /usr/lib/systemd/
        let usr_bin_systemd_tools = [
            "systemctl",
            "journalctl",
            "systemd-tmpfiles",
        ];

        for tool in usr_bin_systemd_tools {
            assert!(
                BIN_UTILS.contains(&tool),
                "{} should be in BIN_UTILS (it's at /usr/bin/)",
                tool
            );
        }
    }
}
