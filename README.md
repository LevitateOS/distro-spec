# distro-spec

Installation specifications for LevitateOS and AcornOS. Single source of truth for installation constants, paths, partition layouts, and user configuration.

## Status

| Metric | Value |
|--------|-------|
| Stage | Beta |
| Target | x86_64 Linux (no_std compatible) |
| Last verified | 2026-01-23 |

### Works

- LevitateOS spec (Rocky/systemd/glibc)
- AcornOS spec (Alpine/OpenRC/musl)
- Shared partition layouts, user specs, chroot binds
- Boot entry and loader config types

### Known Issues

- See parent repo issues

---

## Author

<!-- HUMAN WRITTEN - DO NOT MODIFY -->

[Waiting for human input]

<!-- END HUMAN WRITTEN -->

---

## Overview

This crate defines the canonical specs consumed by:
- **leviso** - ISO builder
- **recstrap** - Installer
- **install-tests** - E2E verification

If something is defined here, it MUST be produced by the builder, tested by install-tests, and documented. Any mismatch is a bug.

## Installation

```toml
[dependencies]
distro-spec = { path = "../distro-spec" }

# For no_std environments
distro-spec = { path = "../distro-spec", default-features = false }
```

## Usage

```rust
// LevitateOS spec
use distro_spec::levitate;
let user = levitate::default_user("alice");
println!("Shell: {}", levitate::DEFAULT_SHELL);

// AcornOS spec
use distro_spec::acorn;
let user = acorn::default_user("bob");
println!("Shell: {}", acorn::DEFAULT_SHELL);

// Shared types
use distro_spec::shared::{PartitionLayout, UserSpec};
use distro_spec::{CHROOT_BIND_MOUNTS, EFI_PARTITION_SIZE_MB};
```

## Modules

### `levitate`

LevitateOS specifications: Rocky Linux base, systemd, glibc, GNU coreutils.

### `acorn`

AcornOS specifications: Alpine Linux base, OpenRC, musl, busybox.

### `shared`

Common types and constants used by both variants:

| Module | Contents |
|--------|----------|
| `shared::partitions` | `PartitionLayout`, `PartitionSpec`, `EFI_PARTITION_SIZE_MB` |
| `shared::users` | `UserSpec`, `MIN_UID`, `MIN_GID`, `SUDOERS_WHEEL_LINE` |
| `shared::chroot` | `BindMount`, `CHROOT_BIND_MOUNTS`, mount ordering functions |

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | Yes | Standard library support |

Disable default features for `no_std` environments (embedded installers, bootloaders).

## Philosophy

1. **Single source of truth** - All installation constants live here
2. **Testable** - Every spec can be verified by install-tests
3. **Portable** - `no_std` support for constrained environments
4. **Multi-distro** - Same patterns, different defaults per variant

## License

MIT
