//! Shared boot configuration types for UEFI systems.
//!
//! These types are used by both LevitateOS and AcornOS since both
//! use systemd-boot as the bootloader.

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
#[derive(Debug, Clone)]
pub struct BootEntry {
    /// Entry filename (without .conf extension)
    pub filename: String,
    /// Title shown in boot menu
    pub title: String,
    /// Path to kernel (relative to ESP)
    pub linux: String,
    /// Path to initramfs (relative to ESP)
    pub initrd: String,
    /// Kernel command line options
    pub options: String,
}

impl BootEntry {
    /// Create a new boot entry with all fields specified.
    pub fn new(
        filename: impl Into<String>,
        title: impl Into<String>,
        linux: impl Into<String>,
        initrd: impl Into<String>,
        options: impl Into<String>,
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
            filename: os_id.to_string(),
            title: os_name.to_string(),
            linux: format!("/{}", kernel_filename),
            initrd: format!("/{}", initramfs_filename),
            options: "root=LABEL=root rw quiet".to_string(),
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
        Self {
            options: format!("root={} rw quiet", root_device.into()),
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
        self.initrd = format!("{}\ninitrd  {}", ucode_path, self.initrd);
        self
    }

    /// Update the root device in options.
    pub fn set_root(mut self, root_device: impl Into<String>) -> Self {
        self.options = format!("root={} rw quiet", root_device.into());
        self
    }
}

// =============================================================================
// Loader Configuration
// =============================================================================

/// Loader configuration (loader.conf) for systemd-boot.
#[derive(Debug, Clone)]
pub struct LoaderConfig {
    /// Default entry to boot (filename without .conf)
    pub default_entry: String,
    /// Timeout in seconds (0 = no menu)
    pub timeout: u32,
    /// Console mode (keep, auto, max, or resolution)
    pub console_mode: Option<String>,
    /// Editor enabled (allows kernel cmdline editing)
    pub editor: bool,
}

impl LoaderConfig {
    /// Create a loader config with distro-specific defaults.
    pub fn with_defaults(os_id: &str) -> Self {
        Self {
            default_entry: os_id.to_string(),
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
    pub fn with_console_mode(mut self, mode: impl Into<String>) -> Self {
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
