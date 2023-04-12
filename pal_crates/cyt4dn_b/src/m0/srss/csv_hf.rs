#[doc = r"Register block"]
#[repr(C)]
pub struct CSV_HF {
    #[doc = "0x00..0x0c - Active domain Clock Supervisor (CSV) registers"]
    pub csv0: CSV,
    _reserved1: [u8; 0x04],
    #[doc = "0x10..0x1c - Active domain Clock Supervisor (CSV) registers"]
    pub csv1: CSV,
    _reserved2: [u8; 0x04],
    #[doc = "0x20..0x2c - Active domain Clock Supervisor (CSV) registers"]
    pub csv2: CSV,
    _reserved3: [u8; 0x04],
    #[doc = "0x30..0x3c - Active domain Clock Supervisor (CSV) registers"]
    pub csv3: CSV,
    _reserved4: [u8; 0x04],
    #[doc = "0x40..0x4c - Active domain Clock Supervisor (CSV) registers"]
    pub csv4: CSV,
    _reserved5: [u8; 0x04],
    #[doc = "0x50..0x5c - Active domain Clock Supervisor (CSV) registers"]
    pub csv5: CSV,
    _reserved6: [u8; 0x04],
    #[doc = "0x60..0x6c - Active domain Clock Supervisor (CSV) registers"]
    pub csv6: CSV,
    _reserved7: [u8; 0x04],
    #[doc = "0x70..0x7c - Active domain Clock Supervisor (CSV) registers"]
    pub csv7: CSV,
    _reserved8: [u8; 0x04],
    #[doc = "0x80..0x8c - Active domain Clock Supervisor (CSV) registers"]
    pub csv8: CSV,
    _reserved9: [u8; 0x04],
    #[doc = "0x90..0x9c - Active domain Clock Supervisor (CSV) registers"]
    pub csv9: CSV,
    _reserved10: [u8; 0x04],
    #[doc = "0xa0..0xac - Active domain Clock Supervisor (CSV) registers"]
    pub csv10: CSV,
    _reserved11: [u8; 0x04],
    #[doc = "0xb0..0xbc - Active domain Clock Supervisor (CSV) registers"]
    pub csv11: CSV,
    _reserved12: [u8; 0x04],
    #[doc = "0xc0..0xcc - Active domain Clock Supervisor (CSV) registers"]
    pub csv12: CSV,
    _reserved13: [u8; 0x04],
    #[doc = "0xd0..0xdc - Active domain Clock Supervisor (CSV) registers"]
    pub csv13: CSV,
}
#[doc = "Active domain Clock Supervisor (CSV) registers"]
pub use self::csv::CSV;
#[doc = r"Cluster"]
#[doc = "Active domain Clock Supervisor (CSV) registers"]
pub mod csv;
