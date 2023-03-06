#[doc = "Register `SIGCRCBLUE5` reader"]
pub struct R(crate::R<SIGCRCBLUE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCBLUE5` reader - See SigCRCBlue0."]
pub type SIGCRCBLUE5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlue0."]
    #[inline(always)]
    pub fn sigcrcblue5(&self) -> SIGCRCBLUE5_R {
        SIGCRCBLUE5_R::new(self.bits)
    }
}
#[doc = "Measured signature of blue channel for evaluation window 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblue5](index.html) module"]
pub struct SIGCRCBLUE5_SPEC;
impl crate::RegisterSpec for SIGCRCBLUE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblue5::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUE5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCBLUE5 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUE5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
