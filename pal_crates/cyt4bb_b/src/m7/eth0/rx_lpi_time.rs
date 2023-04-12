#[doc = "Register `RX_LPI_TIME` reader"]
pub struct R(crate::R<RX_LPI_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_LPI_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_LPI_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_LPI_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPI_TIME` reader - Time in LPI. This register increments once every 16 pclk cycles when the LPI indication bit 20 is set in the receive configuration register. Cleared on read."]
pub type LPI_TIME_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Time in LPI. This register increments once every 16 pclk cycles when the LPI indication bit 20 is set in the receive configuration register. Cleared on read."]
    #[inline(always)]
    pub fn lpi_time(&self) -> LPI_TIME_R {
        LPI_TIME_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Received LPI time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_lpi_time](index.html) module"]
pub struct RX_LPI_TIME_SPEC;
impl crate::RegisterSpec for RX_LPI_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_lpi_time::R](R) reader structure"]
impl crate::Readable for RX_LPI_TIME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_LPI_TIME to value 0"]
impl crate::Resettable for RX_LPI_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
