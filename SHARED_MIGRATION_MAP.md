# `distro-spec/src/shared` Migration Map (Draft)

## Goal

Deprecate `distro-spec` without losing single-source-of-truth guarantees.

Target architecture:
- Per-OS declarations -> `distro-variants/<distro>/`
- Cross-distro build/runtime invariants -> `distro-builder`
- Test-only oracles/checklists -> `testing/fsdbg` and `testing/install-tests`
- Generic installer/tool primitives -> shared core module (initially in `distro-builder`, split later if needed)

## Why

- `distro-spec` currently mixes variant declarations, builder policy, and test oracles.
- That causes duplication with per-OS crates and weakens conformance boundaries.
- CP0 and later checkpoints need strict ownership to prevent drift/reward-hacking.

## Proposed Module Ownership

| Current module(s) | Proposed destination | Notes |
|---|---|---|
| `shared/kernel.rs` | `distro-builder` + `distro-variants/*` | Keep kernel source struct/helpers in builder; move per-OS kernel constants into each variant declaration. |
| `shared/modules.rs`, `shared/boot_modules.rs` | `distro-builder` (policy) + variant overlays | Base module sets belong in builder; variant-specific deltas stay in variant folders. |
| `shared/initramfs.rs`, `shared/iso.rs`, `shared/rootfs.rs`, `shared/uki.rs`, `shared/qemu.rs`, `shared/devices.rs`, `shared/firmware.rs`, `shared/paths.rs` | `distro-builder` | These are build/runtime policy constants used by builders/tools. |
| `shared/chroot.rs`, `shared/partitions.rs`, `shared/users.rs`, `shared/services.rs`, `shared/system.rs`, `shared/error.rs` | Shared core (first in `distro-builder`) | Tool-facing primitives used by `recstrap`/`recchroot` and build flows. |
| `shared/auth/*` | `distro-variants/*` + `testing/fsdbg` | Auth/PAM payloads are distro policy declarations first; tests should consume variant declarations, not `distro-spec`. |
| `shared/components/*` | `distro-variants/*` + `testing/fsdbg` | Rootfs component inventories are conformance declarations, so they should live with each variant. |
| `shared/requirements.rs` | `distro-variants/*` | Hardware/runtime expectations are variant contract declarations. |

## Immediate Migration Rules

1. New declarations must not be added to `distro-spec/src/{levitate,acorn,iuppiter,ralph}`.
2. New shared build policy must be added to `distro-builder`.
3. New test oracles must be added under `testing/*` and read from variant declarations.
4. `distro-spec` remains transitional until all consumers are rewired.
