#[doc = "Register `BROADCAST_TXED` reader"]
pub struct R(crate::R<BROADCAST_TXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROADCAST_TXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROADCAST_TXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROADCAST_TXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_BROADCAST` reader - Broadcast frames transmitted without error. A 32 bit register counting the number of broadcast frames successfully transmitted without error, i.e. no under run and not too many retries. Excludes pause frames."]
pub type COUNT_BROADCAST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast frames transmitted without error. A 32 bit register counting the number of broadcast frames successfully transmitted without error, i.e. no under run and not too many retries. Excludes pause frames."]
    #[inline(always)]
    pub fn count_broadcast(&self) -> COUNT_BROADCAST_R {
        COUNT_BROADCAST_R::new(self.bits)
    }
}
#[doc = "Broadcast Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [broadcast_txed](index.html) module"]
pub struct BROADCAST_TXED_SPEC;
impl crate::RegisterSpec for BROADCAST_TXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [broadcast_txed::R](R) reader structure"]
impl crate::Readable for BROADCAST_TXED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BROADCAST_TXED to value 0"]
impl crate::Resettable for BROADCAST_TXED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
