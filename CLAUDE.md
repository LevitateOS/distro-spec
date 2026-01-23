# CLAUDE.md - Distro Spec

## STOP. READ. THEN ACT.

Before modifying this crate:
1. Read existing modules (`levitate/`, `acorn/`, `shared/`)
2. Understand what's already defined
3. Changes here affect multiple crates (leviso, recstrap, install-tests)

---

## What is distro-spec?

**Single Source of Truth (SSOT)** for installation specifications. Defines constants, partition layouts, user specs, boot configuration, and service management for LevitateOS and AcornOS.

## Architecture

```
src/
├── lib.rs              # Re-exports (BootEntry, LoaderConfig, etc.)
├── shared/
│   ├── mod.rs
│   ├── boot.rs         # BootEntry, LoaderConfig (shared types)
│   ├── chroot.rs       # BindMount, CHROOT_BIND_MOUNTS
│   ├── partitions.rs   # PartitionLayout, PartitionSpec
│   ├── services.rs     # ServiceManager trait
│   └── users.rs        # UserSpec
├── levitate/
│   ├── mod.rs          # Re-exports for LevitateOS
│   ├── boot.rs         # BOOT_MODULES + distro constructors
│   ├── paths.rs        # ISO_LABEL, SQUASHFS_*, filenames
│   └── services.rs     # SystemdService impl of ServiceManager
└── acorn/
    ├── mod.rs          # Re-exports for AcornOS
    ├── boot.rs         # BOOT_MODULES (Alpine) + distro constructors
    ├── paths.rs        # ISO_LABEL, SQUASHFS_* (Alpine variants)
    └── services.rs     # OpenRCService impl of ServiceManager
```

### Why This Structure?

| Component | Location | Reason |
|-----------|----------|--------|
| BootEntry, LoaderConfig | `shared/` | Both distros use systemd-boot |
| ServiceManager trait | `shared/` | Common interface for init systems |
| ServiceSpec structs | `levitate/`, `acorn/` | Different init systems (systemd vs OpenRC) |
| BOOT_MODULES | `levitate/`, `acorn/` | Different kernel module paths |
| ISO_LABEL, SQUASHFS_* | `levitate/`, `acorn/` | Different OS names/configs |

### Adding a New Distro Variant

1. Create `src/<variant>/` directory
2. Add modules: `mod.rs`, `boot.rs`, `paths.rs`, `services.rs`
3. Re-export from `lib.rs`
4. Implement `ServiceManager` for your init system

## Development

```bash
cargo build
cargo test
cargo clippy
cargo doc --no-deps  # Generate documentation
```

## Key Rules

1. **This is the source of truth** - Don't duplicate specs elsewhere
2. **Keep no_std compatible** - Don't add std-only code without feature gates
3. **Document changes** - Other crates depend on these values
4. **Test coverage** - Every spec should be verifiable by install-tests
5. **Break the API, fix at compile time** - Rust's compiler catches breaking changes

## Usage Examples

```rust
// LevitateOS boot entry
use distro_spec::levitate::{default_boot_entry, default_loader_config};
let entry = default_boot_entry().set_root("UUID=xxx");
let loader = default_loader_config().disable_editor();

// AcornOS boot entry
use distro_spec::acorn::{default_boot_entry, default_loader_config};
let entry = default_boot_entry().set_root("LABEL=root");

// Polymorphic service handling
use distro_spec::ServiceManager;
fn enable_service(svc: &impl ServiceManager) -> String {
    svc.enable_command()
}
```

## Consumers

| Crate | Uses |
|-------|------|
| install-tests | BootEntry, LoaderConfig, ENABLED_SERVICES, PartitionLayout |
| leviso | (should use) ISO_LABEL, SQUASHFS_*, BOOT_MODULES |
| recstrap | (should use) SQUASHFS_SEARCH_PATHS |
