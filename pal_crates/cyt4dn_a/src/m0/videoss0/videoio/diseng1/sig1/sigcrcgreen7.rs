#[doc = "Register `SIGCRCGREEN7` reader"]
pub struct R(crate::R<SIGCRCGREEN7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREEN7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREEN7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREEN7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCGREEN7` reader - See SigCRCGreen0."]
pub type SIGCRCGREEN7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreen0."]
    #[inline(always)]
    pub fn sigcrcgreen7(&self) -> SIGCRCGREEN7_R {
        SIGCRCGREEN7_R::new(self.bits)
    }
}
#[doc = "Measured signature of green channel for evaluation window 7.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreen7](index.html) module"]
pub struct SIGCRCGREEN7_SPEC;
impl crate::RegisterSpec for SIGCRCGREEN7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreen7::R](R) reader structure"]
impl crate::Readable for SIGCRCGREEN7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCGREEN7 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREEN7_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
