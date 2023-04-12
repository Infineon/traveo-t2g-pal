#[doc = "Register `EXCESSIVE_RX_LENGTH` reader"]
pub struct R(crate::R<EXCESSIVE_RX_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXCESSIVE_RX_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXCESSIVE_RX_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXCESSIVE_RX_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_OVERSIZE` reader - Oversize frames received - a 10 bit register counting the number of frames received exceeding 1518 bytes (1536 bytes if bit 8 is set in network configuration register, 10,240 bytes if bit 3 is set in the network configuration register) in length but do not have either a CRC error, an alignment error nor a receive symbol error."]
pub type COUNT_OVERSIZE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Oversize frames received - a 10 bit register counting the number of frames received exceeding 1518 bytes (1536 bytes if bit 8 is set in network configuration register, 10,240 bytes if bit 3 is set in the network configuration register) in length but do not have either a CRC error, an alignment error nor a receive symbol error."]
    #[inline(always)]
    pub fn count_oversize(&self) -> COUNT_OVERSIZE_R {
        COUNT_OVERSIZE_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Oversize Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [excessive_rx_length](index.html) module"]
pub struct EXCESSIVE_RX_LENGTH_SPEC;
impl crate::RegisterSpec for EXCESSIVE_RX_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [excessive_rx_length::R](R) reader structure"]
impl crate::Readable for EXCESSIVE_RX_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXCESSIVE_RX_LENGTH to value 0"]
impl crate::Resettable for EXCESSIVE_RX_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
