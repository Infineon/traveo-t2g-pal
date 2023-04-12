#[doc = "Register `SIGCRCRED3` reader"]
pub struct R(crate::R<SIGCRCRED3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCRED3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCRED3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCRED3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCRED3` reader - See SigCRCRed0."]
pub type SIGCRCRED3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCRed0."]
    #[inline(always)]
    pub fn sigcrcred3(&self) -> SIGCRCRED3_R {
        SIGCRCRED3_R::new(self.bits)
    }
}
#[doc = "Measured signature of red channel for evaluation window 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcred3](index.html) module"]
pub struct SIGCRCRED3_SPEC;
impl crate::RegisterSpec for SIGCRCRED3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcred3::R](R) reader structure"]
impl crate::Readable for SIGCRCRED3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCRED3 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCRED3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
