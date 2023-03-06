#[doc = "Register `RX_RESOURCE_ERRORS` reader"]
pub struct R(crate::R<RX_RESOURCE_ERRORS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_RESOURCE_ERRORS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_RESOURCE_ERRORS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_RESOURCE_ERRORS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_RESOURCE_ERR` reader - Receive resource errors - an 18 bit register counting the number of frames that were successfully received by the MAC (correct address matched frame and adequate slot time) but could not be copied to memory because no receive buffer was available. This occurs when the GEM reads a buffer descriptor with its ownership (or used) bit set."]
pub type COUNT_RESOURCE_ERR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Receive resource errors - an 18 bit register counting the number of frames that were successfully received by the MAC (correct address matched frame and adequate slot time) but could not be copied to memory because no receive buffer was available. This occurs when the GEM reads a buffer descriptor with its ownership (or used) bit set."]
    #[inline(always)]
    pub fn count_resource_err(&self) -> COUNT_RESOURCE_ERR_R {
        COUNT_RESOURCE_ERR_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Receive Resource Errors\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_resource_errors](index.html) module"]
pub struct RX_RESOURCE_ERRORS_SPEC;
impl crate::RegisterSpec for RX_RESOURCE_ERRORS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_resource_errors::R](R) reader structure"]
impl crate::Readable for RX_RESOURCE_ERRORS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_RESOURCE_ERRORS to value 0"]
impl crate::Resettable for RX_RESOURCE_ERRORS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
