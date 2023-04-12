#[doc = "Register `SIGCRCRED6` reader"]
pub struct R(crate::R<SIGCRCRED6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCRED6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCRED6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCRED6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCRED6` reader - See SigCRCRed0."]
pub type SIGCRCRED6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCRed0."]
    #[inline(always)]
    pub fn sigcrcred6(&self) -> SIGCRCRED6_R {
        SIGCRCRED6_R::new(self.bits)
    }
}
#[doc = "Measured signature of red channel for evaluation window 6.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcred6](index.html) module"]
pub struct SIGCRCRED6_SPEC;
impl crate::RegisterSpec for SIGCRCRED6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcred6::R](R) reader structure"]
impl crate::Readable for SIGCRCRED6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCRED6 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCRED6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
