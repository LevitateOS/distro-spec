//! Getty (console) configuration for login services.
//!
//! This module contains getty/agetty configuration constants used in the
//! serial-getty@.service override. The critical `-L` flag for QEMU is
//! documented here.

/// Serial getty override for QEMU compatibility.
///
/// # The Problem
///
/// Standard `serial-getty@ttyS0.service` uses this ExecStart line:
/// ```text
/// ExecStart=-/sbin/agetty -o '-p -- \u' 115200,57600,38400,9600 ttyS0 $TERM
/// ```
///
/// QEMU's emulated serial port doesn't generate proper modem signals (CD - Carrier Detect).
/// Without the `-L` flag, agetty waits for CD signal, causing serial console to hang.
///
/// # The Solution
///
/// The `-L` flag tells agetty to ignore modem signals and treat the line as always active:
/// ```text
/// ExecStart=-/sbin/agetty -L -o '-p -- \u' 115200,57600,38400,9600 ttyS0 $TERM
/// ```
///
/// # Implementation
///
/// In `leviso/src/component/definitions.rs`, when defining `serial-getty` component:
/// 1. Import standard serial-getty from systemd
/// 2. Create override directory
/// 3. Replace the `-` placeholder with `-L` in the baud rate string
///
/// # References
///
/// - agetty man page: `-L --skip-login` options
/// - QEMU serial console documentation
/// - TEAM_108: Root cause analysis of getty issues
pub const SERIAL_GETTY_OVERRIDE: &str = "\
# Override for QEMU serial console compatibility
[Service]
ExecStart=
ExecStart=-/sbin/agetty -L -o '-p -- \\u' 115200,57600,38400,9600 ttyS0 $TERM
";

/// Baud rate configuration for serial consoles.
///
/// Multiple rates allow connection from systems with different serial configurations.
/// agetty negotiates the highest common rate.
pub const SERIAL_BAUD_RATES: &str = "115200,57600,38400,9600";

/// Terminal type for getty services.
///
/// VT102 is the standard for Linux systems, providing support for:
/// - Arrow keys (up/down/left/right)
/// - Function keys (F1-F12)
/// - Insert/Delete/Home/End
/// - Page up/down
pub const GETTY_TERM_TYPE: &str = "vt102";

/// Console login service unit name.
///
/// Provides autologin on console (live ISO only).
/// Used in live-overlay for passwordless login.
pub const LIVE_CONSOLE_AUTOLOGIN_SERVICE: &str = "console-autologin.service";

/// Serial console service unit name.
///
/// Provides autologin on serial port (for QEMU testing).
/// Used in live-overlay for passwordless serial login.
pub const LIVE_SERIAL_CONSOLE_SERVICE: &str = "serial-console.service";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_getty_override_contains_l_flag() {
        // Verify the critical -L flag is present in QEMU override
        assert!(SERIAL_GETTY_OVERRIDE.contains("-L"), "Serial getty must include -L flag for QEMU");
    }

    #[test]
    fn test_baud_rates_configured() {
        // Verify standard baud rates are defined
        assert!(SERIAL_BAUD_RATES.contains("115200"), "Primary baud rate 115200 required");
        assert!(SERIAL_BAUD_RATES.contains("57600"), "Fallback baud rate 57600 required");
    }
}
