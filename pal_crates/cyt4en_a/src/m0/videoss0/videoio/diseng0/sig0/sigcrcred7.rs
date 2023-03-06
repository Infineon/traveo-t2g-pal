#[doc = "Register `SIGCRCRED7` reader"]
pub struct R(crate::R<SIGCRCRED7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCRED7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCRED7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCRED7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCRED7` reader - See SigCRCRed0."]
pub type SIGCRCRED7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCRed0."]
    #[inline(always)]
    pub fn sigcrcred7(&self) -> SIGCRCRED7_R {
        SIGCRCRED7_R::new(self.bits)
    }
}
#[doc = "Measured signature of red channel for evaluation window 7.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcred7](index.html) module"]
pub struct SIGCRCRED7_SPEC;
impl crate::RegisterSpec for SIGCRCRED7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcred7::R](R) reader structure"]
impl crate::Readable for SIGCRCRED7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCRED7 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCRED7_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
