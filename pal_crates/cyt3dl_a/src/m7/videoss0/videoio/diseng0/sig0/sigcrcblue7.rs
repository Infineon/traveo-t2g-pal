#[doc = "Register `SIGCRCBLUE7` reader"]
pub struct R(crate::R<SIGCRCBLUE7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUE7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUE7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUE7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCBLUE7` reader - See SigCRCBlue0."]
pub type SIGCRCBLUE7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlue0."]
    #[inline(always)]
    pub fn sigcrcblue7(&self) -> SIGCRCBLUE7_R {
        SIGCRCBLUE7_R::new(self.bits)
    }
}
#[doc = "Measured signature of blue channel for evaluation window 7.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblue7](index.html) module"]
pub struct SIGCRCBLUE7_SPEC;
impl crate::RegisterSpec for SIGCRCBLUE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblue7::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUE7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCBLUE7 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUE7_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
