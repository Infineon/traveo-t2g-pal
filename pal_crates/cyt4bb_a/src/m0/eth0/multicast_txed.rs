#[doc = "Register `MULTICAST_TXED` reader"]
pub struct R(crate::R<MULTICAST_TXED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULTICAST_TXED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULTICAST_TXED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULTICAST_TXED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_MULTICAST` reader - Multicast frames transmitted without error. A 32 bit register counting the number of multicast frames successfully transmitted without error, i.e. no under run and not too many retries. Excludes pause frames."]
pub type COUNT_MULTICAST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast frames transmitted without error. A 32 bit register counting the number of multicast frames successfully transmitted without error, i.e. no under run and not too many retries. Excludes pause frames."]
    #[inline(always)]
    pub fn count_multicast(&self) -> COUNT_MULTICAST_R {
        COUNT_MULTICAST_R::new(self.bits)
    }
}
#[doc = "Multicast Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [multicast_txed](index.html) module"]
pub struct MULTICAST_TXED_SPEC;
impl crate::RegisterSpec for MULTICAST_TXED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [multicast_txed::R](R) reader structure"]
impl crate::Readable for MULTICAST_TXED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MULTICAST_TXED to value 0"]
impl crate::Resettable for MULTICAST_TXED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
