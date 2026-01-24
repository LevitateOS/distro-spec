# CLAUDE.md - distro-spec

## What is distro-spec?

**Single Source of Truth (SSOT)** for installation specifications. Defines boot entries, partition layouts, user specs, paths, and service configuration for both LevitateOS and AcornOS.

## What Belongs Here

- Boot configuration types (`BootEntry`, `LoaderConfig`)
- Partition specifications (`PartitionLayout`, `PartitionSpec`)
- User specifications (`UserSpec`)
- Chroot bind mounts (`CHROOT_BIND_MOUNTS`)
- Service management traits and implementations
- Distro-specific paths and constants

## What Does NOT Belong Here

| Don't put here | Put it in |
|----------------|-----------|
| Build logic | `leviso/` or `distro-builder/` |
| CLI tools | `tools/` |
| Tests that boot VMs | `testing/` |

## Architecture

```
src/
├── lib.rs              # Re-exports public API
├── shared/             # Cross-distro types
│   ├── boot.rs         # BootEntry, LoaderConfig
│   ├── partitions.rs   # PartitionLayout, PartitionSpec
│   ├── users.rs        # UserSpec
│   ├── chroot.rs       # BindMount, CHROOT_BIND_MOUNTS
│   └── services.rs     # ServiceManager trait
├── levitate/           # LevitateOS-specific
│   ├── boot.rs         # BOOT_MODULES, default_boot_entry()
│   ├── paths.rs        # ISO_LABEL, SQUASHFS_PATH
│   └── services.rs     # SystemdService implementation
└── acorn/              # AcornOS-specific
    ├── boot.rs         # Alpine boot modules
    ├── paths.rs        # Alpine paths
    └── services.rs     # OpenRCService implementation
```

## Commands

```bash
cargo build
cargo test
cargo doc --no-deps    # Generate documentation
```

## Usage

```rust
// LevitateOS boot entry
use distro_spec::levitate::{default_boot_entry, default_loader_config};
let entry = default_boot_entry().set_root("UUID=xxx");

// AcornOS boot entry
use distro_spec::acorn::{default_boot_entry, default_loader_config};

// Polymorphic service handling
use distro_spec::ServiceManager;
fn enable_service(svc: &impl ServiceManager) -> String {
    svc.enable_command()
}
```

## Consumers

| Crate | Uses |
|-------|------|
| leviso | Paths, boot modules |
| testing/install-tests | BootEntry, LoaderConfig, partitions |
| tools/recstrap | Squashfs search paths |

## Key Rule

**Changes here affect multiple crates.** Run `cargo build --workspace` after changes.
