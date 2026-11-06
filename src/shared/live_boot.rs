//! The live-boot payload protocol.
//!
//! Every fact that the live init template (recinit), the ISO layout (reciso),
//! rootfs discovery in the live system (recstrap), the overlay builder
//! (distro-builder) and the test harness must agree on is defined here once.
//! recinit fills the init template from `init_template_vars`, so the template
//! never spells a protocol value itself.

macro_rules! live_media_dir {
    () => {
        "/run/live-media"
    };
}
macro_rules! rootfs_iso_path {
    () => {
        concat!(super::iso::iso_live_dir!(), "/filesystem.erofs")
    };
}
pub(crate) use rootfs_iso_path;

/// Where the boot medium stays mounted in the live system.
pub const LIVE_MEDIA_DIR: &str = live_media_dir!();

/// Stable handle to the rootfs payload in the live system: a symlink to the
/// payload partition or to the image file on the boot medium.
pub const ROOTFS_HANDLE: &str = "/run/live-rootfs.erofs";

/// Stable handle to the live overlay payload, when there is one.
pub const OVERLAY_HANDLE: &str = "/run/live-overlay.erofs";

/// Where an installer finds the rootfs on the live system, in order.
pub const ROOTFS_SEARCH_PATHS: &[&str] = &[
    ROOTFS_HANDLE,
    concat!(live_media_dir!(), "/", rootfs_iso_path!()),
];

/// File at the root of a payload image naming its role.
pub const PAYLOAD_ROLE_FILE: &str = ".live-payload-role";
pub const ROLE_ROOTFS: &str = "rootfs";
pub const ROLE_OVERLAY: &str = "overlay";

/// Appended ISO partitions that carry payload images (rootfs, then overlay).
pub const ROOTFS_PAYLOAD_PARTITION: u32 = 2;
pub const OVERLAY_PAYLOAD_PARTITION: u32 = 3;

/// Created in the live root so services can tell a live boot.
pub const BOOT_MARKER: &str = "/live-boot-marker";

/// Test channel: QEMU fw_cfg entry carrying a boot-injection payload.
pub const BOOT_INJECTION_FW_CFG: &str = "opt/levitate/boot-injection";
/// Test channel: payload file name on a secondary ISO.
pub const BOOT_INJECTION_FILE: &str = "boot-injection.env";
/// Test channel: volume label of that secondary ISO.
pub const BOOT_INJECTION_ISO_LABEL: &str = "LEVITATE_INJECT";
/// Test channel: where the live init leaves the imported payload (KEY=VALUE
/// lines) and, in `source`, where it came from.
pub const BOOT_INJECTION_DIR: &str = "/run/boot-injection";
pub const BOOT_INJECTION_ENV: &str = "/run/boot-injection/payload.env";
/// Test channel: devices probed for that secondary ISO, after the boot medium.
pub const BOOT_INJECTION_DEVICES: &[&str] = &["/dev/sr1", "/dev/sr2", "/dev/sdb", "/dev/vdb"];

/// Printed by the live init when no rootfs payload is found; boot tests match it.
pub const ROOTFS_MISSING_MESSAGE: &str = "Rootfs payload not found";

/// Placeholder values for the live init template.
pub fn init_template_vars() -> Vec<(&'static str, String)> {
    vec![
        ("LIVE_MEDIA_DIR", LIVE_MEDIA_DIR.to_string()),
        ("ROOTFS_HANDLE", ROOTFS_HANDLE.to_string()),
        ("OVERLAY_HANDLE", OVERLAY_HANDLE.to_string()),
        ("PAYLOAD_ROLE_FILE", PAYLOAD_ROLE_FILE.to_string()),
        ("ROLE_ROOTFS", ROLE_ROOTFS.to_string()),
        ("ROLE_OVERLAY", ROLE_OVERLAY.to_string()),
        (
            "PAYLOAD_PARTITIONS",
            format!("{ROOTFS_PAYLOAD_PARTITION} {OVERLAY_PAYLOAD_PARTITION}"),
        ),
        ("BOOT_MARKER", BOOT_MARKER.to_string()),
        ("BOOT_INJECTION_FW_CFG", BOOT_INJECTION_FW_CFG.to_string()),
        ("BOOT_INJECTION_FILE", BOOT_INJECTION_FILE.to_string()),
        ("BOOT_INJECTION_DEVICES", BOOT_INJECTION_DEVICES.join(" ")),
        ("BOOT_INJECTION_DIR", BOOT_INJECTION_DIR.to_string()),
        ("BOOT_INJECTION_ENV", BOOT_INJECTION_ENV.to_string()),
        ("ROOTFS_MISSING_MESSAGE", ROOTFS_MISSING_MESSAGE.to_string()),
    ]
}

/// POSIX shell that exports the imported boot-injection payload's KEY=VALUE
/// lines (as data: nothing in it is executed). Live shell scripts embed it.
pub fn boot_injection_import_sh() -> String {
    format!(
        r#"if [ -r {env} ]; then
    while IFS= read -r line; do
        case "$line" in
            ""|\#*) continue ;;
            *=*)
                key="${{line%%=*}}"
                value="${{line#*=}}"
                case "$key" in
                    [A-Za-z_][A-Za-z0-9_]*) export "$key=$value" ;;
                esac
                ;;
        esac
    done < {env}
fi"#,
        env = BOOT_INJECTION_ENV
    )
}
