//! System user, group, and installation tool definitions.

/// System users that must exist in /etc/passwd.
pub const SYSTEM_USERS: &[&str] = &[
    "root", "dbus", "sshd", "chrony", "polkitd",
    "pipewire", // For PipeWire system mode (optional)
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
    "bluetooth", // Users in this group can use bluetooth
    "audio",     // Users in this group can use audio
    "video",     // Users in this group can use video devices
];

/// LevitateOS-specific installation tools (ALL).
///
/// These are REQUIRED for installing LevitateOS from the live ISO.
/// They are automatically rebuilt during ISO creation to ensure latest versions.
///
/// - recstrap: Rootfs extractor (like pacstrap)
/// - recfstab: Fstab generator (like genfstab)
/// - recchroot: Chroot helper (like arch-chroot)
/// - recipe: Package manager (like pacman)
/// - levitate-docs: Interactive documentation TUI
pub const LEVITATE_TOOLS: &[&str] = &[
    "recstrap",
    "recfstab",
    "recchroot",
    "recipe",
    "levitate-docs",
];

/// Cargo-based installation tools that are built together.
///
/// These are Rust crates in tools/ that share the same build command:
/// `cargo build --release -p recstrap -p recfstab -p recchroot`
///
/// Note: `recipe` is also a cargo crate but has a different package name
/// (`levitate-recipe`) so it's built separately.
pub const LEVITATE_CARGO_TOOLS: &[&str] = &["recstrap", "recfstab", "recchroot"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levitate_tools_complete() {
        // CRITICAL: These tools are required for installation from the live ISO.
        // If any are missing, users cannot install LevitateOS.
        let required_tools = [
            "recstrap",      // Rootfs extractor (like pacstrap)
            "recfstab",      // Fstab generator (like genfstab)
            "recchroot",     // Chroot helper (like arch-chroot)
            "recipe",        // Package manager
            "levitate-docs", // Interactive documentation TUI
        ];

        for tool in required_tools {
            assert!(
                LEVITATE_TOOLS.contains(&tool),
                "LEVITATE_TOOLS missing '{}' - users cannot install without this!",
                tool
            );
        }
    }

    // CRITICAL REGRESSION TEST: Documentation TUI must always be in live environment
    #[test]
    fn test_levitate_docs_in_tools() {
        assert!(
            LEVITATE_TOOLS.contains(&"levitate-docs"),
            "CRITICAL: levitate-docs missing from LEVITATE_TOOLS! \
             Users cannot access documentation during installation. \
             Without this, installation process is undocumented and confusing."
        );
    }

    #[test]
    fn test_all_levitate_tools_present() {
        let required_installation_tools = [
            "recstrap",      // Rootfs extractor - required to extract the OS
            "recfstab",      // Fstab generator - required to configure boot
            "recchroot",     // Chroot helper - required to configure installed system
            "recipe",        // Package manager - required for custom packages
            "levitate-docs", // Documentation - required for installation guidance
        ];

        for tool in required_installation_tools {
            assert!(
                LEVITATE_TOOLS.contains(&tool),
                "CRITICAL: {} missing from LEVITATE_TOOLS! \
                 Users cannot complete installation without this tool.",
                tool
            );
        }
    }
}
