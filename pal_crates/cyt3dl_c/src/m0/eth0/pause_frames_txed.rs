#[doc = "Register `PAUSE_FRAMES_TXED` reader"]
pub struct R(crate::R<PAUSE_FRAMES_TXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUSE_FRAMES_TXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUSE_FRAMES_TXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUSE_FRAMES_TXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_PAUSE` reader - Transmitted pause frames - a 16 bit register counting the number of pause frames transmitted. Only pause frames triggered by the register interface or through the external pause pins are counted as pause frames. Pause frames received through the external FIFO interface are counted in the frames transmitted counter."]
pub type COUNT_PAUSE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted pause frames - a 16 bit register counting the number of pause frames transmitted. Only pause frames triggered by the register interface or through the external pause pins are counted as pause frames. Pause frames received through the external FIFO interface are counted in the frames transmitted counter."]
    #[inline(always)]
    pub fn count_pause(&self) -> COUNT_PAUSE_R {
        COUNT_PAUSE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Pause Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pause_frames_txed](index.html) module"]
pub struct PAUSE_FRAMES_TXED_SPEC;
impl crate::RegisterSpec for PAUSE_FRAMES_TXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pause_frames_txed::R](R) reader structure"]
impl crate::Readable for PAUSE_FRAMES_TXED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PAUSE_FRAMES_TXED to value 0"]
impl crate::Resettable for PAUSE_FRAMES_TXED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
