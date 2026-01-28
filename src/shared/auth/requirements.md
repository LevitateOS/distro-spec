# Authentication/Login Subsystem Requirements

## Overview

LevitateOS authentication subsystem provides:
- PAM (Pluggable Authentication Modules) for password-based authentication
- User/group database management
- SSH server configuration
- Sudo privilege escalation
- Security policies (faillock, password quality, etc.)
- Live ISO autologin via OverlayFS
- Secure installed system with locked root

This document serves as the **single source of truth** for all login/authentication requirements.

---

## 1. Authentication Mechanisms

### PAM (Pluggable Authentication Modules)

**Requirement**: Use YESCRYPT password hashing algorithm
- Modern, memory-hard, resistant to GPU attacks
- More secure than bcrypt or sha512crypt
- Configured in `/etc/pam.d/system-auth` via `password ... pam_unix.so yescrypt`

**Requirement**: Systemd integration via pam_systemd.so
- Register sessions with systemd-logind
- Enables session tracking, device access control, cgroup management
- Part of core authentication stack in `system-auth`

**Requirement**: Account lockout via pam_faillock
- Lock account after 3 failed attempts
- Unlock after 15 minutes or manual admin intervention
- Prevents brute-force attacks
- Configured in `/etc/security/faillock.conf`

**Requirement**: Password quality enforcement via pam_pwquality
- Minimum 12 characters
- Minimum 3 character classes (uppercase, lowercase, digits, symbols)
- No more than 3 repeated characters
- Configured in `/etc/security/pwquality.conf`

**Requirement**: Resource limits via pam_limits.so
- Per-user file descriptor limits (nofile)
- Core dump limits
- Process limits
- Configured in `/etc/security/limits.conf`

**Requirement**: Namespace isolation via pam_namespace.so
- Per-user `/tmp` directories (polyinstantiation)
- Prevents cross-user tampering in shared directories
- Currently minimal configuration (mostly empty)

### 18 PAM Configuration Files

**Core Authentication Stacks**:
1. `system-auth` - Main authentication stack (passwords, sessions, account)
2. `password-auth` - Password-based auth for remote services

**Login Services**:
3. `login` - Console login via agetty
4. `sshd` - SSH login
5. `remote` - Remote login services

**Privilege Escalation**:
6. `sudo` - sudo command
7. `su` - su command (switch user)
8. `su-l` - su - (login shell)
9. `runuser` - runuser (root-only su)
10. `runuser-l` - runuser - (login shell)

**Password/User Management**:
11. `passwd` - passwd command (user password change)
12. `chpasswd` - chpasswd command (batch password setting)
13. `chfn` - chfn command (change full name)
14. `chsh` - chsh command (change shell)

**System Services**:
15. `crond` - cron daemon
16. `systemd-user` - systemd user sessions

**Post-Login**:
17. `postlogin` - Post-login session setup (included by login services)

**Fallback**:
18. `other` - Fallback for unconfigured services (should deny)

Note: Other user/group management commands (useradd, userdel, groupadd, groupdel, chage, chgpasswd, groupmems, newusers) fall back to the "other" policy and don't have custom PAM configs.

---

## 2. Console Login

### Getty Services

**Requirement**: getty@tty1.service enabled on console
- Starts agetty on /dev/tty1
- Displays login prompt
- Calls `/usr/sbin/login` for authentication
- Part of `getty.target`

