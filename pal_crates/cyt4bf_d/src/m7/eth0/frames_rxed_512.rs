#[doc = "Register `FRAMES_RXED_512` reader"]
pub struct R(crate::R<FRAMES_RXED_512_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMES_RXED_512_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMES_RXED_512_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMES_RXED_512_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_512` reader - 512 to 1023 byte frames received without error. A 32 bit register counting the number of 512 to 1023 byte frames successfully received without error. Excludes pause frames, and is only incremented if the frame is successfully filtered."]
pub type COUNT_512_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 512 to 1023 byte frames received without error. A 32 bit register counting the number of 512 to 1023 byte frames successfully received without error. Excludes pause frames, and is only incremented if the frame is successfully filtered."]
    #[inline(always)]
    pub fn count_512(&self) -> COUNT_512_R {
        COUNT_512_R::new(self.bits)
    }
}
#[doc = "512 to 1023 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frames_rxed_512](index.html) module"]
pub struct FRAMES_RXED_512_SPEC;
impl crate::RegisterSpec for FRAMES_RXED_512_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frames_rxed_512::R](R) reader structure"]
impl crate::Readable for FRAMES_RXED_512_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAMES_RXED_512 to value 0"]
impl crate::Resettable for FRAMES_RXED_512_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
