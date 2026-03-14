//! Legacy conformance declarations for distro stage contracts.
//!
//! # Deprecation Notice
//!
//! Runtime build-host conformance authority has moved to `distro-variants/*/build-host.toml`
//! and is loaded by `distro-contract`.
//!
//! This module remains as a compatibility bridge while remaining variants are
//! migrated to `distro-variants/*`.

use distro_contract::{
    require_valid_contract, ArtifactIdentity, ArtifactTransform, AuthMode, AutomatedLoginStage,
    BootStage, BuildCapabilityStage, BuildContract, ConformanceContract, DistroIdentity,
    InstallStage, KernelBuildContract, ProductContract, ProductDecl, ReleaseContract, ReleaseStage,
    RootfsMutability, RuntimePolicyStage, ScenarioContract, ScriptEvidence, Stage00IsoAssembly,
    Stage00NonKernelInputs, StageContract, ToolsStage, TransformContract, CONTRACT_SCHEMA_VERSION,
};

fn str_vec(values: &[&str]) -> Vec<String> {
    values.iter().map(|v| (*v).to_string()).collect()
}

fn baseline_stage_08_metadata() -> Vec<String> {
    str_vec(&[
        "kernel_source.version",
        "kernel_source.sha256",
        "kernel_source.localversion",
        "artifact.rootfs_name",
        "artifact.iso_filename",
    ])
}

