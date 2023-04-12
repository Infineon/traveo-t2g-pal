#[doc = "Register `RATIO` reader"]
pub struct R(crate::R<RATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RATIO` writer"]
pub struct W(crate::W<RATIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RATIO_SPEC>;
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
impl From<crate::W<RATIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RATIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAC8` reader - Fractional component of ratio value."]
pub type FRAC8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC8` writer - Fractional component of ratio value."]
pub type FRAC8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RATIO_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT16` reader - Integer component of ratio value."]
pub type INT16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT16` writer - Integer component of ratio value."]
pub type INT16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RATIO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 8:15 - Fractional component of ratio value."]
    #[inline(always)]
    pub fn frac8(&self) -> FRAC8_R {
        FRAC8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Integer component of ratio value."]
    #[inline(always)]
    pub fn int16(&self) -> INT16_R {
        INT16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - Fractional component of ratio value."]
    #[inline(always)]
    #[must_use]
    pub fn frac8(&mut self) -> FRAC8_W<8> {
        FRAC8_W::new(self)
    }
    #[doc = "Bits 16:31 - Integer component of ratio value."]
    #[inline(always)]
    #[must_use]
    pub fn int16(&mut self) -> INT16_W<16> {
        INT16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratio](index.html) module"]
pub struct RATIO_SPEC;
impl crate::RegisterSpec for RATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ratio::R](R) reader structure"]
impl crate::Readable for RATIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ratio::W](W) writer structure"]
impl crate::Writable for RATIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RATIO to value 0"]
impl crate::Resettable for RATIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
