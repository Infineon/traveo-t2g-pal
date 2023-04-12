#[doc = "Register `SIGCRCREDREF4` reader"]
pub struct R(crate::R<SIGCRCREDREF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCREDREF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCREDREF4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCREDREF4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGCRCREDREF4` writer"]
pub struct W(crate::W<SIGCRCREDREF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGCRCREDREF4_SPEC>;
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
impl From<crate::W<SIGCRCREDREF4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGCRCREDREF4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGCRCREDREF4` reader - See SigCRCRedRef0."]
pub type SIGCRCREDREF4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGCRCREDREF4` writer - See SigCRCRedRef0."]
pub type SIGCRCREDREF4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGCRCREDREF4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCRedRef0."]
    #[inline(always)]
    pub fn sigcrcredref4(&self) -> SIGCRCREDREF4_R {
        SIGCRCREDREF4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See SigCRCRedRef0."]
    #[inline(always)]
    #[must_use]
    pub fn sigcrcredref4(&mut self) -> SIGCRCREDREF4_W<0> {
        SIGCRCREDREF4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference signature of red channel for evaluation window 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcredref4](index.html) module"]
pub struct SIGCRCREDREF4_SPEC;
impl crate::RegisterSpec for SIGCRCREDREF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcredref4::R](R) reader structure"]
impl crate::Readable for SIGCRCREDREF4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigcrcredref4::W](W) writer structure"]
impl crate::Writable for SIGCRCREDREF4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGCRCREDREF4 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCREDREF4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
