#[doc = "Register `RX_LENGTH_ERRORS` reader"]
pub struct R(crate::R<RX_LENGTH_ERRORS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_LENGTH_ERRORS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_LENGTH_ERRORS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_LENGTH_ERRORS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_LENGTH_ERR` reader - Length field frame errors - this 10-bit register counts the number of frames received that have a measured length shorter than that extracted from the length field (bytes 13 and 14). This condition is only counted if the value of the length field is less than 0x0600, the frame is not of excessive length and checking is enabled through bit 16 of the network configuration register."]
pub type COUNT_LENGTH_ERR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Length field frame errors - this 10-bit register counts the number of frames received that have a measured length shorter than that extracted from the length field (bytes 13 and 14). This condition is only counted if the value of the length field is less than 0x0600, the frame is not of excessive length and checking is enabled through bit 16 of the network configuration register."]
    #[inline(always)]
    pub fn count_length_err(&self) -> COUNT_LENGTH_ERR_R {
        COUNT_LENGTH_ERR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Length Field Frame Errors\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_length_errors](index.html) module"]
pub struct RX_LENGTH_ERRORS_SPEC;
impl crate::RegisterSpec for RX_LENGTH_ERRORS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_length_errors::R](R) reader structure"]
impl crate::Readable for RX_LENGTH_ERRORS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_LENGTH_ERRORS to value 0"]
impl crate::Resettable for RX_LENGTH_ERRORS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
