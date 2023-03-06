#[doc = "Register `SIGCRCRED1` reader"]
pub struct R(crate::R<SIGCRCRED1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCRED1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCRED1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCRED1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCRED1` reader - See SigCRCRed0."]
pub type SIGCRCRED1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCRed0."]
    #[inline(always)]
    pub fn sigcrcred1(&self) -> SIGCRCRED1_R {
        SIGCRCRED1_R::new(self.bits)
    }
}
#[doc = "Measured signature of red channel for evaluation window 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcred1](index.html) module"]
pub struct SIGCRCRED1_SPEC;
impl crate::RegisterSpec for SIGCRCRED1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcred1::R](R) reader structure"]
impl crate::Readable for SIGCRCRED1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCRED1 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCRED1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
