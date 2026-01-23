# Checklist: Wire distro-spec SSOT to leviso

## Context

**leviso** = LevitateOS ISO builder, uses `distro_spec::levitate` only.

**distro-spec** = Single Source of Truth for both LevitateOS and AcornOS:
- `shared/` = common to both distros
- `levitate/` = LevitateOS-specific (Rocky, systemd, glibc)
- `acorn/` = AcornOS-specific (Alpine, OpenRC, musl, busybox)

---

## Part 1: Add Constants to distro-spec

### 1.1 Create `distro-spec/src/shared/iso.rs` (new file)

- [x] Create new file `distro-spec/src/shared/iso.rs`
- [x] Add ISO directory structure constants:

```rust
//! ISO structure constants shared between LevitateOS and AcornOS.

// =============================================================================
// ISO Directory Structure
// =============================================================================

/// Boot directory on ISO (contains kernel, initramfs)
pub const ISO_BOOT_DIR: &str = "boot";

/// Live directory on ISO (contains squashfs, overlay)
pub const ISO_LIVE_DIR: &str = "live";

/// EFI boot directory on ISO
pub const ISO_EFI_DIR: &str = "EFI/BOOT";

/// Path to squashfs inside ISO (relative to ISO root)
pub const SQUASHFS_ISO_PATH: &str = "live/filesystem.squashfs";

/// Path to kernel inside ISO (relative to ISO root)
pub const KERNEL_ISO_PATH: &str = "boot/vmlinuz";

/// Path to initramfs inside ISO (relative to ISO root)
pub const INITRAMFS_ISO_PATH: &str = "boot/initramfs.img";

/// Path to live overlay inside ISO (relative to ISO root)
pub const LIVE_OVERLAY_ISO_PATH: &str = "live/overlay";

// =============================================================================
// EFI Boot Files
// =============================================================================

/// EFI boot image filename
pub const EFIBOOT_FILENAME: &str = "efiboot.img";

/// EFI boot image size in MB (FAT16 minimum is 16MB)
pub const EFIBOOT_SIZE_MB: u32 = 16;

/// Primary EFI bootloader filename
pub const EFI_BOOTLOADER: &str = "BOOTX64.EFI";

/// GRUB EFI binary filename
pub const EFI_GRUB: &str = "grubx64.efi";

// =============================================================================
// Console Configuration
// =============================================================================

/// Serial console kernel parameter
pub const SERIAL_CONSOLE: &str = "console=ttyS0,115200n8";

/// VGA console kernel parameter
pub const VGA_CONSOLE: &str = "console=tty0";

/// Serial console baud rate (for documentation/validation)
pub const SERIAL_BAUD_RATE: u32 = 115200;

// =============================================================================
// Kernel Boot Parameters
// =============================================================================

/// SELinux disable parameter (Rocky/RHEL based need this for live boot)
pub const SELINUX_DISABLE: &str = "selinux=0";

// =============================================================================
// Checksum
// =============================================================================

/// ISO checksum file suffix
pub const ISO_CHECKSUM_SUFFIX: &str = ".sha512";

/// SHA512 checksum format separator (two spaces per sha512sum standard)
pub const SHA512_SEPARATOR: &str = "  ";

// =============================================================================
// xorriso Parameters
// =============================================================================

/// MBR partition offset for hybrid ISO
pub const XORRISO_PARTITION_OFFSET: u32 = 16;

/// ISO filesystem flags for xorriso
pub const XORRISO_FS_FLAGS: &[&str] = &[
    "-full-iso9660-filenames",
    "-joliet",
    "-rational-rock",
];
```

### 1.2 Create `distro-spec/src/shared/initramfs.rs` (new file)

- [x] Create new file `distro-spec/src/shared/initramfs.rs`
- [x] Add initramfs constants:

