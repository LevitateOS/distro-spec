//! Udev facts about the rootfs.

/// Udev helper programs needed for device identification.
///
/// These binaries are invoked by udev rules to probe device attributes.
/// Located in `/usr/lib/udev/`.
pub const UDEV_HELPERS: &[&str] = &[
    "ata_id",        // ATA device identification (SATA drives)
    "scsi_id",       // SCSI device identification
    "cdrom_id",      // CD/DVD detection
    "v4l_id",        // Video4Linux identification
    "dmi_memory_id", // DMI memory identification
    "mtd_probe",     // MTD (Memory Technology Device) probe
];
