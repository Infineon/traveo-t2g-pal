#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_HEADER_DONE` reader - HW sets this field to '1', when a frame header (PID field or PType field) is transmitted (the CMD.TX_HEADER is completed). Specifically: - For PID transmission only and without prior transmission of PTYPE, when followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the message frame transfer. - For PID transmission only and without prior transmission of PTYPE, when not followed by a response command, this field is set to '1' after completion of the header transfer. -Note for the case of PTYPE is transmitted follow by CMD.RX_RESPONSE or no following response, HW sets this field to '1' after transmitting PTYPE."]
pub type TX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_DONE` writer - HW sets this field to '1', when a frame header (PID field or PType field) is transmitted (the CMD.TX_HEADER is completed). Specifically: - For PID transmission only and without prior transmission of PTYPE, when followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the message frame transfer. - For PID transmission only and without prior transmission of PTYPE, when not followed by a response command, this field is set to '1' after completion of the header transfer. -Note for the case of PTYPE is transmitted follow by CMD.RX_RESPONSE or no following response, HW sets this field to '1' after transmitting PTYPE."]
pub type TX_HEADER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_RESPONSE_DONE` reader - HW sets this field to '1', when a frame response (frame information fields, data fields, and crc field) is transmitted (the CMD.TX_RESPONSE is completed)."]
pub type TX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_DONE` writer - HW sets this field to '1', when a frame response (frame information fields, data fields, and crc field) is transmitted (the CMD.TX_RESPONSE is completed)."]
pub type TX_RESPONSE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_WAKEUP_DONE` reader - HW sets this field to '1', when a wakeup signal is transmitted (per CTL2.T_WAKEUP_LENGTH). This interrupt cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal."]
pub type TX_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_WAKEUP_DONE` writer - HW sets this field to '1', when a wakeup signal is transmitted (per CTL2.T_WAKEUP_LENGTH). This interrupt cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal."]
pub type TX_WAKEUP_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_TRIGGER` reader - HW sets this field to '1', when TX trigger is generated (#used TX FIFO &lt; TRIGGER_LEVEL)."]
pub type TX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_TRIGGER` writer - HW sets this field to '1', when TX trigger is generated (#used TX FIFO &lt; TRIGGER_LEVEL)."]
pub type TX_FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_DONE` reader - HW sets this field to '1', when a frame header (PID field or PType field) is received (the CMD.RX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the message frame transfer. - When not followed by a response command, this field is set to '1' after completion of the header transfer if RXPIDZERO_CHECK_EN=1 and header received is PID. If RXPIDZERO_CHECK_EN=0 and response commands are not set, HW will set this field to '1' after receiving PID or PTYPE."]
pub type RX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_DONE` writer - HW sets this field to '1', when a frame header (PID field or PType field) is received (the CMD.RX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the message frame transfer. - When not followed by a response command, this field is set to '1' after completion of the header transfer if RXPIDZERO_CHECK_EN=1 and header received is PID. If RXPIDZERO_CHECK_EN=0 and response commands are not set, HW will set this field to '1' after receiving PID or PTYPE."]
pub type RX_HEADER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE_DONE` reader - HW sets this field to '1', when a frame response (frame information fields, data fields, and crc field) is received (the CMD.RX_RESPONSE is completed)."]
pub type RX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_DONE` writer - HW sets this field to '1', when a frame response (frame information fields, data fields, and crc field) is received (the CMD.RX_RESPONSE is completed)."]
pub type RX_RESPONSE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_WAKEUP_DETECT` reader - HW sets this field to '1', when RX fall is detected in Sleep mode."]
pub type RX_WAKEUP_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RX_WAKEUP_DETECT` writer - HW sets this field to '1', when RX fall is detected in Sleep mode."]
pub type RX_WAKEUP_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_TRIGGER` reader - HW sets this field to '1', when RX trigger is generated (#used RX FIFO > TRIGGER_LEVEL)."]
pub type RX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_TRIGGER` writer - HW sets this field to '1', when RX trigger is generated (#used RX FIFO > TRIGGER_LEVEL)."]
pub type RX_FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_PID_DONE` reader - HW sets this field to '1', when RX header (PID/PTYPE field) is received."]
pub type RX_HEADER_PID_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PID_DONE` writer - HW sets this field to '1', when RX header (PID/PTYPE field) is received."]
pub type RX_HEADER_PID_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TXRX_COMPLETE` reader - HW sets this field to '1', when message frame ends after EOF is completed and TX/RX_DATA_LENGTH_ERROR=0."]
pub type TXRX_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `TXRX_COMPLETE` writer - HW sets this field to '1', when message frame ends after EOF is completed and TX/RX_DATA_LENGTH_ERROR=0."]
pub type TXRX_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - HW sets this field to '1', when the transmitted/received bytes space within a message frame is > TIMEOUT_LENGTH. SW needs to set TIMEOUT_SEL=0 before clearing TIMEOUT=0 to ensure HW does not immediately sets back the interrupt."]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - HW sets this field to '1', when the transmitted/received bytes space within a message frame is > TIMEOUT_LENGTH. SW needs to set TIMEOUT_SEL=0 before clearing TIMEOUT=0 to ensure HW does not immediately sets back the interrupt."]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_HEADER_ARB_LOST` reader - HW sets this field to '1', when it detects arbitration lost after the number of retries has exceed the maximum allowed retries. Note: The ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated and the TX_HEADER and TX_RESPONSE command is cleared to 0)."]
pub type TX_HEADER_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_ARB_LOST` writer - HW sets this field to '1', when it detects arbitration lost after the number of retries has exceed the maximum allowed retries. Note: The ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated and the TX_HEADER and TX_RESPONSE command is cleared to 0)."]
pub type TX_HEADER_ARB_LOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_BIT_ERROR` reader - HW sets this field to '1', when a transmitted 'cxpi_tx_out' value does NOT match a received 'cxpi_rx_in' value. The match is performed for the PID fields or PType (for the START bit and STOP bit only) and for the rest of the response i.e. frame information fields, data fields and the crc field (for the START bit, DATA bits, and STOP bits). Note: When CTL0.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. When CTL0.BIT_ERROR_IGNORE is '1', the ongoing message transfer would be transferred."]
pub type TX_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_BIT_ERROR` writer - HW sets this field to '1', when a transmitted 'cxpi_tx_out' value does NOT match a received 'cxpi_rx_in' value. The match is performed for the PID fields or PType (for the START bit and STOP bit only) and for the rest of the response i.e. frame information fields, data fields and the crc field (for the START bit, DATA bits, and STOP bits). Note: When CTL0.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. When CTL0.BIT_ERROR_IGNORE is '1', the ongoing message transfer would be transferred."]
pub type TX_BIT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_CRC_ERROR` reader - HW sets this field to '1', when received CRC is not matching with the compute CRC from header and response. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated)."]
pub type RX_CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_CRC_ERROR` writer - HW sets this field to '1', when received CRC is not matching with the compute CRC from header and response. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated)."]
pub type RX_CRC_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` reader - HW sets this field to '1', when the received PID field or PType field has a parity error. Note: The ongoing message transfer is aborted (INTR.RX_HEADER_PID_DONE is NOT activated)."]
pub type RX_HEADER_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` writer - HW sets this field to '1', when the received PID field or PType field has a parity error. Note: The ongoing message transfer is aborted (INTR.RX_HEADER_PID_DONE is NOT activated)."]
pub type RX_HEADER_PARITY_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_DATA_LENGTH_ERROR` reader - HW sets this field to '1, when the received message frame's data fields are more than the value specified in DLC (for normal frame) or DLCEXT (for long frame) i.e. after receiving CRC byte(s), HW is receiving logical '0' during EOF. For the case of receiving data length less than DLC/DLCEXT, HW will also set this field to '1'. This is the case where IBS>9 before the number of data reaches data length, then HW will report as data length error. HW starts checking after frame information byte. Note: SW needs to handle the message transfer i.e. discard or flush out. HW will still set the RX_RESPONSE_DONE."]
pub type RX_DATA_LENGTH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_LENGTH_ERROR` writer - HW sets this field to '1, when the received message frame's data fields are more than the value specified in DLC (for normal frame) or DLCEXT (for long frame) i.e. after receiving CRC byte(s), HW is receiving logical '0' during EOF. For the case of receiving data length less than DLC/DLCEXT, HW will also set this field to '1'. This is the case where IBS>9 before the number of data reaches data length, then HW will report as data length error. HW starts checking after frame information byte. Note: SW needs to handle the message transfer i.e. discard or flush out. HW will still set the RX_RESPONSE_DONE."]
pub type RX_DATA_LENGTH_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_DATA_LENGTH_ERROR` reader - HW sets this field to '1, when the transmit message frame's data fields are more than the value specified in DLC (for normal frame) or DLCEXT (for long frame) i.e. after transmitting CRC(s) byte, HW is receiving logical '0' during EOF. Note: HW will still set TX_RESPONSE_DONE and the TX_HEADER and TX_RESPONSE commands are set to '0'."]
pub type TX_DATA_LENGTH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_LENGTH_ERROR` writer - HW sets this field to '1, when the transmit message frame's data fields are more than the value specified in DLC (for normal frame) or DLCEXT (for long frame) i.e. after transmitting CRC(s) byte, HW is receiving logical '0' during EOF. Note: HW will still set TX_RESPONSE_DONE and the TX_HEADER and TX_RESPONSE commands are set to '0'."]
pub type TX_DATA_LENGTH_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_OVERFLOW_ERROR` reader - HW sets this field to '1', when the RX data is overwritten by HW before the SW reads from it. In CXPI spec, this error is denoted as overrun error. Note: Upon this error, SW should discard the RX data in RX FIFO."]
pub type RX_OVERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERFLOW_ERROR` writer - HW sets this field to '1', when the RX data is overwritten by HW before the SW reads from it. In CXPI spec, this error is denoted as overrun error. Note: Upon this error, SW should discard the RX data in RX FIFO."]
pub type RX_OVERFLOW_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_OVERFLOW_ERROR` reader - HW sets this field to '1', when the TX data is overwritten by SW before the HW reads from it to transmit to CXPI bus. Note: The ongoing message transfer will continue when this error happens however, data transferred at CXPI bus will be bogus and HW will invert the CRC to invalidate the message at the receiving node. TX_HEADER and TX_RESPONSE commands are set to '0'."]
pub type TX_OVERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_OVERFLOW_ERROR` writer - HW sets this field to '1', when the TX data is overwritten by SW before the HW reads from it to transmit to CXPI bus. Note: The ongoing message transfer will continue when this error happens however, data transferred at CXPI bus will be bogus and HW will invert the CRC to invalidate the message at the receiving node. TX_HEADER and TX_RESPONSE commands are set to '0'."]
pub type TX_OVERFLOW_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_UNDERFLOW_ERROR` reader - HW sets this field to '1', when RX FIFO is empty and SW reads from it. Note: Upon this error, SW should discard the RX data in RX FIFO."]
pub type RX_UNDERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_UNDERFLOW_ERROR` writer - HW sets this field to '1', when RX FIFO is empty and SW reads from it. Note: Upon this error, SW should discard the RX data in RX FIFO."]
pub type RX_UNDERFLOW_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_UNDERFLOW_ERROR` reader - HW sets this field to '1', when TX FIFO is empty and HW reads from it. Note: The ongoing message transfer will continue when this error happens however, data transferred at CXPI will be bogus and HW will invert the CRC to invalidate the message at the receiving node. TX_HEADER and TX_RESPONSE commands are set to '0'."]
pub type TX_UNDERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_UNDERFLOW_ERROR` writer - HW sets this field to '1', when TX FIFO is empty and HW reads from it. Note: The ongoing message transfer will continue when this error happens however, data transferred at CXPI will be bogus and HW will invert the CRC to invalidate the message at the receiving node. TX_HEADER and TX_RESPONSE commands are set to '0'."]
pub type TX_UNDERFLOW_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_FRAME_ERROR` reader - HW sets this field to '1', when the stop bit of a byte frame is incorrect. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated and the INTR.RX_HEADER_DONE/TX_HEADER_DONE is NOT activated if the frame error occurs during header byte or if frame error occurs during response byte (if the HEADER and RESPONSE commands are set together))."]
pub type RX_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_FRAME_ERROR` writer - HW sets this field to '1', when the stop bit of a byte frame is incorrect. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated and the INTR.RX_HEADER_DONE/TX_HEADER_DONE is NOT activated if the frame error occurs during header byte or if frame error occurs during response byte (if the HEADER and RESPONSE commands are set together))."]
pub type RX_FRAME_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_FRAME_ERROR` reader - HW sets this field to '1', when the stop bit of a byte frame is incorrect. This error would be a subset of TX_BIT_ERROR and also subjected to BIT_ERROR_IGNORE field. Note: The ongoing message transfer is aborted (INTR.TX_HEADER_DONE/RX_HEADER_DONE and INTR.TX_RESPONSE_DONE are NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. Note: When CTL0.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. When CTL0.BIT_ERROR_IGNORE is '1', the ongoing message transfer would be transferred."]
pub type TX_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_FRAME_ERROR` writer - HW sets this field to '1', when the stop bit of a byte frame is incorrect. This error would be a subset of TX_BIT_ERROR and also subjected to BIT_ERROR_IGNORE field. Note: The ongoing message transfer is aborted (INTR.TX_HEADER_DONE/RX_HEADER_DONE and INTR.TX_RESPONSE_DONE are NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. Note: When CTL0.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. When CTL0.BIT_ERROR_IGNORE is '1', the ongoing message transfer would be transferred."]
pub type TX_FRAME_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HW sets this field to '1', when a frame header (PID field or PType field) is transmitted (the CMD.TX_HEADER is completed). Specifically: - For PID transmission only and without prior transmission of PTYPE, when followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the message frame transfer. - For PID transmission only and without prior transmission of PTYPE, when not followed by a response command, this field is set to '1' after completion of the header transfer. -Note for the case of PTYPE is transmitted follow by CMD.RX_RESPONSE or no following response, HW sets this field to '1' after transmitting PTYPE."]
    #[inline(always)]
    pub fn tx_header_done(&self) -> TX_HEADER_DONE_R {
        TX_HEADER_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HW sets this field to '1', when a frame response (frame information fields, data fields, and crc field) is transmitted (the CMD.TX_RESPONSE is completed)."]
    #[inline(always)]
    pub fn tx_response_done(&self) -> TX_RESPONSE_DONE_R {
        TX_RESPONSE_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HW sets this field to '1', when a wakeup signal is transmitted (per CTL2.T_WAKEUP_LENGTH). This interrupt cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal."]
    #[inline(always)]
    pub fn tx_wakeup_done(&self) -> TX_WAKEUP_DONE_R {
        TX_WAKEUP_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HW sets this field to '1', when TX trigger is generated (#used TX FIFO &lt; TRIGGER_LEVEL)."]
    #[inline(always)]
    pub fn tx_fifo_trigger(&self) -> TX_FIFO_TRIGGER_R {
        TX_FIFO_TRIGGER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when a frame header (PID field or PType field) is received (the CMD.RX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the message frame transfer. - When not followed by a response command, this field is set to '1' after completion of the header transfer if RXPIDZERO_CHECK_EN=1 and header received is PID. If RXPIDZERO_CHECK_EN=0 and response commands are not set, HW will set this field to '1' after receiving PID or PTYPE."]
    #[inline(always)]
    pub fn rx_header_done(&self) -> RX_HEADER_DONE_R {
        RX_HEADER_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HW sets this field to '1', when a frame response (frame information fields, data fields, and crc field) is received (the CMD.RX_RESPONSE is completed)."]
    #[inline(always)]
    pub fn rx_response_done(&self) -> RX_RESPONSE_DONE_R {
        RX_RESPONSE_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HW sets this field to '1', when RX fall is detected in Sleep mode."]
    #[inline(always)]
    pub fn rx_wakeup_detect(&self) -> RX_WAKEUP_DETECT_R {
        RX_WAKEUP_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HW sets this field to '1', when RX trigger is generated (#used RX FIFO > TRIGGER_LEVEL)."]
    #[inline(always)]
    pub fn rx_fifo_trigger(&self) -> RX_FIFO_TRIGGER_R {
        RX_FIFO_TRIGGER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HW sets this field to '1', when RX header (PID/PTYPE field) is received."]
    #[inline(always)]
    pub fn rx_header_pid_done(&self) -> RX_HEADER_PID_DONE_R {
        RX_HEADER_PID_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HW sets this field to '1', when message frame ends after EOF is completed and TX/RX_DATA_LENGTH_ERROR=0."]
    #[inline(always)]
    pub fn txrx_complete(&self) -> TXRX_COMPLETE_R {
        TXRX_COMPLETE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - HW sets this field to '1', when the transmitted/received bytes space within a message frame is > TIMEOUT_LENGTH. SW needs to set TIMEOUT_SEL=0 before clearing TIMEOUT=0 to ensure HW does not immediately sets back the interrupt."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HW sets this field to '1', when it detects arbitration lost after the number of retries has exceed the maximum allowed retries. Note: The ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated and the TX_HEADER and TX_RESPONSE command is cleared to 0)."]
    #[inline(always)]
    pub fn tx_header_arb_lost(&self) -> TX_HEADER_ARB_LOST_R {
        TX_HEADER_ARB_LOST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HW sets this field to '1', when a transmitted 'cxpi_tx_out' value does NOT match a received 'cxpi_rx_in' value. The match is performed for the PID fields or PType (for the START bit and STOP bit only) and for the rest of the response i.e. frame information fields, data fields and the crc field (for the START bit, DATA bits, and STOP bits). Note: When CTL0.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. When CTL0.BIT_ERROR_IGNORE is '1', the ongoing message transfer would be transferred."]
    #[inline(always)]
    pub fn tx_bit_error(&self) -> TX_BIT_ERROR_R {
        TX_BIT_ERROR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HW sets this field to '1', when received CRC is not matching with the compute CRC from header and response. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated)."]
    #[inline(always)]
    pub fn rx_crc_error(&self) -> RX_CRC_ERROR_R {
        RX_CRC_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HW sets this field to '1', when the received PID field or PType field has a parity error. Note: The ongoing message transfer is aborted (INTR.RX_HEADER_PID_DONE is NOT activated)."]
    #[inline(always)]
    pub fn rx_header_parity_error(&self) -> RX_HEADER_PARITY_ERROR_R {
        RX_HEADER_PARITY_ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - HW sets this field to '1, when the received message frame's data fields are more than the value specified in DLC (for normal frame) or DLCEXT (for long frame) i.e. after receiving CRC byte(s), HW is receiving logical '0' during EOF. For the case of receiving data length less than DLC/DLCEXT, HW will also set this field to '1'. This is the case where IBS>9 before the number of data reaches data length, then HW will report as data length error. HW starts checking after frame information byte. Note: SW needs to handle the message transfer i.e. discard or flush out. HW will still set the RX_RESPONSE_DONE."]
    #[inline(always)]
    pub fn rx_data_length_error(&self) -> RX_DATA_LENGTH_ERROR_R {
        RX_DATA_LENGTH_ERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - HW sets this field to '1, when the transmit message frame's data fields are more than the value specified in DLC (for normal frame) or DLCEXT (for long frame) i.e. after transmitting CRC(s) byte, HW is receiving logical '0' during EOF. Note: HW will still set TX_RESPONSE_DONE and the TX_HEADER and TX_RESPONSE commands are set to '0'."]
    #[inline(always)]
    pub fn tx_data_length_error(&self) -> TX_DATA_LENGTH_ERROR_R {
        TX_DATA_LENGTH_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HW sets this field to '1', when the RX data is overwritten by HW before the SW reads from it. In CXPI spec, this error is denoted as overrun error. Note: Upon this error, SW should discard the RX data in RX FIFO."]
    #[inline(always)]
    pub fn rx_overflow_error(&self) -> RX_OVERFLOW_ERROR_R {
        RX_OVERFLOW_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HW sets this field to '1', when the TX data is overwritten by SW before the HW reads from it to transmit to CXPI bus. Note: The ongoing message transfer will continue when this error happens however, data transferred at CXPI bus will be bogus and HW will invert the CRC to invalidate the message at the receiving node. TX_HEADER and TX_RESPONSE commands are set to '0'."]
    #[inline(always)]
    pub fn tx_overflow_error(&self) -> TX_OVERFLOW_ERROR_R {
        TX_OVERFLOW_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HW sets this field to '1', when RX FIFO is empty and SW reads from it. Note: Upon this error, SW should discard the RX data in RX FIFO."]
    #[inline(always)]
    pub fn rx_underflow_error(&self) -> RX_UNDERFLOW_ERROR_R {
        RX_UNDERFLOW_ERROR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HW sets this field to '1', when TX FIFO is empty and HW reads from it. Note: The ongoing message transfer will continue when this error happens however, data transferred at CXPI will be bogus and HW will invert the CRC to invalidate the message at the receiving node. TX_HEADER and TX_RESPONSE commands are set to '0'."]
    #[inline(always)]
    pub fn tx_underflow_error(&self) -> TX_UNDERFLOW_ERROR_R {
        TX_UNDERFLOW_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HW sets this field to '1', when the stop bit of a byte frame is incorrect. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated and the INTR.RX_HEADER_DONE/TX_HEADER_DONE is NOT activated if the frame error occurs during header byte or if frame error occurs during response byte (if the HEADER and RESPONSE commands are set together))."]
    #[inline(always)]
    pub fn rx_frame_error(&self) -> RX_FRAME_ERROR_R {
        RX_FRAME_ERROR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HW sets this field to '1', when the stop bit of a byte frame is incorrect. This error would be a subset of TX_BIT_ERROR and also subjected to BIT_ERROR_IGNORE field. Note: The ongoing message transfer is aborted (INTR.TX_HEADER_DONE/RX_HEADER_DONE and INTR.TX_RESPONSE_DONE are NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. Note: When CTL0.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. When CTL0.BIT_ERROR_IGNORE is '1', the ongoing message transfer would be transferred."]
    #[inline(always)]
    pub fn tx_frame_error(&self) -> TX_FRAME_ERROR_R {
        TX_FRAME_ERROR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HW sets this field to '1', when a frame header (PID field or PType field) is transmitted (the CMD.TX_HEADER is completed). Specifically: - For PID transmission only and without prior transmission of PTYPE, when followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the message frame transfer. - For PID transmission only and without prior transmission of PTYPE, when not followed by a response command, this field is set to '1' after completion of the header transfer. -Note for the case of PTYPE is transmitted follow by CMD.RX_RESPONSE or no following response, HW sets this field to '1' after transmitting PTYPE."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header_done(&mut self) -> TX_HEADER_DONE_W<0> {
        TX_HEADER_DONE_W::new(self)
    }
    #[doc = "Bit 1 - HW sets this field to '1', when a frame response (frame information fields, data fields, and crc field) is transmitted (the CMD.TX_RESPONSE is completed)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_response_done(&mut self) -> TX_RESPONSE_DONE_W<1> {
        TX_RESPONSE_DONE_W::new(self)
    }
    #[doc = "Bit 3 - HW sets this field to '1', when a wakeup signal is transmitted (per CTL2.T_WAKEUP_LENGTH). This interrupt cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wakeup_done(&mut self) -> TX_WAKEUP_DONE_W<3> {
        TX_WAKEUP_DONE_W::new(self)
    }
    #[doc = "Bit 4 - HW sets this field to '1', when TX trigger is generated (#used TX FIFO &lt; TRIGGER_LEVEL)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_trigger(&mut self) -> TX_FIFO_TRIGGER_W<4> {
        TX_FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when a frame header (PID field or PType field) is received (the CMD.RX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the message frame transfer. - When not followed by a response command, this field is set to '1' after completion of the header transfer if RXPIDZERO_CHECK_EN=1 and header received is PID. If RXPIDZERO_CHECK_EN=0 and response commands are not set, HW will set this field to '1' after receiving PID or PTYPE."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_done(&mut self) -> RX_HEADER_DONE_W<8> {
        RX_HEADER_DONE_W::new(self)
    }
    #[doc = "Bit 9 - HW sets this field to '1', when a frame response (frame information fields, data fields, and crc field) is received (the CMD.RX_RESPONSE is completed)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response_done(&mut self) -> RX_RESPONSE_DONE_W<9> {
        RX_RESPONSE_DONE_W::new(self)
    }
    #[doc = "Bit 10 - HW sets this field to '1', when RX fall is detected in Sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wakeup_detect(&mut self) -> RX_WAKEUP_DETECT_W<10> {
        RX_WAKEUP_DETECT_W::new(self)
    }
    #[doc = "Bit 11 - HW sets this field to '1', when RX trigger is generated (#used RX FIFO > TRIGGER_LEVEL)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_trigger(&mut self) -> RX_FIFO_TRIGGER_W<11> {
        RX_FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 12 - HW sets this field to '1', when RX header (PID/PTYPE field) is received."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_pid_done(&mut self) -> RX_HEADER_PID_DONE_W<12> {
        RX_HEADER_PID_DONE_W::new(self)
    }
    #[doc = "Bit 13 - HW sets this field to '1', when message frame ends after EOF is completed and TX/RX_DATA_LENGTH_ERROR=0."]
    #[inline(always)]
    #[must_use]
    pub fn txrx_complete(&mut self) -> TXRX_COMPLETE_W<13> {
        TXRX_COMPLETE_W::new(self)
    }
    #[doc = "Bit 18 - HW sets this field to '1', when the transmitted/received bytes space within a message frame is > TIMEOUT_LENGTH. SW needs to set TIMEOUT_SEL=0 before clearing TIMEOUT=0 to ensure HW does not immediately sets back the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<18> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 19 - HW sets this field to '1', when it detects arbitration lost after the number of retries has exceed the maximum allowed retries. Note: The ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated and the TX_HEADER and TX_RESPONSE command is cleared to 0)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header_arb_lost(&mut self) -> TX_HEADER_ARB_LOST_W<19> {
        TX_HEADER_ARB_LOST_W::new(self)
    }
    #[doc = "Bit 20 - HW sets this field to '1', when a transmitted 'cxpi_tx_out' value does NOT match a received 'cxpi_rx_in' value. The match is performed for the PID fields or PType (for the START bit and STOP bit only) and for the rest of the response i.e. frame information fields, data fields and the crc field (for the START bit, DATA bits, and STOP bits). Note: When CTL0.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. When CTL0.BIT_ERROR_IGNORE is '1', the ongoing message transfer would be transferred."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bit_error(&mut self) -> TX_BIT_ERROR_W<20> {
        TX_BIT_ERROR_W::new(self)
    }
    #[doc = "Bit 21 - HW sets this field to '1', when received CRC is not matching with the compute CRC from header and response. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_error(&mut self) -> RX_CRC_ERROR_W<21> {
        RX_CRC_ERROR_W::new(self)
    }
    #[doc = "Bit 22 - HW sets this field to '1', when the received PID field or PType field has a parity error. Note: The ongoing message transfer is aborted (INTR.RX_HEADER_PID_DONE is NOT activated)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_parity_error(&mut self) -> RX_HEADER_PARITY_ERROR_W<22> {
        RX_HEADER_PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 23 - HW sets this field to '1, when the received message frame's data fields are more than the value specified in DLC (for normal frame) or DLCEXT (for long frame) i.e. after receiving CRC byte(s), HW is receiving logical '0' during EOF. For the case of receiving data length less than DLC/DLCEXT, HW will also set this field to '1'. This is the case where IBS>9 before the number of data reaches data length, then HW will report as data length error. HW starts checking after frame information byte. Note: SW needs to handle the message transfer i.e. discard or flush out. HW will still set the RX_RESPONSE_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_length_error(&mut self) -> RX_DATA_LENGTH_ERROR_W<23> {
        RX_DATA_LENGTH_ERROR_W::new(self)
    }
    #[doc = "Bit 24 - HW sets this field to '1, when the transmit message frame's data fields are more than the value specified in DLC (for normal frame) or DLCEXT (for long frame) i.e. after transmitting CRC(s) byte, HW is receiving logical '0' during EOF. Note: HW will still set TX_RESPONSE_DONE and the TX_HEADER and TX_RESPONSE commands are set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_length_error(&mut self) -> TX_DATA_LENGTH_ERROR_W<24> {
        TX_DATA_LENGTH_ERROR_W::new(self)
    }
    #[doc = "Bit 25 - HW sets this field to '1', when the RX data is overwritten by HW before the SW reads from it. In CXPI spec, this error is denoted as overrun error. Note: Upon this error, SW should discard the RX data in RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow_error(&mut self) -> RX_OVERFLOW_ERROR_W<25> {
        RX_OVERFLOW_ERROR_W::new(self)
    }
    #[doc = "Bit 26 - HW sets this field to '1', when the TX data is overwritten by SW before the HW reads from it to transmit to CXPI bus. Note: The ongoing message transfer will continue when this error happens however, data transferred at CXPI bus will be bogus and HW will invert the CRC to invalidate the message at the receiving node. TX_HEADER and TX_RESPONSE commands are set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn tx_overflow_error(&mut self) -> TX_OVERFLOW_ERROR_W<26> {
        TX_OVERFLOW_ERROR_W::new(self)
    }
    #[doc = "Bit 27 - HW sets this field to '1', when RX FIFO is empty and SW reads from it. Note: Upon this error, SW should discard the RX data in RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_underflow_error(&mut self) -> RX_UNDERFLOW_ERROR_W<27> {
        RX_UNDERFLOW_ERROR_W::new(self)
    }
    #[doc = "Bit 28 - HW sets this field to '1', when TX FIFO is empty and HW reads from it. Note: The ongoing message transfer will continue when this error happens however, data transferred at CXPI will be bogus and HW will invert the CRC to invalidate the message at the receiving node. TX_HEADER and TX_RESPONSE commands are set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn tx_underflow_error(&mut self) -> TX_UNDERFLOW_ERROR_W<28> {
        TX_UNDERFLOW_ERROR_W::new(self)
    }
    #[doc = "Bit 29 - HW sets this field to '1', when the stop bit of a byte frame is incorrect. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated and the INTR.RX_HEADER_DONE/TX_HEADER_DONE is NOT activated if the frame error occurs during header byte or if frame error occurs during response byte (if the HEADER and RESPONSE commands are set together))."]
    #[inline(always)]
    #[must_use]
    pub fn rx_frame_error(&mut self) -> RX_FRAME_ERROR_W<29> {
        RX_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 30 - HW sets this field to '1', when the stop bit of a byte frame is incorrect. This error would be a subset of TX_BIT_ERROR and also subjected to BIT_ERROR_IGNORE field. Note: The ongoing message transfer is aborted (INTR.TX_HEADER_DONE/RX_HEADER_DONE and INTR.TX_RESPONSE_DONE are NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. Note: When CTL0.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE and INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER and TX_RESPONSE commands are set to '0'. When CTL0.BIT_ERROR_IGNORE is '1', the ongoing message transfer would be transferred."]
    #[inline(always)]
    #[must_use]
    pub fn tx_frame_error(&mut self) -> TX_FRAME_ERROR_W<30> {
        TX_FRAME_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
