//! Kernel source specification.
//!
//! Defines kernel version and tarball download info for each distro variant.
//! All three distros download kernel tarballs from cdn.kernel.org instead of
//! using a shared git submodule, enabling different kernel versions per distro.
//!
//! SHA256 hashes are from <https://cdn.kernel.org/pub/linux/kernel/v6.x/sha256sums.asc>
//! (PGP-signed by Greg Kroah-Hartman).

/// Kernel source tarball specification.
pub struct KernelSource {
    /// Kernel version string (e.g., "6.19" or "6.18.9")
    pub version: &'static str,
    /// SHA256 hash of the .tar.xz tarball for verification
    pub sha256: &'static str,
    /// Localversion suffix (e.g., "-acorn", "-iuppiter", "-levitate")
    pub localversion: &'static str,
}

impl KernelSource {
    /// Get the tarball URL for this kernel version.
    ///
    /// All 6.x kernels live under `v6.x/` on cdn.kernel.org.
    pub fn tarball_url(&self) -> String {
        let major = self.version.split('.').next().unwrap_or("6");
        format!(
            "https://cdn.kernel.org/pub/linux/kernel/v{}.x/linux-{}.tar.xz",
            major, self.version
        )
    }

    /// Get the expected tarball filename.
    pub fn tarball_filename(&self) -> String {
        format!("linux-{}.tar.xz", self.version)
    }

    /// Get the extracted directory name.
    pub fn source_dir_name(&self) -> String {
        format!("linux-{}", self.version)
    }
}

/// AcornOS kernel: mainline 6.19
pub const ACORN_KERNEL: KernelSource = KernelSource {
    version: "6.19",
    sha256: "303079a8250b8f381f82b03f90463d12ac98d4f6b149b761ea75af1323521357",
    localversion: "-acorn",
};

/// IuppiterOS kernel: longterm (LTS) 6.12.71
pub const IUPPITER_KERNEL: KernelSource = KernelSource {
    version: "6.12.71",
    sha256: "143e8bc76cc41f831b51aa5e75819bed55bed41f299d35922820f1d2d2b02600",
    localversion: "-iuppiter",
};

/// LevitateOS kernel: longterm (LTS) 6.12.71
pub const LEVITATE_KERNEL: KernelSource = KernelSource {
    version: "6.12.71",
    sha256: "143e8bc76cc41f831b51aa5e75819bed55bed41f299d35922820f1d2d2b02600",
    localversion: "-levitate",
};
