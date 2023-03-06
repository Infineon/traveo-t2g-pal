#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_IDX` reader - Number of transferred data and checksum fields in the response (also acts as an index/address into response data field and checksum field registers (DATA0, DATA1, PID_CHECKSUM)) : '0': No data fields transferred. '1': Data field 1 transferred. ... '7': Data fields 1, 2, 3, ... and 7 transferred. '8': Data fields 1, 2, 3, ... and 8 transferred. '9': Data fields 1, 2, 3, ..., 8 and checksum field transferred. '10'-'15': Unused. Set to '0' on the start of a TX_HEADER or RX_HEADER command."]
pub type DATA_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HEADER_RESPONSE` reader - Frame header / response identifier (only valid when TX_BUSY or RX_BUSY is '1'): '0': Frame header being transferred. '1': Frame response being transferred."]
pub type HEADER_RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA0_FRAME_ERROR` reader - Frame response, first data field frame error. HW sets this field to '1' when the received STOP bits of the first response data field have an unexpected value (only after a RX_HEADER command), and this data byte is 0x00. HW clears this field to '0' at the falling edge of SYNC start bit (after INTR.RX_HEADER_BREAK_WAKEUP_DONE). This field is used together with INTR.RX_RESPONSE_FRAME_ERROR to distinguish 'no response', 'error response' and 'correct response' scenarios. Note: The ongoing message transfer is NOT aborted."]
pub type RX_DATA0_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUSY` reader - Transmitter busy. - Set to '1' on the start of the following commands: TX_HEADER, TX_RESPONSE, TX_WAKEUP. - Set to '0' on successful completion of previous commands or when an error is detected. In 'TX_HEADER, RX_RESPONSE' case, set to '0' at the start bit falling edge in the first response data byte, after header transmission"]
pub type TX_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUSY` reader - Receiver busy. - Set to '1' on the start of the following commands: RX_HEADER, RX_RESPONSE. in RX_HEADER case, set at Break filed rising edge. in RX_RESPONSE case, set at the start bit falling edge in the first response data byte. - Set to '0' on successful completion of previous commands or when an error is detected."]
pub type RX_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `TX_DONE` reader - Transmitter done: - Set to '0' on the start of a new command. - Set to '1' on successful completion of the following command sequences (if CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble): - TX_HEADER. - TX_HEADER, TX_RESPONSE. - RX_HEADER, TX_RESPONSE. - TX_WAKEUP."]
pub type TX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_DONE` reader - Receiver done: - Set to '0' on the start of a new command. - Set to '1' on successful completion of the following command sequences (if CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble): - RX_HEADER, RX_RESPONSE. - TX_HEADER, RX_RESPONSE."]
pub type RX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_BIT_ERROR` reader - Copy of INTR.TX_HEADER_BIT_ERROR."]
pub type TX_HEADER_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_BIT_ERROR` reader - Copy of INTR.TX_RESPONSE_BIT_ERROR."]
pub type TX_RESPONSE_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_FRAME_ERROR` reader - Copy of INTR.RX_HEADER_FRAME_ERROR."]
pub type RX_HEADER_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_SYNC_ERROR` reader - Copy of INTR.RX_HEADER_SYNC_ERROR."]
pub type RX_HEADER_SYNC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` reader - Copy of INTR.RX_HEADER_PARITY_ERROR."]
pub type RX_HEADER_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_FRAME_ERROR` reader - Copy of INTR.RX_RESPONSE_FRAME_ERROR."]
pub type RX_RESPONSE_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_CHECKSUM_ERROR` reader - Copy of INTR.RX_RESPONSE_CHECKSUM_ERROR."]
pub type RX_RESPONSE_CHECKSUM_ERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Number of transferred data and checksum fields in the response (also acts as an index/address into response data field and checksum field registers (DATA0, DATA1, PID_CHECKSUM)) : '0': No data fields transferred. '1': Data field 1 transferred. ... '7': Data fields 1, 2, 3, ... and 7 transferred. '8': Data fields 1, 2, 3, ... and 8 transferred. '9': Data fields 1, 2, 3, ..., 8 and checksum field transferred. '10'-'15': Unused. Set to '0' on the start of a TX_HEADER or RX_HEADER command."]
    #[inline(always)]
    pub fn data_idx(&self) -> DATA_IDX_R {
        DATA_IDX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Frame header / response identifier (only valid when TX_BUSY or RX_BUSY is '1'): '0': Frame header being transferred. '1': Frame response being transferred."]
    #[inline(always)]
    pub fn header_response(&self) -> HEADER_RESPONSE_R {
        HEADER_RESPONSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame response, first data field frame error. HW sets this field to '1' when the received STOP bits of the first response data field have an unexpected value (only after a RX_HEADER command), and this data byte is 0x00. HW clears this field to '0' at the falling edge of SYNC start bit (after INTR.RX_HEADER_BREAK_WAKEUP_DONE). This field is used together with INTR.RX_RESPONSE_FRAME_ERROR to distinguish 'no response', 'error response' and 'correct response' scenarios. Note: The ongoing message transfer is NOT aborted."]
    #[inline(always)]
    pub fn rx_data0_frame_error(&self) -> RX_DATA0_FRAME_ERROR_R {
        RX_DATA0_FRAME_ERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitter busy. - Set to '1' on the start of the following commands: TX_HEADER, TX_RESPONSE, TX_WAKEUP. - Set to '0' on successful completion of previous commands or when an error is detected. In 'TX_HEADER, RX_RESPONSE' case, set to '0' at the start bit falling edge in the first response data byte, after header transmission"]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receiver busy. - Set to '1' on the start of the following commands: RX_HEADER, RX_RESPONSE. in RX_HEADER case, set at Break filed rising edge. in RX_RESPONSE case, set at the start bit falling edge in the first response data byte. - Set to '0' on successful completion of previous commands or when an error is detected."]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmitter done: - Set to '0' on the start of a new command. - Set to '1' on successful completion of the following command sequences (if CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble): - TX_HEADER. - TX_HEADER, TX_RESPONSE. - RX_HEADER, TX_RESPONSE. - TX_WAKEUP."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receiver done: - Set to '0' on the start of a new command. - Set to '1' on successful completion of the following command sequences (if CTL.AUTO_EN is '1', this includes the 4-bit period external transceiver disable post-amble): - RX_HEADER, RX_RESPONSE. - TX_HEADER, RX_RESPONSE."]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Copy of INTR.TX_HEADER_BIT_ERROR."]
    #[inline(always)]
    pub fn tx_header_bit_error(&self) -> TX_HEADER_BIT_ERROR_R {
        TX_HEADER_BIT_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Copy of INTR.TX_RESPONSE_BIT_ERROR."]
    #[inline(always)]
    pub fn tx_response_bit_error(&self) -> TX_RESPONSE_BIT_ERROR_R {
        TX_RESPONSE_BIT_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Copy of INTR.RX_HEADER_FRAME_ERROR."]
    #[inline(always)]
    pub fn rx_header_frame_error(&self) -> RX_HEADER_FRAME_ERROR_R {
        RX_HEADER_FRAME_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Copy of INTR.RX_HEADER_SYNC_ERROR."]
    #[inline(always)]
    pub fn rx_header_sync_error(&self) -> RX_HEADER_SYNC_ERROR_R {
        RX_HEADER_SYNC_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Copy of INTR.RX_HEADER_PARITY_ERROR."]
    #[inline(always)]
    pub fn rx_header_parity_error(&self) -> RX_HEADER_PARITY_ERROR_R {
        RX_HEADER_PARITY_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Copy of INTR.RX_RESPONSE_FRAME_ERROR."]
    #[inline(always)]
    pub fn rx_response_frame_error(&self) -> RX_RESPONSE_FRAME_ERROR_R {
        RX_RESPONSE_FRAME_ERROR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Copy of INTR.RX_RESPONSE_CHECKSUM_ERROR."]
    #[inline(always)]
    pub fn rx_response_checksum_error(&self) -> RX_RESPONSE_CHECKSUM_ERROR_R {
        RX_RESPONSE_CHECKSUM_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
