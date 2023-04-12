#[doc = "Register `PTSR2` reader"]
pub struct R(crate::R<PTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR2` writer"]
pub struct W(crate::W<PTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR2_SPEC>;
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
impl From<crate::W<PTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAC1B1` reader - CA setting for LPDDR4 channel 1 bit 1"]
pub type CAC1B1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC1B1` writer - CA setting for LPDDR4 channel 1 bit 1"]
pub type CAC1B1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC1B2` reader - CA setting for LPDDR4 channel 1 bit 2"]
pub type CAC1B2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC1B2` writer - CA setting for LPDDR4 channel 1 bit 2"]
pub type CAC1B2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC1B3` reader - CA setting for LPDDR4 channel 1 bit 3"]
pub type CAC1B3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC1B3` writer - CA setting for LPDDR4 channel 1 bit 3"]
pub type CAC1B3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC1B4` reader - CA setting for LPDDR4 channel 1 bit 4"]
pub type CAC1B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC1B4` writer - CA setting for LPDDR4 channel 1 bit 4"]
pub type CAC1B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC1B5` reader - CA setting for LPDDR4 channel 1 bit 5"]
pub type CAC1B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC1B5` writer - CA setting for LPDDR4 channel 1 bit 5"]
pub type CAC1B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - CA setting for LPDDR4 channel 1 bit 1"]
    #[inline(always)]
    pub fn cac1b1(&self) -> CAC1B1_R {
        CAC1B1_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - CA setting for LPDDR4 channel 1 bit 2"]
    #[inline(always)]
    pub fn cac1b2(&self) -> CAC1B2_R {
        CAC1B2_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - CA setting for LPDDR4 channel 1 bit 3"]
    #[inline(always)]
    pub fn cac1b3(&self) -> CAC1B3_R {
        CAC1B3_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - CA setting for LPDDR4 channel 1 bit 4"]
    #[inline(always)]
    pub fn cac1b4(&self) -> CAC1B4_R {
        CAC1B4_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - CA setting for LPDDR4 channel 1 bit 5"]
    #[inline(always)]
    pub fn cac1b5(&self) -> CAC1B5_R {
        CAC1B5_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CA setting for LPDDR4 channel 1 bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cac1b1(&mut self) -> CAC1B1_W<0> {
        CAC1B1_W::new(self)
    }
    #[doc = "Bits 6:11 - CA setting for LPDDR4 channel 1 bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cac1b2(&mut self) -> CAC1B2_W<6> {
        CAC1B2_W::new(self)
    }
    #[doc = "Bits 12:17 - CA setting for LPDDR4 channel 1 bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cac1b3(&mut self) -> CAC1B3_W<12> {
        CAC1B3_W::new(self)
    }
    #[doc = "Bits 18:23 - CA setting for LPDDR4 channel 1 bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cac1b4(&mut self) -> CAC1B4_W<18> {
        CAC1B4_W::new(self)
    }
    #[doc = "Bits 24:29 - CA setting for LPDDR4 channel 1 bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cac1b5(&mut self) -> CAC1B5_W<24> {
        CAC1B5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr2](index.html) module"]
pub struct PTSR2_SPEC;
impl crate::RegisterSpec for PTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr2::R](R) reader structure"]
impl crate::Readable for PTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr2::W](W) writer structure"]
impl crate::Writable for PTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR2 to value 0"]
impl crate::Resettable for PTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
