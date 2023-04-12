#[doc = "Register `RX_UDP_CK_ERRORS` reader"]
pub struct R(crate::R<RX_UDP_CK_ERRORS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_UDP_CK_ERRORS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_UDP_CK_ERRORS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_UDP_CK_ERRORS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_UDPCK_ERR` reader - UDP checksum errors - an 8-bit register counting the number of frames discarded due to an incorrect UDP checksum, but are between 64 and 1518 bytes (1536 bytes if bit 8 is set in the network configuration register or 10240 bytes if bit 3 is in the network configuration register) and do not have a CRC error, an alignment error, nor a symbol error."]
pub type COUNT_UDPCK_ERR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - UDP checksum errors - an 8-bit register counting the number of frames discarded due to an incorrect UDP checksum, but are between 64 and 1518 bytes (1536 bytes if bit 8 is set in the network configuration register or 10240 bytes if bit 3 is in the network configuration register) and do not have a CRC error, an alignment error, nor a symbol error."]
    #[inline(always)]
    pub fn count_udpck_err(&self) -> COUNT_UDPCK_ERR_R {
        COUNT_UDPCK_ERR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UDP Checksum Errors\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_udp_ck_errors](index.html) module"]
pub struct RX_UDP_CK_ERRORS_SPEC;
impl crate::RegisterSpec for RX_UDP_CK_ERRORS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_udp_ck_errors::R](R) reader structure"]
impl crate::Readable for RX_UDP_CK_ERRORS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_UDP_CK_ERRORS to value 0"]
impl crate::Resettable for RX_UDP_CK_ERRORS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