```rust
//! Initramfs build constants shared between distros.

// =============================================================================
// Initramfs Directory Structure
// =============================================================================

/// Directories to create in initramfs root
pub const INITRAMFS_DIRS: &[&str] = &[
    "bin",
    "dev",
    "proc",
    "sys",
    "tmp",
    "mnt",
    "squashfs",
    "overlay",
    "newroot",
    "live-overlay",
];

// =============================================================================
// Mount Points (used by init script)
// =============================================================================

/// Squashfs mount point in initramfs
pub const MOUNT_SQUASHFS: &str = "/squashfs";

/// Overlay lower dir mount point
pub const MOUNT_OVERLAY: &str = "/overlay";

/// New root mount point (switch_root target)
pub const MOUNT_NEWROOT: &str = "/newroot";

/// Live overlay mount point
pub const MOUNT_LIVE_OVERLAY: &str = "/live-overlay";

// =============================================================================
// Compression
// =============================================================================

/// CPIO compression level for gzip
pub const CPIO_GZIP_LEVEL: u32 = 9;
```

### 1.3 Create `distro-spec/src/shared/devices.rs` (new file)

- [x] Create new file `distro-spec/src/shared/devices.rs`
- [x] Add device detection constants:

```rust
//! Device paths for boot media detection.

/// Devices to probe for ISO/boot media (in order)
pub const BOOT_DEVICE_PROBE_ORDER: &[&str] = &[
    "/dev/sr0",      // CD/DVD drive
    "/dev/sda",      // First SATA/SCSI disk
    "/dev/sdb",      // Second SATA/SCSI disk
    "/dev/vda",      // VirtIO disk (QEMU)
    "/dev/nvme0n1",  // NVMe drive
];
```

### 1.4 Create `distro-spec/src/shared/qemu.rs` (new file)

- [x] Create new file `distro-spec/src/shared/qemu.rs`
- [x] Add QEMU testing constants:

```rust
//! QEMU testing defaults.

/// Virtual disk filename
pub const QEMU_DISK_FILENAME: &str = "virtual-disk.qcow2";

/// Serial log path
pub const QEMU_SERIAL_LOG: &str = "/tmp/levitateos-serial.log";

/// CPU emulation mode
pub const QEMU_CPU_MODE: &str = "max";
```

### 1.5 Update `distro-spec/src/shared/mod.rs`

- [x] Add new module declarations:

```rust
pub mod devices;
pub mod initramfs;
pub mod iso;
pub mod qemu;
```

- [x] Add re-exports for all new constants

### 1.6 Update `distro-spec/src/levitate/paths.rs`

- [x] Add these LevitateOS-specific constants:

```rust
// =============================================================================
// ISO Output
// =============================================================================

/// ISO output filename
pub const ISO_FILENAME: &str = "levitateos.iso";

// =============================================================================
// QEMU Testing Defaults
// =============================================================================

/// QEMU memory allocation (GB) - LevitateOS needs more for glibc + systemd
pub const QEMU_MEMORY_GB: u32 = 4;

/// QEMU virtual disk size (GB)
pub const QEMU_DISK_GB: u32 = 20;

// =============================================================================
// Initramfs Build
// =============================================================================

/// Busybox binary URL (default, can be overridden via BUSYBOX_URL env)
pub const BUSYBOX_URL: &str = "https://busybox.net/downloads/binaries/1.35.0-x86_64-linux-musl/busybox";

/// Environment variable name for busybox URL override
pub const BUSYBOX_URL_ENV: &str = "BUSYBOX_URL";

/// Initramfs build directory name
pub const INITRAMFS_BUILD_DIR: &str = "initramfs-tiny-root";

/// Initramfs output filename
pub const INITRAMFS_OUTPUT: &str = "initramfs-tiny.cpio.gz";

// =============================================================================
// Live System
// =============================================================================

/// /etc/issue message for live boot
pub const LIVE_ISSUE_MESSAGE: &str = "\nLevitateOS Live - \\l\n\n";
```

### 1.7 Update `distro-spec/src/levitate/mod.rs`

- [x] Add re-exports from all new shared modules
- [x] Add re-exports for new paths.rs constants

### 1.8 Update `distro-spec/src/acorn/paths.rs`

- [x] Add AcornOS-specific constants (mirror of levitate):

