//! SSH server configuration constants.
//!
//! Contains default sshd configuration settings and runtime requirements.

/// Default sshd_config settings.
///
/// These are the base configuration settings for the SSH server.
/// Most are defaults that come with openssh-server, but are documented here
/// for clarity about the authentication policy.
pub const SSHD_CONFIG_SETTINGS: &[(&str, &str)] = &[
    // Port configuration
    ("Port", "22"),
    // Access control
    ("PermitRootLogin", "yes"), // Allow root login (can be tightened post-install)
    ("PasswordAuthentication", "yes"), // Allow password auth (convenient for live ISO)
    ("PubkeyAuthentication", "yes"), // Allow public key auth (secure)
    // Protocol
    ("Protocol", "2"),        // SSH protocol version 2 only
    ("AddressFamily", "any"), // Listen on IPv4 and IPv6
    // Session
    ("X11Forwarding", "no"), // Don't allow X11 forwarding (security)
    ("PrintMotD", "yes"),    // Display message of the day
    // Security
    ("PermitEmptyPasswords", "no"), // Never allow empty passwords
    ("ClientAliveInterval", "300"), // Send keepalive every 5 minutes
    ("ClientAliveCountMax", "2"),   // Close after 2 missed keepalives
];

/// tmpfiles.d configuration for SSH runtime directory.
///
/// The sshd daemon needs /run/sshd directory for privilege separation
/// (unprivileged subprocess communication). Since /run is typically tmpfs,
/// we need to ensure the directory exists on every boot.
pub const SSHD_TMPFILES_CONFIG: &str = "\
# /run/sshd is needed by sshd for privilege separation
d /run/sshd 0755 root root -
";

/// SSH host key generation configuration.
///
/// Pre-generated host keys allow sshd to start immediately without
/// waiting for key generation via sshd-keygen@.service.
#[derive(Debug, Clone)]
pub struct HostKeyConfig {
    /// Key type (rsa, ecdsa, ed25519)
    pub key_type: &'static str,
    /// Bits for RSA/ECDSA (0 for fixed-size like ed25519)
    pub bits: u32,
    /// Minimum recommended size for this key type
    pub min_recommended_bits: u32,
}

/// Pre-generated SSH host key specifications.
///
/// These define all three host key types that modern SSH should have:
/// - RSA: Widely compatible, but large keys
/// - ECDSA: Elliptic Curve, smaller keys
/// - Ed25519: Modern, best for new systems
///
/// For live ISO: shared keys are acceptable (public system, read-only)
/// For installed systems: keys should be regenerated on first boot
pub const HOST_KEY_CONFIGS: &[HostKeyConfig] = &[
    HostKeyConfig {
        key_type: "rsa",
        bits: 3072,
        min_recommended_bits: 2048, // We use 3072 for better security
    },
    HostKeyConfig {
        key_type: "ecdsa",
        bits: 256,
        min_recommended_bits: 256, // Fixed for P-256 curve
    },
    HostKeyConfig {
        key_type: "ed25519",
        bits: 0,                 // Fixed size, no bits parameter
        min_recommended_bits: 0, // N/A
    },
];

/// SSH key types in order of preference.
///
/// Modern SSH prefers keys in this order.
pub const SSH_KEY_TYPES_PREFERRED: &[&str] = &[
    "ssh-ed25519", // Modern, best choice
    "ecdsa-sha2-nistp256",
    "ssh-rsa", // Widely compatible but less secure
];
