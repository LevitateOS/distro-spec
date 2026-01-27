//! License mappings for redistributed binaries and libraries.
//!
//! Maps binaries and libraries to their source package names for license compliance.
//! When we copy a binary or library, we need to also copy the corresponding
//! license files from `/usr/share/licenses/<package>/`.

/// Binary name → package name mapping.
///
/// Used to determine which license directory to copy when a binary is included.
pub const BINARY_TO_PACKAGE: &[(&str, &str)] = &[
    // === COREUTILS ===
    ("ls", "coreutils-common"),
    ("cat", "coreutils-common"),
    ("cp", "coreutils-common"),
    ("mv", "coreutils-common"),
    ("rm", "coreutils-common"),
    ("mkdir", "coreutils-common"),
    ("rmdir", "coreutils-common"),
    ("touch", "coreutils-common"),
    ("chmod", "coreutils-common"),
    ("chown", "coreutils-common"),
    ("chgrp", "coreutils-common"),
    ("ln", "coreutils-common"),
    ("readlink", "coreutils-common"),
    ("realpath", "coreutils-common"),
    ("stat", "coreutils-common"),
    ("mknod", "coreutils-common"),
    ("mkfifo", "coreutils-common"),
    ("timeout", "coreutils-common"),
    ("sleep", "coreutils-common"),
    ("true", "coreutils-common"),
    ("false", "coreutils-common"),
    ("test", "coreutils-common"),
    ("[", "coreutils-common"),
    ("echo", "coreutils-common"),
    ("head", "coreutils-common"),
    ("tail", "coreutils-common"),
    ("wc", "coreutils-common"),
    ("sort", "coreutils-common"),
    ("cut", "coreutils-common"),
    ("tr", "coreutils-common"),
    ("tee", "coreutils-common"),
    ("printf", "coreutils-common"),
    ("uniq", "coreutils-common"),
    ("seq", "coreutils-common"),
    ("pwd", "coreutils-common"),
    ("uname", "coreutils-common"),
    ("date", "coreutils-common"),
    ("env", "coreutils-common"),
    ("id", "coreutils-common"),
    ("hostname", "coreutils-common"),
    ("printenv", "coreutils-common"),
    ("whoami", "coreutils-common"),
    ("groups", "coreutils-common"),
    ("kill", "coreutils-common"),
    ("nice", "coreutils-common"),
    ("nohup", "coreutils-common"),
    ("expr", "coreutils-common"),
    ("yes", "coreutils-common"),
    ("mktemp", "coreutils-common"),
    ("df", "coreutils-common"),
    ("du", "coreutils-common"),
    ("sync", "coreutils-common"),
    ("dirname", "coreutils-common"),
    ("basename", "coreutils-common"),
    ("base64", "coreutils-common"),
    ("md5sum", "coreutils-common"),
    ("sha256sum", "coreutils-common"),
    ("sha512sum", "coreutils-common"),
    // === BASH ===
    ("bash", "bash"),
    // === SED ===
    ("sed", "sed"),
    // === GAWK ===
    ("awk", "gawk"),
    ("gawk", "gawk"),
    // === GREP ===
    ("grep", "grep"),
    // === FINDUTILS ===
    ("find", "findutils"),
    ("xargs", "findutils"),
    // === DIFFUTILS ===
    ("diff", "diffutils"),
    ("cmp", "diffutils"),
    // === FILE ===
    ("file", "file"),
    // === WHICH ===
    ("which", "which"),
    // === PROCPS-NG ===
    ("ps", "procps-ng"),
    ("pgrep", "procps-ng"),
    ("pkill", "procps-ng"),
    ("top", "procps-ng"),
    ("free", "procps-ng"),
    ("uptime", "procps-ng"),
    ("w", "procps-ng"),
    ("vmstat", "procps-ng"),
    ("watch", "procps-ng"),
    ("sysctl", "procps-ng"),
    // === SYSTEMD ===
    ("systemctl", "systemd"),
    ("journalctl", "systemd"),
    ("timedatectl", "systemd"),
    ("hostnamectl", "systemd"),
    ("localectl", "systemd"),
    ("loginctl", "systemd"),
    ("bootctl", "systemd"),
    ("udevadm", "systemd"),
    ("systemd", "systemd"),
    ("systemd-executor", "systemd"),
    ("systemd-shutdown", "systemd"),
    ("systemd-sulogin-shell", "systemd"),
    ("systemd-cgroups-agent", "systemd"),
    ("systemd-journald", "systemd"),
    ("systemd-modules-load", "systemd"),
    ("systemd-sysctl", "systemd"),
    ("systemd-tmpfiles", "systemd"),
    ("systemd-timedated", "systemd"),
    ("systemd-hostnamed", "systemd"),
    ("systemd-localed", "systemd"),
    ("systemd-logind", "systemd"),
    ("systemd-networkd", "systemd"),
    ("systemd-resolved", "systemd"),
    ("systemd-udevd", "systemd"),
    ("systemd-fsck", "systemd"),
    // Udev helpers (part of systemd)
    ("ata_id", "systemd"),
    ("scsi_id", "systemd"),
    ("cdrom_id", "systemd"),
    ("v4l_id", "systemd"),
    ("dmi_memory_id", "systemd"),
    ("mtd_probe", "systemd"),
    ("systemd-remount-fs", "systemd"),
    ("systemd-vconsole-setup", "systemd"),
    ("systemd-random-seed", "systemd"),
    // === VIM ===
    ("vi", "vim-data"),
    ("vim", "vim-data"),
    // === NANO ===
    ("nano", "nano"),
    // === LESS ===
    ("less", "less"),
    ("more", "util-linux"),
    // === UTIL-LINUX ===
    ("mount", "util-linux"),
    ("umount", "util-linux"),
    ("lsblk", "util-linux"),
    ("findmnt", "util-linux"),
    ("flock", "util-linux"),
    ("getopt", "util-linux"),
    ("setsid", "util-linux"),
    ("dmesg", "util-linux"),
    ("fsck", "util-linux"),
    ("blkid", "util-linux"),
    ("losetup", "util-linux"),
    ("mkswap", "util-linux"),
    ("swapon", "util-linux"),
    ("swapoff", "util-linux"),
    ("fdisk", "util-linux"),
    ("sfdisk", "util-linux"),
    ("wipefs", "util-linux"),
    ("blockdev", "util-linux"),
    ("pivot_root", "util-linux"),
    ("chroot", "util-linux"),
    ("switch_root", "util-linux"),
    ("agetty", "util-linux"),
    ("login", "util-linux"),
    ("sulogin", "util-linux"),
    ("nologin", "util-linux"),
    ("hwclock", "util-linux"),
    ("hexdump", "util-linux"),
    // === PARTED ===
    ("parted", "parted"),
    // === E2FSPROGS ===
    ("fsck.ext4", "e2fsprogs-libs"),
    ("fsck.ext2", "e2fsprogs-libs"),
    ("fsck.ext3", "e2fsprogs-libs"),
    ("e2fsck", "e2fsprogs-libs"),
    ("mke2fs", "e2fsprogs-libs"),
    ("mkfs.ext4", "e2fsprogs-libs"),
    ("mkfs.ext2", "e2fsprogs-libs"),
    ("mkfs.ext3", "e2fsprogs-libs"),
    ("tune2fs", "e2fsprogs-libs"),
    ("resize2fs", "e2fsprogs-libs"),
    // === DOSFSTOOLS ===
    ("mkfs.fat", "dosfstools"),
    ("mkfs.vfat", "dosfstools"),
    ("fsck.fat", "dosfstools"),
    ("fsck.vfat", "dosfstools"),
    // === BTRFS-PROGS ===
    ("btrfs", "btrfs-progs"),
    ("btrfsck", "btrfs-progs"),
    ("mkfs.btrfs", "btrfs-progs"),
    ("btrfs-convert", "btrfs-progs"),
    ("btrfs-find-root", "btrfs-progs"),
    ("btrfs-image", "btrfs-progs"),
    ("btrfs-map-logical", "btrfs-progs"),
    ("btrfs-select-super", "btrfs-progs"),
    // === NTFS-3G ===
    ("mkfs.ntfs", "ntfs-3g"),
    ("ntfsresize", "ntfs-3g"),
    ("ntfsclone", "ntfs-3g"),
    ("ntfscp", "ntfs-3g"),
    ("ntfslabel", "ntfs-3g"),
    ("ntfsfix", "ntfs-3g"),
    ("ntfscat", "ntfs-3g"),
    ("ntfscluster", "ntfs-3g"),
    ("ntfscmp", "ntfs-3g"),
    ("ntfsfallocate", "ntfs-3g"),
    ("ntfsinfo", "ntfs-3g"),
    ("ntfsls", "ntfs-3g"),
    ("ntfsmove", "ntfs-3g"),
    ("ntfsrecover", "ntfs-3g"),
    ("ntfssecaudit", "ntfs-3g"),
    ("ntfstruncate", "ntfs-3g"),
    ("ntfsusermap", "ntfs-3g"),
    ("ntfswipe", "ntfs-3g"),
    // === XFS ===
    ("mkfs.xfs", "xfsprogs"),
    ("xfs_repair", "xfsprogs"),
    ("xfs_admin", "xfsprogs"),
    ("xfs_copy", "xfsprogs"),
    ("xfs_db", "xfsprogs"),
    ("xfs_freeze", "xfsprogs"),
    ("xfs_growfs", "xfsprogs"),
    ("xfs_info", "xfsprogs"),
    ("xfs_io", "xfsprogs"),
    ("xfs_logprint", "xfsprogs"),
    ("xfs_mdrestore", "xfsprogs"),
    ("xfs_metadump", "xfsprogs"),
    ("xfs_ncheck", "xfsprogs"),
    ("xfs_quota", "xfsprogs"),
    ("xfs_rtcp", "xfsprogs"),
    ("xfs_spaceman", "xfsprogs"),
    // === KMOD ===
    ("insmod", "kmod"),
    ("rmmod", "kmod"),
    ("modprobe", "kmod"),
    ("lsmod", "kmod"),
    ("depmod", "kmod"),
    ("modinfo", "kmod"),
    // === SHADOW-UTILS ===
    ("useradd", "shadow-utils"),
    ("userdel", "shadow-utils"),
    ("usermod", "shadow-utils"),
    ("groupadd", "shadow-utils"),
    ("groupdel", "shadow-utils"),
    ("groupmod", "shadow-utils"),
    ("chpasswd", "shadow-utils"),
    ("passwd", "shadow-utils"),
    ("faillock", "shadow-utils"),
    ("chage", "shadow-utils"),
    ("newusers", "shadow-utils"),
    ("chgpasswd", "shadow-utils"),
    ("pwck", "shadow-utils"),
    ("grpck", "shadow-utils"),
    ("vipw", "shadow-utils"),
    ("vigr", "shadow-utils"),
    ("pwconv", "shadow-utils"),
    ("pwunconv", "shadow-utils"),
    ("grpconv", "shadow-utils"),
    ("grpunconv", "shadow-utils"),
    // === IPROUTE ===
    ("ip", "iproute"),
    ("ss", "iproute"),
    ("bridge", "iproute"),
    // === NET-TOOLS ===
    ("ifconfig", "net-tools"),
    ("route", "net-tools"),
    // === SUDO ===
    ("su", "util-linux"),
    ("sudo", "sudo"),
    ("sudoedit", "sudo"),
    ("sudoreplay", "sudo"),
    ("visudo", "sudo"),
    // === OPENSSH ===
    ("ssh", "openssh"),
    ("scp", "openssh"),
    ("sftp", "openssh"),
    ("ssh-keygen", "openssh"),
    ("ssh-add", "openssh"),
    ("ssh-agent", "openssh"),
    ("sshd", "openssh-server"),
    // === NETWORKMANAGER ===
    ("nmcli", "NetworkManager"),
    ("nm-online", "NetworkManager"),
    ("nmtui", "NetworkManager"),
    ("NetworkManager", "NetworkManager"),
    // === WPA_SUPPLICANT ===
    ("wpa_supplicant", "wpa_supplicant"),
    ("wpa_cli", "wpa_supplicant"),
    ("wpa_passphrase", "wpa_supplicant"),
    // === DBUS-BROKER ===
    ("dbus-broker", "dbus-broker"),
    ("dbus-broker-launch", "dbus-broker"),
    // === CHRONY ===
    ("chronyd", "chrony"),
    // === GZIP ===
    ("gzip", "gzip"),
    ("gunzip", "gzip"),
    // === XZ ===
    ("xz", "xz"),
    ("unxz", "xz"),
    // === TAR ===
    ("tar", "tar"),
    // === BZIP2 ===
    ("bzip2", "bzip2"),
    ("bunzip2", "bzip2"),
    // === CPIO ===
    ("cpio", "cpio"),
    // === GLIBC ===
    ("getent", "glibc"),
    ("ldd", "glibc"),
    ("ldconfig", "glibc"),
    ("localedef", "glibc"),
    // === PCIUTILS ===
    ("lspci", "pciutils"),
    // === USBUTILS ===
    ("lsusb", "usbutils"),
    // === IPUTILS ===
    ("ping", "iputils"),
    ("tracepath", "iputils"),
    // === CURL ===
    ("curl", "curl"),
    // === WGET ===
    ("wget", "wget"),
    // === NCURSES ===
    ("clear", "ncurses"),
    ("tty", "coreutils-common"),
    ("stty", "coreutils-common"),
    // === KBD ===
    ("loadkeys", "kbd"),
    // === EFIBOOTMGR ===
    ("efibootmgr", "efibootmgr"),
    // === SQUASHFS-TOOLS ===
    ("unsquashfs", "squashfs-tools"),
    // === CRYPTSETUP ===
    ("cryptsetup", "cryptsetup"),
    // === LVM2 ===
    ("lvm", "lvm2"),
    // === MDADM ===
    ("mdadm", "mdadm"),
    ("mdmon", "mdadm"),
    // === DMIDECODE ===
    ("dmidecode", "dmidecode"),
    // === ETHTOOL ===
    ("ethtool", "ethtool"),
    // === SMARTMONTOOLS ===
    ("smartctl", "smartmontools"),
    // === HDPARM ===
    ("hdparm", "hdparm"),
    // === NVME-CLI ===
    ("nvme", "nvme-cli"),
    // === DDRESCUE ===
    ("ddrescue", "ddrescue"),
    // === TESTDISK ===
    ("testdisk", "testdisk"),
    ("photorec", "testdisk"),
    // === TMUX ===
    ("tmux", "tmux"),
    // === SCREEN ===
    ("screen", "screen"),
    // === BIND-UTILS ===
    ("dig", "bind-utils"),
    ("nslookup", "bind-utils"),
    // === BINUTILS ===
    ("strings", "binutils"),
    // === RSYNC ===
    ("rsync", "rsync"),
    // === MAN-DB ===
    ("man", "man-db"),
    ("mandb", "man-db"),
    ("apropos", "man-db"),
    ("whatis", "man-db"),
    // === MC ===
    ("mc", "mc"),
    ("mcedit", "mc"),
    ("mcview", "mc"),
    // === PV ===
    ("pv", "pv"),
    // === LYNX ===
    ("lynx", "lynx"),
    // === NMAP ===
    ("nmap", "nmap"),
    // === ALSA-UTILS ===
    ("alsamixer", "alsa-utils"),
    ("amixer", "alsa-utils"),
    ("aplay", "alsa-utils"),
    ("arecord", "alsa-utils"),
    ("speaker-test", "alsa-utils"),
    // === GNUPG2 ===
    ("gpg", "gnupg2"),
    ("gpg2", "gnupg2"),
    ("gpgconf", "gnupg2"),
    ("gpg-agent", "gnupg2"),
    // === GIT ===
    ("git", "git-core"),
    // === PYTHON ===
    ("python3", "python3"),
    ("python", "python3"),
    // === PERL ===
    ("perl", "perl"),
    // === HTOP ===
    ("htop", "htop"),
    // === ZIP ===
    ("zip", "zip"),
    ("unzip", "unzip"),
    // === P7ZIP ===
    ("7z", "p7zip"),
    ("7za", "p7zip"),
    ("7zr", "p7zip"),
    // === TREE ===
    ("tree", "tree"),
    // === BLUEZ ===
    ("bluetoothctl", "bluez"),
    ("bluetoothd", "bluez"),
    // === PIPEWIRE ===
    ("pw-cli", "pipewire"),
    ("pw-dump", "pipewire"),
    ("pw-cat", "pipewire"),
    ("pw-play", "pipewire"),
    ("pw-record", "pipewire"),
    ("pw-top", "pipewire"),
    ("pw-metadata", "pipewire"),
    ("pw-mon", "pipewire"),
    ("pw-link", "pipewire"),
    ("pipewire", "pipewire"),
    ("pipewire-pulse", "pipewire-pulseaudio"),
    // === WIREPLUMBER ===
    ("wpctl", "wireplumber"),
    ("wireplumber", "wireplumber"),
    // === PULSEAUDIO (compatibility tools from pipewire-pulseaudio) ===
    ("pactl", "pipewire-pulseaudio"),
    ("pacmd", "pipewire-pulseaudio"),
    ("paplay", "pipewire-pulseaudio"),
    ("parecord", "pipewire-pulseaudio"),
    // === POLKIT ===
    ("pkexec", "polkit"),
    ("pkaction", "polkit"),
    ("pkcheck", "polkit"),
    ("polkitd", "polkit"),
    // === UDISKS2 ===
    ("udisksctl", "udisks2"),
    ("udisksd", "udisks2"),
    // === UPOWER ===
    ("upower", "upower"),
    ("upowerd", "upower"),
    // === LINUX-PAM ===
    ("unix_chkpwd", "pam"),
];

