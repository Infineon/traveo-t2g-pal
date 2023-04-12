#[doc = "Register `SIGCRCBLUEREF5` reader"]
pub struct R(crate::R<SIGCRCBLUEREF5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUEREF5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUEREF5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUEREF5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCBLUEREF5` writer"]
pub struct W(crate::W<SIGCRCBLUEREF5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCBLUEREF5_SPEC>;
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
impl From<crate::W<SIGCRCBLUEREF5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCBLUEREF5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCBLUEREF5` reader - See SigCRCBlueRef0."]
pub type SIGCRCBLUEREF5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCBLUEREF5` writer - See SigCRCBlueRef0."]
pub type SIGCRCBLUEREF5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCBLUEREF5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlueRef0."]
    #[inline(always)]
    pub fn sigcrcblueref5(&self) -> SIGCRCBLUEREF5_R {
        SIGCRCBLUEREF5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See SigCRCBlueRef0."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcblueref5(&mut self) -> SIGCRCBLUEREF5_W<0> {
        SIGCRCBLUEREF5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of blue channel for evaluation window 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblueref5](index.html) module"]
pub struct SIGCRCBLUEREF5_SPEC;
impl crate::RegisterSpec for SIGCRCBLUEREF5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblueref5::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUEREF5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcblueref5::W](W) writer structure"]
impl crate::Writable for SIGCRCBLUEREF5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCBLUEREF5 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUEREF5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