fn baseline_stage_00_build_tools() -> Vec<String> {
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

fn baseline_stage_00_non_kernel_inputs(
    rootfs_name: &str,
    initramfs_live_output: &str,
    _initramfs_installed_output: Option<&str>,
) -> Stage00NonKernelInputs {
    Stage00NonKernelInputs {
        required_for_00build: vec![
            rootfs_name.to_string(),
            initramfs_live_output.to_string(),
            overlay_output_name(rootfs_name),
        ],
        deferred_to_01boot: vec![],
        deferred_to_02livetools: vec![],
        deferred_to_03install_plus: vec![],
    }
}

fn overlay_output_name(rootfs_name: &str) -> String {
    let replaced = rootfs_name.replacen("filesystem.erofs", "overlayfs.erofs", 1);
    if replaced != rootfs_name {
        return replaced;
    }
    "overlayfs.erofs".to_string()
}

fn build_contract_from_legacy(stage_00: &BuildCapabilityStage) -> BuildContract {
    BuildContract {
        required_build_tools: stage_00.required_build_tools.clone(),
        kernel: KernelBuildContract {
            kconfig_path: stage_00.kernel_kconfig_path.clone(),
            recipe_script: stage_00.recipe_kernel_script.clone(),
            recipe_invocation: stage_00.recipe_kernel_invocation.clone(),
            release_path: stage_00.kernel_release_path.clone(),
            image_path: stage_00.kernel_image_path.clone(),
            modules_path: stage_00.kernel_modules_path.clone(),
            version: stage_00.kernel_version.clone(),
            sha256: stage_00.kernel_sha256.clone(),
            localversion: stage_00.kernel_localversion.clone(),
            module_install_path: stage_00.module_install_path.clone(),
        },
        evidence: stage_00.evidence.clone(),
    }
}

fn product_contract_from_legacy(artifacts: &ArtifactIdentity) -> ProductContract {
    ProductContract {
        rootfs_base: ProductDecl {
            logical_name: "product.rootfs.base".to_string(),
            description: "Canonical base root filesystem tree".to_string(),
            extends: None,
        },
        live_overlay: ProductDecl {
            logical_name: "product.payload.live_overlay".to_string(),
            description: "Read-only live overlay payload tree".to_string(),
            extends: None,
        },
        boot_live: ProductDecl {
            logical_name: "product.payload.boot.live".to_string(),
            description: "Live boot payload inputs".to_string(),
            extends: Some("product.rootfs.base".to_string()),
        },
        live_tools: ProductDecl {
            logical_name: "product.payload.live_tools".to_string(),
            description: "Live tools payload tree".to_string(),
            extends: Some("product.payload.boot.live".to_string()),
        },
        boot_installed: (artifacts.initramfs_installed_output.is_some()
            || !artifacts.installed_uki_outputs.is_empty()
            || artifacts.disk_image_output.is_some())
        .then_some(ProductDecl {
            logical_name: "product.payload.boot.installed".to_string(),
            description: "Installed-system boot payload inputs".to_string(),
            extends: Some("product.rootfs.base".to_string()),
        }),
        kernel_staging: ProductDecl {
            logical_name: "product.kernel.staging".to_string(),
            description: "Kernel image and modules staging product".to_string(),
            extends: None,
        },
    }
}

fn transform_contract_from_legacy(
    artifacts: &ArtifactIdentity,
    stage_00: &BuildCapabilityStage,
) -> TransformContract {
    TransformContract {
        rootfs_image: ArtifactTransform {
            logical_name: "artifact.rootfs.erofs".to_string(),
            dependencies: vec!["product.rootfs.base".to_string()],
            output_names: vec![artifacts.rootfs_name.clone()],
            format: "erofs".to_string(),
            extra_cmdline: None,
        },
        overlay_image: ArtifactTransform {
            logical_name: "artifact.overlay.erofs".to_string(),
            dependencies: vec!["product.payload.live_overlay".to_string()],
            output_names: vec![overlay_output_name(&artifacts.rootfs_name)],
            format: "erofs".to_string(),
            extra_cmdline: None,
        },
        initramfs_live: ArtifactTransform {
            logical_name: "artifact.initramfs.live".to_string(),
            dependencies: vec![
                "product.payload.boot.live".to_string(),
                "product.kernel.staging".to_string(),
            ],
            output_names: vec![artifacts.initramfs_live_output.clone()],
            format: "cpio.gz".to_string(),
            extra_cmdline: None,
        },
        initramfs_installed: artifacts.initramfs_installed_output.as_ref().map(|output| {
            ArtifactTransform {
                logical_name: "artifact.initramfs.installed".to_string(),
                dependencies: vec![
                    "product.payload.boot.installed".to_string(),
                    "product.kernel.staging".to_string(),
                ],
                output_names: vec![output.clone()],
                format: "img".to_string(),
                extra_cmdline: None,
            }
        }),
        live_uki: ArtifactTransform {
            logical_name: "artifact.uki.live".to_string(),
            dependencies: vec![
                "product.payload.boot.live".to_string(),
                "product.kernel.staging".to_string(),
            ],
            output_names: vec![
                stage_00.iso_assembly.live_uki_filename.clone(),
                stage_00.iso_assembly.emergency_uki_filename.clone(),
                stage_00.iso_assembly.debug_uki_filename.clone(),
            ],
            format: "uki".to_string(),
            extra_cmdline: Some(stage_00.iso_assembly.live_cmdline.clone()),
        },
        installed_uki: (!artifacts.installed_uki_outputs.is_empty()).then_some(ArtifactTransform {
            logical_name: "artifact.uki.installed".to_string(),
            dependencies: vec![
                "product.payload.boot.installed".to_string(),
                "product.kernel.staging".to_string(),
            ],
            output_names: artifacts.installed_uki_outputs.clone(),
            format: "uki".to_string(),
            extra_cmdline: None,
        }),
        iso: ArtifactTransform {
            logical_name: "artifact.iso".to_string(),
            dependencies: vec![
                "artifact.rootfs.erofs".to_string(),
                "artifact.overlay.erofs".to_string(),
                "artifact.initramfs.live".to_string(),
                "artifact.uki.live".to_string(),
            ],
            output_names: vec![artifacts.iso_filename.clone()],
            format: "iso".to_string(),
            extra_cmdline: None,
        },
        disk_image: artifacts
            .disk_image_output
            .as_ref()
            .map(|output| ArtifactTransform {
                logical_name: "artifact.disk".to_string(),
                dependencies: vec![
                    "product.rootfs.base".to_string(),
                    "product.kernel.staging".to_string(),
                ],
                output_names: vec![output.clone()],
                format: "img".to_string(),
                extra_cmdline: None,
            }),
    }
}

fn scenario_contract_from_legacy(stages: &StageContract) -> ScenarioContract {
    ScenarioContract {
        live_boot: Some(stages.stage_01_live_boot.clone()),
        live_tools: Some(stages.stage_02_live_tools.clone()),
        install: Some(stages.stage_03_install.clone()),
        installed_boot: Some(stages.stage_04_installed_boot.clone()),
        automated_login: Some(stages.stage_05_automated_login.clone()),
        installed_tools: Some(stages.stage_06_installed_tools.clone()),
        runtime_policy: Some(stages.stage_07_runtime_policy.clone()),
    }
}

fn release_contract_from_legacy(
    artifacts: &ArtifactIdentity,
    stage_08: &ReleaseStage,
) -> ReleaseContract {
    let mut primary_outputs = Vec::new();
    let mut supporting_artifacts = Vec::new();
    for artifact in &stage_08.required_artifacts {
        if artifact == &artifacts.iso_filename
            || artifacts
                .disk_image_output
                .as_ref()
                .map(|disk| artifact == disk)
                .unwrap_or(false)
        {
            primary_outputs.push(artifact.clone());
        } else {
            supporting_artifacts.push(artifact.clone());
        }
    }

    let mut metadata_outputs = Vec::new();
    let mut metadata_facts = Vec::new();
    for entry in &stage_08.required_metadata {
        if entry.starts_with("artifact.") || entry.starts_with("kernel_source.") {
            metadata_facts.push(entry.clone());
        } else if entry.ends_with(".sha512")
            || entry.ends_with(".sha256")
            || entry.ends_with(".sig")
            || entry.ends_with(".json")
        {
            metadata_outputs.push(entry.clone());
        } else {
            metadata_facts.push(entry.clone());
        }
    }

    ReleaseContract {
        primary_outputs,
        supporting_artifacts,
        metadata_outputs,
        metadata_facts,
    }
}

fn compat_contract(
    identity: DistroIdentity,
    artifacts: ArtifactIdentity,
    stages: StageContract,
) -> ConformanceContract {
    let build = build_contract_from_legacy(&stages.stage_00_build);
    let products = product_contract_from_legacy(&artifacts);
    let transforms = transform_contract_from_legacy(&artifacts, &stages.stage_00_build);
    let scenarios = scenario_contract_from_legacy(&stages);
    let release = release_contract_from_legacy(&artifacts, &stages.stage_08_release);

    ConformanceContract {
        schema_version: CONTRACT_SCHEMA_VERSION,
        identity,
        build,
        products,
        transforms,
        scenarios,
        release,
        artifacts,
        stages,
    }
}

fn levitate_contract() -> ConformanceContract {
    use crate::levitate;

    let identity = DistroIdentity {
        os_name: levitate::OS_NAME.to_string(),
        os_id: levitate::OS_ID.to_string(),
        iso_label: levitate::ISO_LABEL.to_string(),
        os_version: levitate::OS_VERSION.to_string(),
        default_hostname: levitate::DEFAULT_HOSTNAME.to_string(),
    };
    let artifacts = ArtifactIdentity {
        rootfs_name: levitate::ROOTFS_NAME.to_string(),
        initramfs_live_output: levitate::INITRAMFS_LIVE_OUTPUT.to_string(),
        iso_filename: levitate::ISO_FILENAME.to_string(),
        initramfs_installed_output: Some(levitate::INITRAMFS_INSTALLED_OUTPUT.to_string()),
        installed_uki_outputs: vec![
            levitate::UKI_INSTALLED_FILENAME.to_string(),
            levitate::UKI_INSTALLED_RECOVERY_FILENAME.to_string(),
        ],
        disk_image_output: None,
    };
    let stages = StageContract {
        stage_00_build: BuildCapabilityStage {
            required_build_tools: baseline_stage_00_build_tools(),
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
            non_kernel_inputs: baseline_stage_00_non_kernel_inputs(
                levitate::ROOTFS_NAME,
                levitate::INITRAMFS_LIVE_OUTPUT,
                Some(levitate::INITRAMFS_INSTALLED_OUTPUT),
            ),
            iso_assembly: Stage00IsoAssembly {
                live_uki_filename: "levitateos-live.efi".to_string(),
                emergency_uki_filename: "levitateos-emergency.efi".to_string(),
                debug_uki_filename: "levitateos-debug.efi".to_string(),
                live_cmdline: "video=1920x1080".to_string(),
            },
            evidence: ScriptEvidence {
                script_path: "build-capability.sh".to_string(),
                pass_marker: "STAGE 00 PASSED".to_string(),
            },
        },
        stage_01_live_boot: BootStage {
            success_patterns: str_vec(&["LevitateOS Live", "Reached target Multi-User System."]),
            fatal_patterns: str_vec(&[
                "No bootable device",
                "Kernel panic",
                "VFS: Cannot open root device",
                "No init found",
                "SQUASHFS error",
                "EROFS:",
                "emergency shell",
            ]),
            required_kernel_cmdline: str_vec(&["audit=1", "inst.sshd=0"]),
            required_live_services: vec!["sshd".to_string()],
            evidence: ScriptEvidence {
                script_path: "live-boot.sh".to_string(),
                pass_marker: "STAGE 01 PASSED".to_string(),
            },
        },
        stage_02_live_tools: ToolsStage {
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
                script_path: "live-tools.sh".to_string(),
                pass_marker: "STAGE 02 PASSED".to_string(),
            },
        },
        stage_03_install: InstallStage {
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
                script_path: "install.sh".to_string(),
                pass_marker: "STAGE 03 PASSED".to_string(),
            },
        },
        stage_04_installed_boot: BootStage {
            success_patterns: str_vec(&["levitateos login:", "Reached target Multi-User System."]),
            fatal_patterns: str_vec(&[
                "Kernel panic",
                "VFS: Cannot open root device",
                "No init found",
                "emergency shell",
                "Failed to start",
            ]),
            required_kernel_cmdline: str_vec(&["audit=1", "inst.sshd=0"]),
            required_live_services: vec![],
            evidence: ScriptEvidence {
                script_path: "installed-boot.sh".to_string(),
                pass_marker: "STAGE 04 PASSED".to_string(),
            },
        },
        stage_05_automated_login: AutomatedLoginStage {
            auth_mode: AuthMode::DefaultPasswordLogin,
            default_username: Some("levitate".to_string()),
            default_password: Some("levitate".to_string()),
            login_prompt_pattern: "levitateos login:".to_string(),
            evidence: ScriptEvidence {
                script_path: "automated-login.sh".to_string(),
                pass_marker: "STAGE 05 PASSED".to_string(),
            },
        },
        stage_06_installed_tools: ToolsStage {
            required_tools: str_vec(&["sudo", "ip", "ssh", "mount", "umount", "dmesg"]),
            evidence: ScriptEvidence {
                script_path: "installed-tools.sh".to_string(),
                pass_marker: "STAGE 06 PASSED".to_string(),
            },
        },
        stage_07_runtime_policy: RuntimePolicyStage {
            rootfs_mutability: RootfsMutability::Mutable,
            mutable_required_rw_paths: str_vec(&["/etc", "/var", "/home", "/usr/local"]),
            immutable_required_ro_paths: vec![],
        },
        stage_08_release: ReleaseStage {
            required_artifacts: str_vec(&[
                levitate::ROOTFS_NAME,
                levitate::INITRAMFS_LIVE_OUTPUT,
                levitate::INITRAMFS_INSTALLED_OUTPUT,
                levitate::ISO_FILENAME,
            ]),
            required_metadata: baseline_stage_08_metadata(),
        },
    };

    compat_contract(identity, artifacts, stages)
}

