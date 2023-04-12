#[doc = "Register `PTSR0` reader"]
pub struct R(crate::R<PTSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR0` writer"]
pub struct W(crate::W<PTSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR0_SPEC>;
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
impl From<crate::W<PTSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREFCAR` reader - VREF-CA range VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type VREFCAR_R = crate::BitReader<bool>;
#[doc = "Field `VREFCAR` writer - VREF-CA range VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
pub type VREFCAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTSR0_SPEC, bool, O>;
#[doc = "Field `VREFCAS` reader - VREF-CA setting"]
pub type VREFCAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFCAS` writer - VREF-CA setting"]
pub type VREFCAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `CSC0` reader - CS setting for LPDDR4 channel 0"]
pub type CSC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSC0` writer - CS setting for LPDDR4 channel 0"]
pub type CSC0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `CSC1` reader - CS setting for LPDDR4 channel 1"]
pub type CSC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSC1` writer - CS setting for LPDDR4 channel 1"]
pub type CSC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC0B0` reader - CA setting for LPDDR4 channel 0 bit 0"]
pub type CAC0B0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC0B0` writer - CA setting for LPDDR4 channel 0 bit 0"]
pub type CAC0B0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC0B1` reader - CA setting for LPDDR4 channel 0 bit 1"]
pub type CAC0B1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC0B1` writer - CA setting for LPDDR4 channel 0 bit 1"]
pub type CAC0B1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR0_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - VREF-CA range VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    pub fn vrefcar(&self) -> VREFCAR_R {
        VREFCAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - VREF-CA setting"]
    #[inline(always)]
    pub fn vrefcas(&self) -> VREFCAS_R {
        VREFCAS_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:12 - CS setting for LPDDR4 channel 0"]
    #[inline(always)]
    pub fn csc0(&self) -> CSC0_R {
        CSC0_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:18 - CS setting for LPDDR4 channel 1"]
    #[inline(always)]
    pub fn csc1(&self) -> CSC1_R {
        CSC1_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:24 - CA setting for LPDDR4 channel 0 bit 0"]
    #[inline(always)]
    pub fn cac0b0(&self) -> CAC0B0_R {
        CAC0B0_R::new(((self.bits >> 19) & 0x3f) as u8)
    }
    #[doc = "Bits 25:30 - CA setting for LPDDR4 channel 0 bit 1"]
    #[inline(always)]
    pub fn cac0b1(&self) -> CAC0B1_R {
        CAC0B1_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VREF-CA range VREF Range according to JESD209-4: RANGE0 = 0B Range\\[0\\]
enabled RANGE1 =1B Range\\[1\\]
enabled"]
    #[inline(always)]
    #[must_use]
    pub fn vrefcar(&mut self) -> VREFCAR_W<0> {
        VREFCAR_W::new(self)
    }
    #[doc = "Bits 1:6 - VREF-CA setting"]
    #[inline(always)]
    #[must_use]
    pub fn vrefcas(&mut self) -> VREFCAS_W<1> {
        VREFCAS_W::new(self)
    }
    #[doc = "Bits 7:12 - CS setting for LPDDR4 channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn csc0(&mut self) -> CSC0_W<7> {
        CSC0_W::new(self)
    }
    #[doc = "Bits 13:18 - CS setting for LPDDR4 channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn csc1(&mut self) -> CSC1_W<13> {
        CSC1_W::new(self)
    }
    #[doc = "Bits 19:24 - CA setting for LPDDR4 channel 0 bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cac0b0(&mut self) -> CAC0B0_W<19> {
        CAC0B0_W::new(self)
    }
    #[doc = "Bits 25:30 - CA setting for LPDDR4 channel 0 bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cac0b1(&mut self) -> CAC0B1_W<25> {
        CAC0B1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr0](index.html) module"]
pub struct PTSR0_SPEC;
impl crate::RegisterSpec for PTSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr0::R](R) reader structure"]
impl crate::Readable for PTSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr0::W](W) writer structure"]
impl crate::Writable for PTSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR0 to value 0"]
impl crate::Resettable for PTSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
