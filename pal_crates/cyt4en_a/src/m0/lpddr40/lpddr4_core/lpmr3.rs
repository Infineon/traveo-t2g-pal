#[doc = "Register `LPMR3` reader"]
pub struct R(crate::R<LPMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMR3` writer"]
pub struct W(crate::W<LPMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMR3_SPEC>;
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
impl From<crate::W<LPMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS0_PUCAL` reader - Pull-up Calibration point for frequency set 0 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
pub type FS0_PUCAL_R = crate::BitReader<bool>;
#[doc = "Field `FS0_PUCAL` writer - Pull-up Calibration point for frequency set 0 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
pub type FS0_PUCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
#[doc = "Field `FS0_WPST` reader - Write post-amble for frequency set 0 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
pub type FS0_WPST_R = crate::BitReader<bool>;
#[doc = "Field `FS0_WPST` writer - Write post-amble for frequency set 0 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
pub type FS0_WPST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
#[doc = "Field `PPRP` reader - Post package repair protection Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PPRP_R = crate::BitReader<bool>;
#[doc = "Field `PPRP` writer - Post package repair protection Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type PPRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
#[doc = "Field `FS0_PDDS` reader - N/A"]
pub type FS0_PDDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_PDDS` writer - N/A"]
pub type FS0_PDDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS0_RDBI` reader - Read DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type FS0_RDBI_R = crate::BitReader<bool>;
#[doc = "Field `FS0_RDBI` writer - Read DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type FS0_RDBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
#[doc = "Field `FS0_WDBI` reader - Write DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type FS0_WDBI_R = crate::BitReader<bool>;
#[doc = "Field `FS0_WDBI` writer - Write DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type FS0_WDBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
#[doc = "Field `FS1_PUCAL` reader - Pull-up Calibration point for frequency set 1 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
pub type FS1_PUCAL_R = crate::BitReader<bool>;
#[doc = "Field `FS1_PUCAL` writer - Pull-up Calibration point for frequency set 1 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
pub type FS1_PUCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
#[doc = "Field `FS1_WPST` reader - Write post-amble for frequency set 1 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
pub type FS1_WPST_R = crate::BitReader<bool>;
#[doc = "Field `FS1_WPST` writer - Write post-amble for frequency set 1 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
pub type FS1_WPST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
#[doc = "Field `FS1_PDDS` reader - N/A"]
pub type FS1_PDDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_PDDS` writer - N/A"]
pub type FS1_PDDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS1_RDBI` reader - Read DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type FS1_RDBI_R = crate::BitReader<bool>;
#[doc = "Field `FS1_RDBI` writer - Read DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type FS1_RDBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
#[doc = "Field `FS1_WDBI` reader - Write DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type FS1_WDBI_R = crate::BitReader<bool>;
#[doc = "Field `FS1_WDBI` writer - Write DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type FS1_WDBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pull-up Calibration point for frequency set 0 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
    #[inline(always)]
    pub fn fs0_pucal(&self) -> FS0_PUCAL_R {
        FS0_PUCAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write post-amble for frequency set 0 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
    #[inline(always)]
    pub fn fs0_wpst(&self) -> FS0_WPST_R {
        FS0_WPST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Post package repair protection Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pprp(&self) -> PPRP_R {
        PPRP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - N/A"]
    #[inline(always)]
    pub fn fs0_pdds(&self) -> FS0_PDDS_R {
        FS0_PDDS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Read DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn fs0_rdbi(&self) -> FS0_RDBI_R {
        FS0_RDBI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn fs0_wdbi(&self) -> FS0_WDBI_R {
        FS0_WDBI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull-up Calibration point for frequency set 1 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
    #[inline(always)]
    pub fn fs1_pucal(&self) -> FS1_PUCAL_R {
        FS1_PUCAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write post-amble for frequency set 1 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
    #[inline(always)]
    pub fn fs1_wpst(&self) -> FS1_WPST_R {
        FS1_WPST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - N/A"]
    #[inline(always)]
    pub fn fs1_pdds(&self) -> FS1_PDDS_R {
        FS1_PDDS_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - Read DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn fs1_rdbi(&self) -> FS1_RDBI_R {
        FS1_RDBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn fs1_wdbi(&self) -> FS1_WDBI_R {
        FS1_WDBI_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull-up Calibration point for frequency set 0 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_pucal(&mut self) -> FS0_PUCAL_W<0> {
        FS0_PUCAL_W::new(self)
    }
    #[doc = "Bit 1 - Write post-amble for frequency set 0 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_wpst(&mut self) -> FS0_WPST_W<1> {
        FS0_WPST_W::new(self)
    }
    #[doc = "Bit 2 - Post package repair protection Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pprp(&mut self) -> PPRP_W<2> {
        PPRP_W::new(self)
    }
    #[doc = "Bits 3:5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_pdds(&mut self) -> FS0_PDDS_W<3> {
        FS0_PDDS_W::new(self)
    }
    #[doc = "Bit 6 - Read DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_rdbi(&mut self) -> FS0_RDBI_W<6> {
        FS0_RDBI_W::new(self)
    }
    #[doc = "Bit 7 - Write DBI mode for frequency set 0 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_wdbi(&mut self) -> FS0_WDBI_W<7> {
        FS0_WDBI_W::new(self)
    }
    #[doc = "Bit 8 - Pull-up Calibration point for frequency set 1 Pull-up Calibration point values according to JESD209-4: VDDQ_DIV_2P5 = 0B VDDQ/2.5 VDDQ_DIV_3 = 1B VDDQ/3 (default)"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_pucal(&mut self) -> FS1_PUCAL_W<8> {
        FS1_PUCAL_W::new(self)
    }
    #[doc = "Bit 9 - Write post-amble for frequency set 1 Write post-amble values according to JESD209-4: WR_POSTAMBLE_0P5 = 0B WR Post-amble = 0.5*tCK (default) WR_POSTAMBLE_1P5 = 1B WR Post-amble = 1.5*tCK (Vendor specific function)"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_wpst(&mut self) -> FS1_WPST_W<9> {
        FS1_WPST_W::new(self)
    }
    #[doc = "Bits 10:12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_pdds(&mut self) -> FS1_PDDS_W<10> {
        FS1_PDDS_W::new(self)
    }
    #[doc = "Bit 13 - Read DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_rdbi(&mut self) -> FS1_RDBI_W<13> {
        FS1_RDBI_W::new(self)
    }
    #[doc = "Bit 14 - Write DBI mode for frequency set 1 Values according to JESD209-4: Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_wdbi(&mut self) -> FS1_WDBI_W<14> {
        FS1_WDBI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPDDR Mode Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmr3](index.html) module"]
pub struct LPMR3_SPEC;
impl crate::RegisterSpec for LPMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmr3::R](R) reader structure"]
impl crate::Readable for LPMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmr3::W](W) writer structure"]
impl crate::Writable for LPMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMR3 to value 0x1931"]
impl crate::Resettable for LPMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1931;
}
