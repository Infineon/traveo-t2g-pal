#[doc = "Register `SIGCRCGREENREF5` reader"]
pub struct R(crate::R<SIGCRCGREENREF5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREENREF5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREENREF5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREENREF5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCGREENREF5` writer"]
pub struct W(crate::W<SIGCRCGREENREF5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCGREENREF5_SPEC>;
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
impl From<crate::W<SIGCRCGREENREF5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCGREENREF5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCGREENREF5` reader - See SigCRCGreenRef0."]
pub type SIGCRCGREENREF5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCGREENREF5` writer - See SigCRCGreenRef0."]
pub type SIGCRCGREENREF5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCGREENREF5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreenRef0."]
    #[inline(always)]
    pub fn sigcrcgreenref5(&self) -> SIGCRCGREENREF5_R {
        SIGCRCGREENREF5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See SigCRCGreenRef0."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcgreenref5(&mut self) -> SIGCRCGREENREF5_W<0> {
        SIGCRCGREENREF5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of green channel for evaluation window 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreenref5](index.html) module"]
pub struct SIGCRCGREENREF5_SPEC;
impl crate::RegisterSpec for SIGCRCGREENREF5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreenref5::R](R) reader structure"]
impl crate::Readable for SIGCRCGREENREF5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcgreenref5::W](W) writer structure"]
impl crate::Writable for SIGCRCGREENREF5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCGREENREF5 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREENREF5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
