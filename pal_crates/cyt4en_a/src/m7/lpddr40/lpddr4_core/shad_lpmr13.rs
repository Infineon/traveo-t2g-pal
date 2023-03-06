#[doc = "Register `SHAD_LPMR13` reader"]
pub struct R(crate::R<SHAD_LPMR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAD_LPMR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAD_LPMR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAD_LPMR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHAD_LPMR13_CBT` reader - Command bus training mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR13_CBT_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR13_RPT` reader - Read preamble training mode Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR13_RPT_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR13_VRO` reader - VREF output enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR13_VRO_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR13_VRCG` reader - VREF current generator VREF Current Generator values according to JESD209-4: NORMAL = 0B Normal Operation (default) VREF_FAST = 1B VREF Fast Response (high current) mode"]
pub type SHAD_LPMR13_VRCG_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR13_RRO` reader - Refresh rate option Refresh Rate Option values according to JESD209-4: DISABLE_RR4X2X = 0B Disable codes 001 (Refresh Rate x 4) and 010 (Refresh Rate x 2) in MR4 OP\\[2:0\\]
ENABLE_ALL = 1B Enable all codes in MR4 OP\\[2:0\\]"]
pub type SHAD_LPMR13_RRO_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR13_DMD` reader - Data mask disable Data Mask Disable values according to JESD209-4: ENABLED = 0B Data Mask Operation Enabled (default) DISABLED = 1B Data Mask Operation Disabled"]
pub type SHAD_LPMR13_DMD_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR13_FSPWR` reader - Frequency Set Point write enable Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
pub type SHAD_LPMR13_FSPWR_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR13_FSPOP` reader - Frequency Set Point operating mode Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
pub type SHAD_LPMR13_FSPOP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Command bus training mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr13_cbt(&self) -> SHAD_LPMR13_CBT_R {
        SHAD_LPMR13_CBT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read preamble training mode Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr13_rpt(&self) -> SHAD_LPMR13_RPT_R {
        SHAD_LPMR13_RPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VREF output enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr13_vro(&self) -> SHAD_LPMR13_VRO_R {
        SHAD_LPMR13_VRO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREF current generator VREF Current Generator values according to JESD209-4: NORMAL = 0B Normal Operation (default) VREF_FAST = 1B VREF Fast Response (high current) mode"]
    #[inline(always)]
    pub fn shad_lpmr13_vrcg(&self) -> SHAD_LPMR13_VRCG_R {
        SHAD_LPMR13_VRCG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Refresh rate option Refresh Rate Option values according to JESD209-4: DISABLE_RR4X2X = 0B Disable codes 001 (Refresh Rate x 4) and 010 (Refresh Rate x 2) in MR4 OP\\[2:0\\]
ENABLE_ALL = 1B Enable all codes in MR4 OP\\[2:0\\]"]
    #[inline(always)]
    pub fn shad_lpmr13_rro(&self) -> SHAD_LPMR13_RRO_R {
        SHAD_LPMR13_RRO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data mask disable Data Mask Disable values according to JESD209-4: ENABLED = 0B Data Mask Operation Enabled (default) DISABLED = 1B Data Mask Operation Disabled"]
    #[inline(always)]
    pub fn shad_lpmr13_dmd(&self) -> SHAD_LPMR13_DMD_R {
        SHAD_LPMR13_DMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Frequency Set Point write enable Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
    #[inline(always)]
    pub fn shad_lpmr13_fspwr(&self) -> SHAD_LPMR13_FSPWR_R {
        SHAD_LPMR13_FSPWR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frequency Set Point operating mode Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
    #[inline(always)]
    pub fn shad_lpmr13_fspop(&self) -> SHAD_LPMR13_FSPOP_R {
        SHAD_LPMR13_FSPOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Shadow LPDDR Mode Register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shad_lpmr13](index.html) module"]
pub struct SHAD_LPMR13_SPEC;
impl crate::RegisterSpec for SHAD_LPMR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shad_lpmr13::R](R) reader structure"]
impl crate::Readable for SHAD_LPMR13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHAD_LPMR13 to value 0"]
impl crate::Resettable for SHAD_LPMR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
