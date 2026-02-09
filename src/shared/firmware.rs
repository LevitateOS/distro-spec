//! Firmware specifications.
//!
//! Defines which firmware directories to copy for WiFi support.

/// WiFi firmware directories to copy.
///
/// These cover the most common WiFi chipsets:
/// - Intel: iwlwifi (most common on laptops)
/// - Realtek: rtlwifi, rtw88, rtw89
/// - Atheros: ath9k, ath10k, ath11k
/// - MediaTek: mediatek (mt76xx)
/// - Broadcom: brcm
pub const WIFI_FIRMWARE_DIRS: &[&str] = &[
    // Intel
    "iwlwifi",
    // Realtek
    "rtlwifi",
    "rtl_bt",
    "rtl_nic",
    "rtw88",
    "rtw89",
    // Atheros
    "ath9k_htc",
    "ath10k",
    "ath11k",
    // MediaTek
    "mediatek",
    // Broadcom
    "brcm",
    // Marvell
    "mrvl",
    // Ralink
    "rt2870.bin",
    "rt3070.bin",
    "rt3290.bin",
];
