#[doc = r"Register block"]
#[repr(C)]
pub struct CSV_ILO {
    #[doc = "0x00..0x0c - HVILO clock DeepSleep domain Clock Supervisor registers"]
    pub csv: CSV,
}
#[doc = "HVILO clock DeepSleep domain Clock Supervisor registers"]
pub use self::csv::CSV;
#[doc = r"Cluster"]
#[doc = "HVILO clock DeepSleep domain Clock Supervisor registers"]
pub mod csv;
