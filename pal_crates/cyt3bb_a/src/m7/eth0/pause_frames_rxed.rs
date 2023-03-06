#[doc = "Register `PAUSE_FRAMES_RXED` reader"]
pub struct R(crate::R<PAUSE_FRAMES_RXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUSE_FRAMES_RXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUSE_FRAMES_RXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUSE_FRAMES_RXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_PAUSE` reader - Received pause frames - a 16 bit register counting the number of pause frames received without error."]
pub type COUNT_PAUSE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received pause frames - a 16 bit register counting the number of pause frames received without error."]
    #[inline(always)]
    pub fn count_pause(&self) -> COUNT_PAUSE_R {
        COUNT_PAUSE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pause Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pause_frames_rxed](index.html) module"]
pub struct PAUSE_FRAMES_RXED_SPEC;
impl crate::RegisterSpec for PAUSE_FRAMES_RXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pause_frames_rxed::R](R) reader structure"]
impl crate::Readable for PAUSE_FRAMES_RXED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PAUSE_FRAMES_RXED to value 0"]
impl crate::Resettable for PAUSE_FRAMES_RXED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
