#[doc = "Register `BROADCAST_RXED` reader"]
pub struct R(crate::R<BROADCAST_RXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROADCAST_RXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROADCAST_RXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROADCAST_RXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_BROADCAST` reader - Broadcast frames received without error. A 32 bit register counting the number of broadcast frames successfully received without error. Excludes pause frames, and is only incremented if the frame is successfully filtered."]
pub type COUNT_BROADCAST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast frames received without error. A 32 bit register counting the number of broadcast frames successfully received without error. Excludes pause frames, and is only incremented if the frame is successfully filtered."]
    #[inline(always)]
    pub fn count_broadcast(&self) -> COUNT_BROADCAST_R {
        COUNT_BROADCAST_R::new(self.bits)
    }
}
#[doc = "Broadcast Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [broadcast_rxed](index.html) module"]
pub struct BROADCAST_RXED_SPEC;
impl crate::RegisterSpec for BROADCAST_RXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [broadcast_rxed::R](R) reader structure"]
impl crate::Readable for BROADCAST_RXED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BROADCAST_RXED to value 0"]
impl crate::Resettable for BROADCAST_RXED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
