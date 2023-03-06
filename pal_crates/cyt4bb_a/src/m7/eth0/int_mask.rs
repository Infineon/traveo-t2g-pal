#[doc = "Register `INT_MASK` reader"]
pub struct R(crate::R<INT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MANAGEMENT_DONE_INTERRUPT_MASK` reader - management done interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type MANAGEMENT_DONE_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_COMPLETE_INTERRUPT_MASK` reader - receive complete interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RECEIVE_COMPLETE_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_USED_BIT_READ_INTERRUPT_MASK` reader - receive used bit read interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RECEIVE_USED_BIT_READ_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMIT_USED_BIT_READ_INTERRUPT_MASK` reader - transmit used bit read interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type TRANSMIT_USED_BIT_READ_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_MASK` reader - transmit buffer under run interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_MASK` reader - A read of this register returns the value of the retry limit exceeded or late collision (gigabit mode only) interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_MASK_R = crate::BitReader<bool>;
#[doc = "Field `AMBA_ERROR_INTERRUPT_MASK` reader - transmit frame corruption due to AMBA (AXI) error interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type AMBA_ERROR_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMIT_COMPLETE_INTERRUPT_MASK` reader - transmit complete interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type TRANSMIT_COMPLETE_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED_8` reader - Not used"]
pub type UNUSED_8_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED_9` reader - Not used"]
pub type UNUSED_9_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_OVERRUN_INTERRUPT_MASK` reader - receive overrun interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RECEIVE_OVERRUN_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RESP_NOT_OK_INTERRUPT_MASK` reader - bresp not OK interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RESP_NOT_OK_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_MASK` reader - pause frame with non-zero pause quantum interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_TIME_ZERO_INTERRUPT_MASK` reader - pause time zero interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PAUSE_TIME_ZERO_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_FRAME_TRANSMITTED_INTERRUPT_MASK` reader - pause frame transmitted interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PAUSE_FRAME_TRANSMITTED_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED_15` reader - Not used"]
pub type UNUSED_15_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED_16` reader - Not used"]
pub type UNUSED_16_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED_17` reader - Not used"]
pub type UNUSED_17_R = crate::BitReader<bool>;
#[doc = "Field `PTP_DELAY_REQ_FRAME_RECEIVED_MASK` reader - A read of this register returns the value of the PTP delay_req frame received mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PTP_DELAY_REQ_FRAME_RECEIVED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PTP_SYNC_FRAME_RECEIVED_MASK` reader - A read of this register returns the value of the PTP sync frame received mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PTP_SYNC_FRAME_RECEIVED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PTP_DELAY_REQ_FRAME_TRANSMITTED_MASK` reader - A read of this register returns the value of the PTP delay_req frame transmitted mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PTP_DELAY_REQ_FRAME_TRANSMITTED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PTP_SYNC_FRAME_TRANSMITTED_MASK` reader - A read of this register returns the value of the PTP sync frame transmitted mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PTP_SYNC_FRAME_TRANSMITTED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PTP_PDELAY_REQ_FRAME_RECEIVED_MASK` reader - A read of this register returns the value of the PTP pdelay_req frame received mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PTP_PDELAY_REQ_FRAME_RECEIVED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PTP_PDELAY_RESP_FRAME_RECEIVED_MASK` reader - A read of this register returns the value of the PTP pdelay_resp frame received mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PTP_PDELAY_RESP_FRAME_RECEIVED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PTP_PDELAY_REQ_FRAME_TRANSMITTED_MASK` reader - A read of this register returns the value of the PTP pdelay_req frame transmitted mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PTP_PDELAY_REQ_FRAME_TRANSMITTED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `PTP_PDELAY_RESP_FRAME_TRANSMITTED_MASK` reader - A read of this register returns the value of the PTP pdelay_resp frame transmitted mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type PTP_PDELAY_RESP_FRAME_TRANSMITTED_MASK_R = crate::BitReader<bool>;
#[doc = "Field `TSU_SECONDS_REGISTER_INCREMENT_MASK` reader - A read of this register returns the value of the TSU seconds register increment mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type TSU_SECONDS_REGISTER_INCREMENT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RX_LPI_INDICATION_MASK` reader - A read of this register returns the value of the RX LPI indication mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written"]
pub type RX_LPI_INDICATION_MASK_R = crate::BitReader<bool>;
#[doc = "Field `UNUSED_28` reader - unused"]
pub type UNUSED_28_R = crate::BitReader<bool>;
#[doc = "Field `TSU_TIMER_COMPARISON_MASK` reader - Enable TSU timer comparison interrupt mask."]
pub type TSU_TIMER_COMPARISON_MASK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - management done interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn management_done_interrupt_mask(&self) -> MANAGEMENT_DONE_INTERRUPT_MASK_R {
        MANAGEMENT_DONE_INTERRUPT_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - receive complete interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn receive_complete_interrupt_mask(&self) -> RECEIVE_COMPLETE_INTERRUPT_MASK_R {
        RECEIVE_COMPLETE_INTERRUPT_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - receive used bit read interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn receive_used_bit_read_interrupt_mask(&self) -> RECEIVE_USED_BIT_READ_INTERRUPT_MASK_R {
        RECEIVE_USED_BIT_READ_INTERRUPT_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transmit used bit read interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn transmit_used_bit_read_interrupt_mask(&self) -> TRANSMIT_USED_BIT_READ_INTERRUPT_MASK_R {
        TRANSMIT_USED_BIT_READ_INTERRUPT_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - transmit buffer under run interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn transmit_buffer_under_run_interrupt_mask(
        &self,
    ) -> TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_MASK_R {
        TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A read of this register returns the value of the retry limit exceeded or late collision (gigabit mode only) interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn retry_limit_exceeded_or_late_collision_mask(
        &self,
    ) -> RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_MASK_R {
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - transmit frame corruption due to AMBA (AXI) error interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn amba_error_interrupt_mask(&self) -> AMBA_ERROR_INTERRUPT_MASK_R {
        AMBA_ERROR_INTERRUPT_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transmit complete interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn transmit_complete_interrupt_mask(&self) -> TRANSMIT_COMPLETE_INTERRUPT_MASK_R {
        TRANSMIT_COMPLETE_INTERRUPT_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not used"]
    #[inline(always)]
    pub fn unused_8(&self) -> UNUSED_8_R {
        UNUSED_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Not used"]
    #[inline(always)]
    pub fn unused_9(&self) -> UNUSED_9_R {
        UNUSED_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - receive overrun interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn receive_overrun_interrupt_mask(&self) -> RECEIVE_OVERRUN_INTERRUPT_MASK_R {
        RECEIVE_OVERRUN_INTERRUPT_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - bresp not OK interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn resp_not_ok_interrupt_mask(&self) -> RESP_NOT_OK_INTERRUPT_MASK_R {
        RESP_NOT_OK_INTERRUPT_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pause frame with non-zero pause quantum interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn pause_frame_with_non_zero_pause_quantum_interrupt_mask(
        &self,
    ) -> PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_MASK_R {
        PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pause time zero interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn pause_time_zero_interrupt_mask(&self) -> PAUSE_TIME_ZERO_INTERRUPT_MASK_R {
        PAUSE_TIME_ZERO_INTERRUPT_MASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pause frame transmitted interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn pause_frame_transmitted_interrupt_mask(
        &self,
    ) -> PAUSE_FRAME_TRANSMITTED_INTERRUPT_MASK_R {
        PAUSE_FRAME_TRANSMITTED_INTERRUPT_MASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Not used"]
    #[inline(always)]
    pub fn unused_15(&self) -> UNUSED_15_R {
        UNUSED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Not used"]
    #[inline(always)]
    pub fn unused_16(&self) -> UNUSED_16_R {
        UNUSED_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Not used"]
    #[inline(always)]
    pub fn unused_17(&self) -> UNUSED_17_R {
        UNUSED_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A read of this register returns the value of the PTP delay_req frame received mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn ptp_delay_req_frame_received_mask(&self) -> PTP_DELAY_REQ_FRAME_RECEIVED_MASK_R {
        PTP_DELAY_REQ_FRAME_RECEIVED_MASK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - A read of this register returns the value of the PTP sync frame received mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn ptp_sync_frame_received_mask(&self) -> PTP_SYNC_FRAME_RECEIVED_MASK_R {
        PTP_SYNC_FRAME_RECEIVED_MASK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - A read of this register returns the value of the PTP delay_req frame transmitted mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn ptp_delay_req_frame_transmitted_mask(&self) -> PTP_DELAY_REQ_FRAME_TRANSMITTED_MASK_R {
        PTP_DELAY_REQ_FRAME_TRANSMITTED_MASK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - A read of this register returns the value of the PTP sync frame transmitted mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn ptp_sync_frame_transmitted_mask(&self) -> PTP_SYNC_FRAME_TRANSMITTED_MASK_R {
        PTP_SYNC_FRAME_TRANSMITTED_MASK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - A read of this register returns the value of the PTP pdelay_req frame received mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn ptp_pdelay_req_frame_received_mask(&self) -> PTP_PDELAY_REQ_FRAME_RECEIVED_MASK_R {
        PTP_PDELAY_REQ_FRAME_RECEIVED_MASK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - A read of this register returns the value of the PTP pdelay_resp frame received mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn ptp_pdelay_resp_frame_received_mask(&self) -> PTP_PDELAY_RESP_FRAME_RECEIVED_MASK_R {
        PTP_PDELAY_RESP_FRAME_RECEIVED_MASK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - A read of this register returns the value of the PTP pdelay_req frame transmitted mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn ptp_pdelay_req_frame_transmitted_mask(&self) -> PTP_PDELAY_REQ_FRAME_TRANSMITTED_MASK_R {
        PTP_PDELAY_REQ_FRAME_TRANSMITTED_MASK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - A read of this register returns the value of the PTP pdelay_resp frame transmitted mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn ptp_pdelay_resp_frame_transmitted_mask(
        &self,
    ) -> PTP_PDELAY_RESP_FRAME_TRANSMITTED_MASK_R {
        PTP_PDELAY_RESP_FRAME_TRANSMITTED_MASK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - A read of this register returns the value of the TSU seconds register increment mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn tsu_seconds_register_increment_mask(&self) -> TSU_SECONDS_REGISTER_INCREMENT_MASK_R {
        TSU_SECONDS_REGISTER_INCREMENT_MASK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - A read of this register returns the value of the RX LPI indication mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written"]
    #[inline(always)]
    pub fn rx_lpi_indication_mask(&self) -> RX_LPI_INDICATION_MASK_R {
        RX_LPI_INDICATION_MASK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - unused"]
    #[inline(always)]
    pub fn unused_28(&self) -> UNUSED_28_R {
        UNUSED_28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable TSU timer comparison interrupt mask."]
    #[inline(always)]
    pub fn tsu_timer_comparison_mask(&self) -> TSU_TIMER_COMPARISON_MASK_R {
        TSU_TIMER_COMPARISON_MASK_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mask](index.html) module"]
pub struct INT_MASK_SPEC;
impl crate::RegisterSpec for INT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_mask::R](R) reader structure"]
impl crate::Readable for INT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_MASK to value 0x3fff_ffff"]
impl crate::Resettable for INT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_ffff;
}
