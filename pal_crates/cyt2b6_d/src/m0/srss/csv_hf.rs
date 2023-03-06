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
}
#[doc = "Active domain Clock Supervisor (CSV) registers"]
pub use self::csv::CSV;
#[doc = r"Cluster"]
#[doc = "Active domain Clock Supervisor (CSV) registers"]
pub mod csv;
