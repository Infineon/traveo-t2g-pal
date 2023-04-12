#[doc = "Register `SIGCRCGREEN3` reader"]
pub struct R(crate::R<SIGCRCGREEN3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREEN3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREEN3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREEN3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCGREEN3` reader - See SigCRCGreen0."]
pub type SIGCRCGREEN3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreen0."]
    #[inline(always)]
    pub fn sigcrcgreen3(&self) -> SIGCRCGREEN3_R {
        SIGCRCGREEN3_R::new(self.bits)
    }
}
#[doc = "Measured signature of green channel for evaluation window 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreen3](index.html) module"]
pub struct SIGCRCGREEN3_SPEC;
impl crate::RegisterSpec for SIGCRCGREEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreen3::R](R) reader structure"]
impl crate::Readable for SIGCRCGREEN3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCGREEN3 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREEN3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
