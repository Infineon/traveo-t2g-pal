#[doc = "Register `SIGCRCGREEN1` reader"]
pub struct R(crate::R<SIGCRCGREEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCGREEN1` reader - See SigCRCGreen0."]
pub type SIGCRCGREEN1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreen0."]
    #[inline(always)]
    pub fn sigcrcgreen1(&self) -> SIGCRCGREEN1_R {
        SIGCRCGREEN1_R::new(self.bits)
    }
}
#[doc = "Measured signature of green channel for evaluation window 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreen1](index.html) module"]
pub struct SIGCRCGREEN1_SPEC;
impl crate::RegisterSpec for SIGCRCGREEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreen1::R](R) reader structure"]
impl crate::Readable for SIGCRCGREEN1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCGREEN1 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
