#[doc = "Register `SIGCRCGREEN6` reader"]
pub struct R(crate::R<SIGCRCGREEN6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREEN6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREEN6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREEN6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCGREEN6` reader - See SigCRCGreen0."]
pub type SIGCRCGREEN6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreen0."]
    #[inline(always)]
    pub fn sigcrcgreen6(&self) -> SIGCRCGREEN6_R {
        SIGCRCGREEN6_R::new(self.bits)
    }
}
#[doc = "Measured signature of green channel for evaluation window 6.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreen6](index.html) module"]
pub struct SIGCRCGREEN6_SPEC;
impl crate::RegisterSpec for SIGCRCGREEN6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreen6::R](R) reader structure"]
impl crate::Readable for SIGCRCGREEN6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCGREEN6 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREEN6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
