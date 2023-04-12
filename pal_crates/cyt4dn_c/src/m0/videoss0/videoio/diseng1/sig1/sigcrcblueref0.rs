#[doc = "Register `SIGCRCBLUEREF0` reader"]
pub struct R(crate::R<SIGCRCBLUEREF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUEREF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUEREF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUEREF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCBLUEREF0` writer"]
pub struct W(crate::W<SIGCRCBLUEREF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCBLUEREF0_SPEC>;
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
impl From<crate::W<SIGCRCBLUEREF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCBLUEREF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCBLUEREF0` reader - Reference value that is compared against measured SigCRCBlue value."]
pub type SIGCRCBLUEREF0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCBLUEREF0` writer - Reference value that is compared against measured SigCRCBlue value."]
pub type SIGCRCBLUEREF0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCBLUEREF0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reference value that is compared against measured SigCRCBlue value."]
    #[inline(always)]
    pub fn sigcrcblueref0(&self) -> SIGCRCBLUEREF0_R {
        SIGCRCBLUEREF0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reference value that is compared against measured SigCRCBlue value."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcblueref0(&mut self) -> SIGCRCBLUEREF0_W<0> {
        SIGCRCBLUEREF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of blue channel for evaluation window 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblueref0](index.html) module"]
pub struct SIGCRCBLUEREF0_SPEC;
impl crate::RegisterSpec for SIGCRCBLUEREF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblueref0::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUEREF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcblueref0::W](W) writer structure"]
impl crate::Writable for SIGCRCBLUEREF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCBLUEREF0 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUEREF0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
