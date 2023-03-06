#[doc = "Register `FRAMES_RXED_128` reader"]
pub struct R(crate::R<FRAMES_RXED_128_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMES_RXED_128_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMES_RXED_128_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMES_RXED_128_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_128` reader - 128 to 255 byte frames received without error. A 32 bit register counting the number of 128 to 255 byte frames successfully received without error. Excludes pause frames, and is only incremented if the frame is successfully filtered."]
pub type COUNT_128_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 128 to 255 byte frames received without error. A 32 bit register counting the number of 128 to 255 byte frames successfully received without error. Excludes pause frames, and is only incremented if the frame is successfully filtered."]
    #[inline(always)]
    pub fn count_128(&self) -> COUNT_128_R {
        COUNT_128_R::new(self.bits)
    }
}
#[doc = "128 to 255 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frames_rxed_128](index.html) module"]
pub struct FRAMES_RXED_128_SPEC;
impl crate::RegisterSpec for FRAMES_RXED_128_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frames_rxed_128::R](R) reader structure"]
impl crate::Readable for FRAMES_RXED_128_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAMES_RXED_128 to value 0"]
impl crate::Resettable for FRAMES_RXED_128_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
