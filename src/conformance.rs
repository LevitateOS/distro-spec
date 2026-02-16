//! Legacy conformance declarations for distro checkpoint contracts.
//!
//! # Deprecation Notice
//!
//! Runtime CP0 conformance authority has moved to `distro-variants/*/cp0.toml`
//! and is loaded by `distro-contract`.
//!
//! This module remains as a compatibility bridge while remaining variants are
//! migrated to `distro-variants/*`.

use distro_contract::{
    require_valid_contract, ArtifactIdentity, AuthMode, AutomatedLoginCheckpoint, BootCheckpoint,
    BuildCapabilityCheckpoint, CheckpointContract, ConformanceContract, DistroIdentity,
    InstallCheckpoint, ReleaseCheckpoint, RootfsMutability, RuntimePolicyCheckpoint,
    ScriptEvidence, ToolsCheckpoint, CONTRACT_SCHEMA_VERSION,
};

fn str_vec(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| (*v).to_string()).collect()
}

fn baseline_cp8_metadata() -> Vec<String> {
    str_vec(&[
        "kernel_source.version",
        "kernel_source.sha256",
        "kernel_source.localversion",
        "artifact.rootfs_name",
        "artifact.iso_filename",
    ])
}

fn baseline_cp0_build_tools() -> Vec<String> {
    str_vec(&[
        "recipe",
        "cargo",
        "make",
        "recuki",
        "ukify",
        "mkfs.erofs",
        "xorriso",
        "reciso",
        "recinit",
        "recstrap",
        "recfstab",
        "recchroot",
    ])
}

