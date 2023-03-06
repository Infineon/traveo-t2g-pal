#[doc = "Register `SIGCRCRED5` reader"]
pub struct R(crate::R<SIGCRCRED5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCRED5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCRED5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCRED5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCRED5` reader - See SigCRCRed0."]
pub type SIGCRCRED5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCRed0."]
    #[inline(always)]
    pub fn sigcrcred5(&self) -> SIGCRCRED5_R {
        SIGCRCRED5_R::new(self.bits)
    }
}
#[doc = "Measured signature of red channel for evaluation window 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcred5](index.html) module"]
pub struct SIGCRCRED5_SPEC;
impl crate::RegisterSpec for SIGCRCRED5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcred5::R](R) reader structure"]
impl crate::Readable for SIGCRCRED5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCRED5 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCRED5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
