#[doc = "Register `RX_JABBERS` reader"]
pub struct R(crate::R<RX_JABBERS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_JABBERS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_JABBERS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_JABBERS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_JABBERS` reader - Jabbers received - a 10 bit register counting the number of frames received exceeding 1518 bytes (1536 if bit 8 set in network configuration register, 10,240 bytes if bit 3 is set in the network configuration register) in length and have either a CRC error, an alignment error or a receive symbol error."]
pub type COUNT_JABBERS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Jabbers received - a 10 bit register counting the number of frames received exceeding 1518 bytes (1536 if bit 8 set in network configuration register, 10,240 bytes if bit 3 is set in the network configuration register) in length and have either a CRC error, an alignment error or a receive symbol error."]
    #[inline(always)]
    pub fn count_jabbers(&self) -> COUNT_JABBERS_R {
        COUNT_JABBERS_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Jabbers Received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_jabbers](index.html) module"]
pub struct RX_JABBERS_SPEC;
impl crate::RegisterSpec for RX_JABBERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_jabbers::R](R) reader structure"]
impl crate::Readable for RX_JABBERS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_JABBERS to value 0"]
impl crate::Resettable for RX_JABBERS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
