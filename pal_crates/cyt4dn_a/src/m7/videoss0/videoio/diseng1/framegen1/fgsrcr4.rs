#[doc = "Register `FGSRCR4` reader"]
pub struct R(crate::R<FGSRCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSRCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSRCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSRCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGSRCR4` writer"]
pub struct W(crate::W<FGSRCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGSRCR4_SPEC>;
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
impl From<crate::W<FGSRCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGSRCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGETSKEW` reader - Horizontal target skew value for horizontal and vertical skew regulation (signed value)."]
pub type TARGETSKEW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TARGETSKEW` writer - Horizontal target skew value for horizontal and vertical skew regulation (signed value)."]
pub type TARGETSKEW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGSRCR4_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - Horizontal target skew value for horizontal and vertical skew regulation (signed value)."]
    #[inline(always)]
    pub fn targetskew(&self) -> TARGETSKEW_R {
        TARGETSKEW_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Horizontal target skew value for horizontal and vertical skew regulation (signed value)."]
    #[inline(always)]
    #[must_use]
    pub fn targetskew(&mut self) -> TARGETSKEW_W<0> {
        TARGETSKEW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Skew Regulation Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsrcr4](index.html) module"]
pub struct FGSRCR4_SPEC;
impl crate::RegisterSpec for FGSRCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsrcr4::R](R) reader structure"]
impl crate::Readable for FGSRCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgsrcr4::W](W) writer structure"]
impl crate::Writable for FGSRCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGSRCR4 to value 0xc8"]
impl crate::Resettable for FGSRCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0xc8;
}
