#[doc = "Register `INT_STATUS` reader"]
pub struct R(crate::R<INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STATUS` writer"]
pub struct W(crate::W<INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STATUS_SPEC>;
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
impl From<crate::W<INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MANAGEMENT_FRAME_SENT` reader - Management frame sent - the PHY maintenance register has completed its operation. Cleared on read."]
pub type MANAGEMENT_FRAME_SENT_R = crate::BitReader<bool>;
#[doc = "Field `MANAGEMENT_FRAME_SENT` writer - Management frame sent - the PHY maintenance register has completed its operation. Cleared on read."]
pub type MANAGEMENT_FRAME_SENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `RECEIVE_COMPLETE` reader - Receive complete - a frame has been stored in memory. Cleared on read."]
pub type RECEIVE_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_COMPLETE` writer - Receive complete - a frame has been stored in memory. Cleared on read."]
pub type RECEIVE_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `RX_USED_BIT_READ` reader - RX used bit read - set when a receive buffer descriptor is read with its used bit set. Cleared on read."]
pub type RX_USED_BIT_READ_R = crate::BitReader<bool>;
#[doc = "Field `RX_USED_BIT_READ` writer - RX used bit read - set when a receive buffer descriptor is read with its used bit set. Cleared on read."]
pub type RX_USED_BIT_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `TX_USED_BIT_READ` reader - TX used bit read - set when a transmit buffer descriptor is read with its used bit set. Cleared on read."]
pub type TX_USED_BIT_READ_R = crate::BitReader<bool>;
#[doc = "Field `TX_USED_BIT_READ` writer - TX used bit read - set when a transmit buffer descriptor is read with its used bit set. Cleared on read."]
pub type TX_USED_BIT_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `TRANSMIT_UNDER_RUN` reader - Transmit under run - this interrupt is set if the transmitter was forced to terminate a frame that it has already began transmitting due to further data being unavailable. If an under run occurs, the transmitter will force bad crc and tx_er high. This interrupt is set if a transmitter status write back has not completed when another status write back is attempted. When using the DMA interface configured for internal FIFO mode, this interrupt is also set when the transmit DMA has written the SOP data into the FIFO and either the AHB bus was not granted in time for further data, or because an AHB/AXI error response was returned by the connected slave, or because a used bit was read. When using the DMA interface configured for packet buffer mode, this bit will never be set. When using the external FIFO interface, this interrupt is also set when the tx_r_underflow input was asserted during a frame transfer. Cleared on read."]
pub type TRANSMIT_UNDER_RUN_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMIT_UNDER_RUN` writer - Transmit under run - this interrupt is set if the transmitter was forced to terminate a frame that it has already began transmitting due to further data being unavailable. If an under run occurs, the transmitter will force bad crc and tx_er high. This interrupt is set if a transmitter status write back has not completed when another status write back is attempted. When using the DMA interface configured for internal FIFO mode, this interrupt is also set when the transmit DMA has written the SOP data into the FIFO and either the AHB bus was not granted in time for further data, or because an AHB/AXI error response was returned by the connected slave, or because a used bit was read. When using the DMA interface configured for packet buffer mode, this bit will never be set. When using the external FIFO interface, this interrupt is also set when the tx_r_underflow input was asserted during a frame transfer. Cleared on read."]
pub type TRANSMIT_UNDER_RUN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION` reader - Retry limit exceeded or late collision - transmit error. Late collision will only cause this status bit to be set in gigabit mode (as a retry is not attempted). Cleared on read."]
pub type RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION` writer - Retry limit exceeded or late collision - transmit error. Late collision will only cause this status bit to be set in gigabit mode (as a retry is not attempted). Cleared on read."]
pub type RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `AMBA_ERROR` reader - Transmit frame corruption due to AMBA (AXI/AHB) error. Set if an error occurs whilst midway through reading transmit frame from external system memory, including HRESP (AHB), RRESP or BRESP(AXI) errors and buffers exhausted mid frame (if the buffers run out during transmission of a frame then transmission stops, FCS shall be bad and tx_er asserted). Also set in DMA packet buffer mode if single frame is too large for configured packet buffer memory size. Cleared on a read."]
pub type AMBA_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `AMBA_ERROR` writer - Transmit frame corruption due to AMBA (AXI/AHB) error. Set if an error occurs whilst midway through reading transmit frame from external system memory, including HRESP (AHB), RRESP or BRESP(AXI) errors and buffers exhausted mid frame (if the buffers run out during transmission of a frame then transmission stops, FCS shall be bad and tx_er asserted). Also set in DMA packet buffer mode if single frame is too large for configured packet buffer memory size. Cleared on a read."]
pub type AMBA_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `TRANSMIT_COMPLETE` writer - Transmit complete - set when a frame has been transmitted. Cleared on read."]
pub type TRANSMIT_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `REMOVED_9` reader - Write ignore, read 0"]
pub type REMOVED_9_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_OVERRUN` writer - Receive overrun - set when the receive overrun status bit gets set. Cleared on read."]
pub type RECEIVE_OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `RESP_NOT_OK` writer - bresp not OK - set when the DMA block sees bresp not OK. Cleared on read."]
pub type RESP_NOT_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_RECEIVED` writer - Pause frame with non-zero pause quantum received - indicates a valid pause has been received that has a non-zero pause quantum field. Cleared on read."]
pub type PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PAUSE_TIME_ELAPSED` writer - Pause Time elapsed. set when either the pause time register at address 0x38 decrements to zero, or when a valid pause frame is received with a zero pause quantum field. Cleared on read."]
pub type PAUSE_TIME_ELAPSED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PAUSE_FRAME_TRANSMITTED` writer - Pause frame transmitted - indicates a pause frame has been successfully transmitted after being initiated from the network control register or from the tx_pause control pin. Cleared on read."]
pub type PAUSE_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `REMOVED_15` reader - Write ignore, read 0"]
pub type REMOVED_15_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_16` reader - Write ignore, read 0"]
pub type REMOVED_16_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_17` reader - Write ignore, read 0"]
pub type REMOVED_17_R = crate::BitReader<bool>;
#[doc = "Field `PTP_DELAY_REQ_FRAME_RECEIVED` writer - PTP delay_req frame received indicates a PTP delay_req frame has been received. Cleared on read."]
pub type PTP_DELAY_REQ_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PTP_SYNC_FRAME_RECEIVED` writer - PTP sync frame received indicates a PTP sync frame has been received. Cleared on read."]
pub type PTP_SYNC_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PTP_DELAY_REQ_FRAME_TRANSMITTED` writer - PTP delay_req frame transmitted indicates a PTP delay_req frame has been transmitted. Cleared on read."]
pub type PTP_DELAY_REQ_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PTP_SYNC_FRAME_TRANSMITTED` writer - PTP sync frame transmitted indicates a PTP sync frame has been transmitted. Cleared on read."]
pub type PTP_SYNC_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PTP_PDELAY_REQ_FRAME_RECEIVED` writer - PTP pdelay_req frame received indicates a PTP pdelay_req frame has been received. Cleared on read."]
pub type PTP_PDELAY_REQ_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PTP_PDELAY_RESP_FRAME_RECEIVED` writer - PTP pdelay_resp frame received indicates a PTP pdelay_resp frame has been received. Cleared on read."]
pub type PTP_PDELAY_RESP_FRAME_RECEIVED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PTP_PDELAY_REQ_FRAME_TRANSMITTED` writer - PTP pdelay_req frame transmitted indicates a PTP pdelay_req frame has been transmitted. Cleared on read."]
pub type PTP_PDELAY_REQ_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `PTP_PDELAY_RESP_FRAME_TRANSMITTED` writer - PTP pdelay_resp frame transmitted indicates a PTP pdelay_resp frame has been transmitted. Cleared on read."]
pub type PTP_PDELAY_RESP_FRAME_TRANSMITTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `TSU_SECONDS_REGISTER_INCREMENT` writer - TSU seconds register increment indicates the register has incremented. Cleared on read."]
pub type TSU_SECONDS_REGISTER_INCREMENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `RECEIVE_LPI_INDICATION_STATUS_BIT_CHANGE` writer - Receive LPI indication status bit change"]
pub type RECEIVE_LPI_INDICATION_STATUS_BIT_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `REMOVED_28` reader - Write ignore, read 0"]
pub type REMOVED_28_R = crate::BitReader<bool>;
#[doc = "Field `TSU_TIMER_COMPARISON_INTERRUPT` writer - TSU timer comparison interrupt. Indicates when TSU timer count value is equal to programmed value."]
pub type TSU_TIMER_COMPARISON_INTERRUPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Management frame sent - the PHY maintenance register has completed its operation. Cleared on read."]
    #[inline(always)]
    pub fn management_frame_sent(&self) -> MANAGEMENT_FRAME_SENT_R {
        MANAGEMENT_FRAME_SENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive complete - a frame has been stored in memory. Cleared on read."]
    #[inline(always)]
    pub fn receive_complete(&self) -> RECEIVE_COMPLETE_R {
        RECEIVE_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX used bit read - set when a receive buffer descriptor is read with its used bit set. Cleared on read."]
    #[inline(always)]
    pub fn rx_used_bit_read(&self) -> RX_USED_BIT_READ_R {
        RX_USED_BIT_READ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX used bit read - set when a transmit buffer descriptor is read with its used bit set. Cleared on read."]
    #[inline(always)]
    pub fn tx_used_bit_read(&self) -> TX_USED_BIT_READ_R {
        TX_USED_BIT_READ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit under run - this interrupt is set if the transmitter was forced to terminate a frame that it has already began transmitting due to further data being unavailable. If an under run occurs, the transmitter will force bad crc and tx_er high. This interrupt is set if a transmitter status write back has not completed when another status write back is attempted. When using the DMA interface configured for internal FIFO mode, this interrupt is also set when the transmit DMA has written the SOP data into the FIFO and either the AHB bus was not granted in time for further data, or because an AHB/AXI error response was returned by the connected slave, or because a used bit was read. When using the DMA interface configured for packet buffer mode, this bit will never be set. When using the external FIFO interface, this interrupt is also set when the tx_r_underflow input was asserted during a frame transfer. Cleared on read."]
    #[inline(always)]
    pub fn transmit_under_run(&self) -> TRANSMIT_UNDER_RUN_R {
        TRANSMIT_UNDER_RUN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision - transmit error. Late collision will only cause this status bit to be set in gigabit mode (as a retry is not attempted). Cleared on read."]
    #[inline(always)]
    pub fn retry_limit_exceeded_or_late_collision(
        &self,
    ) -> RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_R {
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AXI/AHB) error. Set if an error occurs whilst midway through reading transmit frame from external system memory, including HRESP (AHB), RRESP or BRESP(AXI) errors and buffers exhausted mid frame (if the buffers run out during transmission of a frame then transmission stops, FCS shall be bad and tx_er asserted). Also set in DMA packet buffer mode if single frame is too large for configured packet buffer memory size. Cleared on a read."]
    #[inline(always)]
    pub fn amba_error(&self) -> AMBA_ERROR_R {
        AMBA_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_9(&self) -> REMOVED_9_R {
        REMOVED_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_15(&self) -> REMOVED_15_R {
        REMOVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_16(&self) -> REMOVED_16_R {
        REMOVED_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_17(&self) -> REMOVED_17_R {
        REMOVED_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 28 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_28(&self) -> REMOVED_28_R {
        REMOVED_28_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management frame sent - the PHY maintenance register has completed its operation. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn management_frame_sent(&mut self) -> MANAGEMENT_FRAME_SENT_W<0> {
        MANAGEMENT_FRAME_SENT_W::new(self)
    }
    #[doc = "Bit 1 - Receive complete - a frame has been stored in memory. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn receive_complete(&mut self) -> RECEIVE_COMPLETE_W<1> {
        RECEIVE_COMPLETE_W::new(self)
    }
    #[doc = "Bit 2 - RX used bit read - set when a receive buffer descriptor is read with its used bit set. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn rx_used_bit_read(&mut self) -> RX_USED_BIT_READ_W<2> {
        RX_USED_BIT_READ_W::new(self)
    }
    #[doc = "Bit 3 - TX used bit read - set when a transmit buffer descriptor is read with its used bit set. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn tx_used_bit_read(&mut self) -> TX_USED_BIT_READ_W<3> {
        TX_USED_BIT_READ_W::new(self)
    }
    #[doc = "Bit 4 - Transmit under run - this interrupt is set if the transmitter was forced to terminate a frame that it has already began transmitting due to further data being unavailable. If an under run occurs, the transmitter will force bad crc and tx_er high. This interrupt is set if a transmitter status write back has not completed when another status write back is attempted. When using the DMA interface configured for internal FIFO mode, this interrupt is also set when the transmit DMA has written the SOP data into the FIFO and either the AHB bus was not granted in time for further data, or because an AHB/AXI error response was returned by the connected slave, or because a used bit was read. When using the DMA interface configured for packet buffer mode, this bit will never be set. When using the external FIFO interface, this interrupt is also set when the tx_r_underflow input was asserted during a frame transfer. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn transmit_under_run(&mut self) -> TRANSMIT_UNDER_RUN_W<4> {
        TRANSMIT_UNDER_RUN_W::new(self)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision - transmit error. Late collision will only cause this status bit to be set in gigabit mode (as a retry is not attempted). Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn retry_limit_exceeded_or_late_collision(
        &mut self,
    ) -> RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_W<5> {
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_W::new(self)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AXI/AHB) error. Set if an error occurs whilst midway through reading transmit frame from external system memory, including HRESP (AHB), RRESP or BRESP(AXI) errors and buffers exhausted mid frame (if the buffers run out during transmission of a frame then transmission stops, FCS shall be bad and tx_er asserted). Also set in DMA packet buffer mode if single frame is too large for configured packet buffer memory size. Cleared on a read."]
    #[inline(always)]
    #[must_use]
    pub fn amba_error(&mut self) -> AMBA_ERROR_W<6> {
        AMBA_ERROR_W::new(self)
    }
    #[doc = "Bit 7 - Transmit complete - set when a frame has been transmitted. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn transmit_complete(&mut self) -> TRANSMIT_COMPLETE_W<7> {
        TRANSMIT_COMPLETE_W::new(self)
    }
    #[doc = "Bit 10 - Receive overrun - set when the receive overrun status bit gets set. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn receive_overrun(&mut self) -> RECEIVE_OVERRUN_W<10> {
        RECEIVE_OVERRUN_W::new(self)
    }
    #[doc = "Bit 11 - bresp not OK - set when the DMA block sees bresp not OK. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn resp_not_ok(&mut self) -> RESP_NOT_OK_W<11> {
        RESP_NOT_OK_W::new(self)
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum received - indicates a valid pause has been received that has a non-zero pause quantum field. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn pause_frame_with_non_zero_pause_quantum_received(
        &mut self,
    ) -> PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_RECEIVED_W<12> {
        PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_RECEIVED_W::new(self)
    }
    #[doc = "Bit 13 - Pause Time elapsed. set when either the pause time register at address 0x38 decrements to zero, or when a valid pause frame is received with a zero pause quantum field. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn pause_time_elapsed(&mut self) -> PAUSE_TIME_ELAPSED_W<13> {
        PAUSE_TIME_ELAPSED_W::new(self)
    }
    #[doc = "Bit 14 - Pause frame transmitted - indicates a pause frame has been successfully transmitted after being initiated from the network control register or from the tx_pause control pin. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn pause_frame_transmitted(&mut self) -> PAUSE_FRAME_TRANSMITTED_W<14> {
        PAUSE_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 18 - PTP delay_req frame received indicates a PTP delay_req frame has been received. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_delay_req_frame_received(&mut self) -> PTP_DELAY_REQ_FRAME_RECEIVED_W<18> {
        PTP_DELAY_REQ_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 19 - PTP sync frame received indicates a PTP sync frame has been received. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_sync_frame_received(&mut self) -> PTP_SYNC_FRAME_RECEIVED_W<19> {
        PTP_SYNC_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted indicates a PTP delay_req frame has been transmitted. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_delay_req_frame_transmitted(&mut self) -> PTP_DELAY_REQ_FRAME_TRANSMITTED_W<20> {
        PTP_DELAY_REQ_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 21 - PTP sync frame transmitted indicates a PTP sync frame has been transmitted. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_sync_frame_transmitted(&mut self) -> PTP_SYNC_FRAME_TRANSMITTED_W<21> {
        PTP_SYNC_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received indicates a PTP pdelay_req frame has been received. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_pdelay_req_frame_received(&mut self) -> PTP_PDELAY_REQ_FRAME_RECEIVED_W<22> {
        PTP_PDELAY_REQ_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received indicates a PTP pdelay_resp frame has been received. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_pdelay_resp_frame_received(&mut self) -> PTP_PDELAY_RESP_FRAME_RECEIVED_W<23> {
        PTP_PDELAY_RESP_FRAME_RECEIVED_W::new(self)
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted indicates a PTP pdelay_req frame has been transmitted. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_pdelay_req_frame_transmitted(&mut self) -> PTP_PDELAY_REQ_FRAME_TRANSMITTED_W<24> {
        PTP_PDELAY_REQ_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted indicates a PTP pdelay_resp frame has been transmitted. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_pdelay_resp_frame_transmitted(&mut self) -> PTP_PDELAY_RESP_FRAME_TRANSMITTED_W<25> {
        PTP_PDELAY_RESP_FRAME_TRANSMITTED_W::new(self)
    }
    #[doc = "Bit 26 - TSU seconds register increment indicates the register has incremented. Cleared on read."]
    #[inline(always)]
    #[must_use]
    pub fn tsu_seconds_register_increment(&mut self) -> TSU_SECONDS_REGISTER_INCREMENT_W<26> {
        TSU_SECONDS_REGISTER_INCREMENT_W::new(self)
    }
    #[doc = "Bit 27 - Receive LPI indication status bit change"]
    #[inline(always)]
    #[must_use]
    pub fn receive_lpi_indication_status_bit_change(
        &mut self,
    ) -> RECEIVE_LPI_INDICATION_STATUS_BIT_CHANGE_W<27> {
        RECEIVE_LPI_INDICATION_STATUS_BIT_CHANGE_W::new(self)
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt. Indicates when TSU timer count value is equal to programmed value."]
    #[inline(always)]
    #[must_use]
    pub fn tsu_timer_comparison_interrupt(&mut self) -> TSU_TIMER_COMPARISON_INTERRUPT_W<29> {
        TSU_TIMER_COMPARISON_INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "If not configured for priority queueing, the GEM generates a single interrupt. This register indicates the source of this interrupt. The corresponding bit in the mask register must be clear for a bit to be set. If any bit is set in this register the ethernet_int signal will be asserted. For test purposes each bit can be set or reset by writing to the interrupt mask register. The default configuration is shown below whereby all bits are reset to zero on read. Changing the validity of the `gem_irq_read_clear define will instead require a one to be written to the appropriate bit in order to clear it. In this mode reading has no affect on the status of the bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](index.html) module"]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status::R](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_status::W](W) writer structure"]
impl crate::Writable for INT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
