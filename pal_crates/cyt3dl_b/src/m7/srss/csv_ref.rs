#[doc = r"Register block"]
#[repr(C)]
pub struct CSV_REF {
    #[doc = "0x00..0x0c - Active domain Clock Supervisor (CSV) registers for CSV Reference clock"]
    pub csv: CSV,
}
#[doc = "Active domain Clock Supervisor (CSV) registers for CSV Reference clock"]
pub use self::csv::CSV;
#[doc = r"Cluster"]
#[doc = "Active domain Clock Supervisor (CSV) registers for CSV Reference clock"]
pub mod csv;
