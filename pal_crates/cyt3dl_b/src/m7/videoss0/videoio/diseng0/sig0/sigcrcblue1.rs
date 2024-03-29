#[doc = "Register `SIGCRCBLUE1` reader"]
pub struct R(crate::R<SIGCRCBLUE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCBLUE1` reader - See SigCRCBlue0."]
pub type SIGCRCBLUE1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlue0."]
    #[inline(always)]
    pub fn sigcrcblue1(&self) -> SIGCRCBLUE1_R {
        SIGCRCBLUE1_R::new(self.bits)
    }
}
#[doc = "Measured signature of blue channel for evaluation window 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblue1](index.html) module"]
pub struct SIGCRCBLUE1_SPEC;
impl crate::RegisterSpec for SIGCRCBLUE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblue1::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCBLUE1 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUE1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