**Requirement**: Serial getty with QEMU-specific override
- serial-getty@ttyS0.service listens on serial port
- Critical flag: `-L` (ignore modem signals, don't wait for CD)
- Without `-L`, QEMU serial console hangs (waiting for carrier signal)
- Override in `definitions.rs`: replaces `-` with `-L` in baud rate string
- Reason: QEMU emulates raw serial without proper modem signaling

**Requirement**: Multiple baud rates for serial console
- Primary: 115200 baud (standard modern rate)
- Fallbacks: 57600, 38400, 9600 (older systems)
- Format: `115200,57600,38400,9600`
- Allows connection at any rate; agetty negotiates best match

**Requirement**: VT102 terminal emulation
- Default for Linux systems
- Ensures arrow keys, function keys work correctly
- Terminal type: `vt102`

### Login Binary

**Requirement**: `/usr/sbin/login` binary
- From `shadow-utils` package
- Actual login command that prompts for password
- Invoked by getty (agetty) after user enters username

**CRITICAL REQUIREMENT**: `/usr/bin/login` symlink
- agetty searches PATH for login binary
- Looks in /usr/bin by default (symlink required)
- Points to `/usr/sbin/login` (actual binary)
- **If missing**: agetty fails silently, no login prompt appears
- See TEAM_108 for root cause analysis
- Created by: `symlink("usr/bin/login", "../sbin/login")`

**Requirement**: PAM integration for authentication
- login binary calls PAM `login` service
- Uses system-auth stack via login PAM config
- Checks password against `/etc/shadow`

**Requirement**: Environment setup
- HOME, SHELL, USER, LOGNAME, PATH variables
- Source `/etc/profile` for system-wide env setup
- Source user's `~/.bash_profile` if exists

### Live ISO Autologin (OverlayFS-based)

**Requirement**: console-autologin.service on tty1
- Autologins root user without password prompt
- Service: `ExecStart=/bin/bash --noprofile`
- **Conflicts**: `getty@tty1.service` (prevents getty from starting)
- Lives in: `/live/overlay/etc/systemd/system/console-autologin.service`
- Applied ONLY during live boot, NOT to installed systems

**Requirement**: serial-console.service on ttyS0
- Autologins root on serial port (for QEMU testing)
- Service: `ExecStart=/bin/bash --noprofile --nol ...`
- Enabled in multi-user.target for serial-only boot
- Lives in: `/live/overlay/etc/systemd/system/serial-console.service`

**Requirement**: No password prompt on live ISO
- Empty root password in `/live/overlay/etc/shadow`
- Root hash field: empty string (`:`)
- Allows immediate login without password

**Requirement**: Autologin applied ONLY via OverlayFS
- **NOT** in base system EROFS
- Base system has locked root (`root:!:...`)
- Live overlay only exists on ISO, not extracted by recstrap
- Installed systems boot without OverlayFS, so get locked root

---

## 3. Remote Login (SSH)

### OpenSSH Server

**Requirement**: sshd binary and sshd.service
- From `openssh` package
- Provides remote login capability
- Unit file: `/usr/lib/systemd/system/sshd.service`
- Socket activation: `sshd.socket` (optional, service direct)

**Requirement**: sshd system user
- UID: Usually < 1000 (system account)
- Privilege separation: sshd runs as unprivileged user
- Home directory: `/etc/ssh` or `/run/sshd`
- Shell: `/usr/sbin/nologin`

**Requirement**: Pre-generated SSH host keys (three types)
- RSA: 3072-bit (minimum recommended size)
- ECDSA: P-256 curve (256-bit)
- ED25519: Fixed size (most modern)
- Generated at build time: `ssh-keygen -t rsa -f ... -N "" -q`
- Ensures sshd can start immediately without key generation delay

**Requirement**: PermitRootLogin yes
- Allow root login via SSH (for live ISO and initial setup)
- Security note: OK for live ISO (read-only), acceptable for servers with SSH key auth
- Can be tightened post-installation

**Requirement**: PasswordAuthentication yes
- Allow password-based login (convenient for live ISO)
- Can be disabled post-installation to enforce keys only

**Requirement**: Key Storage
- Host keys stored in `/etc/ssh/`:
  - `ssh_host_rsa_key` (private)
  - `ssh_host_rsa_key.pub` (public)
  - `ssh_host_ecdsa_key`, `ssh_host_ecdsa_key.pub`
  - `ssh_host_ed25519_key`, `ssh_host_ed25519_key.pub`
- Client config: `/etc/ssh/ssh_config`
- Server config: `/etc/ssh/sshd_config`

### SSH Runtime Requirements

**Requirement**: `/run/sshd` directory created on boot
- Created via tmpfiles.d: `d /run/sshd 0755 root root -`
- File: `/usr/lib/tmpfiles.d/sshd.conf`
- Used by sshd for privilege separation socket
- Permissions: 0755 (root:root)

**Requirement**: SSH keys regenerated on installed systems
- Live ISO: Uses shared pre-generated keys (acceptable, ISO is public)
- Installed system: Should regenerate to avoid sharing keys across installations
- Generated by: `sshd-keygen.target` and `sshd-keygen@.service`
- Happens automatically on first boot if keys are missing

---

## 4. Privilege Escalation

### sudo

**Requirement**: sudo binary and supporting libraries
- From `sudo` package
- `/usr/bin/sudo` (setuid bit set)
- Libraries: `sudo-libs` (multiple .so files)

**Requirement**: `/etc/sudoers` configuration
- Permissions: **0440** (read-only for root, group sudoers)
- **Never world-readable**
- Always edit with `visudo` (validates syntax, prevents lockout)

**Requirement**: wheel group NOPASSWD access
```
%wheel ALL=(ALL:ALL) NOPASSWD: ALL
```
- Members of wheel group can sudo without password
- Suitable for live ISO and power users
- Can be tightened post-installation

**Requirement**: sudo.conf for plugin configuration
- Usually minimal/empty
- Allows advanced sudo configurations (LDAP, Kerberos, etc.)

### su / runuser

**Requirement**: su binary for switching users
- From `util-linux` or `shadow-utils`
- Prompts for target user's password
- Uses PAM `su` service

**Requirement**: runuser / runuser-l for rootless user switching
- Allows root to switch to other users without password
- runuser-l runs login shell
- Uses PAM `runuser` service

---

## 5. Password Management

### Binaries Required

**Requirement**: passwd - change user password
- From `shadow-utils`
- Updates `/etc/shadow` with new password hash
- Calls PAM `passwd` service
- CRITICAL: Requires `/usr/sbin/unix_chkpwd` for PAM (hardcoded path in pam_unix.so)

**Requirement**: chpasswd - batch password changes
- From `shadow-utils`
- Read from stdin: `username:password` (one per line)
- Useful for installation scripts
- Calls PAM `chpasswd` service

**Requirement**: chage - password expiry management
- From `shadow-utils`
- View/set password aging info
- Change minimum/maximum age, warning period

**Requirement**: faillock - account lockout management
- From `linux-pam`
- View failed login attempts
- Unlock account or reset counter
- Works with pam_faillock

### Shadow Suite

**Requirement**: useradd, usermod, userdel
- From `shadow-utils`
- Add/modify/delete user accounts
- Set UID, GID, home directory, shell, groups
- PAM services: useradd, usermod, userdel

**Requirement**: groupadd, groupmod, groupdel
- From `shadow-utils`
- Add/modify/delete groups
- Set GID, manage group membership
- PAM services: groupadd, groupmod, groupdel

**Requirement**: visudo - safe sudoers editing
- From `sudo`
- Validates sudoers syntax before writing
- Prevents accidental lockout
- Always use instead of direct editing

---

## 6. OverlayFS Three-Layer Architecture

### How Live Boot Works

The LevitateOS live ISO uses **OverlayFS** to merge three layers:

```
┌────────────────────────────────────────────────┐
│ Layer 3 (Top):    tmpfs (/overlay/upper)       │ read-write, ephemeral
│                   [runtime writes]             │
├────────────────────────────────────────────────┤
│ Layer 2 (Middle): /live/overlay from ISO       │ read-only, live-specific
│                   [empty root password,        │
│                    autologin services]         │
├────────────────────────────────────────────────┤
│ Layer 1 (Bottom): EROFS (/rootfs)              │ read-only, base system
│                   [locked root password,       │
│                    standard getty services]    │
└────────────────────────────────────────────────┘
```

**OverlayFS Mount Command** (from init_tiny.template):
```bash
mount -t overlay overlay \
  -o lowerdir=/live-overlay:/rootfs,upperdir=/overlay/upper,workdir=/overlay/work \
  /newroot
```

**Result**: Files override bottom-up
- If file exists in `/live/overlay`: use that version
- Else if exists in `/rootfs` (EROFS): use that version
- Else: doesn't exist in merged filesystem

### Live Overlay Creation

**File**: `leviso/src/component/custom/live.rs`
**Function**: `create_live_overlay_at()` creates `output/live-overlay/` directory

Files created:
1. `/etc/systemd/system/console-autologin.service`
2. `/etc/systemd/system/serial-console.service`
3. Symlinks in `getty.target.wants/` and `multi-user.target.wants/`
4. `/etc/shadow` with empty root password
5. `/etc/profile.d/*.sh` scripts (test mode, docs launcher)

### Applied by reciso (ISO builder)

1. `reciso` copies `/output/live-overlay/` to `/live/overlay` on ISO
2. init_tiny bind-mounts `/mnt/live/overlay` to `/live-overlay` (read-only from ISO)
3. OverlayFS merges it into rootfs

### Kernel Requirements

**CONFIG_EROFS_FS=y** - EROFS filesystem support
- Compressed read-only filesystem (base system)
- Immutable, space-efficient

**CONFIG_OVERLAY_FS=y** - OverlayFS support
- Merges multiple layers into single mount
- Enables live system with base+overlay

**CONFIG_BLK_DEV_LOOP=y** - Loop device support
- Allows mounting EROFS image from ISO file
- Without this: can't load compressed rootfs

---

## 7. Live ISO Behavior vs. Installed Systems

### Live ISO (`init_tiny`)

1. **Boot Flow**:
   - Kernel boots init_tiny (tiny initramfs)
   - init_tiny creates OverlayFS layers:
     - EROFS: Read-only base system
     - /live/overlay: Read-only live-specific files (from ISO)
     - tmpfs: Read-write upper layer for runtime changes
   - Pivots to merged rootfs

2. **Shadow Database**:
   - `/live/overlay/etc/shadow` has empty root password
   - **Single field**: `root::19000:0:99999:7:::`
   - Empty password hash allows passwordless login

3. **Services**:
   - `console-autologin.service` on tty1 (conflicts with getty)
   - `serial-console.service` on ttyS0 (for QEMU)
   - Provides immediate root shell without password prompt

4. **Why overlay is temporary**:
   - OverlayFS uses tmpfs for upper layer
   - All changes lost on reboot
   - Live system returns to original state
   - Designed for live testing, not persistence

### Installed Systems (via recstrap)

1. **Installation Flow**:
   - User boots live ISO
   - User runs `recstrap /mnt` to extract rootfs to partition
   - recstrap extracts **EROFS only** (not the live-overlay)
   - No OverlayFS on installed system (uses systemd-based initramfs)

2. **Shadow Database**:
   - `/etc/shadow` from EROFS has **locked root**: `root:!:19000:0:99999:7:::`
   - `!` in password field = account locked
   - Cannot login as root without password

3. **Services**:
   - Normal getty services (getty@tty1, serial-getty@ttyS0)
   - No autologin
   - Normal login prompt

4. **Why installed systems don't get live overlay**:
   - Live overlay exists **on ISO filesystem**, not in EROFS image
   - recstrap extracts from EROFS squashfs image
   - The ISO file is not part of the extracted system
   - Therefore: live overlay doesn't exist in installed system

---

## 8. Root Password on Installed Systems

### Current Situation

**Problem**: Root account is locked on installed systems
- EROFS base has: `root:!:...` (locked)
- Live overlay is not available post-installation
- Users cannot login as root without additional steps

### Why This Design

**Philosophy**: Encourage user account creation
- Desktop systems shouldn't require root login
- Users should use sudo for admin tasks
- Prevents accidental root damage

**Reference**: Arch Linux uses similar approach
- Live ISO: passwordless root (temporary, for installation)
- Installed system: root locked (users run `sudo` or set password manually)

### Solution Options (Pick One)

**Option A: Prompt during installation (Recommended)**
- Modify `recstrap` to prompt: "Create initial user account? (y/n)"
- If yes: prompt for username, password, shell, groups
- Default groups: wheel, audio, video, input
- Advantage: User-driven, matches Arch philosophy
- Disadvantage: Requires recstrap modification

**Option B: Unlock root during installation**
- Modify `recstrap` to prompt: "Set root password: "
- Hash password and update `/etc/shadow`
- Advantage: User can login immediately
- Disadvantage: Encourages root usage (less secure)

**Option C: Post-install script**
- Create `/root/create-first-user.sh` in rootfs
- Script prompts for user creation on first login
- User runs script manually: `sudo /root/create-first-user.sh`
- Advantage: Simple to implement
- Disadvantage: User must remember to run it

**Option D: Documentation only**
- Document in installation guide
- Provide `recstrap` commands for user creation and root password setting
- Advantage: No code changes needed
- Disadvantage: Users might miss instructions

### Recommendation

**Use Option A + Documentation**: Prompt during recstrap + document post-install steps
- Installation is interactive anyway (disk selection, partition confirmation)
- Adding user creation prompt is natural extension
- Document: "Use `sudo` for admin tasks, or set root password: `sudo passwd root`"

---

## 9. Name Service Switch (NSS)

### /etc/nsswitch.conf

**Requirement**: `passwd: files systemd`
- Check `/etc/passwd` (local file)
- Then check systemd user management database
- Allows both local accounts and systemd-managed accounts

**Requirement**: `group: files systemd`
- Check `/etc/group` (local file)
- Then systemd group database

**Requirement**: `shadow: files systemd`
- Check `/etc/shadow` (local file)
- Then systemd shadow database

**Requirement**: `hosts: files dns`
- Check `/etc/hosts` (local file)
- Then DNS (resolv.conf)
- Allows local hostname overrides

**Requirement**: `networks, protocols, services: files`
- Local-only (no network lookups needed for these)
- Files: `/etc/networks`, `/etc/protocols`, `/etc/services`

---

## 10. User Database Files

### Permissions (CRITICAL)

**Requirement**: `/etc/shadow` and `/etc/gshadow`
- Mode: **0600** (readable ONLY by root)
- Never world-readable
- Contains password hashes (secret)

**Requirement**: `/etc/passwd` and `/etc/group`
- Mode: **0644** (world-readable)
- Contains user/group names and metadata (non-secret)
- Needed by processes to map UID to username

### Content Requirements

**Requirement**: System users (UID < 1000)
- `root:x:0:0:root:/root:/bin/bash`
- `bin:x:1:1:bin:/bin:/usr/sbin/nologin`
- `daemon:x:2:2:daemon:/sbin:/usr/sbin/nologin`
- Many system users for various services

**Requirement**: Regular users (UID >= 1000)
- Added by user during/after installation
- Default shell: `/bin/bash`
- Home directory: `/home/username`

**Requirement**: Password aging fields
- Minimum age: 0 (can change immediately after password set)
- Maximum age: 99999 (effectively no expiry)
- Warning days: 7 (warn 7 days before expiry)
- Inactivity days: empty (never lock inactive accounts)

**Requirement**: Locked accounts
- By default, system accounts are locked: `*` in password field
- User accounts added during installation should be unlocked
- Root account is locked on installed systems: `!`

### /etc/login.defs

**Requirement**: Default password expiry settings
```
PASS_MAX_DAYS   99999  # 274 years (effectively no expiry)
PASS_MIN_DAYS   0      # Can change immediately
PASS_WARN_AGE   7      # Warn 7 days before expiry
```

**Requirement**: UID/GID ranges
```
UID_MIN          1000   # Regular users start at 1000
UID_MAX        60000   # Regular users up to 60000
GID_MIN          1000
GID_MAX        60000
SYSLOG_SU_ENAB      yes  # Log su attempts
SYSLOG_SULOG_FILE  /var/log/faillog  # Log failed logins
```

---

## 11. Security Policies

### /etc/security/limits.conf

**Requirement**: Resource limits
```
# Core dumps (disabled for security, can enable for debugging)
* soft core 0
* hard core 0

# File descriptors (for busy systems)
* soft nofile 1024
* hard nofile 2048

# Process limits
* soft nproc 4096
* hard nproc 4096
```

### /etc/security/access.conf

**Requirement**: Login access control
```
+ : root : LOCAL
+ : ALL : ALL
```
- Allow root from local terminals
- Allow everyone else from everywhere

### /etc/security/namespace.conf

**Requirement**: Polyinstantiation (empty by default)
- Can be configured for per-user /tmp directories
- Currently no special configuration needed

### /etc/security/pwquality.conf

**Requirement**: Password quality rules
```
minlen = 12            # Minimum 12 characters
minclass = 3           # 3 different character classes
dcredit = -1           # At least 1 digit
ucredit = -1           # At least 1 uppercase
maxrepeat = 3          # No more than 3 repeated chars
```

### /etc/security/faillock.conf

**Requirement**: Account lockout policy
```
silent              # Don't print failed attempt info
dir = /var/run/faillock  # Where to store failure counts
deny = 3            # Deny after 3 failed attempts
unlock_time = 900   # Unlock after 15 minutes
root_unlock_time = 60  # Unlock root faster
```

---

## 12. Verification Checklist

### Static Verification (fsdbg)

- ✅ All authentication binaries present (login, passwd, sudo, etc.)
- ✅ All PAM modules present in `/usr/lib64/security/`
- ✅ All PAM config files present in `/etc/pam.d/`
- ✅ SSH host key files exist and have correct permissions (0600 private, 0644 public)
- ✅ `/etc/shadow` and `/etc/gshadow` have 0600 permissions
- ✅ `/etc/passwd` and `/etc/group` have 0644 permissions
- ✅ Critical symlink: `/usr/bin/login` -> `/usr/sbin/login`
- ✅ No world-writable files in `/etc` (except `/etc/passwd`, `/etc/group`)
- ✅ sudoers file has 0440 permissions, not world-readable
- ✅ `/usr/sbin/unix_chkpwd` exists (required by pam_unix.so)

### Runtime Verification (rootfs-tests)

- ✅ System boots to login prompt
- ✅ Root login works on live ISO (empty password)
- ✅ Normal getty prompts appear on console
- ✅ SSH keys exist and sshd starts
- ✅ sudo works for wheel group (passwordless)
- ✅ PAM authentication flow completes
- ✅ machine-id is generated and unique
- ✅ No errors in systemd-logind journal
- ✅ /run/sshd directory created by tmpfiles.d

---

## 13. Key Dependencies and Relationships

### Critical Path for Login to Work

```
1. Kernel boots
2. init_tiny loads EROFS + live-overlay + tmpfs via OverlayFS
3. systemd starts getty@tty1
4. getty calls agetty
5. agetty searches for /usr/bin/login (symlink required!)
6. agetty calls /usr/bin/login (resolves to /usr/sbin/login)
7. login prompts for password
8. login calls PAM login service
9. PAM calls pam_unix.so
10. pam_unix.so verifies password against /etc/shadow
11. pam_unix.so calls /usr/sbin/unix_chkpwd (hardcoded path!)
12. Authentication succeeds or fails
13. If success: login sets up environment and starts shell
```

**Critical Single Points of Failure**:
- `/usr/bin/login` symlink (if missing: no login prompt)
- `/usr/sbin/unix_chkpwd` (if missing: password verification fails silently)
- `/etc/shadow` with correct permissions (if writable: security hole)
- PAM `login` service (if malformed: login hangs or fails)
- `/etc/passwd` entries for system users (if missing: shell fails to start)

### Live vs. Installed Divergence

**Live ISO Only** (from `/live/overlay`):
- Empty root password in `/etc/shadow`
- console-autologin.service
- serial-console.service

**Installed System** (from EROFS):
- Locked root password in `/etc/shadow`
- Standard getty services
- sshd-keygen.target (regenerates keys on first boot)

**Both**:
- PAM configs
- Login binary + symlink
- User/group databases (base set)
- SSH server (with pre-generated keys)

---

## 14. Implementation Notes

### Code Organization

All authentication constants and documentation live in:
```
distro-spec/src/shared/auth/
├── mod.rs              # Public API
├── components.rs       # Component lists (binaries, modules, configs)
├── requirements.md     # This document
├── pam.rs             # PAM config constants
├── getty.rs           # Getty/console configuration
├── ssh.rs             # SSH configuration
└── users.rs           # User/group specs (re-export from ../users.rs)
```

### Build Logic Stays in leviso

**Keep in leviso, DON'T move to distro-spec**:
- `pam.rs` creation functions (`create_pam_files()`, etc.)
- Live overlay creation logic (`create_live_overlay()`)
- SSH key generation logic
- User/group database manipulation

**Move to distro-spec (constants only)**:
- PAM file contents (include_str!() macros)
- Component lists (AUTH_BIN, AUTH_SBIN, PAM_MODULES, etc.)
- Getty configuration constants
- SSH configuration templates

### Principle

**distro-spec** = data (what needs to be there)
**leviso** = logic (how to build it)
**testing** = verification (does it work)

---

## 15. References

- TEAM_108: Review of login architecture and /usr/bin/login symlink requirement
- TEAM_127: Visual install testing with noVNC
- TEAM_143: Udev subsystem consolidation (model for this work)
- init_tiny.template: OverlayFS mount code
- Arch Linux installer: User creation prompt pattern
- Shadow-utils documentation: passwd/shadow format
- Linux PAM documentation: Module and service configuration

