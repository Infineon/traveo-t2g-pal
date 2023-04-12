#[doc = "Register `INT_ENABLE` writer"]
pub struct W(crate::W<INT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_MANAGEMENT_DONE_INTERRUPT` writer - Enable management done interrupt"]
pub type ENABLE_MANAGEMENT_DONE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RECEIVE_COMPLETE_INTERRUPT` writer - Enable receive complete interrupt"]
pub type ENABLE_RECEIVE_COMPLETE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RECEIVE_USED_BIT_READ_INTERRUPT` writer - Enable receive used bit read interrupt"]
pub type ENABLE_RECEIVE_USED_BIT_READ_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_TRANSMIT_USED_BIT_READ_INTERRUPT` writer - Enable transmit used bit read interrupt"]
pub type ENABLE_TRANSMIT_USED_BIT_READ_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT` writer - Enable transmit buffer under run interrupt"]
pub type ENABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT` writer - Enable retry limit exceeded or late collision interrupt"]
pub type ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT` writer - Enable transmit frame corruption due to AMBA (AXI) error interrupt"]
pub type ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_TRANSMIT_COMPLETE_INTERRUPT` writer - Enable transmit complete interrupt"]
pub type ENABLE_TRANSMIT_COMPLETE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_8` writer - Not used"]
pub type UNUSED_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_9` writer - Not used"]
pub type UNUSED_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RECEIVE_OVERRUN_INTERRUPT` writer - Enable receive overrun interrupt"]
pub type ENABLE_RECEIVE_OVERRUN_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RESP_NOT_OK_INTERRUPT` writer - Enable bresp not OK interrupt"]
pub type ENABLE_RESP_NOT_OK_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT` writer - Enable pause frame with non-zero pause quantum interrupt"]
pub type ENABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PAUSE_TIME_ZERO_INTERRUPT` writer - Enable pause time zero interrupt"]
pub type ENABLE_PAUSE_TIME_ZERO_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT` writer - Enable pause frame transmitted interrupt"]
pub type ENABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_15` writer - Not used"]
pub type UNUSED_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_16` writer - Not used"]
pub type UNUSED_16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_17` writer - Not used"]
pub type UNUSED_17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PTP_DELAY_REQ_FRAME_RECEIVED` writer - Enable PTP delay_req frame received"]
pub type ENABLE_PTP_DELAY_REQ_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PTP_SYNC_FRAME_RECEIVED` writer - Enable PTP sync frame received"]
pub type ENABLE_PTP_SYNC_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED` writer - Enable PTP delay_req frame transmitted"]
pub type ENABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PTP_SYNC_FRAME_TRANSMITTED` writer - Enable PTP sync frame transmitted"]
pub type ENABLE_PTP_SYNC_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PTP_PDELAY_REQ_FRAME_RECEIVED` writer - Enable PTP pdelay_req frame received"]
pub type ENABLE_PTP_PDELAY_REQ_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PTP_PDELAY_RESP_FRAME_RECEIVED` writer - Enable PTP pdelay_resp frame received"]
pub type ENABLE_PTP_PDELAY_RESP_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED` writer - Enable PTP pdelay_req frame transmitted"]
pub type ENABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED` writer - Enable PTP pdelay_resp frame transmitted"]
pub type ENABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_TSU_SECONDS_REGISTER_INCREMENT` writer - Enable TSU seconds register increment"]
pub type ENABLE_TSU_SECONDS_REGISTER_INCREMENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_RX_LPI_INDICATION_INTERRUPT` writer - Enable RX LPI indication interrupt"]
pub type ENABLE_RX_LPI_INDICATION_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_28` writer - Not used"]
pub type UNUSED_28_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENABLE_TSU_TIMER_COMPARISON_INTERRUPT` writer - Enable TSU timer comparison interrupt."]
pub type ENABLE_TSU_TIMER_COMPARISON_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENABLE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Enable management done interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_management_done_interrupt(&mut self) -> ENABLE_MANAGEMENT_DONE_INTERRUPT_W<0> {
        ENABLE_MANAGEMENT_DONE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 1 - Enable receive complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_receive_complete_interrupt(&mut self) -> ENABLE_RECEIVE_COMPLETE_INTERRUPT_W<1> {
        ENABLE_RECEIVE_COMPLETE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 2 - Enable receive used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_receive_used_bit_read_interrupt(
        &mut self,
    ) -> ENABLE_RECEIVE_USED_BIT_READ_INTERRUPT_W<2> {
        ENABLE_RECEIVE_USED_BIT_READ_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 3 - Enable transmit used bit read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_transmit_used_bit_read_interrupt(
        &mut self,
    ) -> ENABLE_TRANSMIT_USED_BIT_READ_INTERRUPT_W<3> {
        ENABLE_TRANSMIT_USED_BIT_READ_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 4 - Enable transmit buffer under run interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_transmit_buffer_under_run_interrupt(
        &mut self,
    ) -> ENABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_W<4> {
        ENABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 5 - Enable retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_retry_limit_exceeded_or_late_collision_interrupt(
        &mut self,
    ) -> ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W<5> {
        ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 6 - Enable transmit frame corruption due to AMBA (AXI) error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_transmit_frame_corruption_due_to_amba_error_interrupt(
        &mut self,
    ) -> ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W<6> {
        ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 7 - Enable transmit complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_transmit_complete_interrupt(
        &mut self,
    ) -> ENABLE_TRANSMIT_COMPLETE_INTERRUPT_W<7> {
        ENABLE_TRANSMIT_COMPLETE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 8 - Not used"]
    #[inline(always)]
    #[must_use]
    pub fn unused_8(&mut self) -> UNUSED_8_W<8> {
        UNUSED_8_W::new(self)
    }
    #[doc = "Bit 9 - Not used"]
    #[inline(always)]
    #[must_use]
    pub fn unused_9(&mut self) -> UNUSED_9_W<9> {
        UNUSED_9_W::new(self)
    }
    #[doc = "Bit 10 - Enable receive overrun interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_receive_overrun_interrupt(&mut self) -> ENABLE_RECEIVE_OVERRUN_INTERRUPT_W<10> {
        ENABLE_RECEIVE_OVERRUN_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 11 - Enable bresp not OK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_resp_not_ok_interrupt(&mut self) -> ENABLE_RESP_NOT_OK_INTERRUPT_W<11> {
        ENABLE_RESP_NOT_OK_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 12 - Enable pause frame with non-zero pause quantum interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_pause_frame_with_non_zero_pause_quantum_interrupt(
        &mut self,
    ) -> ENABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_W<12> {
        ENABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 13 - Enable pause time zero interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_pause_time_zero_interrupt(&mut self) -> ENABLE_PAUSE_TIME_ZERO_INTERRUPT_W<13> {
        ENABLE_PAUSE_TIME_ZERO_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 14 - Enable pause frame transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_pause_frame_transmitted_interrupt(
        &mut self,
    ) -> ENABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT_W<14> {
        ENABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 15 - Not used"]
    #[inline(always)]
    #[must_use]
    pub fn unused_15(&mut self) -> UNUSED_15_W<15> {
        UNUSED_15_W::new(self)
    }
    #[doc = "Bit 16 - Not used"]
    #[inline(always)]
    #[must_use]
    pub fn unused_16(&mut self) -> UNUSED_16_W<16> {
        UNUSED_16_W::new(self)
    }
    #[doc = "Bit 17 - Not used"]
    #[inline(always)]
    #[must_use]
    pub fn unused_17(&mut self) -> UNUSED_17_W<17> {
        UNUSED_17_W::new(self)
    }
    #[doc = "Bit 18 - Enable PTP delay_req frame received"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ptp_delay_req_frame_received(
        &mut self,
    ) -> ENABLE_PTP_DELAY_REQ_FRAME_RECEIVED_W<18> {
        ENABLE_PTP_DELAY_REQ_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 19 - Enable PTP sync frame received"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ptp_sync_frame_received(&mut self) -> ENABLE_PTP_SYNC_FRAME_RECEIVED_W<19> {
        ENABLE_PTP_SYNC_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 20 - Enable PTP delay_req frame transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ptp_delay_req_frame_transmitted(
        &mut self,
    ) -> ENABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED_W<20> {
        ENABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 21 - Enable PTP sync frame transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ptp_sync_frame_transmitted(&mut self) -> ENABLE_PTP_SYNC_FRAME_TRANSMITTED_W<21> {
        ENABLE_PTP_SYNC_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 22 - Enable PTP pdelay_req frame received"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ptp_pdelay_req_frame_received(
        &mut self,
    ) -> ENABLE_PTP_PDELAY_REQ_FRAME_RECEIVED_W<22> {
        ENABLE_PTP_PDELAY_REQ_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 23 - Enable PTP pdelay_resp frame received"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ptp_pdelay_resp_frame_received(
        &mut self,
    ) -> ENABLE_PTP_PDELAY_RESP_FRAME_RECEIVED_W<23> {
        ENABLE_PTP_PDELAY_RESP_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 24 - Enable PTP pdelay_req frame transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ptp_pdelay_req_frame_transmitted(
        &mut self,
    ) -> ENABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED_W<24> {
        ENABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 25 - Enable PTP pdelay_resp frame transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ptp_pdelay_resp_frame_transmitted(
        &mut self,
    ) -> ENABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED_W<25> {
        ENABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 26 - Enable TSU seconds register increment"]
    #[inline(always)]
    #[must_use]
    pub fn enable_tsu_seconds_register_increment(
        &mut self,
    ) -> ENABLE_TSU_SECONDS_REGISTER_INCREMENT_W<26> {
        ENABLE_TSU_SECONDS_REGISTER_INCREMENT_W::new(self)
    }
    #[doc = "Bit 27 - Enable RX LPI indication interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_rx_lpi_indication_interrupt(
        &mut self,
    ) -> ENABLE_RX_LPI_INDICATION_INTERRUPT_W<27> {
        ENABLE_RX_LPI_INDICATION_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 28 - Not used"]
    #[inline(always)]
    #[must_use]
    pub fn unused_28(&mut self) -> UNUSED_28_W<28> {
        UNUSED_28_W::new(self)
    }
    #[doc = "Bit 29 - Enable TSU timer comparison interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn enable_tsu_timer_comparison_interrupt(
        &mut self,
    ) -> ENABLE_TSU_TIMER_COMPARISON_INTERRUPT_W<29> {
        ENABLE_TSU_TIMER_COMPARISON_INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_enable](index.html) module"]
pub struct INT_ENABLE_SPEC;
impl crate::RegisterSpec for INT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_enable::W](W) writer structure"]
impl crate::Writable for INT_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENABLE to value 0"]
impl crate::Resettable for INT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
