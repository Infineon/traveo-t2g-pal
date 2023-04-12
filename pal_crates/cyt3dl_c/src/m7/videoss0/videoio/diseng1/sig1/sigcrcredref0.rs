#[doc = "Register `SIGCRCREDREF0` reader"]
pub struct R(crate::R<SIGCRCREDREF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCREDREF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCREDREF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCREDREF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCREDREF0` writer"]
pub struct W(crate::W<SIGCRCREDREF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCREDREF0_SPEC>;
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
impl From<crate::W<SIGCRCREDREF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCREDREF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCREDREF0` reader - Reference value that is compared against measured SigCRCRed value."]
pub type SIGCRCREDREF0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCREDREF0` writer - Reference value that is compared against measured SigCRCRed value."]
pub type SIGCRCREDREF0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCREDREF0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reference value that is compared against measured SigCRCRed value."]
    #[inline(always)]
    pub fn sigcrcredref0(&self) -> SIGCRCREDREF0_R {
        SIGCRCREDREF0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reference value that is compared against measured SigCRCRed value."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcredref0(&mut self) -> SIGCRCREDREF0_W<0> {
        SIGCRCREDREF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of red channel for evaluation window 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcredref0](index.html) module"]
pub struct SIGCRCREDREF0_SPEC;
impl crate::RegisterSpec for SIGCRCREDREF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcredref0::R](R) reader structure"]
impl crate::Readable for SIGCRCREDREF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcredref0::W](W) writer structure"]
impl crate::Writable for SIGCRCREDREF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCREDREF0 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCREDREF0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
