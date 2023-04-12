#[doc = "Register `MULTICAST_RXED` reader"]
pub struct R(crate::R<MULTICAST_RXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULTICAST_RXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULTICAST_RXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULTICAST_RXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_MULTICAST` reader - Multicast frames received without error. A 32 bit register counting the number of multicast frames successfully received without error. Excludes pause frames, and is only incremented if the frame is successfully filtered."]
pub type COUNT_MULTICAST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast frames received without error. A 32 bit register counting the number of multicast frames successfully received without error. Excludes pause frames, and is only incremented if the frame is successfully filtered."]
    #[inline(always)]
    pub fn count_multicast(&self) -> COUNT_MULTICAST_R {
        COUNT_MULTICAST_R::new(self.bits)
    }
}
#[doc = "Multicast Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [multicast_rxed](index.html) module"]
pub struct MULTICAST_RXED_SPEC;
impl crate::RegisterSpec for MULTICAST_RXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [multicast_rxed::R](R) reader structure"]
impl crate::Readable for MULTICAST_RXED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MULTICAST_RXED to value 0"]
impl crate::Resettable for MULTICAST_RXED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