```rust
/// ISO output filename
pub const ISO_FILENAME: &str = "acornos.iso";

/// QEMU memory allocation (GB) - Alpine is lighter
pub const QEMU_MEMORY_GB: u32 = 2;

/// QEMU virtual disk size (GB)
pub const QEMU_DISK_GB: u32 = 10;

/// Busybox URL (Alpine uses system busybox, but needed for initramfs build)
pub const BUSYBOX_URL: &str = "https://busybox.net/downloads/binaries/1.35.0-x86_64-linux-musl/busybox";

/// Environment variable name for busybox URL override
pub const BUSYBOX_URL_ENV: &str = "BUSYBOX_URL";

/// Initramfs build directory name
pub const INITRAMFS_BUILD_DIR: &str = "initramfs-tiny-root";

/// Initramfs output filename
pub const INITRAMFS_OUTPUT: &str = "initramfs-tiny.cpio.gz";

/// /etc/issue message for live boot
pub const LIVE_ISSUE_MESSAGE: &str = "\nAcornOS Live - \\l\n\n";
```

### 1.9 Update `distro-spec/src/acorn/mod.rs`

- [x] Add re-exports from all new shared modules
- [x] Add re-exports for new paths.rs constants

### 1.10 Verify distro-spec builds

- [x] `cd distro-spec && cargo build`
- [x] `cargo test`
- [x] `cargo clippy`

---

## Part 2: Wire distro-spec into leviso

### 2.1 Update `leviso/src/artifact/iso.rs`

- [x] Add comprehensive import:

```rust
use distro_spec::levitate::{
    // Identity
    ISO_LABEL, ISO_FILENAME, OS_NAME,
    // Squashfs
    SQUASHFS_NAME, SQUASHFS_ISO_PATH,
    // Boot files
    KERNEL_ISO_PATH, INITRAMFS_ISO_PATH,
    // ISO structure
    ISO_BOOT_DIR, ISO_LIVE_DIR, ISO_EFI_DIR,
    LIVE_OVERLAY_ISO_PATH,
    // EFI
    EFIBOOT_FILENAME, EFIBOOT_SIZE_MB,
    EFI_BOOTLOADER, EFI_GRUB,
    // Console
    SERIAL_CONSOLE, VGA_CONSOLE, SELINUX_DISABLE,
    // Checksum
    ISO_CHECKSUM_SUFFIX, SHA512_SEPARATOR,
    // xorriso
    XORRISO_PARTITION_OFFSET, XORRISO_FS_FLAGS,
};
```

- [x] Line ~20: Replace `"LEVITATEOS"` with `ISO_LABEL`
- [x] Line ~40: Replace `"filesystem.squashfs"` with `SQUASHFS_NAME`
- [x] Line ~42: Replace `"levitateos.iso"` with `ISO_FILENAME`
- [x] Line ~135: Replace `"boot"` with `ISO_BOOT_DIR`
- [x] Line ~136: Replace `"live"` with `ISO_LIVE_DIR`
- [x] Line ~137: Replace `"EFI/BOOT"` with `ISO_EFI_DIR`
- [x] Line ~145: Replace `"boot/vmlinuz"` with `KERNEL_ISO_PATH`
- [x] Line ~146: Replace `"boot/initramfs.img"` with `INITRAMFS_ISO_PATH`
- [x] Line ~150: Replace `"live/filesystem.squashfs"` with `SQUASHFS_ISO_PATH`
- [x] Line ~155-156: Replace `"live/overlay"` with `LIVE_OVERLAY_ISO_PATH`
- [x] Line ~199: Replace `'LevitateOS'` in GRUB template with `OS_NAME`
- [x] Line ~200,205,210: Replace `console=ttyS0,115200n8` with `SERIAL_CONSOLE`
- [x] Line ~200,205,210: Replace `console=tty0` with `VGA_CONSOLE`
- [x] Line ~200,205,210: Replace `selinux=0` with `SELINUX_DISABLE`
- [x] Line ~219: Replace `"efiboot.img"` with `EFIBOOT_FILENAME`
- [x] Line ~234: Replace `-partition_offset 16` with `XORRISO_PARTITION_OFFSET`
- [x] Line ~235: Replace hardcoded flags with `XORRISO_FS_FLAGS`
- [x] Line ~271: Replace `"  "` separator with `SHA512_SEPARATOR`
- [x] Line ~274: Replace `".iso.sha512"` with `ISO_CHECKSUM_SUFFIX`
- [x] Line ~311: Replace `count=16` with `EFIBOOT_SIZE_MB`
- [x] Line ~336-352: Replace `"BOOTX64.EFI"` with `EFI_BOOTLOADER`
- [x] Line ~336-352: Replace `"grubx64.efi"` with `EFI_GRUB`