fn ralph_contract() -> ConformanceContract {
    use crate::ralph;

    let mut stages = levitate_contract().stages;
    let identity = DistroIdentity {
        os_name: ralph::OS_NAME.to_string(),
        os_id: ralph::OS_ID.to_string(),
        iso_label: ralph::ISO_LABEL.to_string(),
        os_version: ralph::OS_VERSION.to_string(),
        default_hostname: ralph::DEFAULT_HOSTNAME.to_string(),
    };
    let artifacts = ArtifactIdentity {
        rootfs_name: ralph::ROOTFS_NAME.to_string(),
        initramfs_live_output: ralph::INITRAMFS_LIVE_OUTPUT.to_string(),
        iso_filename: ralph::ISO_FILENAME.to_string(),
        initramfs_installed_output: None,
        installed_uki_outputs: vec![],
        disk_image_output: None,
    };

    stages.stage_00_build.kernel_version = ralph::KERNEL_SOURCE.version.to_string();
    stages.stage_00_build.kernel_sha256 = ralph::KERNEL_SOURCE.sha256.to_string();
    stages.stage_00_build.kernel_localversion = ralph::KERNEL_SOURCE.localversion.to_string();
    stages.stage_00_build.module_install_path = ralph::MODULE_INSTALL_PATH.to_string();
    stages.stage_00_build.non_kernel_inputs =
        baseline_stage_00_non_kernel_inputs(ralph::ROOTFS_NAME, ralph::INITRAMFS_LIVE_OUTPUT, None);
    stages.stage_00_build.iso_assembly = Stage00IsoAssembly {
        live_uki_filename: "ralphos-live.efi".to_string(),
        emergency_uki_filename: "ralphos-emergency.efi".to_string(),
        debug_uki_filename: "ralphos-debug.efi".to_string(),
        live_cmdline: "".to_string(),
    };

    stages.stage_01_live_boot.success_patterns =
        str_vec(&["___SHELL_READY___", "Reached target Multi-User System."]);
    stages.stage_01_live_boot.required_kernel_cmdline = str_vec(&["audit=1", "inst.sshd=0"]);

    stages.stage_04_installed_boot.success_patterns =
        str_vec(&["___SHELL_READY___", "ralphos login:", "multi-user.target"]);

    stages.stage_05_automated_login.default_username = Some("ralph".to_string());
    stages.stage_05_automated_login.default_password = Some("ralph".to_string());
    stages.stage_05_automated_login.login_prompt_pattern = "ralphos login:".to_string();

    stages.stage_08_release.required_artifacts = str_vec(&[
        ralph::ROOTFS_NAME,
        ralph::INITRAMFS_LIVE_OUTPUT,
        ralph::ISO_FILENAME,
    ]);

    compat_contract(identity, artifacts, stages)
}

