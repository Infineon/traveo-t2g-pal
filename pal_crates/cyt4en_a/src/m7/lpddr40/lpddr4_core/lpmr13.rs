#[doc = "Register `LPMR13` reader"]
pub struct R(crate::R<LPMR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMR13` writer"]
pub struct W(crate::W<LPMR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LPMR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMR13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBT` reader - Command bus training mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type CBT_R = crate::BitReader<bool>;
#[doc = "Field `CBT` writer - Command bus training mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type CBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR13_SPEC, bool, O>;
#[doc = "Field `RPT` reader - Read preamble training mode Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type RPT_R = crate::BitReader<bool>;
#[doc = "Field `RPT` writer - Read preamble training mode Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type RPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR13_SPEC, bool, O>;
#[doc = "Field `VRO` reader - VREF output enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type VRO_R = crate::BitReader<bool>;
#[doc = "Field `VRO` writer - VREF output enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type VRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR13_SPEC, bool, O>;
#[doc = "Field `VRCG` reader - VREF current generator VREF Current Generator values according to JESD209-4: NORMAL = 0B Normal Operation (default) VREF_FAST = 1B VREF Fast Response (high current) mode"]
pub type VRCG_R = crate::BitReader<bool>;
#[doc = "Field `VRCG` writer - VREF current generator VREF Current Generator values according to JESD209-4: NORMAL = 0B Normal Operation (default) VREF_FAST = 1B VREF Fast Response (high current) mode"]
pub type VRCG_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR13_SPEC, bool, O>;
#[doc = "Field `RRO` reader - Refresh rate option Refresh Rate Option values according to JESD209-4: DISABLE_RR4X2X = 0B Disable codes 001 (Refresh Rate x 4) and 010 (Refresh Rate x 2) in MR4 OP\\[2:0\\]
ENABLE_ALL = 1B Enable all codes in MR4 OP\\[2:0\\]"]
pub type RRO_R = crate::BitReader<bool>;
#[doc = "Field `RRO` writer - Refresh rate option Refresh Rate Option values according to JESD209-4: DISABLE_RR4X2X = 0B Disable codes 001 (Refresh Rate x 4) and 010 (Refresh Rate x 2) in MR4 OP\\[2:0\\]
ENABLE_ALL = 1B Enable all codes in MR4 OP\\[2:0\\]"]
pub type RRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR13_SPEC, bool, O>;
#[doc = "Field `DMD` reader - Data mask disable Data Mask Disable values according to JESD209-4: ENABLED = 0B Data Mask Operation Enabled (default) DISABLED = 1B Data Mask Operation Disabled"]
pub type DMD_R = crate::BitReader<bool>;
#[doc = "Field `DMD` writer - Data mask disable Data Mask Disable values according to JESD209-4: ENABLED = 0B Data Mask Operation Enabled (default) DISABLED = 1B Data Mask Operation Disabled"]
pub type DMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR13_SPEC, bool, O>;
#[doc = "Field `FSPWR` reader - Frequency Set Point write enable Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
pub type FSPWR_R = crate::BitReader<bool>;
#[doc = "Field `FSPWR` writer - Frequency Set Point write enable Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
pub type FSPWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR13_SPEC, bool, O>;
#[doc = "Field `FSPOP` reader - Frequency Set Point operating mode Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
pub type FSPOP_R = crate::BitReader<bool>;
#[doc = "Field `FSPOP` writer - Frequency Set Point operating mode Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
pub type FSPOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR13_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Command bus training mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn cbt(&self) -> CBT_R {
        CBT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read preamble training mode Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn rpt(&self) -> RPT_R {
        RPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VREF output enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn vro(&self) -> VRO_R {
        VRO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREF current generator VREF Current Generator values according to JESD209-4: NORMAL = 0B Normal Operation (default) VREF_FAST = 1B VREF Fast Response (high current) mode"]
    #[inline(always)]
    pub fn vrcg(&self) -> VRCG_R {
        VRCG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Refresh rate option Refresh Rate Option values according to JESD209-4: DISABLE_RR4X2X = 0B Disable codes 001 (Refresh Rate x 4) and 010 (Refresh Rate x 2) in MR4 OP\\[2:0\\]
ENABLE_ALL = 1B Enable all codes in MR4 OP\\[2:0\\]"]
    #[inline(always)]
    pub fn rro(&self) -> RRO_R {
        RRO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data mask disable Data Mask Disable values according to JESD209-4: ENABLED = 0B Data Mask Operation Enabled (default) DISABLED = 1B Data Mask Operation Disabled"]
    #[inline(always)]
    pub fn dmd(&self) -> DMD_R {
        DMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Frequency Set Point write enable Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
    #[inline(always)]
    pub fn fspwr(&self) -> FSPWR_R {
        FSPWR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frequency Set Point operating mode Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
    #[inline(always)]
    pub fn fspop(&self) -> FSPOP_R {
        FSPOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command bus training mode enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn cbt(&mut self) -> CBT_W<0> {
        CBT_W::new(self)
    }
    #[doc = "Bit 1 - Read preamble training mode Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn rpt(&mut self) -> RPT_W<1> {
        RPT_W::new(self)
    }
    #[doc = "Bit 2 - VREF output enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn vro(&mut self) -> VRO_W<2> {
        VRO_W::new(self)
    }
    #[doc = "Bit 3 - VREF current generator VREF Current Generator values according to JESD209-4: NORMAL = 0B Normal Operation (default) VREF_FAST = 1B VREF Fast Response (high current) mode"]
    #[inline(always)]
    #[must_use]
    pub fn vrcg(&mut self) -> VRCG_W<3> {
        VRCG_W::new(self)
    }
    #[doc = "Bit 4 - Refresh rate option Refresh Rate Option values according to JESD209-4: DISABLE_RR4X2X = 0B Disable codes 001 (Refresh Rate x 4) and 010 (Refresh Rate x 2) in MR4 OP\\[2:0\\]
ENABLE_ALL = 1B Enable all codes in MR4 OP\\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rro(&mut self) -> RRO_W<4> {
        RRO_W::new(self)
    }
    #[doc = "Bit 5 - Data mask disable Data Mask Disable values according to JESD209-4: ENABLED = 0B Data Mask Operation Enabled (default) DISABLED = 1B Data Mask Operation Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn dmd(&mut self) -> DMD_W<5> {
        DMD_W::new(self)
    }
    #[doc = "Bit 6 - Frequency Set Point write enable Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
    #[inline(always)]
    #[must_use]
    pub fn fspwr(&mut self) -> FSPWR_W<6> {
        FSPWR_W::new(self)
    }
    #[doc = "Bit 7 - Frequency Set Point operating mode Frequency Set Point coding according to JESD209-4: FSP0 = 0 Frequency Set Point 0 FSP1 = 1 Frequency Set Point 1"]
    #[inline(always)]
    #[must_use]
    pub fn fspop(&mut self) -> FSPOP_W<7> {
        FSPOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPDDR Mode Register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmr13](index.html) module"]
pub struct LPMR13_SPEC;
impl crate::RegisterSpec for LPMR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmr13::R](R) reader structure"]
impl crate::Readable for LPMR13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmr13::W](W) writer structure"]
impl crate::Writable for LPMR13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMR13 to value 0"]
impl crate::Resettable for LPMR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
