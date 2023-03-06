#[doc = "Register `SIGCRCBLUE6` reader"]
pub struct R(crate::R<SIGCRCBLUE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCBLUE6` reader - See SigCRCBlue0."]
pub type SIGCRCBLUE6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlue0."]
    #[inline(always)]
    pub fn sigcrcblue6(&self) -> SIGCRCBLUE6_R {
        SIGCRCBLUE6_R::new(self.bits)
    }
}
#[doc = "Measured signature of blue channel for evaluation window 6.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblue6](index.html) module"]
pub struct SIGCRCBLUE6_SPEC;
impl crate::RegisterSpec for SIGCRCBLUE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblue6::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUE6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCBLUE6 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUE6_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
