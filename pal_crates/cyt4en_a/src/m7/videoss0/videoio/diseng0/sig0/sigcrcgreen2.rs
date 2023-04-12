#[doc = "Register `SIGCRCGREEN2` reader"]
pub struct R(crate::R<SIGCRCGREEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCGREEN2` reader - See SigCRCGreen0."]
pub type SIGCRCGREEN2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreen0."]
    #[inline(always)]
    pub fn sigcrcgreen2(&self) -> SIGCRCGREEN2_R {
        SIGCRCGREEN2_R::new(self.bits)
    }
}
#[doc = "Measured signature of green channel for evaluation window 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreen2](index.html) module"]
pub struct SIGCRCGREEN2_SPEC;
impl crate::RegisterSpec for SIGCRCGREEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreen2::R](R) reader structure"]
impl crate::Readable for SIGCRCGREEN2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCGREEN2 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
