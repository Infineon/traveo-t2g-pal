#[doc = r"Register block"]
#[repr(C)]
pub struct CSV_LF {
    #[doc = "0x00..0x0c - LF clock Clock Supervisor registers"]
    pub csv: CSV,
}
#[doc = "LF clock Clock Supervisor registers"]
pub use self::csv::CSV;
#[doc = r"Cluster"]
#[doc = "LF clock Clock Supervisor registers"]
pub mod csv;