### 2.2 Update `leviso/src/artifact/squashfs.rs`

- [x] Add import:

```rust
use distro_spec::levitate::{SQUASHFS_COMPRESSION, SQUASHFS_BLOCK_SIZE};
```

- [x] Line ~122: Replace `"-comp", "gzip"` with `"-comp", SQUASHFS_COMPRESSION`
- [x] Line ~123: Replace `"-b", "1M"` with `"-b", SQUASHFS_BLOCK_SIZE`

### 2.3 Update `leviso/src/artifact/initramfs.rs`

- [x] Add import:

```rust
use distro_spec::levitate::{
    // Modules
    BOOT_MODULES,
    // Init script generation
    ISO_LABEL, SQUASHFS_ISO_PATH,
    // Build paths
    BUSYBOX_URL, BUSYBOX_URL_ENV,
    INITRAMFS_BUILD_DIR, INITRAMFS_OUTPUT,
    // Directories
    INITRAMFS_DIRS,
    // Compression
    CPIO_GZIP_LEVEL,
};
```

- [x] Line ~41: Replace busybox URL string with `BUSYBOX_URL`
- [x] Line ~42: Replace `"BUSYBOX_URL"` with `BUSYBOX_URL_ENV`
- [x] Lines ~58-70: Remove local `BOOT_MODULES` array, use `distro_spec::levitate::BOOT_MODULES`
- [x] Line ~77: Replace `"initramfs-tiny-root"` with `INITRAMFS_BUILD_DIR`
- [x] Line ~78: Replace `"initramfs-tiny.cpio.gz"` with `INITRAMFS_OUTPUT`
- [x] Lines ~112-122: Replace hardcoded dir list with `INITRAMFS_DIRS`
- [x] Line ~329: Replace gzip `-9` with `CPIO_GZIP_LEVEL`
- [x] Add `generate_init_script()` function (see Part 3)
- [x] Update `build_initramfs()` to call `generate_init_script()`

### 2.4 Update `leviso/src/qemu.rs`

- [x] Add import:

```rust
use distro_spec::levitate::{
    ISO_FILENAME,
    QEMU_MEMORY_GB, QEMU_DISK_GB,
    QEMU_DISK_FILENAME, QEMU_SERIAL_LOG, QEMU_CPU_MODE,
};
```

- [x] Line ~50: Replace `"-cpu", "max"` with `"-cpu", QEMU_CPU_MODE`
- [x] Line ~53: Replace `"-m", "4G"` with format using `QEMU_MEMORY_GB`
- [x] Line ~92: Replace `/tmp/levitateos-serial.log` with `QEMU_SERIAL_LOG`
- [x] Line ~127: Replace `"levitateos.iso"` with `ISO_FILENAME`
- [x] Line ~144: Replace `"20G"` with format using `QEMU_DISK_GB`
- [x] Line ~145: Replace `"levitateos.iso"` with `ISO_FILENAME`
- [x] Replace `"virtual-disk.qcow2"` with `QEMU_DISK_FILENAME`

### 2.5 Update `leviso/src/clean.rs`

- [x] Add import:

```rust
use distro_spec::levitate::{
    ISO_FILENAME, SQUASHFS_NAME, EFIBOOT_FILENAME,
    INITRAMFS_BUILD_DIR, INITRAMFS_OUTPUT,
};
```

- [x] Line ~63-64: Replace `"levitateos.iso"` with `ISO_FILENAME`
- [x] Line ~65-68: Replace initramfs paths with constants
- [x] Line ~69: Replace `"efiboot.img"` with `EFIBOOT_FILENAME`
- [x] Line ~87-107: Replace initramfs cleanup paths with constants
- [x] Line ~111-113: Replace `"efiboot.img"` with `EFIBOOT_FILENAME`
- [x] Line ~139: Replace `"filesystem.squashfs"` with `SQUASHFS_NAME`

