#[doc = "Register `SIGCRCBLUE3` reader"]
pub struct R(crate::R<SIGCRCBLUE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCBLUE3` reader - See SigCRCBlue0."]
pub type SIGCRCBLUE3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlue0."]
    #[inline(always)]
    pub fn sigcrcblue3(&self) -> SIGCRCBLUE3_R {
        SIGCRCBLUE3_R::new(self.bits)
    }
}
#[doc = "Measured signature of blue channel for evaluation window 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblue3](index.html) module"]
pub struct SIGCRCBLUE3_SPEC;
impl crate::RegisterSpec for SIGCRCBLUE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblue3::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUE3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCBLUE3 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUE3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
