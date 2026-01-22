# CLAUDE.md - Distro Spec

## STOP. READ. THEN ACT.

Before modifying this crate:
1. Read existing modules (`levitate/`, `acorn/`, `shared/`)
2. Understand what's already defined
3. Changes here affect multiple crates (leviso, recstrap, install-tests)

---

## What is distro-spec?

Single source of truth for installation specifications. Defines constants, partition layouts, user specs, and chroot bind mounts for LevitateOS and AcornOS.

## Development

```bash
cargo build
cargo test
cargo clippy
```

## Key Rules

1. **This is the source of truth** - Don't duplicate specs elsewhere
2. **Keep no_std compatible** - Don't add std-only code without feature gates
3. **Document changes** - Other crates depend on these values
4. **Test coverage** - Every spec should be verifiable by install-tests

## Module Structure

```
src/
├── lib.rs          # Re-exports
├── levitate/       # LevitateOS specs (Rocky, systemd, glibc)
├── acorn/          # AcornOS specs (Alpine, OpenRC, musl)
└── shared/         # Common types (partitions, users, chroot)
```
