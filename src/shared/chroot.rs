//! Chroot environment specification.
//!
//! Defines the bind mounts required for a functional chroot environment
//! during installation.

/// Bind mounts required for chroot.
///
/// These must be mounted in order before entering chroot,
/// and unmounted in reverse order when exiting.
///
/// Format: (source, relative_target)
/// The target is relative to the chroot path (e.g., /mnt).
pub const CHROOT_BIND_MOUNTS: &[BindMount] = &[
    BindMount {
        source: "/dev",
        target: "/dev",
        required: true,
    },
    BindMount {
        source: "/dev/pts",
        target: "/dev/pts",
        required: true,
    },
    BindMount {
        source: "/proc",
        target: "/proc",
        required: true,
    },
    BindMount {
        source: "/sys",
        target: "/sys",
        required: true,
    },
    BindMount {
        source: "/sys/firmware/efi/efivars",
        target: "/sys/firmware/efi/efivars",
        // Not required on non-UEFI systems, but we're UEFI-only
        required: true,
    },
    BindMount {
        source: "/run",
        target: "/run",
        required: true,
    },
];

/// A bind mount specification.
#[derive(Debug, Clone, Copy)]
pub struct BindMount {
    /// Source path on the live system
    pub source: &'static str,
    /// Target path relative to chroot root
    pub target: &'static str,
    /// Whether this mount is required (fail if it can't be mounted)
    pub required: bool,
}

impl BindMount {
    /// Get the full target path given a chroot root.
    pub fn full_target(&self, chroot_root: &str) -> String {
        format!("{}{}", chroot_root, self.target)
    }

    /// Generate the mount command for this bind mount.
    pub fn mount_command(&self, chroot_root: &str) -> String {
        format!(
            "mount --bind {} {}",
            self.source,
            self.full_target(chroot_root)
        )
    }

    /// Generate the unmount command for this bind mount.
    pub fn umount_command(&self, chroot_root: &str) -> String {
        format!("umount {}", self.full_target(chroot_root))
    }
}

/// Get bind mounts in mount order.
pub fn mounts_in_order() -> impl Iterator<Item = &'static BindMount> {
    CHROOT_BIND_MOUNTS.iter()
}

/// Get bind mounts in unmount order (reverse).
pub fn mounts_in_unmount_order() -> impl Iterator<Item = &'static BindMount> {
    CHROOT_BIND_MOUNTS.iter().rev()
}
