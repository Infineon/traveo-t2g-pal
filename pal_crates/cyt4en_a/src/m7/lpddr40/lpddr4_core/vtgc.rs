#[doc = "Register `VTGC` reader"]
pub struct R(crate::R<VTGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VTGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VTGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VTGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VTGC` writer"]
pub struct W(crate::W<VTGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VTGC_SPEC>;
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
impl From<crate::W<VTGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VTGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVREFR` reader - Internal PHY VREF-DQ range RANGE1 = 1 Always write 1."]
pub type IVREFR_R = crate::BitReader<bool>;
#[doc = "Field `IVREFR` writer - Internal PHY VREF-DQ range RANGE1 = 1 Always write 1."]
pub type IVREFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, VTGC_SPEC, bool, O>;
#[doc = "Field `IVREFTS` reader - PHY VREF-DQ training step, specify the number training patterns required by PHY. STEPS64 = 64 Always write 64."]
pub type IVREFTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IVREFTS` writer - PHY VREF-DQ training step, specify the number training patterns required by PHY. STEPS64 = 64 Always write 64."]
pub type IVREFTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VTGC_SPEC, u8, u8, 8, O>;
#[doc = "Field `VREFDQSW` reader - DRAM VREF-DQ step width. This field defines the distance between 2 VREF steps in coarse training."]
pub type VREFDQSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDQSW` writer - DRAM VREF-DQ step width. This field defines the distance between 2 VREF steps in coarse training."]
pub type VREFDQSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VTGC_SPEC, u8, u8, 6, O>;
#[doc = "Field `VREFCASW` reader - DRAM VREF-CA step width. This field defines the distance between 2 VREF steps in coarse training."]
pub type VREFCASW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFCASW` writer - DRAM VREF-CA step width. This field defines the distance between 2 VREF steps in coarse training."]
pub type VREFCASW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VTGC_SPEC, u8, u8, 6, O>;
#[doc = "Field `IVREFEN` reader - Use Internal PHY VREF. Always set to 1'b1. ENABLE = 1 Always write 1."]
pub type IVREFEN_R = crate::BitReader<bool>;
#[doc = "Field `IVREFEN` writer - Use Internal PHY VREF. Always set to 1'b1. ENABLE = 1 Always write 1."]
pub type IVREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VTGC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Internal PHY VREF-DQ range RANGE1 = 1 Always write 1."]
    #[inline(always)]
    pub fn ivrefr(&self) -> IVREFR_R {
        IVREFR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - PHY VREF-DQ training step, specify the number training patterns required by PHY. STEPS64 = 64 Always write 64."]
    #[inline(always)]
    pub fn ivrefts(&self) -> IVREFTS_R {
        IVREFTS_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bits 9:14 - DRAM VREF-DQ step width. This field defines the distance between 2 VREF steps in coarse training."]
    #[inline(always)]
    pub fn vrefdqsw(&self) -> VREFDQSW_R {
        VREFDQSW_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bits 15:20 - DRAM VREF-CA step width. This field defines the distance between 2 VREF steps in coarse training."]
    #[inline(always)]
    pub fn vrefcasw(&self) -> VREFCASW_R {
        VREFCASW_R::new(((self.bits >> 15) & 0x3f) as u8)
    }
    #[doc = "Bit 21 - Use Internal PHY VREF. Always set to 1'b1. ENABLE = 1 Always write 1."]
    #[inline(always)]
    pub fn ivrefen(&self) -> IVREFEN_R {
        IVREFEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal PHY VREF-DQ range RANGE1 = 1 Always write 1."]
    #[inline(always)]
    #[must_use]
    pub fn ivrefr(&mut self) -> IVREFR_W<0> {
        IVREFR_W::new(self)
    }
    #[doc = "Bits 1:8 - PHY VREF-DQ training step, specify the number training patterns required by PHY. STEPS64 = 64 Always write 64."]
    #[inline(always)]
    #[must_use]
    pub fn ivrefts(&mut self) -> IVREFTS_W<1> {
        IVREFTS_W::new(self)
    }
    #[doc = "Bits 9:14 - DRAM VREF-DQ step width. This field defines the distance between 2 VREF steps in coarse training."]
    #[inline(always)]
    #[must_use]
    pub fn vrefdqsw(&mut self) -> VREFDQSW_W<9> {
        VREFDQSW_W::new(self)
    }
    #[doc = "Bits 15:20 - DRAM VREF-CA step width. This field defines the distance between 2 VREF steps in coarse training."]
    #[inline(always)]
    #[must_use]
    pub fn vrefcasw(&mut self) -> VREFCASW_W<15> {
        VREFCASW_W::new(self)
    }
    #[doc = "Bit 21 - Use Internal PHY VREF. Always set to 1'b1. ENABLE = 1 Always write 1."]
    #[inline(always)]
    #[must_use]
    pub fn ivrefen(&mut self) -> IVREFEN_W<21> {
        IVREFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY VREF Training General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtgc](index.html) module"]
pub struct VTGC_SPEC;
impl crate::RegisterSpec for VTGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vtgc::R](R) reader structure"]
impl crate::Readable for VTGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vtgc::W](W) writer structure"]
impl crate::Writable for VTGC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTGC to value 0x0020_0000"]
impl crate::Resettable for VTGC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
