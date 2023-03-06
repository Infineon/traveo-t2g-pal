#[doc = "Register `SIGCRCGREEN5` reader"]
pub struct R(crate::R<SIGCRCGREEN5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREEN5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREEN5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREEN5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCGREEN5` reader - See SigCRCGreen0."]
pub type SIGCRCGREEN5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreen0."]
    #[inline(always)]
    pub fn sigcrcgreen5(&self) -> SIGCRCGREEN5_R {
        SIGCRCGREEN5_R::new(self.bits)
    }
}
#[doc = "Measured signature of green channel for evaluation window 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreen5](index.html) module"]
pub struct SIGCRCGREEN5_SPEC;
impl crate::RegisterSpec for SIGCRCGREEN5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreen5::R](R) reader structure"]
impl crate::Readable for SIGCRCGREEN5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCGREEN5 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREEN5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
