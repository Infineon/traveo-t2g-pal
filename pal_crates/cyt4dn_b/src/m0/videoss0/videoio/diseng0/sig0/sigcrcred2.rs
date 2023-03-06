#[doc = "Register `SIGCRCRED2` reader"]
pub struct R(crate::R<SIGCRCRED2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCRED2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCRED2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCRED2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCRED2` reader - See SigCRCRed0."]
pub type SIGCRCRED2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCRed0."]
    #[inline(always)]
    pub fn sigcrcred2(&self) -> SIGCRCRED2_R {
        SIGCRCRED2_R::new(self.bits)
    }
}
#[doc = "Measured signature of red channel for evaluation window 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcred2](index.html) module"]
pub struct SIGCRCRED2_SPEC;
impl crate::RegisterSpec for SIGCRCRED2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcred2::R](R) reader structure"]
impl crate::Readable for SIGCRCRED2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCRED2 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCRED2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