### 2.6 Update `leviso/src/commands/build.rs`

- [x] Add import:

```rust
use distro_spec::levitate::{ISO_FILENAME, SQUASHFS_NAME};
```

- [x] Line ~95-96: Replace `"levitateos.iso"` in status messages with `ISO_FILENAME`
- [x] Line ~96, ~146: Replace `"filesystem.squashfs"` with `SQUASHFS_NAME`

### 2.7 Update `leviso/src/component/custom/live.rs`

- [x] Add import:

```rust
use distro_spec::levitate::{OS_NAME, LIVE_ISSUE_MESSAGE};
```

- [ ] Line ~26: Replace `"live-overlay"` with constant if adding one
- [x] Line ~81: Replace `/etc/issue` message with `LIVE_ISSUE_MESSAGE`

### 2.8 Update `leviso/src/rebuild.rs`

- [x] Add import:

```rust
use distro_spec::levitate::INITRAMFS_OUTPUT;
```

- [x] Line ~62, ~114: Replace `"profile/init_tiny"` references (will become template)

### 2.9 Update `leviso/src/build/users.rs`

- [x] Add import:

```rust
use distro_spec::levitate::{ROOT_SHELL};
```

- [x] Line ~85, ~87: Replace hardcoded root user entry with constants

### 2.10 Update `leviso/src/build/filesystem.rs`

- [ ] Verify `/root` directory handling uses constants if applicable

---

## Part 3: Convert init_tiny to Template

### 3.1 Rename init_tiny to template

- [x] `mv leviso/profile/init_tiny leviso/profile/init_tiny.template`

### 3.2 Add placeholders in `init_tiny.template`

- [x] Line 7: Replace `LEVITATEOS` with `{{ISO_LABEL}}`
- [x] Line 48: Replace `cdrom virtio_scsi sr_mod isofs virtio_blk loop squashfs overlay` with `{{BOOT_MODULES}}`
- [x] Line 83: Replace `LEVITATEOS` with `{{ISO_LABEL}}`
- [x] Line 119-120: Replace device list with `{{BOOT_DEVICES}}` (or keep hardcoded if simpler)
- [x] Line 135: Replace `/live/filesystem.squashfs` with `{{SQUASHFS_PATH}}`
- [x] Line 154: Replace `/live/filesystem.squashfs` with `{{SQUASHFS_PATH}}`
- [x] Line 168: Replace mount point paths with placeholders if desired
- [x] Line 197-200: Replace `/mnt/live/overlay` references with `{{LIVE_OVERLAY_PATH}}`

### 3.3 Add `generate_init_script()` to `leviso/src/artifact/initramfs.rs`

- [x] Add function:

```rust
/// Generate init script from template with distro-spec values.
fn generate_init_script(base_dir: &Path) -> Result<String> {
    use distro_spec::levitate::{
        ISO_LABEL, SQUASHFS_ISO_PATH, BOOT_MODULES,
        BOOT_DEVICE_PROBE_ORDER, LIVE_OVERLAY_ISO_PATH,
    };

    let template = fs::read_to_string(base_dir.join("profile/init_tiny.template"))
        .context("Failed to read init_tiny.template")?;

    // Extract module names from full paths
    // e.g., "kernel/fs/squashfs/squashfs.ko.xz" -> "squashfs"
    let module_names: Vec<&str> = BOOT_MODULES.iter()
        .filter_map(|m| m.rsplit('/').next())
        .map(|m| m.trim_end_matches(".ko.xz").trim_end_matches(".ko.gz"))
        .collect();

    Ok(template
        .replace("{{ISO_LABEL}}", ISO_LABEL)
        .replace("{{SQUASHFS_PATH}}", &format!("/{}", SQUASHFS_ISO_PATH))
        .replace("{{BOOT_MODULES}}", &module_names.join(" "))
        .replace("{{BOOT_DEVICES}}", &BOOT_DEVICE_PROBE_ORDER.join(" "))
        .replace("{{LIVE_OVERLAY_PATH}}", &format!("/mnt/{}", LIVE_OVERLAY_ISO_PATH)))
}
```

