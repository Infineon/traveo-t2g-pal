#[doc = "Register `SIGCRCBLUE2` reader"]
pub struct R(crate::R<SIGCRCBLUE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCBLUE2` reader - See SigCRCBlue0."]
pub type SIGCRCBLUE2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCBlue0."]
    #[inline(always)]
    pub fn sigcrcblue2(&self) -> SIGCRCBLUE2_R {
        SIGCRCBLUE2_R::new(self.bits)
    }
}
#[doc = "Measured signature of blue channel for evaluation window 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblue2](index.html) module"]
pub struct SIGCRCBLUE2_SPEC;
impl crate::RegisterSpec for SIGCRCBLUE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblue2::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCBLUE2 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUE2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