fn acorn_contract() -> ConformanceContract {
    use crate::acorn;

    let identity = DistroIdentity {
        os_name: acorn::OS_NAME.to_string(),
        os_id: acorn::OS_ID.to_string(),
        iso_label: acorn::ISO_LABEL.to_string(),
        os_version: acorn::OS_VERSION.to_string(),
        default_hostname: acorn::DEFAULT_HOSTNAME.to_string(),
    };
    let artifacts = ArtifactIdentity {
        rootfs_name: acorn::ROOTFS_NAME.to_string(),
        initramfs_live_output: acorn::INITRAMFS_LIVE_OUTPUT.to_string(),
        iso_filename: acorn::ISO_FILENAME.to_string(),
        initramfs_installed_output: None,
        installed_uki_outputs: vec![
            acorn::UKI_INSTALLED_FILENAME.to_string(),
            acorn::UKI_INSTALLED_RECOVERY_FILENAME.to_string(),
        ],
        disk_image_output: None,
    };
    let stages = StageContract {
        stage_00_build: BuildCapabilityStage {
            required_build_tools: baseline_stage_00_build_tools(),
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
            non_kernel_inputs: baseline_stage_00_non_kernel_inputs(
                acorn::ROOTFS_NAME,
                acorn::INITRAMFS_LIVE_OUTPUT,
                None,
            ),
            iso_assembly: Stage00IsoAssembly {
                live_uki_filename: "acornos-live.efi".to_string(),
                emergency_uki_filename: "acornos-emergency.efi".to_string(),
                debug_uki_filename: "acornos-debug.efi".to_string(),
                live_cmdline: "video=1920x1080".to_string(),
            },
            evidence: ScriptEvidence {
                script_path: "build-capability.sh".to_string(),
                pass_marker: "STAGE 00 PASSED".to_string(),
            },
        },
        stage_01_live_boot: BootStage {
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
            required_kernel_cmdline: str_vec(&["audit=1", "inst.sshd=0"]),
            required_live_services: vec!["sshd".to_string()],
            evidence: ScriptEvidence {
                script_path: "live-boot.sh".to_string(),
                pass_marker: "STAGE 01 PASSED".to_string(),
            },
        },
        stage_02_live_tools: ToolsStage {
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
                script_path: "live-tools.sh".to_string(),
                pass_marker: "STAGE 02 PASSED".to_string(),
            },
        },
        stage_03_install: InstallStage {
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
                script_path: "install.sh".to_string(),
                pass_marker: "STAGE 03 PASSED".to_string(),
            },
        },
        stage_04_installed_boot: BootStage {
            success_patterns: str_vec(&["acornos login:", "Welcome to AcornOS"]),
            fatal_patterns: str_vec(&[
                "Kernel panic",
                "VFS: Cannot open root device",
                "No init found",
                "emergency shell",
                "Timed out waiting for device",
            ]),
            required_kernel_cmdline: str_vec(&["audit=1", "inst.sshd=0"]),
            required_live_services: vec![],
            evidence: ScriptEvidence {
                script_path: "installed-boot.sh".to_string(),
                pass_marker: "STAGE 04 PASSED".to_string(),
            },
        },
        stage_05_automated_login: AutomatedLoginStage {
            auth_mode: AuthMode::DefaultPasswordLogin,
            default_username: Some("acorn".to_string()),
            default_password: Some("acorn".to_string()),
            login_prompt_pattern: "acornos login:".to_string(),
            evidence: ScriptEvidence {
                script_path: "automated-login.sh".to_string(),
                pass_marker: "STAGE 05 PASSED".to_string(),
            },
        },
        stage_06_installed_tools: ToolsStage {
            required_tools: str_vec(&[
                "sudo", "ip", "ssh", "ash", "mount", "umount", "dmesg", "ps", "ls", "cat",
            ]),
            evidence: ScriptEvidence {
                script_path: "installed-tools.sh".to_string(),
                pass_marker: "STAGE 06 PASSED".to_string(),
            },
        },
        stage_07_runtime_policy: RuntimePolicyStage {
            rootfs_mutability: RootfsMutability::Mutable,
            mutable_required_rw_paths: str_vec(&["/etc", "/var", "/home", "/usr/local"]),
            immutable_required_ro_paths: vec![],
        },
        stage_08_release: ReleaseStage {
            required_artifacts: str_vec(&[
                acorn::ROOTFS_NAME,
                acorn::INITRAMFS_LIVE_OUTPUT,
                acorn::ISO_FILENAME,
            ]),
            required_metadata: baseline_stage_08_metadata(),
        },
    };

    compat_contract(identity, artifacts, stages)
}