/// Library prefix → package name mapping.
///
/// Libraries are matched by prefix (e.g., "libc.so" matches "libc.so.6").
pub const LIB_TO_PACKAGE: &[(&str, &str)] = &[
    // === GLIBC ===
    ("libc.so", "glibc"),
    ("libpthread.so", "glibc"),
    ("libm.so", "glibc"),
    ("libdl.so", "glibc"),
    ("librt.so", "glibc"),
    ("libresolv.so", "glibc"),
    ("libnss_files.so", "glibc"),
    ("libnss_dns.so", "glibc"),
    ("libnss_compat.so", "glibc"),
    ("ld-linux-x86-64.so", "glibc"),
    ("libcrypt.so", "libxcrypt"),
    // === SYSTEMD ===
    ("libsystemd.so", "systemd-libs"),
    ("libudev.so", "systemd-libs"),
    ("libsystemd-shared", "systemd"),
    // === PAM ===
    ("libpam.so", "pam"),
    ("libpam_misc.so", "pam"),
    ("libpamc.so", "pam"),
    // === OPENSSL ===
    ("libcrypto.so", "openssl-libs"),
    ("libssl.so", "openssl-libs"),
    // === ZLIB ===
    ("libz.so", "zlib"),
    // === NCURSES ===
    ("libncurses.so", "ncurses-libs"),
    ("libncursesw.so", "ncurses-libs"),
    ("libtinfo.so", "ncurses-libs"),
    ("libform.so", "ncurses-libs"),
    ("libmenu.so", "ncurses-libs"),
    ("libpanel.so", "ncurses-libs"),
    // === READLINE ===
    ("libreadline.so", "readline"),
    ("libhistory.so", "readline"),
    // === PCRE ===
    ("libpcre.so", "pcre"),
    ("libpcre2-8.so", "pcre2"),
    ("libpcre2-16.so", "pcre2"),
    ("libpcre2-32.so", "pcre2"),
    // === SELINUX ===
    ("libselinux.so", "libselinux"),
    ("libsepol.so", "libsepol"),
    // === CAP ===
    ("libcap.so", "libcap"),
    ("libcap-ng.so", "libcap-ng"),
    // === ACL/ATTR ===
    ("libacl.so", "libacl"),
    ("libattr.so", "libattr"),
    // === BLKID/UUID ===
    ("libblkid.so", "libblkid"),
    ("libuuid.so", "libuuid"),
    ("libmount.so", "libmount"),
    ("libsmartcols.so", "libsmartcols"),
    ("libfdisk.so", "libfdisk"),
    // === EXPAT ===
    ("libexpat.so", "expat"),
    // === BZIP2 ===
    ("libbz2.so", "bzip2-libs"),
    // === XZ ===
    ("liblzma.so", "xz-libs"),
    // === ZSTD ===
    ("libzstd.so", "libzstd"),
    // === LZ4 ===
    ("liblz4.so", "lz4-libs"),
    // === CURL ===
    ("libcurl.so", "libcurl"),
    // === NGHTTP2 ===
    ("libnghttp2.so", "libnghttp2"),
    // === IDN2 ===
    ("libidn2.so", "libidn2"),
    // === PSL ===
    ("libpsl.so", "libpsl"),
    // === SSH2 ===
    ("libssh.so", "libssh"),
    ("libssh2.so", "libssh2"),
    // === KMOD ===
    ("libkmod.so", "kmod-libs"),
    // === DEVICE-MAPPER ===
    ("libdevmapper.so", "device-mapper-libs"),
    // === CRYPTSETUP ===
    ("libcryptsetup.so", "cryptsetup-libs"),
    // === JSON-C ===
    ("libjson-c.so", "json-c"),
    // === ARGON2 ===
    ("libargon2.so", "libargon2"),
    // === GMP ===
    ("libgmp.so", "gmp"),
    // === NETTLE ===
    ("libnettle.so", "nettle"),
    ("libhogweed.so", "nettle"),
    // === GNUTLS ===
    ("libgnutls.so", "gnutls"),
    // === P11-KIT ===
    ("libp11-kit.so", "p11-kit"),
    // === LIBTASN1 ===
    ("libtasn1.so", "libtasn1"),
    // === KEYUTILS ===
    ("libkeyutils.so", "keyutils-libs"),
    // === KRB5 ===
    ("libkrb5.so", "krb5-libs"),
    ("libk5crypto.so", "krb5-libs"),
    ("libkrb5support.so", "krb5-libs"),
    ("libgssapi_krb5.so", "krb5-libs"),
    ("libcom_err.so", "libcom_err"),
    // === AUDIT ===
    ("libaudit.so", "audit-libs"),
    // === DBUS ===
    ("libdbus-1.so", "dbus-libs"),
    // === GIO/GLIB ===
    ("libgio-2.0.so", "glib2"),
    ("libglib-2.0.so", "glib2"),
    ("libgobject-2.0.so", "glib2"),
    ("libgmodule-2.0.so", "glib2"),
    ("libgthread-2.0.so", "glib2"),
    // === POLKIT ===
    ("libpolkit-gobject-1.so", "polkit-libs"),
    ("libpolkit-agent-1.so", "polkit-libs"),
    // === FFI ===
    ("libffi.so", "libffi"),
    // === GCRYPT ===
    ("libgcrypt.so", "libgcrypt"),
    ("libgpg-error.so", "libgpg-error"),
    // === LVM2 ===
    ("liblvm2cmd.so", "lvm2-libs"),
    // === POPT ===
    ("libpopt.so", "popt"),
    // === LIBXML2 ===
    ("libxml2.so", "libxml2"),
    // === SQLITE ===
    ("libsqlite3.so", "sqlite-libs"),
    // === ICU ===
    ("libicui18n.so", "libicu"),
    ("libicuuc.so", "libicu"),
    ("libicudata.so", "libicu"),
    // === LIBEVENT ===
    ("libevent.so", "libevent"),
    ("libevent_core.so", "libevent"),
    // === ALSA ===
    ("libasound.so", "alsa-lib"),
    // === PIPEWIRE ===
    ("libpipewire-0.3.so", "pipewire-libs"),
    // === SPA (part of pipewire) ===
    ("libspa-0.2.so", "pipewire-libs"),
    // === BLUETOOTH ===
    ("libbluetooth.so", "bluez-libs"),
    // === PYTHON ===
    ("libpython3", "python3-libs"),
    // === PERL ===
    ("libperl.so", "perl-libs"),
    // === LIBMNL ===
    ("libmnl.so", "libmnl"),
    // === LIBNFTNL ===
    ("libnftnl.so", "libnftnl"),
    // === LIBNETFILTER ===
    ("libnetfilter_conntrack.so", "libnetfilter_conntrack"),
    // === LIBNL ===
    ("libnl-3.so", "libnl3"),
    ("libnl-genl-3.so", "libnl3"),
    ("libnl-route-3.so", "libnl3"),
    // === MM-GLIB ===
    ("libmm-glib.so", "ModemManager-glib"),
    // === LIBNM ===
    ("libnm.so", "NetworkManager-libnm"),
    // === LIBNDP ===
    ("libndp.so", "libndp"),
    // === NEWT ===
    ("libnewt.so", "newt"),
    ("libslang.so", "slang"),
    // === LIBPWQUALITY ===
    ("libpwquality.so", "libpwquality"),
    // === CRACKLIB ===
    ("libcrack.so", "cracklib"),
    // === UDISKS2 ===
    ("libudisks2.so", "udisks2-libs"),
    // === LIBATASMART ===
    ("libatasmart.so", "libatasmart"),
    // === LIBBYTESIZE ===
    ("libbytesize.so", "libbytesize"),
    // === LIBBLOCKDEV ===
    ("libblockdev.so", "libblockdev"),
    // === PARTED ===
    ("libparted.so", "parted-libs"),
    // === LIBGUDEV ===
    ("libgudev-1.0.so", "libgudev"),
    // === UPOWER ===
    ("libupower-glib.so", "upower"),
    // === LIBIMOBILEDEVICE ===
    ("libimobiledevice-1.0.so", "libimobiledevice"),
    ("libplist-2.0.so", "libplist"),
    ("libusbmuxd-2.0.so", "libusbmuxd"),
];

