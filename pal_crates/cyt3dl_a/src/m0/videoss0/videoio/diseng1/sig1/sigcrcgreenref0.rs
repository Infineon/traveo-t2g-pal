#[doc = "Register `SIGCRCGREENREF0` reader"]
pub struct R(crate::R<SIGCRCGREENREF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREENREF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREENREF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREENREF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCGREENREF0` writer"]
pub struct W(crate::W<SIGCRCGREENREF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCGREENREF0_SPEC>;
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
impl From<crate::W<SIGCRCGREENREF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCGREENREF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCGREENREF0` reader - Reference value that is compared against measured SigCRCGreen value."]
pub type SIGCRCGREENREF0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCGREENREF0` writer - Reference value that is compared against measured SigCRCGreen value."]
pub type SIGCRCGREENREF0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCGREENREF0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reference value that is compared against measured SigCRCGreen value."]
    #[inline(always)]
    pub fn sigcrcgreenref0(&self) -> SIGCRCGREENREF0_R {
        SIGCRCGREENREF0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reference value that is compared against measured SigCRCGreen value."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcgreenref0(&mut self) -> SIGCRCGREENREF0_W<0> {
        SIGCRCGREENREF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of green channel for evaluation window 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreenref0](index.html) module"]
pub struct SIGCRCGREENREF0_SPEC;
impl crate::RegisterSpec for SIGCRCGREENREF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreenref0::R](R) reader structure"]
impl crate::Readable for SIGCRCGREENREF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcgreenref0::W](W) writer structure"]
impl crate::Writable for SIGCRCGREENREF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCGREENREF0 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREENREF0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
