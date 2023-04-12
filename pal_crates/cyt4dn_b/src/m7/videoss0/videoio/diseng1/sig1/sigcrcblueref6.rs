#[doc = "Register `SIGCRCBLUEREF6` reader"]
pub struct R(crate::R<SIGCRCBLUEREF6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUEREF6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUEREF6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUEREF6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCBLUEREF6` writer"]
pub struct W(crate::W<SIGCRCBLUEREF6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCBLUEREF6_SPEC>;
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
impl From<crate::W<SIGCRCBLUEREF6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCBLUEREF6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCBLUEREF6` reader - See SigCRCBlueRef0."]
pub type SIGCRCBLUEREF6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCBLUEREF6` writer - See SigCRCBlueRef0."]
pub type SIGCRCBLUEREF6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCBLUEREF6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlueRef0."]
    #[inline(always)]
    pub fn sigcrcblueref6(&self) -> SIGCRCBLUEREF6_R {
        SIGCRCBLUEREF6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See SigCRCBlueRef0."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcblueref6(&mut self) -> SIGCRCBLUEREF6_W<0> {
        SIGCRCBLUEREF6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of blue channel for evaluation window 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblueref6](index.html) module"]
pub struct SIGCRCBLUEREF6_SPEC;
impl crate::RegisterSpec for SIGCRCBLUEREF6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblueref6::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUEREF6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcblueref6::W](W) writer structure"]
impl crate::Writable for SIGCRCBLUEREF6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCBLUEREF6 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUEREF6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
