//! Partition layout specification.
//!
//! Defines the standard partition scheme for LevitateOS installations.

/// Size of the EFI System Partition in megabytes.
pub const EFI_PARTITION_SIZE_MB: u32 = 512;

/// EFI partition label.
pub const EFI_PARTITION_LABEL: &str = "EFI";

/// Root partition label.
pub const ROOT_PARTITION_LABEL: &str = "root";

/// EFI partition filesystem type.
pub const EFI_FILESYSTEM: &str = "vfat";

/// Root partition filesystem type.
pub const ROOT_FILESYSTEM: &str = "ext4";

/// Standard partition layout for UEFI installations.
#[derive(Debug, Clone)]
pub struct PartitionLayout {
    /// EFI System Partition
    pub efi: PartitionSpec,
    /// Root partition
    pub root: PartitionSpec,
}

/// Specification for a single partition.
#[derive(Debug, Clone)]
pub struct PartitionSpec {
    /// Partition number (1-based)
    pub number: u8,
    /// Size in MB (0 = use remaining space)
    pub size_mb: u32,
    /// Filesystem type
    pub filesystem: &'static str,
    /// Label
    pub label: &'static str,
    /// Mount point
    pub mount_point: &'static str,
    /// GPT partition type (sfdisk format)
    pub gpt_type: &'static str,
}

impl Default for PartitionLayout {
    fn default() -> Self {
        Self {
            efi: PartitionSpec {
                number: 1,
                size_mb: EFI_PARTITION_SIZE_MB,
                filesystem: EFI_FILESYSTEM,
                label: EFI_PARTITION_LABEL,
                mount_point: "/boot",
                gpt_type: "U", // EFI System
            },
            root: PartitionSpec {
                number: 2,
                size_mb: 0, // Use remaining space
                filesystem: ROOT_FILESYSTEM,
                label: ROOT_PARTITION_LABEL,
                mount_point: "/",
                gpt_type: "L", // Linux filesystem
            },
        }
    }
}

impl PartitionLayout {
    /// Generate sfdisk script for this layout.
    pub fn to_sfdisk_script(&self) -> String {
        format!(
            "label: gpt\n,{}M,{},*\n,,{}\n",
            self.efi.size_mb, self.efi.gpt_type, self.root.gpt_type
        )
    }
}
