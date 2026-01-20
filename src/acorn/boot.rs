//! Bootloader configuration.
//!
//! Defines systemd-boot configuration for UEFI systems.
//! AcornOS uses systemd-boot despite using OpenRC for init.

use super::paths::{KERNEL_FILENAME, INITRAMFS_FILENAME, OS_NAME, OS_ID};

/// Path to the EFI System Partition mount point.
pub const ESP_MOUNT_POINT: &str = "/boot";

/// Path to systemd-boot loader configuration.
pub const LOADER_CONF_PATH: &str = "/boot/loader/loader.conf";

/// Path to boot entries directory.
pub const ENTRIES_DIR: &str = "/boot/loader/entries";

/// Default timeout for boot menu (in seconds).
pub const DEFAULT_TIMEOUT: u32 = 3;

/// Boot entry configuration.
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

impl Default for BootEntry {
    fn default() -> Self {
        Self {
            filename: OS_ID.to_string(),
            title: OS_NAME.to_string(),
            linux: format!("/{}", KERNEL_FILENAME),
            initrd: format!("/{}", INITRAMFS_FILENAME),
            options: "root=LABEL=root rw quiet".to_string(),
        }
    }
}

impl BootEntry {
    /// Create a new boot entry with the given root device.
    pub fn with_root(root_device: impl Into<String>) -> Self {
        Self {
            options: format!("root={} rw quiet", root_device.into()),
            ..Default::default()
        }
    }

    /// Create a boot entry using PARTUUID.
    pub fn with_partuuid(partuuid: impl Into<String>) -> Self {
        Self::with_root(format!("PARTUUID={}", partuuid.into()))
    }

    /// Create a boot entry using LABEL.
    pub fn with_label(label: impl Into<String>) -> Self {
        Self::with_root(format!("LABEL={}", label.into()))
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
        self.initrd = format!("{}\ninitrd  {}", ucode_path, self.initrd);
        self
    }
}

/// Loader configuration (loader.conf).
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

impl Default for LoaderConfig {
    fn default() -> Self {
        Self {
            default_entry: OS_ID.to_string(),
            timeout: DEFAULT_TIMEOUT,
            console_mode: None,
            editor: true,
        }
    }
}

impl LoaderConfig {
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
}

/// Command to install systemd-boot to the ESP.
///
/// Note: On Alpine/AcornOS, you may need to install systemd-boot separately
/// since it's not bundled with OpenRC. Consider using gummiboot or efibootmgr.
pub fn bootctl_install_command() -> &'static str {
    "bootctl install"
}
