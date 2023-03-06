#[doc = "Register `SHAD_LPMR3` reader"]
pub struct R(crate::R<SHAD_LPMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAD_LPMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAD_LPMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAD_LPMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHAD_LPMR3_FS0_PUCAL` reader - Pull-up Calibration point for frequency set 0 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
pub type SHAD_LPMR3_FS0_PUCAL_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_FS0_WPST` reader - Write post-amble for frequency set 0 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
pub type SHAD_LPMR3_FS0_WPST_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_PPRP` reader - Post package repair protection Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR3_PPRP_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_FS0_PDDS` reader - N/A"]
pub type SHAD_LPMR3_FS0_PDDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR3_FS0_RDBI` reader - Read DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR3_FS0_RDBI_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_FS0_WDBI` reader - Write DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR3_FS0_WDBI_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_FS1_PUCAL` reader - Pull-up Calibration point for frequency set 1 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
pub type SHAD_LPMR3_FS1_PUCAL_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_FS1_WPST` reader - Write post-amble for frequency set 1 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
pub type SHAD_LPMR3_FS1_WPST_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_RSVD` reader - N/A"]
pub type SHAD_LPMR3_RSVD_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_FS1_PDDS` reader - N/A"]
pub type SHAD_LPMR3_FS1_PDDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR3_FS1_RDBI` reader - Read DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR3_FS1_RDBI_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR3_FS1_WDBI` reader - Write DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR3_FS1_WDBI_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Pull-up Calibration point for frequency set 0 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
    #[inline(always)]
    pub fn shad_lpmr3_fs0_pucal(&self) -> SHAD_LPMR3_FS0_PUCAL_R {
        SHAD_LPMR3_FS0_PUCAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write post-amble for frequency set 0 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
    #[inline(always)]
    pub fn shad_lpmr3_fs0_wpst(&self) -> SHAD_LPMR3_FS0_WPST_R {
        SHAD_LPMR3_FS0_WPST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Post package repair protection Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr3_pprp(&self) -> SHAD_LPMR3_PPRP_R {
        SHAD_LPMR3_PPRP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr3_fs0_pdds(&self) -> SHAD_LPMR3_FS0_PDDS_R {
        SHAD_LPMR3_FS0_PDDS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Read DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr3_fs0_rdbi(&self) -> SHAD_LPMR3_FS0_RDBI_R {
        SHAD_LPMR3_FS0_RDBI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr3_fs0_wdbi(&self) -> SHAD_LPMR3_FS0_WDBI_R {
        SHAD_LPMR3_FS0_WDBI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull-up Calibration point for frequency set 1 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
    #[inline(always)]
    pub fn shad_lpmr3_fs1_pucal(&self) -> SHAD_LPMR3_FS1_PUCAL_R {
        SHAD_LPMR3_FS1_PUCAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write post-amble for frequency set 1 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
    #[inline(always)]
    pub fn shad_lpmr3_fs1_wpst(&self) -> SHAD_LPMR3_FS1_WPST_R {
        SHAD_LPMR3_FS1_WPST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr3_rsvd(&self) -> SHAD_LPMR3_RSVD_R {
        SHAD_LPMR3_RSVD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr3_fs1_pdds(&self) -> SHAD_LPMR3_FS1_PDDS_R {
        SHAD_LPMR3_FS1_PDDS_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Read DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr3_fs1_rdbi(&self) -> SHAD_LPMR3_FS1_RDBI_R {
        SHAD_LPMR3_FS1_RDBI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr3_fs1_wdbi(&self) -> SHAD_LPMR3_FS1_WDBI_R {
        SHAD_LPMR3_FS1_WDBI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Shadow LPDDR Mode Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shad_lpmr3](index.html) module"]
pub struct SHAD_LPMR3_SPEC;
impl crate::RegisterSpec for SHAD_LPMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shad_lpmr3::R](R) reader structure"]
impl crate::Readable for SHAD_LPMR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHAD_LPMR3 to value 0x3131"]
impl crate::Resettable for SHAD_LPMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x3131;
}
