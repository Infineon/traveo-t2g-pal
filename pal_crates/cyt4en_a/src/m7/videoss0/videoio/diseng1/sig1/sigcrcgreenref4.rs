#[doc = "Register `SIGCRCGREENREF4` reader"]
pub struct R(crate::R<SIGCRCGREENREF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREENREF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREENREF4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREENREF4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCGREENREF4` writer"]
pub struct W(crate::W<SIGCRCGREENREF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCGREENREF4_SPEC>;
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
impl From<crate::W<SIGCRCGREENREF4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCGREENREF4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCGREENREF4` reader - See SigCRCGreenRef0."]
pub type SIGCRCGREENREF4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCGREENREF4` writer - See SigCRCGreenRef0."]
pub type SIGCRCGREENREF4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCGREENREF4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreenRef0."]
    #[inline(always)]
    pub fn sigcrcgreenref4(&self) -> SIGCRCGREENREF4_R {
        SIGCRCGREENREF4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See SigCRCGreenRef0."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcgreenref4(&mut self) -> SIGCRCGREENREF4_W<0> {
        SIGCRCGREENREF4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of green channel for evaluation window 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreenref4](index.html) module"]
pub struct SIGCRCGREENREF4_SPEC;
impl crate::RegisterSpec for SIGCRCGREENREF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreenref4::R](R) reader structure"]
impl crate::Readable for SIGCRCGREENREF4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcgreenref4::W](W) writer structure"]
impl crate::Writable for SIGCRCGREENREF4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCGREENREF4 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREENREF4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
