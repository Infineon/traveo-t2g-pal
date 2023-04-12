#[doc = "Register `SIGCRCGREEN4` reader"]
pub struct R(crate::R<SIGCRCGREEN4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREEN4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREEN4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREEN4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCGREEN4` reader - See SigCRCGreen0."]
pub type SIGCRCGREEN4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See SigCRCGreen0."]
    #[inline(always)]
    pub fn sigcrcgreen4(&self) -> SIGCRCGREEN4_R {
        SIGCRCGREEN4_R::new(self.bits)
    }
}
#[doc = "Measured signature of green channel for evaluation window 4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreen4](index.html) module"]
pub struct SIGCRCGREEN4_SPEC;
impl crate::RegisterSpec for SIGCRCGREEN4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreen4::R](R) reader structure"]
impl crate::Readable for SIGCRCGREEN4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCGREEN4 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREEN4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
