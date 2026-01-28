//! Shared boot configuration types for UEFI systems.
//!
//! These types are used by both LevitateOS and AcornOS since both
//! use systemd-boot as the bootloader.

use std::borrow::Cow;

// =============================================================================
// Constants
// =============================================================================

/// Path to the EFI System Partition mount point.
pub const ESP_MOUNT_POINT: &str = "/boot";

/// Path to systemd-boot loader configuration.
pub const LOADER_CONF_PATH: &str = "/boot/loader/loader.conf";

/// Path to boot entries directory.
pub const ENTRIES_DIR: &str = "/boot/loader/entries";

/// Default timeout for boot menu (seconds).
pub const DEFAULT_TIMEOUT: u32 = 3;

// =============================================================================
// Boot Entry
// =============================================================================

/// Boot entry configuration for systemd-boot.
///
/// Represents a single entry in `/boot/loader/entries/*.conf`.
///
/// Uses `Cow<'static, str>` for fields to allow zero-copy when using
/// static defaults, while still supporting owned strings when customized.
#[derive(Debug, Clone)]
pub struct BootEntry {
    /// Entry filename (without .conf extension)
    pub filename: Cow<'static, str>,
    /// Title shown in boot menu
    pub title: Cow<'static, str>,
    /// Path to kernel (relative to ESP)
    pub linux: Cow<'static, str>,
    /// Path to initramfs (relative to ESP)
    pub initrd: Cow<'static, str>,
    /// Kernel command line options
    pub options: Cow<'static, str>,
}

impl BootEntry {
    /// Create a new boot entry with all fields specified.
    pub fn new(
        filename: impl Into<Cow<'static, str>>,
        title: impl Into<Cow<'static, str>>,
        linux: impl Into<Cow<'static, str>>,
        initrd: impl Into<Cow<'static, str>>,
        options: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            filename: filename.into(),
            title: title.into(),
            linux: linux.into(),
            initrd: initrd.into(),
            options: options.into(),
        }
    }

    /// Create a boot entry with distro-specific defaults.
    ///
    /// Used by distro modules to create entries with their OS identity.
    pub fn with_defaults(
        os_id: &str,
        os_name: &str,
        kernel_filename: &str,
        initramfs_filename: &str,
    ) -> Self {
        Self {
            filename: Cow::Owned(os_id.to_string()),
            title: Cow::Owned(os_name.to_string()),
            linux: Cow::Owned(format!("/{}", kernel_filename)),
            initrd: Cow::Owned(format!("/{}", initramfs_filename)),
            // Include console=ttyS0 for serial console output (needed for QEMU testing)
            options: Cow::Borrowed("root=LABEL=root rw console=ttyS0,115200 console=tty0"),
        }
    }

    /// Create a boot entry with the given root device.
    pub fn with_root(
        os_id: &str,
        os_name: &str,
        kernel_filename: &str,
        initramfs_filename: &str,
        root_device: impl Into<String>,
    ) -> Self {
        // Include console=ttyS0 for serial console output (needed for QEMU testing)
        Self {
            options: Cow::Owned(format!("root={} rw console=ttyS0,115200 console=tty0", root_device.into())),
            ..Self::with_defaults(os_id, os_name, kernel_filename, initramfs_filename)
        }
    }

    /// Create a boot entry using PARTUUID.
    pub fn with_partuuid(
        os_id: &str,
        os_name: &str,
        kernel_filename: &str,
        initramfs_filename: &str,
        partuuid: impl Into<String>,
    ) -> Self {
        Self::with_root(
            os_id,
            os_name,
            kernel_filename,
            initramfs_filename,
            format!("PARTUUID={}", partuuid.into()),
        )
    }

    /// Create a boot entry using LABEL.
    pub fn with_label(
        os_id: &str,
        os_name: &str,
        kernel_filename: &str,
        initramfs_filename: &str,
        label: impl Into<String>,
    ) -> Self {
        Self::with_root(
            os_id,
            os_name,
            kernel_filename,
            initramfs_filename,
            format!("LABEL={}", label.into()),
        )
    }

    /// Get the full path for this entry file.
    pub fn entry_path(&self) -> String {
        format!("{}/{}.conf", ENTRIES_DIR, self.filename)
    }

    /// Generate the entry file contents.
    pub fn to_entry_file(&self) -> String {
        format!(
            "title   {}\nlinux   {}\ninitrd  {}\noptions {}\n",
            self.title, self.linux, self.initrd, self.options
        )
    }

    /// Add microcode initrd (for Intel or AMD).
    pub fn with_microcode(mut self, ucode_path: &str) -> Self {
        // Microcode must come before main initrd
        self.initrd = Cow::Owned(format!("{}\ninitrd  {}", ucode_path, self.initrd));
        self
    }

    /// Update the root device in options.
    pub fn set_root(mut self, root_device: impl Into<String>) -> Self {
        self.options = Cow::Owned(format!("root={} rw console=ttyS0,115200 console=tty0", root_device.into()));
        self
    }
}

