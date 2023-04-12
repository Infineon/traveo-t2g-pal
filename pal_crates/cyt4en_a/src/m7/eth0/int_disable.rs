#[doc = "Register `INT_DISABLE` reader"]
pub struct R(crate::R<INT_DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_DISABLE` writer"]
pub struct W(crate::W<INT_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_DISABLE_SPEC>;
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
impl From<crate::W<INT_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISABLE_MANAGEMENT_DONE_INTERRUPT` writer - 'Disable management done interrupt'"]
pub type DISABLE_MANAGEMENT_DONE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RECEIVE_COMPLETE_INTERRUPT` writer - 'Disable receive complete interrupt'"]
pub type DISABLE_RECEIVE_COMPLETE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RECEIVE_USED_BIT_READ_INTERRUPT` writer - 'Disable receive used bit read interrupt'"]
pub type DISABLE_RECEIVE_USED_BIT_READ_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_TRANSMIT_USED_BIT_READ_INTERRUPT` writer - 'Disable transmit used bit read interrupt'"]
pub type DISABLE_TRANSMIT_USED_BIT_READ_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT` writer - 'Disable transmit buffer under run interrupt'"]
pub type DISABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT` writer - 'Disable retry limit exceeded or late collision interrupt'"]
pub type DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT` writer - 'Disable transmit frame corruption due to AMBA (AHB/AXI) error interrupt'"]
pub type DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_TRANSMIT_COMPLETE_INTERRUPT` writer - 'Disable transmit complete interrupt'"]
pub type DISABLE_TRANSMIT_COMPLETE_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_8` writer - Not used"]
pub type UNUSED_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_9` writer - Not used"]
pub type UNUSED_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RECEIVE_OVERRUN_INTERRUPT` writer - 'Disable receive overrun interrupt'"]
pub type DISABLE_RECEIVE_OVERRUN_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RESP_NOT_OK_INTERRUPT` writer - 'Disable bresp/hresp not OK interrupt'"]
pub type DISABLE_RESP_NOT_OK_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT` writer - 'Disable pause frame with non-zero pause quantum interrupt'"]
pub type DISABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PAUSE_TIME_ZERO_INTERRUPT` writer - 'Disable pause time zero interrupt'"]
pub type DISABLE_PAUSE_TIME_ZERO_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT` writer - 'Disable pause frame transmitted interrupt'"]
pub type DISABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_15` writer - Not used"]
pub type UNUSED_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_16` writer - Not used"]
pub type UNUSED_16_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_17` writer - Not used"]
pub type UNUSED_17_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PTP_DELAY_REQ_FRAME_RECEIVED` writer - 'Disable PTP delay_req frame received'"]
pub type DISABLE_PTP_DELAY_REQ_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PTP_SYNC_FRAME_RECEIVED` writer - 'Disable PTP sync frame received'"]
pub type DISABLE_PTP_SYNC_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED` writer - 'Disable PTP delay_req frame transmitted '"]
pub type DISABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PTP_SYNC_FRAME_TRANSMITTED` writer - 'Disable PTP sync frame transmitted '"]
pub type DISABLE_PTP_SYNC_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PTP_PDELAY_REQ_FRAME_RECEIVED` writer - 'Disable PTP pdelay_req frame received'"]
pub type DISABLE_PTP_PDELAY_REQ_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PTP_PDELAY_RESP_FRAME_RECEIVED` writer - 'Disable PTP pdelay_resp frame received'"]
pub type DISABLE_PTP_PDELAY_RESP_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED` writer - 'Disable PTP pdelay_req frame transmitted'"]
pub type DISABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED` writer - 'Disable PTP pdelay_resp frame transmitted'"]
pub type DISABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_TSU_SECONDS_REGISTER_INCREMENT` writer - 'Disable TSU seconds register increment'"]
pub type DISABLE_TSU_SECONDS_REGISTER_INCREMENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_RX_LPI_INDICATION_INTERRUPT` writer - 'Disable RX LPI indication interrupt'"]
pub type DISABLE_RX_LPI_INDICATION_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `UNUSED_28` writer - Not used"]
pub type UNUSED_28_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `DISABLE_TSU_TIMER_COMPARISON_INTERRUPT` writer - 'Disable TSU timer comparison interrupt.'"]
pub type DISABLE_TSU_TIMER_COMPARISON_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_DISABLE_SPEC, bool, O>;
#[doc = "Field `RSVD_30_30` reader - N/A"]
pub type RSVD_30_30_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_31_31` reader - N/A"]
pub type RSVD_31_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn rsvd_30_30(&self) -> RSVD_30_30_R {
        RSVD_30_30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn rsvd_31_31(&self) -> RSVD_31_31_R {
        RSVD_31_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 'Disable management done interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_management_done_interrupt(&mut self) -> DISABLE_MANAGEMENT_DONE_INTERRUPT_W<0> {
        DISABLE_MANAGEMENT_DONE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 1 - 'Disable receive complete interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_receive_complete_interrupt(
        &mut self,
    ) -> DISABLE_RECEIVE_COMPLETE_INTERRUPT_W<1> {
        DISABLE_RECEIVE_COMPLETE_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 2 - 'Disable receive used bit read interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_receive_used_bit_read_interrupt(
        &mut self,
    ) -> DISABLE_RECEIVE_USED_BIT_READ_INTERRUPT_W<2> {
        DISABLE_RECEIVE_USED_BIT_READ_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 3 - 'Disable transmit used bit read interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_transmit_used_bit_read_interrupt(
        &mut self,
    ) -> DISABLE_TRANSMIT_USED_BIT_READ_INTERRUPT_W<3> {
        DISABLE_TRANSMIT_USED_BIT_READ_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 4 - 'Disable transmit buffer under run interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_transmit_buffer_under_run_interrupt(
        &mut self,
    ) -> DISABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_W<4> {
        DISABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 5 - 'Disable retry limit exceeded or late collision interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_retry_limit_exceeded_or_late_collision_interrupt(
        &mut self,
    ) -> DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W<5> {
        DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 6 - 'Disable transmit frame corruption due to AMBA (AHB/AXI) error interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_transmit_frame_corruption_due_to_amba_error_interrupt(
        &mut self,
    ) -> DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W<6> {
        DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 7 - 'Disable transmit complete interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_transmit_complete_interrupt(
        &mut self,
    ) -> DISABLE_TRANSMIT_COMPLETE_INTERRUPT_W<7> {
        DISABLE_TRANSMIT_COMPLETE_INTERRUPT_W::new(self)
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
    #[doc = "Bit 10 - 'Disable receive overrun interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_receive_overrun_interrupt(&mut self) -> DISABLE_RECEIVE_OVERRUN_INTERRUPT_W<10> {
        DISABLE_RECEIVE_OVERRUN_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 11 - 'Disable bresp/hresp not OK interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_resp_not_ok_interrupt(&mut self) -> DISABLE_RESP_NOT_OK_INTERRUPT_W<11> {
        DISABLE_RESP_NOT_OK_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 12 - 'Disable pause frame with non-zero pause quantum interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_pause_frame_with_non_zero_pause_quantum_interrupt(
        &mut self,
    ) -> DISABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_W<12> {
        DISABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 13 - 'Disable pause time zero interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_pause_time_zero_interrupt(&mut self) -> DISABLE_PAUSE_TIME_ZERO_INTERRUPT_W<13> {
        DISABLE_PAUSE_TIME_ZERO_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 14 - 'Disable pause frame transmitted interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_pause_frame_transmitted_interrupt(
        &mut self,
    ) -> DISABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT_W<14> {
        DISABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT_W::new(self)
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
    #[doc = "Bit 18 - 'Disable PTP delay_req frame received'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_ptp_delay_req_frame_received(
        &mut self,
    ) -> DISABLE_PTP_DELAY_REQ_FRAME_RECEIVED_W<18> {
        DISABLE_PTP_DELAY_REQ_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 19 - 'Disable PTP sync frame received'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_ptp_sync_frame_received(&mut self) -> DISABLE_PTP_SYNC_FRAME_RECEIVED_W<19> {
        DISABLE_PTP_SYNC_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 20 - 'Disable PTP delay_req frame transmitted '"]
    #[inline(always)]
    #[must_use]
    pub fn disable_ptp_delay_req_frame_transmitted(
        &mut self,
    ) -> DISABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED_W<20> {
        DISABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 21 - 'Disable PTP sync frame transmitted '"]
    #[inline(always)]
    #[must_use]
    pub fn disable_ptp_sync_frame_transmitted(
        &mut self,
    ) -> DISABLE_PTP_SYNC_FRAME_TRANSMITTED_W<21> {
        DISABLE_PTP_SYNC_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 22 - 'Disable PTP pdelay_req frame received'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_ptp_pdelay_req_frame_received(
        &mut self,
    ) -> DISABLE_PTP_PDELAY_REQ_FRAME_RECEIVED_W<22> {
        DISABLE_PTP_PDELAY_REQ_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 23 - 'Disable PTP pdelay_resp frame received'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_ptp_pdelay_resp_frame_received(
        &mut self,
    ) -> DISABLE_PTP_PDELAY_RESP_FRAME_RECEIVED_W<23> {
        DISABLE_PTP_PDELAY_RESP_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 24 - 'Disable PTP pdelay_req frame transmitted'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_ptp_pdelay_req_frame_transmitted(
        &mut self,
    ) -> DISABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED_W<24> {
        DISABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 25 - 'Disable PTP pdelay_resp frame transmitted'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_ptp_pdelay_resp_frame_transmitted(
        &mut self,
    ) -> DISABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED_W<25> {
        DISABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 26 - 'Disable TSU seconds register increment'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_tsu_seconds_register_increment(
        &mut self,
    ) -> DISABLE_TSU_SECONDS_REGISTER_INCREMENT_W<26> {
        DISABLE_TSU_SECONDS_REGISTER_INCREMENT_W::new(self)
    }
    #[doc = "Bit 27 - 'Disable RX LPI indication interrupt'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_rx_lpi_indication_interrupt(
        &mut self,
    ) -> DISABLE_RX_LPI_INDICATION_INTERRUPT_W<27> {
        DISABLE_RX_LPI_INDICATION_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 28 - Not used"]
    #[inline(always)]
    #[must_use]
    pub fn unused_28(&mut self) -> UNUSED_28_W<28> {
        UNUSED_28_W::new(self)
    }
    #[doc = "Bit 29 - 'Disable TSU timer comparison interrupt.'"]
    #[inline(always)]
    #[must_use]
    pub fn disable_tsu_timer_comparison_interrupt(
        &mut self,
    ) -> DISABLE_TSU_TIMER_COMPARISON_INTERRUPT_W<29> {
        DISABLE_TSU_TIMER_COMPARISON_INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_disable](index.html) module"]
pub struct INT_DISABLE_SPEC;
impl crate::RegisterSpec for INT_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_disable::R](R) reader structure"]
impl crate::Readable for INT_DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_disable::W](W) writer structure"]
impl crate::Writable for INT_DISABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_DISABLE to value 0"]
impl crate::Resettable for INT_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
