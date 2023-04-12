#[doc = "Register `TSU_PEER_TX_NSEC` reader"]
pub struct R(crate::R<TSU_PEER_TX_NSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_PEER_TX_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_PEER_TX_NSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_PEER_TX_NSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_PEER_NSEC` reader - PTP Peer Event Frame Transmitted Nanoseconds. The register is updated with the value that the 1588 timer nanoseconds register held when the SFD of a PTP transmit peer event crosses the MII interface. The actual update occurs when the GEM recognizes the frame as a PTP pdelay_req or pdelay_resp frame. An interrupt is issued when the register is updated."]
pub type TIMER_PEER_NSEC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Peer Event Frame Transmitted Nanoseconds. The register is updated with the value that the 1588 timer nanoseconds register held when the SFD of a PTP transmit peer event crosses the MII interface. The actual update occurs when the GEM recognizes the frame as a PTP pdelay_req or pdelay_resp frame. An interrupt is issued when the register is updated."]
    #[inline(always)]
    pub fn timer_peer_nsec(&self) -> TIMER_PEER_NSEC_R {
        TIMER_PEER_NSEC_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_peer_tx_nsec](index.html) module"]
pub struct TSU_PEER_TX_NSEC_SPEC;
impl crate::RegisterSpec for TSU_PEER_TX_NSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_peer_tx_nsec::R](R) reader structure"]
impl crate::Readable for TSU_PEER_TX_NSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSU_PEER_TX_NSEC to value 0"]
impl crate::Resettable for TSU_PEER_TX_NSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