// =============================================================================
// Loader Configuration
// =============================================================================

/// Loader configuration (loader.conf) for systemd-boot.
///
/// Uses `Cow<'static, str>` where values are often static strings.
#[derive(Debug, Clone)]
pub struct LoaderConfig {
    /// Default entry to boot (filename without .conf)
    pub default_entry: Cow<'static, str>,
    /// Timeout in seconds (0 = no menu)
    pub timeout: u32,
    /// Console mode (keep, auto, max, or resolution)
    pub console_mode: Option<Cow<'static, str>>,
    /// Editor enabled (allows kernel cmdline editing)
    pub editor: bool,
}

impl LoaderConfig {
    /// Create a loader config with distro-specific defaults.
    pub fn with_defaults(os_id: &str) -> Self {
        Self {
            default_entry: Cow::Owned(os_id.to_string()),
            timeout: DEFAULT_TIMEOUT,
            console_mode: None,
            editor: true,
        }
    }

    /// Generate the loader.conf contents.
    pub fn to_loader_conf(&self) -> String {
        let mut conf = format!(
            "default {}.conf\ntimeout {}\n",
            self.default_entry, self.timeout
        );

        if let Some(ref mode) = self.console_mode {
            conf.push_str(&format!("console-mode {}\n", mode));
        }

        if !self.editor {
            conf.push_str("editor no\n");
        }

        conf
    }

    /// Set timeout.
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set console mode.
    pub fn with_console_mode(mut self, mode: impl Into<Cow<'static, str>>) -> Self {
        self.console_mode = Some(mode.into());
        self
    }

    /// Disable editor.
    pub fn disable_editor(mut self) -> Self {
        self.editor = false;
        self
    }
}

/// Command to install systemd-boot to the ESP.
pub fn bootctl_install_command() -> &'static str {
    "bootctl install"
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure BootEntry doesn't grow unexpectedly.
    ///
    /// BootEntry uses Cow<'static, str> for fields, so:
    /// - 5 fields × 24 bytes (Cow = ptr + len + discriminant) = 120 bytes
    /// - Actual size may vary slightly by alignment
    #[test]
    fn boot_entry_size() {
        let size = std::mem::size_of::<BootEntry>();
        // Allow up to 128 bytes (5 Cow fields with padding)
        assert!(
            size <= 128,
            "BootEntry grew too large: {} bytes (max 128)",
            size
        );
        eprintln!("BootEntry size: {} bytes", size);
    }

    /// Ensure LoaderConfig doesn't grow unexpectedly.
    #[test]
    fn loader_config_size() {
        let size = std::mem::size_of::<LoaderConfig>();
        // Cow<str> (24) + u32 (4) + Option<Cow<str>> (24) + bool (1) + padding
        assert!(
            size <= 64,
            "LoaderConfig grew too large: {} bytes (max 64)",
            size
        );
        eprintln!("LoaderConfig size: {} bytes", size);
    }
}