fn levitate_contract() -> ConformanceContract {
    use crate::levitate;

    ConformanceContract {
        schema_version: CONTRACT_SCHEMA_VERSION,
        identity: DistroIdentity {
            os_name: levitate::OS_NAME.to_string(),
            os_id: levitate::OS_ID.to_string(),
            iso_label: levitate::ISO_LABEL.to_string(),
            os_version: levitate::OS_VERSION.to_string(),
            default_hostname: levitate::DEFAULT_HOSTNAME.to_string(),
        },
        artifacts: ArtifactIdentity {
            rootfs_name: levitate::ROOTFS_NAME.to_string(),
            initramfs_live_output: levitate::INITRAMFS_LIVE_OUTPUT.to_string(),
            iso_filename: levitate::ISO_FILENAME.to_string(),
            initramfs_installed_output: Some(levitate::INITRAMFS_INSTALLED_OUTPUT.to_string()),
        },
        checkpoints: CheckpointContract {
            cp0_build: BuildCapabilityCheckpoint {
                required_build_tools: baseline_cp0_build_tools(),
                kernel_kconfig_path: "kconfig".to_string(),
                recipe_kernel_script: "distro-builder/recipes/linux.rhai".to_string(),
                recipe_kernel_invocation: "recipe install".to_string(),
                kernel_release_path: "kernel-build/include/config/kernel.release".to_string(),
                kernel_image_path: "staging/boot/vmlinuz".to_string(),
                kernel_modules_path: "staging/usr/lib/modules/<kernel.release>".to_string(),
                kernel_version: levitate::KERNEL_SOURCE.version.to_string(),
                kernel_sha256: levitate::KERNEL_SOURCE.sha256.to_string(),
                kernel_localversion: levitate::KERNEL_SOURCE.localversion.to_string(),
                module_install_path: levitate::MODULE_INSTALL_PATH.to_string(),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-0-build-capability.sh".to_string(),
                    pass_marker: "CHECKPOINT 0 PASSED".to_string(),
                },
            },
            cp1_live_boot: BootCheckpoint {
                success_patterns: str_vec(&[
                    "LevitateOS Live",
                    "Reached target Multi-User System.",
                ]),
                fatal_patterns: str_vec(&[
                    "No bootable device",
                    "Kernel panic",
                    "VFS: Cannot open root device",
                    "No init found",
                    "SQUASHFS error",
                    "EROFS:",
                    "emergency shell",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-1-live-boot.sh".to_string(),
                    pass_marker: "CHECKPOINT 1 PASSED".to_string(),
                },
            },
            cp2_live_tools: ToolsCheckpoint {
                required_tools: str_vec(&[
                    "recstrap",
                    "recfstab",
                    "recchroot",
                    "sfdisk",
                    "mkfs.ext4",
                    "mount",
                    "ip",
                    "ping",
                    "curl",
                    "lspci",
                    "lsusb",
                    "vi",
                    "less",
                    "grep",
                    "find",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-2-live-tools.sh".to_string(),
                    pass_marker: "CHECKPOINT 2 PASSED".to_string(),
                },
            },
            cp3_install: InstallCheckpoint {
                required_tools: str_vec(&[
                    "recstrap",
                    "recfstab",
                    "recchroot",
                    "sfdisk",
                    "mkfs.ext4",
                    "mount",
                ]),
                required_services: str_vec(&["NetworkManager", "chronyd"]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-3-installation.sh".to_string(),
                    pass_marker: "CHECKPOINT 3 PASSED".to_string(),
                },
            },
            cp4_installed_boot: BootCheckpoint {
                success_patterns: str_vec(&[
                    "levitateos login:",
                    "Reached target Multi-User System.",
                ]),
                fatal_patterns: str_vec(&[
                    "Kernel panic",
                    "VFS: Cannot open root device",
                    "No init found",
                    "emergency shell",
                    "Failed to start",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-4-installed-boot.sh".to_string(),
                    pass_marker: "CHECKPOINT 4 PASSED".to_string(),
                },
            },
            cp5_automated_login: AutomatedLoginCheckpoint {
                auth_mode: AuthMode::DefaultPasswordLogin,
                default_username: Some("levitate".to_string()),
                default_password: Some("levitate".to_string()),
                login_prompt_pattern: "levitateos login:".to_string(),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-5-automated-login.sh".to_string(),
                    pass_marker: "CHECKPOINT 5 PASSED".to_string(),
                },
            },
            cp6_installed_tools: ToolsCheckpoint {
                required_tools: str_vec(&["sudo", "ip", "ssh", "mount", "umount", "dmesg"]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-6-daily-driver.sh".to_string(),
                    pass_marker: "CHECKPOINT 6 PASSED".to_string(),
                },
            },
            cp7_runtime_policy: RuntimePolicyCheckpoint {
                rootfs_mutability: RootfsMutability::Mutable,
                mutable_required_rw_paths: str_vec(&["/etc", "/var", "/home", "/usr/local"]),
                immutable_required_ro_paths: vec![],
            },
            cp8_release: ReleaseCheckpoint {
                required_artifacts: str_vec(&[
                    levitate::ROOTFS_NAME,
                    levitate::INITRAMFS_LIVE_OUTPUT,
                    levitate::INITRAMFS_INSTALLED_OUTPUT,
                    levitate::ISO_FILENAME,
                ]),
                required_metadata: baseline_cp8_metadata(),
            },
        },
    }
}

fn acorn_contract() -> ConformanceContract {
    use crate::acorn;

    ConformanceContract {
        schema_version: CONTRACT_SCHEMA_VERSION,
        identity: DistroIdentity {
            os_name: acorn::OS_NAME.to_string(),
            os_id: acorn::OS_ID.to_string(),
            iso_label: acorn::ISO_LABEL.to_string(),
            os_version: acorn::OS_VERSION.to_string(),
            default_hostname: acorn::DEFAULT_HOSTNAME.to_string(),
        },
        artifacts: ArtifactIdentity {
            rootfs_name: acorn::ROOTFS_NAME.to_string(),
            initramfs_live_output: acorn::INITRAMFS_LIVE_OUTPUT.to_string(),
            iso_filename: acorn::ISO_FILENAME.to_string(),
            initramfs_installed_output: None,
        },
        checkpoints: CheckpointContract {
            cp0_build: BuildCapabilityCheckpoint {
                required_build_tools: baseline_cp0_build_tools(),
                kernel_kconfig_path: "kconfig".to_string(),
                recipe_kernel_script: "distro-builder/recipes/linux.rhai".to_string(),
                recipe_kernel_invocation: "recipe install".to_string(),
                kernel_release_path: "kernel-build/include/config/kernel.release".to_string(),
                kernel_image_path: "staging/boot/vmlinuz".to_string(),
                kernel_modules_path: "staging/usr/lib/modules/<kernel.release>".to_string(),
                kernel_version: acorn::KERNEL_SOURCE.version.to_string(),
                kernel_sha256: acorn::KERNEL_SOURCE.sha256.to_string(),
                kernel_localversion: acorn::KERNEL_SOURCE.localversion.to_string(),
                module_install_path: acorn::MODULE_INSTALL_PATH.to_string(),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-0-build-capability.sh".to_string(),
                    pass_marker: "CHECKPOINT 0 PASSED".to_string(),
                },
            },
            cp1_live_boot: BootCheckpoint {
                success_patterns: str_vec(&["AcornOS", "OpenRC"]),
                fatal_patterns: str_vec(&[
                    "No bootable device",
                    "Kernel panic",
                    "VFS: Cannot open root device",
                    "No init found",
                    "SQUASHFS error",
                    "EROFS:",
                    "emergency shell",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-1-live-boot.sh".to_string(),
                    pass_marker: "CHECKPOINT 1 PASSED".to_string(),
                },
            },
            cp2_live_tools: ToolsCheckpoint {
                required_tools: str_vec(&[
                    "recstrap",
                    "recfstab",
                    "recchroot",
                    "sfdisk",
                    "mkfs.ext4",
                    "mount",
                    "ip",
                    "ping",
                    "curl",
                    "lspci",
                    "lsusb",
                    "smartctl",
                    "hdparm",
                    "vim",
                    "less",
                    "htop",
                    "grep",
                    "find",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-2-live-tools.sh".to_string(),
                    pass_marker: "CHECKPOINT 2 PASSED".to_string(),
                },
            },
            cp3_install: InstallCheckpoint {
                required_tools: str_vec(&[
                    "recstrap",
                    "recfstab",
                    "recchroot",
                    "sfdisk",
                    "mkfs.ext4",
                    "mount",
                ]),
                required_services: str_vec(&["networking", "chronyd"]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-3-installation.sh".to_string(),
                    pass_marker: "CHECKPOINT 3 PASSED".to_string(),
                },
            },
            cp4_installed_boot: BootCheckpoint {
                success_patterns: str_vec(&["acornos login:", "Welcome to AcornOS"]),
                fatal_patterns: str_vec(&[
                    "Kernel panic",
                    "VFS: Cannot open root device",
                    "No init found",
                    "emergency shell",
                    "Timed out waiting for device",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-4-installed-boot.sh".to_string(),
                    pass_marker: "CHECKPOINT 4 PASSED".to_string(),
                },
            },
            cp5_automated_login: AutomatedLoginCheckpoint {
                auth_mode: AuthMode::DefaultPasswordLogin,
                default_username: Some("acorn".to_string()),
                default_password: Some("acorn".to_string()),
                login_prompt_pattern: "acornos login:".to_string(),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-5-automated-login.sh".to_string(),
                    pass_marker: "CHECKPOINT 5 PASSED".to_string(),
                },
            },
            cp6_installed_tools: ToolsCheckpoint {
                required_tools: str_vec(&[
                    "sudo", "ip", "ssh", "ash", "mount", "umount", "dmesg", "ps", "ls", "cat",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-6-daily-driver.sh".to_string(),
                    pass_marker: "CHECKPOINT 6 PASSED".to_string(),
                },
            },
            cp7_runtime_policy: RuntimePolicyCheckpoint {
                rootfs_mutability: RootfsMutability::Mutable,
                mutable_required_rw_paths: str_vec(&["/etc", "/var", "/home", "/usr/local"]),
                immutable_required_ro_paths: vec![],
            },
            cp8_release: ReleaseCheckpoint {
                required_artifacts: str_vec(&[
                    acorn::ROOTFS_NAME,
                    acorn::INITRAMFS_LIVE_OUTPUT,
                    acorn::ISO_FILENAME,
                ]),
                required_metadata: baseline_cp8_metadata(),
            },
        },
    }
}

fn iuppiter_contract() -> ConformanceContract {
    use crate::iuppiter;

    let mut metadata = baseline_cp8_metadata();
    metadata.push("artifact.disk_image_filename".to_string());

    ConformanceContract {
        schema_version: CONTRACT_SCHEMA_VERSION,
        identity: DistroIdentity {
            os_name: iuppiter::OS_NAME.to_string(),
            os_id: iuppiter::OS_ID.to_string(),
            iso_label: iuppiter::ISO_LABEL.to_string(),
            os_version: iuppiter::OS_VERSION.to_string(),
            default_hostname: iuppiter::DEFAULT_HOSTNAME.to_string(),
        },
        artifacts: ArtifactIdentity {
            rootfs_name: iuppiter::ROOTFS_NAME.to_string(),
            initramfs_live_output: iuppiter::INITRAMFS_LIVE_OUTPUT.to_string(),
            iso_filename: iuppiter::ISO_FILENAME.to_string(),
            initramfs_installed_output: None,
        },
        checkpoints: CheckpointContract {
            cp0_build: BuildCapabilityCheckpoint {
                required_build_tools: baseline_cp0_build_tools(),
                kernel_kconfig_path: "kconfig".to_string(),
                recipe_kernel_script: "distro-builder/recipes/linux.rhai".to_string(),
                recipe_kernel_invocation: "recipe install".to_string(),
                kernel_release_path: "kernel-build/include/config/kernel.release".to_string(),
                kernel_image_path: "staging/boot/vmlinuz".to_string(),
                kernel_modules_path: "staging/usr/lib/modules/<kernel.release>".to_string(),
                kernel_version: iuppiter::KERNEL_SOURCE.version.to_string(),
                kernel_sha256: iuppiter::KERNEL_SOURCE.sha256.to_string(),
                kernel_localversion: iuppiter::KERNEL_SOURCE.localversion.to_string(),
                module_install_path: iuppiter::MODULE_INSTALL_PATH.to_string(),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-0-build-capability.sh".to_string(),
                    pass_marker: "CHECKPOINT 0 PASSED".to_string(),
                },
            },
            cp1_live_boot: BootCheckpoint {
                success_patterns: str_vec(&["IuppiterOS", "OpenRC"]),
                fatal_patterns: str_vec(&[
                    "No bootable device",
                    "Kernel panic",
                    "VFS: Cannot open root device",
                    "No init found",
                    "SQUASHFS error",
                    "EROFS:",
                    "emergency shell",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-1-live-boot.sh".to_string(),
                    pass_marker: "CHECKPOINT 1 PASSED".to_string(),
                },
            },
            cp2_live_tools: ToolsCheckpoint {
                required_tools: str_vec(&[
                    "recstrap",
                    "recfstab",
                    "recchroot",
                    "sfdisk",
                    "mkfs.ext4",
                    "mount",
                    "smartctl",
                    "hdparm",
                    "sg_inq",
                    "ip",
                    "ping",
                    "less",
                    "grep",
                    "find",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-2-live-tools.sh".to_string(),
                    pass_marker: "CHECKPOINT 2 PASSED".to_string(),
                },
            },
            cp3_install: InstallCheckpoint {
                required_tools: str_vec(&[
                    "recstrap",
                    "recfstab",
                    "recchroot",
                    "sfdisk",
                    "mkfs.ext4",
                    "mount",
                ]),
                required_services: str_vec(&[
                    "networking",
                    "chronyd",
                    "sshd",
                    "seatd",
                    "iuppiter_dar",
                    "iuppiter_kiosk",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-3-installation.sh".to_string(),
                    pass_marker: "CHECKPOINT 3 PASSED".to_string(),
                },
            },
            cp4_installed_boot: BootCheckpoint {
                success_patterns: str_vec(&["iuppiter login:", "Welcome to IuppiterOS"]),
                fatal_patterns: str_vec(&[
                    "Kernel panic",
                    "VFS: Cannot open root device",
                    "No init found",
                    "emergency shell",
                    "Timed out waiting for device",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-4-installed-boot.sh".to_string(),
                    pass_marker: "CHECKPOINT 4 PASSED".to_string(),
                },
            },
            cp5_automated_login: AutomatedLoginCheckpoint {
                auth_mode: AuthMode::DefaultPasswordLogin,
                default_username: Some("operator".to_string()),
                default_password: Some("iuppiter".to_string()),
                login_prompt_pattern: "iuppiter login:".to_string(),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-5-automated-login.sh".to_string(),
                    pass_marker: "CHECKPOINT 5 PASSED".to_string(),
                },
            },
            cp6_installed_tools: ToolsCheckpoint {
                required_tools: str_vec(&[
                    "sudo", "ip", "ssh", "ash", "smartctl", "hdparm", "sg_inq", "mount", "umount",
                    "dmesg",
                ]),
                evidence: ScriptEvidence {
                    script_path: "checkpoint-6-daily-driver.sh".to_string(),
                    pass_marker: "CHECKPOINT 6 PASSED".to_string(),
                },
            },
            cp7_runtime_policy: RuntimePolicyCheckpoint {
                rootfs_mutability: RootfsMutability::Immutable,
                mutable_required_rw_paths: vec![],
                immutable_required_ro_paths: str_vec(&["/", "/usr", "/opt"]),
            },
            cp8_release: ReleaseCheckpoint {
                required_artifacts: str_vec(&[
                    iuppiter::ROOTFS_NAME,
                    iuppiter::INITRAMFS_LIVE_OUTPUT,
                    iuppiter::ISO_FILENAME,
                    iuppiter::DISK_IMAGE_FILENAME,
                ]),
                required_metadata: metadata,
            },
        },
    }
}

/// Get the legacy canonical conformance contract for a distro id.
///
/// Deprecated runtime source of truth: CP0 now loads from
/// `distro-variants/*/cp0.toml`.
pub fn contract_for_distro(distro_id: &str) -> Option<ConformanceContract> {
    match distro_id {
        "levitate" | "levitateos" => Some(levitate_contract()),
        "acorn" | "acornos" => Some(acorn_contract()),
        "iuppiter" | "iuppiteros" => Some(iuppiter_contract()),
        _ => None,
    }
}

/// Validate the legacy canonical conformance contract for a distro.
pub fn require_valid_contract_for_distro(distro_id: &str) -> Result<(), String> {
    let Some(contract) = contract_for_distro(distro_id) else {
        return Err(format!("unknown distro id '{distro_id}'"));
    };

    require_valid_contract(&contract).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_contracts_are_valid() {
        for distro in ["levitate", "acorn", "iuppiter"] {
            require_valid_contract_for_distro(distro)
                .unwrap_or_else(|err| panic!("{} contract invalid: {}", distro, err));
        }
    }

    #[test]
    fn cp0_baseline_includes_uki_tooling() {
        let tools = baseline_cp0_build_tools();
        assert!(tools.iter().any(|t| t == "recuki"));
        assert!(tools.iter().any(|t| t == "ukify"));
    }
}
