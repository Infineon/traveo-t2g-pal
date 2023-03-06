#[doc = "Register `PTSR1` reader"]
pub struct R(crate::R<PTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR1` writer"]
pub struct W(crate::W<PTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR1_SPEC>;
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
impl From<crate::W<PTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAC0B2` reader - CA setting for LPDDR4 channel 0 bit 2"]
pub type CAC0B2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC0B2` writer - CA setting for LPDDR4 channel 0 bit 2"]
pub type CAC0B2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC0B3` reader - CA setting for LPDDR4 channel 0 bit 3"]
pub type CAC0B3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC0B3` writer - CA setting for LPDDR4 channel 0 bit 3"]
pub type CAC0B3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC0B4` reader - CA setting for LPDDR4 channel 0 bit 4"]
pub type CAC0B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC0B4` writer - CA setting for LPDDR4 channel 0 bit 4"]
pub type CAC0B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC0B5` reader - CA setting for LPDDR4 channel 0 bit 5"]
pub type CAC0B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC0B5` writer - CA setting for LPDDR4 channel 0 bit 5"]
pub type CAC0B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR1_SPEC, u8, u8, 6, O>;
#[doc = "Field `CAC1B0` reader - CA setting for LPDDR4 channel 1 bit 0"]
pub type CAC1B0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAC1B0` writer - CA setting for LPDDR4 channel 1 bit 0"]
pub type CAC1B0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - CA setting for LPDDR4 channel 0 bit 2"]
    #[inline(always)]
    pub fn cac0b2(&self) -> CAC0B2_R {
        CAC0B2_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - CA setting for LPDDR4 channel 0 bit 3"]
    #[inline(always)]
    pub fn cac0b3(&self) -> CAC0B3_R {
        CAC0B3_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - CA setting for LPDDR4 channel 0 bit 4"]
    #[inline(always)]
    pub fn cac0b4(&self) -> CAC0B4_R {
        CAC0B4_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - CA setting for LPDDR4 channel 0 bit 5"]
    #[inline(always)]
    pub fn cac0b5(&self) -> CAC0B5_R {
        CAC0B5_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - CA setting for LPDDR4 channel 1 bit 0"]
    #[inline(always)]
    pub fn cac1b0(&self) -> CAC1B0_R {
        CAC1B0_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CA setting for LPDDR4 channel 0 bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cac0b2(&mut self) -> CAC0B2_W<0> {
        CAC0B2_W::new(self)
    }
    #[doc = "Bits 6:11 - CA setting for LPDDR4 channel 0 bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cac0b3(&mut self) -> CAC0B3_W<6> {
        CAC0B3_W::new(self)
    }
    #[doc = "Bits 12:17 - CA setting for LPDDR4 channel 0 bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cac0b4(&mut self) -> CAC0B4_W<12> {
        CAC0B4_W::new(self)
    }
    #[doc = "Bits 18:23 - CA setting for LPDDR4 channel 0 bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cac0b5(&mut self) -> CAC0B5_W<18> {
        CAC0B5_W::new(self)
    }
    #[doc = "Bits 24:29 - CA setting for LPDDR4 channel 1 bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cac1b0(&mut self) -> CAC1B0_W<24> {
        CAC1B0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr1](index.html) module"]
pub struct PTSR1_SPEC;
impl crate::RegisterSpec for PTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr1::R](R) reader structure"]
impl crate::Readable for PTSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr1::W](W) writer structure"]
impl crate::Writable for PTSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR1 to value 0"]
impl crate::Resettable for PTSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
