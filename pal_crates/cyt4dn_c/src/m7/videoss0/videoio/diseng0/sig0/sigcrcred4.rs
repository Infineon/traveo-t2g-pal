#[doc = "Register `SIGCRCRED4` reader"]
pub struct R(crate::R<SIGCRCRED4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCRED4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCRED4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCRED4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCRED4` reader - See SigCRCRed0."]
pub type SIGCRCRED4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCRed0."]
    #[inline(always)]
    pub fn sigcrcred4(&self) -> SIGCRCRED4_R {
        SIGCRCRED4_R::new(self.bits)
    }
}
#[doc = "Measured signature of red channel for evaluation window 4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcred4](index.html) module"]
pub struct SIGCRCRED4_SPEC;
impl crate::RegisterSpec for SIGCRCRED4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcred4::R](R) reader structure"]
impl crate::Readable for SIGCRCRED4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCRED4 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCRED4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