/// Get the package name for a binary.
///
/// Returns `None` if the binary is not in the mapping (can be added later).
pub fn package_for_binary(binary: &str) -> Option<&'static str> {
    BINARY_TO_PACKAGE
        .iter()
        .find(|(b, _)| *b == binary)
        .map(|(_, pkg)| *pkg)
}

/// Get the package name for a library.
///
/// Libraries are matched by prefix (e.g., "libc.so.6" matches "libc.so").
/// Returns `None` if the library is not in the mapping (can be added later).
pub fn package_for_library(lib: &str) -> Option<&'static str> {
    LIB_TO_PACKAGE
        .iter()
        .find(|(prefix, _)| lib.starts_with(prefix))
        .map(|(_, pkg)| *pkg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_lookup() {
        assert_eq!(package_for_binary("bash"), Some("bash"));
        assert_eq!(package_for_binary("ls"), Some("coreutils-common"));
        assert_eq!(package_for_binary("systemctl"), Some("systemd"));
        assert_eq!(package_for_binary("nonexistent"), None);
    }

    #[test]
    fn test_library_lookup() {
        assert_eq!(package_for_library("libc.so.6"), Some("glibc"));
        assert_eq!(package_for_library("libsystemd.so.0"), Some("systemd-libs"));
        assert_eq!(package_for_library("libpam.so.0"), Some("pam"));
        assert_eq!(package_for_library("libunknown.so.1"), None);
    }

    #[test]
    fn test_critical_binaries_have_packages() {
        // Ensure commonly used binaries are mapped
        let critical = ["bash", "ls", "cat", "systemctl", "mount", "ip", "passwd"];
        for bin in critical {
            assert!(
                package_for_binary(bin).is_some(),
                "Critical binary '{}' should have a package mapping",
                bin
            );
        }
    }

    #[test]
    fn test_critical_libraries_have_packages() {
        // Ensure commonly used libraries are mapped
        let critical = ["libc.so.6", "libpam.so.0", "libsystemd.so.0", "libssl.so.3"];
        for lib in critical {
            assert!(
                package_for_library(lib).is_some(),
                "Critical library '{}' should have a package mapping",
                lib
            );
        }
    }
}
