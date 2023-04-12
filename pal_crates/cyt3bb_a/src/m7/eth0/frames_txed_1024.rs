#[doc = "Register `FRAMES_TXED_1024` reader"]
pub struct R(crate::R<FRAMES_TXED_1024_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMES_TXED_1024_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMES_TXED_1024_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMES_TXED_1024_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_1024` reader - 1024 to 1518 byte frames transmitted without error. A 32 bit register counting the number of 1024 to 1518 byte frames successfully transmitted without error, i.e. no under run and not too many retries."]
pub type COUNT_1024_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1024 to 1518 byte frames transmitted without error. A 32 bit register counting the number of 1024 to 1518 byte frames successfully transmitted without error, i.e. no under run and not too many retries."]
    #[inline(always)]
    pub fn count_1024(&self) -> COUNT_1024_R {
        COUNT_1024_R::new(self.bits)
    }
}
#[doc = "1024 to 1518 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frames_txed_1024](index.html) module"]
pub struct FRAMES_TXED_1024_SPEC;
impl crate::RegisterSpec for FRAMES_TXED_1024_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frames_txed_1024::R](R) reader structure"]
impl crate::Readable for FRAMES_TXED_1024_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAMES_TXED_1024 to value 0"]
impl crate::Resettable for FRAMES_TXED_1024_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