fn iuppiter_contract() -> ConformanceContract {
    use crate::iuppiter;

    let mut metadata = baseline_stage_08_metadata();
    metadata.push("artifact.disk_image_filename".to_string());

    let identity = DistroIdentity {
        os_name: iuppiter::OS_NAME.to_string(),
        os_id: iuppiter::OS_ID.to_string(),
        iso_label: iuppiter::ISO_LABEL.to_string(),
        os_version: iuppiter::OS_VERSION.to_string(),
        default_hostname: iuppiter::DEFAULT_HOSTNAME.to_string(),
    };
    let artifacts = ArtifactIdentity {
        rootfs_name: iuppiter::ROOTFS_NAME.to_string(),
        initramfs_live_output: iuppiter::INITRAMFS_LIVE_OUTPUT.to_string(),
        iso_filename: iuppiter::ISO_FILENAME.to_string(),
        initramfs_installed_output: None,
        installed_uki_outputs: vec![
            iuppiter::UKI_INSTALLED_FILENAME.to_string(),
            iuppiter::UKI_INSTALLED_RECOVERY_FILENAME.to_string(),
        ],
        disk_image_output: Some(iuppiter::DISK_IMAGE_FILENAME.to_string()),
    };
    let stages = StageContract {
        stage_00_build: BuildCapabilityStage {
            required_build_tools: baseline_stage_00_build_tools(),
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
            non_kernel_inputs: baseline_stage_00_non_kernel_inputs(
                iuppiter::ROOTFS_NAME,
                iuppiter::INITRAMFS_LIVE_OUTPUT,
                None,
            ),
            iso_assembly: Stage00IsoAssembly {
                live_uki_filename: "iuppiter-live.efi".to_string(),
                emergency_uki_filename: "iuppiter-emergency.efi".to_string(),
                debug_uki_filename: "iuppiter-debug.efi".to_string(),
                live_cmdline: String::new(),
            },
            evidence: ScriptEvidence {
                script_path: "build-capability.sh".to_string(),
                pass_marker: "STAGE 00 PASSED".to_string(),
            },
        },
        stage_01_live_boot: BootStage {
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
            required_kernel_cmdline: str_vec(&["audit=1", "inst.sshd=0"]),
            required_live_services: vec!["sshd".to_string()],
            evidence: ScriptEvidence {
                script_path: "live-boot.sh".to_string(),
                pass_marker: "STAGE 01 PASSED".to_string(),
            },
        },
        stage_02_live_tools: ToolsStage {
            required_tools: str_vec(&[
                "recstrap",
                "recfstab",
                "recchroot",
                "recab",
                "iuppiter-dar",
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
                script_path: "live-tools.sh".to_string(),
                pass_marker: "STAGE 02 PASSED".to_string(),
            },
        },
        stage_03_install: InstallStage {
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
                script_path: "install.sh".to_string(),
                pass_marker: "STAGE 03 PASSED".to_string(),
            },
        },
        stage_04_installed_boot: BootStage {
            success_patterns: str_vec(&["iuppiter login:", "Welcome to IuppiterOS"]),
            fatal_patterns: str_vec(&[
                "Kernel panic",
                "VFS: Cannot open root device",
                "No init found",
                "emergency shell",
                "Timed out waiting for device",
            ]),
            required_kernel_cmdline: vec![],
            required_live_services: vec![],
            evidence: ScriptEvidence {
                script_path: "installed-boot.sh".to_string(),
                pass_marker: "STAGE 04 PASSED".to_string(),
            },
        },
        stage_05_automated_login: AutomatedLoginStage {
            auth_mode: AuthMode::DefaultPasswordLogin,
            default_username: Some("operator".to_string()),
            default_password: Some("iuppiter".to_string()),
            login_prompt_pattern: "iuppiter login:".to_string(),
            evidence: ScriptEvidence {
                script_path: "automated-login.sh".to_string(),
                pass_marker: "STAGE 05 PASSED".to_string(),
            },
        },
        stage_06_installed_tools: ToolsStage {
            required_tools: str_vec(&[
                "sudo",
                "ip",
                "ssh",
                "ash",
                "smartctl",
                "hdparm",
                "sg_inq",
                "recab",
                "iuppiter-dar",
                "mount",
                "umount",
                "dmesg",
            ]),
            evidence: ScriptEvidence {
                script_path: "installed-tools.sh".to_string(),
                pass_marker: "STAGE 06 PASSED".to_string(),
            },
        },
        stage_07_runtime_policy: RuntimePolicyStage {
            rootfs_mutability: RootfsMutability::Immutable,
            mutable_required_rw_paths: vec![],
            immutable_required_ro_paths: str_vec(&["/", "/usr", "/opt"]),
        },
        stage_08_release: ReleaseStage {
            required_artifacts: str_vec(&[
                iuppiter::ROOTFS_NAME,
                iuppiter::INITRAMFS_LIVE_OUTPUT,
                iuppiter::ISO_FILENAME,
                iuppiter::DISK_IMAGE_FILENAME,
            ]),
            required_metadata: metadata,
        },
    };

    compat_contract(identity, artifacts, stages)
}

