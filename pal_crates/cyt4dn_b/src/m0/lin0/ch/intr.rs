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
#[doc = "Field `TX_HEADER_DONE` reader - HW sets this field to '1', when a frame header (break field, synchronization field and PID field) is transmitted (the CMD.TX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the frame header transfer. - When not followed by a response command, this field is set to '1' after completion of the frame header transfer. If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: used in UART mode."]
pub type TX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_DONE` writer - HW sets this field to '1', when a frame header (break field, synchronization field and PID field) is transmitted (the CMD.TX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the frame header transfer. - When not followed by a response command, this field is set to '1' after completion of the frame header transfer. If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: used in UART mode."]
pub type TX_HEADER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_RESPONSE_DONE` reader - HW sets this field to '1', when a frame response (data fields and checksum field) is transmitted (the CMD.TX_RESPONSE is completed). If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble."]
pub type TX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_DONE` writer - HW sets this field to '1', when a frame response (data fields and checksum field) is transmitted (the CMD.TX_RESPONSE is completed). If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble."]
pub type TX_RESPONSE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_WAKEUP_DONE` reader - HW sets this field to '1', when a wakeup signal is transmitted (per CTL.BREAK_WAKEUP_LENGTH). This cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal."]
pub type TX_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_WAKEUP_DONE` writer - HW sets this field to '1', when a wakeup signal is transmitted (per CTL.BREAK_WAKEUP_LENGTH). This cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal."]
pub type TX_WAKEUP_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_DONE` reader - HW sets this field to '1', when a frame header (break field, synchronization field and PID field) is received (the CMD.RX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the frame header transfer. - When not followed by a response command, this field is set to '1' after completion of the frame header transfer. If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: used in UART mode."]
pub type RX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_DONE` writer - HW sets this field to '1', when a frame header (break field, synchronization field and PID field) is received (the CMD.RX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the frame header transfer. - When not followed by a response command, this field is set to '1' after completion of the frame header transfer. If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: used in UART mode."]
pub type RX_HEADER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE_DONE` reader - HW sets this field to '1', when a frame response (data fields and checksum field) is received (the CMD.RX_RESPONSE is completed). If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: activation implies that RX_RESPONSE_FRAME_ERROR and RX_RESPONSE_CHECKSUM_ERROR are not activated during response reception"]
pub type RX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_DONE` writer - HW sets this field to '1', when a frame response (data fields and checksum field) is received (the CMD.RX_RESPONSE is completed). If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: activation implies that RX_RESPONSE_FRAME_ERROR and RX_RESPONSE_CHECKSUM_ERROR are not activated during response reception"]
pub type RX_RESPONSE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_BREAK_WAKEUP_DONE` reader - HW sets this field to '1', when a break or wakeup signal is received (per CTL.BREAK_WAKEUP_LENGTH). This cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal. The break or wakeup detection is always enabled, regardless of CMD register setting."]
pub type RX_BREAK_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_BREAK_WAKEUP_DONE` writer - HW sets this field to '1', when a break or wakeup signal is received (per CTL.BREAK_WAKEUP_LENGTH). This cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal. The break or wakeup detection is always enabled, regardless of CMD register setting."]
pub type RX_BREAK_WAKEUP_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_SYNC_DONE` reader - HW sets this field to '1', when a synchronization field is received (including trailing STOP bits)."]
pub type RX_HEADER_SYNC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_SYNC_DONE` writer - HW sets this field to '1', when a synchronization field is received (including trailing STOP bits)."]
pub type RX_HEADER_SYNC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_NOISE_DETECT` reader - HW sets this field to '1', when isolated '0' or '1' 'in_rx_in' values are observed or when during sampling the last three 'lin_rx_in' values do NOT all have the same value. This mismatch is an indication of noise on the LIN line. Note: The ongoing frame transfer is NOT aborted. Note: Used in UART mode."]
pub type RX_NOISE_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RX_NOISE_DETECT` writer - HW sets this field to '1', when isolated '0' or '1' 'in_rx_in' values are observed or when during sampling the last three 'lin_rx_in' values do NOT all have the same value. This mismatch is an indication of noise on the LIN line. Note: The ongoing frame transfer is NOT aborted. Note: Used in UART mode."]
pub type RX_NOISE_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - HW sets this field to '1', when a frame, frame header or frame response timeout is detected (per CTL.FRAME_TIMEOUT_SEL). Note: The ongoing frame transfer is NOT aborted."]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - HW sets this field to '1', when a frame, frame header or frame response timeout is detected (per CTL.FRAME_TIMEOUT_SEL). Note: The ongoing frame transfer is NOT aborted."]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_HEADER_BIT_ERROR` reader - HW sets this field to '1', when a transmitted 'lin_tx_out' value does NOT match a received 'lin_rx_in' value (during header transmission). This specific test allows for delay through the external transceiver. This mismatch is an indication of bus collisions on the LIN line. The match is performed for the Wakeup, Break, SYNC and the PID fields (for the START bit, data Byte and STOP bit). Note: When CTL.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. Note: Used in UART mode."]
pub type TX_HEADER_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_BIT_ERROR` writer - HW sets this field to '1', when a transmitted 'lin_tx_out' value does NOT match a received 'lin_rx_in' value (during header transmission). This specific test allows for delay through the external transceiver. This mismatch is an indication of bus collisions on the LIN line. The match is performed for the Wakeup, Break, SYNC and the PID fields (for the START bit, data Byte and STOP bit). Note: When CTL.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. Note: Used in UART mode."]
pub type TX_HEADER_BIT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_RESPONSE_BIT_ERROR` reader - HW sets this field to '1', when a transmitted 'lin_tx_out' value does NOT match a received 'lin_rx_in' value (during response transmission). The match is performed for the data fields and the checksum field (for the START bit, data Byte and STOP bit). Note: When CTL.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
pub type TX_RESPONSE_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_BIT_ERROR` writer - HW sets this field to '1', when a transmitted 'lin_tx_out' value does NOT match a received 'lin_rx_in' value (during response transmission). The match is performed for the data fields and the checksum field (for the START bit, data Byte and STOP bit). Note: When CTL.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
pub type TX_RESPONSE_BIT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_FRAME_ERROR` reader - HW sets this field to '1', when the received START or STOP bits have an unexpected value (during header reception). Note: The ongoing message transfer is aborted (INTR.RX_HEADER_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. Note: Used in UART mode."]
pub type RX_HEADER_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_FRAME_ERROR` writer - HW sets this field to '1', when the received START or STOP bits have an unexpected value (during header reception). Note: The ongoing message transfer is aborted (INTR.RX_HEADER_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. Note: Used in UART mode."]
pub type RX_HEADER_FRAME_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_SYNC_ERROR` reader - HW sets this field to '1', when the received synchronization field is not received within the synchronization counter range \\[106, 152\\]
(see TX_RX_STATUS.SYNC_COUNTER). Note: The ongoing message transfer is aborted (INTR.RX_HEADER_SYNC_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
pub type RX_HEADER_SYNC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_SYNC_ERROR` writer - HW sets this field to '1', when the received synchronization field is not received within the synchronization counter range \\[106, 152\\]
(see TX_RX_STATUS.SYNC_COUNTER). Note: The ongoing message transfer is aborted (INTR.RX_HEADER_SYNC_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
pub type RX_HEADER_SYNC_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` reader - HW sets this field to '1', when the received PID field has a parity error. Note: The ongoing message transfer is aborted (INTR.RX_PID_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. +G119 HW sets this field to '1', when the received data field has a parity error (when CTL0.PARITY_EN is '1')."]
pub type RX_HEADER_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` writer - HW sets this field to '1', when the received PID field has a parity error. Note: The ongoing message transfer is aborted (INTR.RX_PID_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. +G119 HW sets this field to '1', when the received data field has a parity error (when CTL0.PARITY_EN is '1')."]
pub type RX_HEADER_PARITY_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE_FRAME_ERROR` reader - HW sets this field to '1', when the received START or STOP bits have an unexpected value (during response reception). HW does NOT use this field for the STOP bits of the first data field after a RX_HEADER command, if the received data byte is 0x00. (STATUS.RX_DATA0_FRAME_ERROR is used instead). Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
pub type RX_RESPONSE_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_FRAME_ERROR` writer - HW sets this field to '1', when the received START or STOP bits have an unexpected value (during response reception). HW does NOT use this field for the STOP bits of the first data field after a RX_HEADER command, if the received data byte is 0x00. (STATUS.RX_DATA0_FRAME_ERROR is used instead). Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
pub type RX_RESPONSE_FRAME_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE_CHECKSUM_ERROR` reader - HW sets this field to '1', when the calculated checksum over the received PID and data fields is not the same as the received checksum. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
pub type RX_RESPONSE_CHECKSUM_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_CHECKSUM_ERROR` writer - HW sets this field to '1', when the calculated checksum over the received PID and data fields is not the same as the received checksum. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
pub type RX_RESPONSE_CHECKSUM_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HW sets this field to '1', when a frame header (break field, synchronization field and PID field) is transmitted (the CMD.TX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the frame header transfer. - When not followed by a response command, this field is set to '1' after completion of the frame header transfer. If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: used in UART mode."]
    #[inline(always)]
    pub fn tx_header_done(&self) -> TX_HEADER_DONE_R {
        TX_HEADER_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HW sets this field to '1', when a frame response (data fields and checksum field) is transmitted (the CMD.TX_RESPONSE is completed). If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble."]
    #[inline(always)]
    pub fn tx_response_done(&self) -> TX_RESPONSE_DONE_R {
        TX_RESPONSE_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HW sets this field to '1', when a wakeup signal is transmitted (per CTL.BREAK_WAKEUP_LENGTH). This cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal."]
    #[inline(always)]
    pub fn tx_wakeup_done(&self) -> TX_WAKEUP_DONE_R {
        TX_WAKEUP_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when a frame header (break field, synchronization field and PID field) is received (the CMD.RX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the frame header transfer. - When not followed by a response command, this field is set to '1' after completion of the frame header transfer. If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: used in UART mode."]
    #[inline(always)]
    pub fn rx_header_done(&self) -> RX_HEADER_DONE_R {
        RX_HEADER_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HW sets this field to '1', when a frame response (data fields and checksum field) is received (the CMD.RX_RESPONSE is completed). If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: activation implies that RX_RESPONSE_FRAME_ERROR and RX_RESPONSE_CHECKSUM_ERROR are not activated during response reception"]
    #[inline(always)]
    pub fn rx_response_done(&self) -> RX_RESPONSE_DONE_R {
        RX_RESPONSE_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HW sets this field to '1', when a break or wakeup signal is received (per CTL.BREAK_WAKEUP_LENGTH). This cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal. The break or wakeup detection is always enabled, regardless of CMD register setting."]
    #[inline(always)]
    pub fn rx_break_wakeup_done(&self) -> RX_BREAK_WAKEUP_DONE_R {
        RX_BREAK_WAKEUP_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HW sets this field to '1', when a synchronization field is received (including trailing STOP bits)."]
    #[inline(always)]
    pub fn rx_header_sync_done(&self) -> RX_HEADER_SYNC_DONE_R {
        RX_HEADER_SYNC_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - HW sets this field to '1', when isolated '0' or '1' 'in_rx_in' values are observed or when during sampling the last three 'lin_rx_in' values do NOT all have the same value. This mismatch is an indication of noise on the LIN line. Note: The ongoing frame transfer is NOT aborted. Note: Used in UART mode."]
    #[inline(always)]
    pub fn rx_noise_detect(&self) -> RX_NOISE_DETECT_R {
        RX_NOISE_DETECT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HW sets this field to '1', when a frame, frame header or frame response timeout is detected (per CTL.FRAME_TIMEOUT_SEL). Note: The ongoing frame transfer is NOT aborted."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - HW sets this field to '1', when a transmitted 'lin_tx_out' value does NOT match a received 'lin_rx_in' value (during header transmission). This specific test allows for delay through the external transceiver. This mismatch is an indication of bus collisions on the LIN line. The match is performed for the Wakeup, Break, SYNC and the PID fields (for the START bit, data Byte and STOP bit). Note: When CTL.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. Note: Used in UART mode."]
    #[inline(always)]
    pub fn tx_header_bit_error(&self) -> TX_HEADER_BIT_ERROR_R {
        TX_HEADER_BIT_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HW sets this field to '1', when a transmitted 'lin_tx_out' value does NOT match a received 'lin_rx_in' value (during response transmission). The match is performed for the data fields and the checksum field (for the START bit, data Byte and STOP bit). Note: When CTL.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
    #[inline(always)]
    pub fn tx_response_bit_error(&self) -> TX_RESPONSE_BIT_ERROR_R {
        TX_RESPONSE_BIT_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - HW sets this field to '1', when the received START or STOP bits have an unexpected value (during header reception). Note: The ongoing message transfer is aborted (INTR.RX_HEADER_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. Note: Used in UART mode."]
    #[inline(always)]
    pub fn rx_header_frame_error(&self) -> RX_HEADER_FRAME_ERROR_R {
        RX_HEADER_FRAME_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HW sets this field to '1', when the received synchronization field is not received within the synchronization counter range \\[106, 152\\]
(see TX_RX_STATUS.SYNC_COUNTER). Note: The ongoing message transfer is aborted (INTR.RX_HEADER_SYNC_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
    #[inline(always)]
    pub fn rx_header_sync_error(&self) -> RX_HEADER_SYNC_ERROR_R {
        RX_HEADER_SYNC_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HW sets this field to '1', when the received PID field has a parity error. Note: The ongoing message transfer is aborted (INTR.RX_PID_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. +G119 HW sets this field to '1', when the received data field has a parity error (when CTL0.PARITY_EN is '1')."]
    #[inline(always)]
    pub fn rx_header_parity_error(&self) -> RX_HEADER_PARITY_ERROR_R {
        RX_HEADER_PARITY_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HW sets this field to '1', when the received START or STOP bits have an unexpected value (during response reception). HW does NOT use this field for the STOP bits of the first data field after a RX_HEADER command, if the received data byte is 0x00. (STATUS.RX_DATA0_FRAME_ERROR is used instead). Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
    #[inline(always)]
    pub fn rx_response_frame_error(&self) -> RX_RESPONSE_FRAME_ERROR_R {
        RX_RESPONSE_FRAME_ERROR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HW sets this field to '1', when the calculated checksum over the received PID and data fields is not the same as the received checksum. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
    #[inline(always)]
    pub fn rx_response_checksum_error(&self) -> RX_RESPONSE_CHECKSUM_ERROR_R {
        RX_RESPONSE_CHECKSUM_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HW sets this field to '1', when a frame header (break field, synchronization field and PID field) is transmitted (the CMD.TX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the frame header transfer. - When not followed by a response command, this field is set to '1' after completion of the frame header transfer. If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: used in UART mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header_done(&mut self) -> TX_HEADER_DONE_W<0> {
        TX_HEADER_DONE_W::new(self)
    }
    #[doc = "Bit 1 - HW sets this field to '1', when a frame response (data fields and checksum field) is transmitted (the CMD.TX_RESPONSE is completed). If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble."]
    #[inline(always)]
    #[must_use]
    pub fn tx_response_done(&mut self) -> TX_RESPONSE_DONE_W<1> {
        TX_RESPONSE_DONE_W::new(self)
    }
    #[doc = "Bit 2 - HW sets this field to '1', when a wakeup signal is transmitted (per CTL.BREAK_WAKEUP_LENGTH). This cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wakeup_done(&mut self) -> TX_WAKEUP_DONE_W<2> {
        TX_WAKEUP_DONE_W::new(self)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when a frame header (break field, synchronization field and PID field) is received (the CMD.RX_HEADER is completed). Specifically: - When followed by CMD.TX_RESPONSE or CMD.RX_RESPONSE, this field is set to '1' after completion of the frame header transfer. - When not followed by a response command, this field is set to '1' after completion of the frame header transfer. If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: used in UART mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_done(&mut self) -> RX_HEADER_DONE_W<8> {
        RX_HEADER_DONE_W::new(self)
    }
    #[doc = "Bit 9 - HW sets this field to '1', when a frame response (data fields and checksum field) is received (the CMD.RX_RESPONSE is completed). If CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble. Note: activation implies that RX_RESPONSE_FRAME_ERROR and RX_RESPONSE_CHECKSUM_ERROR are not activated during response reception"]
    #[inline(always)]
    #[must_use]
    pub fn rx_response_done(&mut self) -> RX_RESPONSE_DONE_W<9> {
        RX_RESPONSE_DONE_W::new(self)
    }
    #[doc = "Bit 10 - HW sets this field to '1', when a break or wakeup signal is received (per CTL.BREAK_WAKEUP_LENGTH). This cause is activated on a transition from dominant/'0' state to recessive/'1' state; i.e. at the end of the wakeup signal. The break or wakeup detection is always enabled, regardless of CMD register setting."]
    #[inline(always)]
    #[must_use]
    pub fn rx_break_wakeup_done(&mut self) -> RX_BREAK_WAKEUP_DONE_W<10> {
        RX_BREAK_WAKEUP_DONE_W::new(self)
    }
    #[doc = "Bit 11 - HW sets this field to '1', when a synchronization field is received (including trailing STOP bits)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_sync_done(&mut self) -> RX_HEADER_SYNC_DONE_W<11> {
        RX_HEADER_SYNC_DONE_W::new(self)
    }
    #[doc = "Bit 13 - HW sets this field to '1', when isolated '0' or '1' 'in_rx_in' values are observed or when during sampling the last three 'lin_rx_in' values do NOT all have the same value. This mismatch is an indication of noise on the LIN line. Note: The ongoing frame transfer is NOT aborted. Note: Used in UART mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_noise_detect(&mut self) -> RX_NOISE_DETECT_W<13> {
        RX_NOISE_DETECT_W::new(self)
    }
    #[doc = "Bit 14 - HW sets this field to '1', when a frame, frame header or frame response timeout is detected (per CTL.FRAME_TIMEOUT_SEL). Note: The ongoing frame transfer is NOT aborted."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<14> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 16 - HW sets this field to '1', when a transmitted 'lin_tx_out' value does NOT match a received 'lin_rx_in' value (during header transmission). This specific test allows for delay through the external transceiver. This mismatch is an indication of bus collisions on the LIN line. The match is performed for the Wakeup, Break, SYNC and the PID fields (for the START bit, data Byte and STOP bit). Note: When CTL.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_HEADER_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. Note: Used in UART mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header_bit_error(&mut self) -> TX_HEADER_BIT_ERROR_W<16> {
        TX_HEADER_BIT_ERROR_W::new(self)
    }
    #[doc = "Bit 17 - HW sets this field to '1', when a transmitted 'lin_tx_out' value does NOT match a received 'lin_rx_in' value (during response transmission). The match is performed for the data fields and the checksum field (for the START bit, data Byte and STOP bit). Note: When CTL.BIT_ERROR_IGNORE is '0', the ongoing message transfer is aborted (INTR.TX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn tx_response_bit_error(&mut self) -> TX_RESPONSE_BIT_ERROR_W<17> {
        TX_RESPONSE_BIT_ERROR_W::new(self)
    }
    #[doc = "Bit 24 - HW sets this field to '1', when the received START or STOP bits have an unexpected value (during header reception). Note: The ongoing message transfer is aborted (INTR.RX_HEADER_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. Note: Used in UART mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_frame_error(&mut self) -> RX_HEADER_FRAME_ERROR_W<24> {
        RX_HEADER_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 25 - HW sets this field to '1', when the received synchronization field is not received within the synchronization counter range \\[106, 152\\]
(see TX_RX_STATUS.SYNC_COUNTER). Note: The ongoing message transfer is aborted (INTR.RX_HEADER_SYNC_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_sync_error(&mut self) -> RX_HEADER_SYNC_ERROR_W<25> {
        RX_HEADER_SYNC_ERROR_W::new(self)
    }
    #[doc = "Bit 26 - HW sets this field to '1', when the received PID field has a parity error. Note: The ongoing message transfer is aborted (INTR.RX_PID_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'. +G119 HW sets this field to '1', when the received data field has a parity error (when CTL0.PARITY_EN is '1')."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_parity_error(&mut self) -> RX_HEADER_PARITY_ERROR_W<26> {
        RX_HEADER_PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 27 - HW sets this field to '1', when the received START or STOP bits have an unexpected value (during response reception). HW does NOT use this field for the STOP bits of the first data field after a RX_HEADER command, if the received data byte is 0x00. (STATUS.RX_DATA0_FRAME_ERROR is used instead). Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response_frame_error(&mut self) -> RX_RESPONSE_FRAME_ERROR_W<27> {
        RX_RESPONSE_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 28 - HW sets this field to '1', when the calculated checksum over the received PID and data fields is not the same as the received checksum. Note: The ongoing message transfer is aborted (INTR.RX_RESPONSE_DONE is NOT activated) and the TX_HEADER, TX_RESPONSE and TX_WAKEUP commands are set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response_checksum_error(&mut self) -> RX_RESPONSE_CHECKSUM_ERROR_W<28> {
        RX_RESPONSE_CHECKSUM_ERROR_W::new(self)
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
