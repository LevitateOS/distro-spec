# distro-spec

Shared constants for LevitateOS installation. Partition layouts, user specs, boot configuration, chroot bind mounts.

## Status

**Beta.** Used by leviso, recstrap, install-tests.

| Implemented | Stubbed |
|-------------|---------|
| LevitateOS specs (Rocky/systemd) | AcornOS specs (Alpine/OpenRC) |
| Partition layouts | |
| User specs | |
| Boot entry types | |
| Chroot bind mounts | |

## Usage

```rust
use distro_spec::levitate;

// Default user spec
let user = levitate::default_user("alice");

// Boot entry
let entry = levitate::default_boot_entry();

// Constants
use distro_spec::{CHROOT_BIND_MOUNTS, EFI_PARTITION_SIZE_MB};
```

## Modules

### `levitate`

LevitateOS specs: Rocky Linux packages, systemd, glibc.

### `acorn`

AcornOS specs: Alpine packages, OpenRC, musl. **Stubbed, not implemented.**

### `shared`

| Module | Contents |
|--------|----------|
| `partitions` | `PartitionLayout`, `EFI_PARTITION_SIZE_MB` |
| `users` | `UserSpec`, `MIN_UID`, `SUDOERS_WHEEL_LINE` |
| `chroot` | `BindMount`, `CHROOT_BIND_MOUNTS` |
| `boot` | `BootEntry`, `LoaderConfig` |

## no_std Support

```toml
[dependencies]
distro-spec = { path = "../distro-spec", default-features = false }
```

Disable `std` feature for embedded/bootloader contexts.

## Consumers

- `leviso` - Uses paths, boot config
- `recstrap` - Uses squashfs paths
- `install-tests` - Uses all specs for verification

## Known Limitations

- AcornOS module exists but is not implemented
- No runtime validation of specs
- Changes here require updates to all consumers

## Building

```bash
cargo build
cargo test
```

## License

MIT
