//! Hardware requirements for LevitateOS installation.
//!
//! MENTAL MODEL: This is for daily-driver desktops competing with Windows/macOS.
//! Think "gaming PC" or "developer workstation", NOT "Raspberry Pi" or "Docker container".
//!
//! When reasoning about hardware, think modern desktop/laptop (2020+), not server/embedded.

/// Hardware requirements for system installation.
///
/// These values define the floor for a usable daily-driver desktop experience.
/// They are NOT "minimum to boot" values - they represent what users actually need.
#[derive(Debug, Clone, Copy)]
pub struct SystemRequirements {
    /// Minimum RAM in gigabytes (floor for daily desktop use)
    pub min_ram_gb: u32,
    /// Recommended RAM in gigabytes (comfortable multi-tasking)
    pub recommended_ram_gb: u32,

    /// Minimum storage in gigabytes (base system + dev tools)
    pub min_disk_gb: u32,
    /// Recommended storage in gigabytes (room for projects, VMs, games)
    pub recommended_disk_gb: u32,

    /// CPU microarchitecture level (x86-64-v2, x86-64-v3, etc.)
    pub cpu_microarch: &'static str,
    /// Supported CPU vendors (AMD first - dominant in desktop market)
    pub supported_vendors: &'static [&'static str],

    /// Supported GPU vendors (discrete or integrated - both valid)
    pub gpu_vendors: &'static [&'static str],
}

/// LevitateOS system requirements.
///
/// These values target modern desktop/laptop hardware (2020+):
/// - 8-32 GB RAM (typical desktop/laptop)
/// - 256GB-2TB NVMe SSD (not spinning rust)
/// - Discrete GPU (NVIDIA/AMD) OR integrated (Intel/AMD)
/// - AMD Ryzen OR Intel Core (x86-64-v3 = Haswell+ / Zen+)
pub const LEVITATE_REQUIREMENTS: SystemRequirements = SystemRequirements {
    min_ram_gb: 8,           // Floor for daily desktop use
    recommended_ram_gb: 16,  // Comfortable multi-tasking

    min_disk_gb: 64,          // Bare minimum (base + dev tools)
    recommended_disk_gb: 256, // Room for projects, VMs, games

    cpu_microarch: "x86-64-v3",           // Haswell+ / Zen+ (AVX2 required)
    supported_vendors: &["AMD", "Intel"], // AMD first - dominant in desktop

    gpu_vendors: &["AMD", "NVIDIA", "Intel"], // Discrete or integrated
};

/// AcornOS system requirements.
///
/// Slightly lower floor than LevitateOS due to musl/busybox base,
/// but still targeting daily-driver desktop use, NOT embedded.
pub const ACORN_REQUIREMENTS: SystemRequirements = SystemRequirements {
    min_ram_gb: 4,           // Lower floor due to musl/busybox
    recommended_ram_gb: 8,   // Comfortable for Alpine-based desktop

    min_disk_gb: 32,          // Alpine is more compact
    recommended_disk_gb: 128, // Room for packages and user data

    cpu_microarch: "x86-64-v3",           // Same CPU requirements
    supported_vendors: &["AMD", "Intel"],

    gpu_vendors: &["AMD", "NVIDIA", "Intel"],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levitate_requirements_are_sane() {
        // Minimum should be less than or equal to recommended
        assert!(LEVITATE_REQUIREMENTS.min_ram_gb <= LEVITATE_REQUIREMENTS.recommended_ram_gb);
        assert!(LEVITATE_REQUIREMENTS.min_disk_gb <= LEVITATE_REQUIREMENTS.recommended_disk_gb);

        // Should have at least 2 CPU vendors
        assert!(LEVITATE_REQUIREMENTS.supported_vendors.len() >= 2);

        // Should have at least 2 GPU vendors
        assert!(LEVITATE_REQUIREMENTS.gpu_vendors.len() >= 2);

        // Minimum RAM should be at least 8GB for a daily-driver desktop
        assert!(LEVITATE_REQUIREMENTS.min_ram_gb >= 8);
    }

    #[test]
    fn acorn_requirements_are_sane() {
        assert!(ACORN_REQUIREMENTS.min_ram_gb <= ACORN_REQUIREMENTS.recommended_ram_gb);
        assert!(ACORN_REQUIREMENTS.min_disk_gb <= ACORN_REQUIREMENTS.recommended_disk_gb);
    }
}