### 3.4 Update `build_initramfs()` to use generated script

- [x] Replace static file copy with:

```rust
let init_content = generate_init_script(base_dir)?;
fs::write(initramfs_root.join("init"), init_content)?;
```

---

## Part 4: Verification

### 4.1 Build verification

- [x] `cd distro-spec && cargo build`
- [x] `cd distro-spec && cargo test`
- [x] `cd distro-spec && cargo clippy`
- [x] `cd leviso && cargo build`
- [x] `cd leviso && cargo clippy`

### 4.2 Functional verification

- [ ] `cd leviso && cargo run -- build` - Full ISO build succeeds
- [ ] `cd leviso && cargo run -- run` - ISO boots in QEMU
- [ ] Verify autologin to root shell works
- [ ] Verify basic commands work (ls, cat, mount)

### 4.3 SSOT verification - No hardcoded values remain

- [x] Check ISO label:
```bash
isoinfo -d -i output/levitateos.iso | grep "Volume id"
# Should show: LEVITATEOS
```

- [x] Check for remaining hardcoded strings in leviso/src/:
```bash
grep -rn '"LEVITATEOS"' leviso/src/
grep -rn '"filesystem.squashfs"' leviso/src/
grep -rn '"levitateos.iso"' leviso/src/
grep -rn '"efiboot.img"' leviso/src/
grep -rn '"initramfs-tiny' leviso/src/
grep -rn '"/live/' leviso/src/
grep -rn '"boot/vmlinuz"' leviso/src/
grep -rn '"console=ttyS0' leviso/src/
grep -rn '"BOOTX64.EFI"' leviso/src/
# All should return zero matches
```

### 4.4 Init script verification

- [ ] Extract and verify generated init script:
```bash
mkdir -p /tmp/initramfs-check
cd /tmp/initramfs-check
zcat /path/to/leviso/output/initramfs-tiny.cpio.gz | cpio -idmv
cat init | grep -E "LEVITATEOS|filesystem.squashfs|cdrom.*squashfs"
# Should show values matching distro-spec constants
```

### 4.5 Template verification

- [x] Verify template has placeholders:
```bash
grep -E '\{\{[A-Z_]+\}\}' leviso/profile/init_tiny.template
# Should show: {{ISO_LABEL}}, {{SQUASHFS_PATH}}, {{BOOT_MODULES}}, etc.
```

---

## Summary

### distro-spec files

| File | Action |
|------|--------|
| `src/shared/iso.rs` | **NEW** - ISO structure, EFI, console, xorriso constants |
| `src/shared/initramfs.rs` | **NEW** - initramfs dirs, mount points, compression |
| `src/shared/devices.rs` | **NEW** - boot device probe order |
| `src/shared/qemu.rs` | **NEW** - QEMU testing defaults |
| `src/shared/mod.rs` | Add module declarations and re-exports |
| `src/levitate/paths.rs` | Add ISO_FILENAME, QEMU defaults, busybox, live message |
| `src/levitate/mod.rs` | Add re-exports from shared and paths |
| `src/acorn/paths.rs` | Add matching AcornOS-specific constants |
| `src/acorn/mod.rs` | Add re-exports from shared and paths |

### leviso files

| File | Action |
|------|--------|
| `src/artifact/iso.rs` | Replace ~25 hardcoded values |
| `src/artifact/squashfs.rs` | Replace 2 compression settings |
| `src/artifact/initramfs.rs` | Replace ~10 values, add generate_init_script() |
| `src/qemu.rs` | Replace ~7 hardcoded values |
| `src/clean.rs` | Replace ~10 cleanup paths |
| `src/commands/build.rs` | Replace ~4 status message paths |
| `src/component/custom/live.rs` | Replace OS name, issue message |
| `src/rebuild.rs` | Update init script path reference |
| `src/build/users.rs` | Replace root user entry if applicable |
| `profile/init_tiny` | Rename to `init_tiny.template`, add ~6 placeholders |

### Total changes

- **distro-spec**: 4 new files, 5 modified files
- **leviso**: 9 modified files, 1 renamed file
- **~60+ hardcoded values** replaced with SSOT constants
