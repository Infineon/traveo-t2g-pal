#[doc = "Register `SIGCRCBLUE4` reader"]
pub struct R(crate::R<SIGCRCBLUE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCBLUE4` reader - See SigCRCBlue0."]
pub type SIGCRCBLUE4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlue0."]
    #[inline(always)]
    pub fn sigcrcblue4(&self) -> SIGCRCBLUE4_R {
        SIGCRCBLUE4_R::new(self.bits)
    }
}
#[doc = "Measured signature of blue channel for evaluation window 4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblue4](index.html) module"]
pub struct SIGCRCBLUE4_SPEC;
impl crate::RegisterSpec for SIGCRCBLUE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblue4::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUE4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCBLUE4 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUE4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
