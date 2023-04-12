#[doc = "Register `TSU_PTP_RX_MSB_SEC` reader"]
pub struct R(crate::R<TSU_PTP_RX_MSB_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_PTP_RX_MSB_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_PTP_RX_MSB_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_PTP_RX_MSB_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_SECONDS` reader - PTP Event Frame TX Seconds. The register is updated with the value that the 1588 timer seconds register held when the SFD of a PTP receive primary event crosses the MII interface. The actual update occurs when the GEM recognizes the frame as a PTP sync or delay_req frame. An interrupt is issued when the register is updated."]
pub type TIMER_SECONDS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PTP Event Frame TX Seconds. The register is updated with the value that the 1588 timer seconds register held when the SFD of a PTP receive primary event crosses the MII interface. The actual update occurs when the GEM recognizes the frame as a PTP sync or delay_req frame. An interrupt is issued when the register is updated."]
    #[inline(always)]
    pub fn timer_seconds(&self) -> TIMER_SECONDS_R {
        TIMER_SECONDS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "PTP Event Frame Received Seconds Register (47 to 32 bits)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_ptp_rx_msb_sec](index.html) module"]
pub struct TSU_PTP_RX_MSB_SEC_SPEC;
impl crate::RegisterSpec for TSU_PTP_RX_MSB_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_ptp_rx_msb_sec::R](R) reader structure"]
impl crate::Readable for TSU_PTP_RX_MSB_SEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSU_PTP_RX_MSB_SEC to value 0"]
impl crate::Resettable for TSU_PTP_RX_MSB_SEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
