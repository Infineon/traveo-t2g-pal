#[doc = "Register `SIGCRCGREENREF6` reader"]
pub struct R(crate::R<SIGCRCGREENREF6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREENREF6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREENREF6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREENREF6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCGREENREF6` writer"]
pub struct W(crate::W<SIGCRCGREENREF6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCGREENREF6_SPEC>;
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
impl From<crate::W<SIGCRCGREENREF6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCGREENREF6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCGREENREF6` reader - See SigCRCGreenRef0."]
pub type SIGCRCGREENREF6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCGREENREF6` writer - See SigCRCGreenRef0."]
pub type SIGCRCGREENREF6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCGREENREF6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreenRef0."]
    #[inline(always)]
    pub fn sigcrcgreenref6(&self) -> SIGCRCGREENREF6_R {
        SIGCRCGREENREF6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See SigCRCGreenRef0."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcgreenref6(&mut self) -> SIGCRCGREENREF6_W<0> {
        SIGCRCGREENREF6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of green channel for evaluation window 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreenref6](index.html) module"]
pub struct SIGCRCGREENREF6_SPEC;
impl crate::RegisterSpec for SIGCRCGREENREF6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreenref6::R](R) reader structure"]
impl crate::Readable for SIGCRCGREENREF6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcgreenref6::W](W) writer structure"]
impl crate::Writable for SIGCRCGREENREF6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCGREENREF6 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREENREF6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