/// Get the legacy canonical conformance contract for a distro id.
///
/// Deprecated runtime source of truth: Stage 00 now loads from
/// `distro-variants/*/build-host.toml`.
pub fn contract_for_distro(distro_id: &str) -> Option<ConformanceContract> {
    match distro_id {
        "levitate" | "levitateos" => Some(levitate_contract()),
        "ralph" | "ralphos" => Some(ralph_contract()),
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
        for distro in ["levitate", "ralph", "acorn", "iuppiter"] {
            require_valid_contract_for_distro(distro)
                .unwrap_or_else(|err| panic!("{} contract invalid: {}", distro, err));
        }
    }

    #[test]
    fn stage_00_baseline_includes_uki_tooling() {
        let tools = baseline_stage_00_build_tools();
        assert!(tools.iter().any(|t| t == "recuki"));
        assert!(tools.iter().any(|t| t == "ukify"));
    }

    #[test]
    fn legacy_release_bridge_splits_primary_outputs_and_metadata_facts() {
        let levitate = contract_for_distro("levitate").expect("levitate contract");
        assert_eq!(
            levitate.release.primary_outputs,
            vec![crate::levitate::ISO_FILENAME.to_string()]
        );
        assert_eq!(
            levitate.release.supporting_artifacts,
            vec![
                crate::levitate::ROOTFS_NAME.to_string(),
                crate::levitate::INITRAMFS_LIVE_OUTPUT.to_string(),
                crate::levitate::INITRAMFS_INSTALLED_OUTPUT.to_string(),
            ]
        );
        assert!(
            levitate.release.metadata_outputs.is_empty(),
            "{:?}",
            levitate.release.metadata_outputs
        );
        assert_eq!(
            levitate.release.metadata_facts,
            vec![
                "kernel_source.version".to_string(),
                "kernel_source.sha256".to_string(),
                "kernel_source.localversion".to_string(),
                "artifact.rootfs_name".to_string(),
                "artifact.iso_filename".to_string(),
            ]
        );

        let iuppiter = contract_for_distro("iuppiter").expect("iuppiter contract");
        assert_eq!(
            iuppiter.release.primary_outputs,
            vec![
                crate::iuppiter::ISO_FILENAME.to_string(),
                crate::iuppiter::DISK_IMAGE_FILENAME.to_string(),
            ]
        );
        assert_eq!(
            iuppiter.release.supporting_artifacts,
            vec![
                crate::iuppiter::ROOTFS_NAME.to_string(),
                crate::iuppiter::INITRAMFS_LIVE_OUTPUT.to_string(),
            ]
        );
        assert!(
            iuppiter.release.metadata_outputs.is_empty(),
            "{:?}",
            iuppiter.release.metadata_outputs
        );
        assert_eq!(
            iuppiter.release.metadata_facts,
            vec![
                "kernel_source.version".to_string(),
                "kernel_source.sha256".to_string(),
                "kernel_source.localversion".to_string(),
                "artifact.rootfs_name".to_string(),
                "artifact.iso_filename".to_string(),
                "artifact.disk_image_filename".to_string(),
            ]
        );
    }
}
