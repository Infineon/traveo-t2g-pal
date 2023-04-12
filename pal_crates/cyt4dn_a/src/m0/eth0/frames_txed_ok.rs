#[doc = "Register `FRAMES_TXED_OK` reader"]
pub struct R(crate::R<FRAMES_TXED_OK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMES_TXED_OK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMES_TXED_OK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMES_TXED_OK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_OK` reader - Frames transmitted without error. A 32 bit register counting the number of frames successfully transmitted, i.e. no under run and not too many retries. Excludes pause frames."]
pub type COUNT_OK_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Frames transmitted without error. A 32 bit register counting the number of frames successfully transmitted, i.e. no under run and not too many retries. Excludes pause frames."]
    #[inline(always)]
    pub fn count_ok(&self) -> COUNT_OK_R {
        COUNT_OK_R::new(self.bits)
    }
}
#[doc = "Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frames_txed_ok](index.html) module"]
pub struct FRAMES_TXED_OK_SPEC;
impl crate::RegisterSpec for FRAMES_TXED_OK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frames_txed_ok::R](R) reader structure"]
impl crate::Readable for FRAMES_TXED_OK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAMES_TXED_OK to value 0"]
impl crate::Resettable for FRAMES_TXED_OK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
