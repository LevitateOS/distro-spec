# Authentication Subsystem

## Overview

The authentication subsystem consolidates all LevitateOS authentication and authorization components into a single source of truth. This includes:

- **PAM (Pluggable Authentication Modules)**: 18 core modules for authentication, authorization, and session management
- **Binaries**: su, sudo, passwd, chpasswd, login, agetty, and supporting tools
- **Configuration Files**: /etc/pam.d/*, /etc/security/*, /etc/sudoers, login.defs
- **User Database**: passwd, shadow, group, gshadow

## Architecture

```
distro-spec/src/shared/auth/
├── mod.rs              # Public API and architecture documentation
├── requirements.md     # Complete 11-category requirements specification
├── components.rs       # Binary, module, config lists (SINGLE SOURCE OF TRUTH)
├── pam.rs              # PAM configuration file contents
├── getty.rs            # Getty service configuration constants
└── ssh.rs              # SSH service configuration constants
```

## Single Source of Truth (SSOT)

All authentication-related constants are defined once in this module and imported by consumers:

- **distro-spec/src/shared/components.rs** - ❌ OLD (DO NOT USE) - AUTH_* constants moved here
- **leviso** - Uses imports from auth subsystem (PAM configs, SSH configs)
- **testing/fsdbg** - Uses imports for verification checklists
- **testing/rootfs-tests** - Uses imports for boot-time auth tests

### Key Constants

```rust
// From auth/components.rs - Component lists
pub const AUTH_BIN: &[&str] = &["su", "sudo", "sudoedit", "sudoreplay"];
pub const AUTH_SBIN: &[&str] = &["visudo", "unix_chkpwd"];  // unix_chkpwd is CRITICAL
pub const SHADOW_SBIN: &[&str] = &[...];                    // User/group management
pub const SSH_BIN: &[&str] = &["ssh", "scp", "sftp", ...];
pub const SSH_SBIN: &[&str] = &["sshd"];
pub const PAM_MODULES: &[&str] = &[...];                    // 18 actual modules used
pub const PAM_CONFIGS: &[&str] = &[...];                    // 18 configuration files
pub const SECURITY_FILES: &[&str] = &[...];                 // 5 policy files
pub const SUDO_LIBS: &[&str] = &[...];                      // Sudo support libraries

// From auth/pam.rs - PAM configuration contents
pub const PAM_SYSTEM_AUTH: &str = include_str!(...);
pub const PAM_LOGIN: &str = include_str!(...);
pub const PAM_SSHD: &str = include_str!(...);
pub const PAM_SUDO: &str = include_str!(...);
pub const PAM_OTHER: &str = include_str!(...);

// From auth/getty.rs - Getty configuration
pub const SERIAL_GETTY_OVERRIDE_QEMU: &str = "...";  // Includes -L flag for QEMU
pub const SERIAL_BAUD_RATES: &str = "115200,57600,38400,9600";

// From auth/ssh.rs - SSH configuration
pub const SSHD_CONFIG_SETTINGS: &[&str] = &["PermitRootLogin yes", ...];
```

## Critical Dependencies

### 1. unix_chkpwd - Single Point of Failure

**The Problem**: pam_unix.so has a hardcoded path `/usr/sbin/unix_chkpwd`. Without this binary, password changes silently fail:
- `passwd` runs and returns success
- `/etc/shadow` is NOT updated
- User's password hasn't actually changed
- User discovers this only when they can't login next time

**The Fix**: This binary is CRITICAL and marked as such in AUTH_SBIN.

### 2. Login Symlink - Missing by Default

**The Problem**: agetty (getty) expects `/bin/login` in PATH. On modern systems with /usr/sbin, we create:
```
/usr/bin/login -> /usr/sbin/login
```

Without this symlink, console login fails silently (agetty can't find the login binary).

### 3. Serial Getty -L Flag - QEMU Requirement

**The Problem**: QEMU serial console sends modem control signals. The `-L` flag tells getty to ignore them.

Without `-L`: Serial login hangs after typing username (getty thinks line dropped).

## OverlayFS Architecture (Live ISO)

The live ISO uses a three-layer OverlayFS mount to enable autologin without modifying the base system:

```
Layer 3 (top):    tmpfs (/overlay/upper)         [read-write, ephemeral]
                     ↓ (runtime writes go here)
Layer 2 (middle): /live/overlay from ISO         [read-only, live configs]
                     ↓ (overrides base system)
Layer 1 (bottom): EROFS (/rootfs)                [read-only, base system]
```

### Files Modified by Live Overlay

- `/etc/shadow` - Empty root password (root::19000:...)
- `/etc/systemd/system/console-autologin.service` - Auto-login on tty1
- `/etc/systemd/system/serial-console.service` - Auto-login on ttyS0 (QEMU)
- `/etc/systemd/system/getty.target.wants/` - Symlinks for services

### Why Live Overlay Doesn't Affect Installed Systems

When user runs `recstrap /mnt` to install:

1. Live system has OverlayFS active (EROFS + live-overlay + tmpfs)
2. recstrap extracts from EROFS ONLY (not from /live/overlay)
3. The /live/overlay directory is on the ISO filesystem, NOT in the EROFS image
4. Installed system boots with systemd initramfs (no OverlayFS)
5. Result: `/etc/shadow` has locked root from base EROFS

**The consequence**: Installed systems MUST handle root authentication differently than the live ISO.

## Root Password Security Policy

### Live ISO
- Root password: EMPTY (via live overlay)
- Root can login directly: YES
- Use case: Installation environment

### Installed System
- Root password: LOCKED (base system default)
- Root can login: NO
- Required action: User must either:
  - Set root password: `passwd root` in chroot
  - Create user account: `useradd -m -G wheel username`

## Boot Flow

```
Live ISO Boot
├── Kernel mounts EROFS from ISO
├── init_tiny creates OverlayFS (EROFS + live-overlay + tmpfs)
├── Mounts merged view to /newroot
├── Root has empty password (from live-overlay/etc/shadow)
├── console-autologin.service auto-logs in root on tty1
└── Root shell available

Installed System Boot
├── Kernel mounts /boot (EFI)
├── systemd-boot loads UKI (unified kernel image)
├── initramfs decompresses (systemd-based, not busybox)
├── Mounts rootfs to / (NO OverlayFS)
├── systemd starts login services
├── Root password is locked (from /etc/shadow base)
├── User must create account or set root password in chroot
└── System reaches login prompt
```

## Password Aging Policy

From `login.defs`:

```
PASS_MAX_DAYS   99999   # No expiry (disabled)
PASS_MIN_DAYS   0       # Can change immediately
PASS_WARN_DAYS  7       # Warn 7 days before expiry
```

All passwords use YESCRYPT (modern secure hashing).

## Security Configuration

### PAM Stack Order

1. **account** - Check if account is valid/not expired
2. **auth** - Authenticate user (password, fingerprint, etc.)
3. **password** - Password change policy
4. **session** - Session setup (environment, limits, logging)

### Modules Used (18 total)

**Core Authentication**:
- `pam_unix.so` - Unix password authentication (CRITICAL)
- `pam_deny.so` - Fallback deny policy
- `pam_rootok.so` - Skip auth for root

**Hardening**:
- `pam_pwquality.so` - Password strength requirements
- `pam_faillock.so` - Account lockout after failed attempts
- `pam_loginuid.so` - Audit UID tracking
- `pam_securetty.so` - Restrict root to secure terminals

**Session**:
- `pam_systemd.so` - Register with systemd-logind
- `pam_keyinit.so` - Initialize kernel keyring
- `pam_namespace.so` - Polyinstantiated /tmp per-user
- `pam_limits.so` - Resource limits (ulimit)
- `pam_umask.so` - Default umask
- `pam_env.so` - Environment setup
- `pam_lastlog.so` - Last login logging
- `pam_motd.so` - Message of the day
- `pam_xauth.so` - X11 forwarding on su

**Access Control**:
- `pam_access.so` - /etc/security/access.conf rules
- `pam_nologin.so` - Honor /etc/nologin
- `pam_succeed_if.so` - Conditional success (cron bypass)

**Removed/Not Used**:
- `pam_permit.so` - Unused (would always permit)
- `pam_wheel.so` - Handled via sudo config instead
- 25+ others - Verified not referenced in any PAM config file

## Testing

### Static Verification (fsdbg)

```bash
cargo run -p fsdbg -- verify rootfs.erofs --type auth-audit --verbose
```

Checks:
- All AUTH_BIN present in /usr/bin
- All AUTH_SBIN present in /usr/sbin
- All SHADOW_SBIN present
- All PAM_MODULES present in /usr/lib64/security/
- All PAM_CONFIGS present in /etc/pam.d/
- All SECURITY_FILES present in /etc/security/
- Correct permissions on shadow/gshadow (0600)
- /usr/bin/login symlink exists
- systemd-logind present

### Runtime Verification (rootfs-tests)

```bash
cargo test -p rootfs-tests -- --ignored security::
```

Checks:
- Boot to login prompt
- Root is locked on installed system
- SSH keys regenerated
- machine-id generated
- No world-writable files in /etc

## Updating the Auth Subsystem

### Adding a New PAM Module

1. Edit `distro-spec/src/shared/auth/components.rs` - Add to PAM_MODULES
2. Create `/path/to/profile/etc/pam.d/...` - Add PAM config using it
3. Edit `distro-spec/src/shared/auth/pam.rs` - Add const for config content
4. Run verification: `cargo run -p fsdbg -- verify rootfs.erofs --type auth-audit`

### Adding a New Binaries/Tool

1. Edit `distro-spec/src/shared/auth/components.rs` - Add to appropriate list
2. Edit `leviso/profile/etc/...` - Add any config files needed
3. Update tests if adding critical component
4. Run verification

### Updating PAM Configuration

PAM configurations are defined in two places (for different reasons):

1. **Profile files** (`leviso/profile/etc/pam.d/...`) - Source of truth, human readable
2. **Code constants** (`distro-spec/src/shared/auth/pam.rs`) - Compiled into binaries

Workflow:
1. Edit profile file
2. Update the code constant with new content
3. Test with `cargo test -p distro-spec`
4. Verify with `cargo run -p fsdbg -- verify rootfs.erofs --type auth-audit`

## References

- Linux-PAM Documentation: http://www.linux-pam.org/
- Shadow Utilities: https://github.com/shadow-maint/shadow
- OpenSSH Configuration: https://man7.org/linux/man-pages/man5/sshd_config.5.html
- OverlayFS: https://www.kernel.org/doc/html/latest/filesystems/overlayfs.html
- EROFS: https://www.kernel.org/doc/html/latest/filesystems/erofs.html

## Known Issues and Workarounds

### Issue: Root password locked on installed systems

**Workaround**: Set root password in chroot after recstrap:
```bash
recstrap /mnt
recchroot /mnt
passwd root    # This works even though root is locked
exit
```

### Issue: Serial getty hangs on QEMU

**Cause**: QEMU sends modem control signals, getty thinks line dropped.
**Workaround**: Serial getty override includes `-L` flag to ignore these signals.

### Issue: /etc/shadow permissions incorrect after extraction

**Prevention**: distro-spec/src/shared/auth/components.rs includes tests to verify 0600.

## Architecture Decisions

### Why 18 PAM Modules (not 40+)?

Verified each module against ALL /etc/pam.d/* files to find actual usage. Included only modules actually referenced. This prevents:
- Bloated initramfs (every unused module = extra space)
- Security surface (fewer modules = less attack surface)
- Testing complexity (verify only what's actually used)

### Why Encrypt Passwords with YESCRYPT?

Modern secure algorithm, resistant to GPU/ASIC attacks, part of Linux-PAM standard as of 2022.

### Why OverlayFS for Live ISO?

- Kernel built-in feature (no dependencies)
- Atomic layer merge (no race conditions)
- Performance efficient (CoW on write)
- Clean separation: ISO is read-only, overlay is ephemeral

### Why Not Auto-Create User During Installation?

Keeps recstrap minimal and user-controlled. Different users have different needs:
- Some prefer root for admin tasks
- Some prefer passwordless sudo
- Some want multiple users

Better to document and let users choose.

## Contributing

When making changes to authentication:

1. Update BOTH profile file and code constant
2. Add test if adding critical component
3. Run full verification: `cargo test -p distro-spec && cargo run -p fsdbg -- verify rootfs.erofs --type auth-audit`
4. Update this README if changing architecture
5. Update TEAM file if fixing issues

