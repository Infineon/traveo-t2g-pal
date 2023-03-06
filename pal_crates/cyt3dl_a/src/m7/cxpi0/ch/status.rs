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
#[doc = "Field `RETRIES_COUNT` reader - Retries count. The value reflects the number of retries that HW tries to transmit a header/response. HW will reset counter (either case below): 1. after successfully transmit the retry attempt 2. SW clears the CMD.TX_HEADER='0' 3. HW clearing CMD.TX_HEADER=0 due to errors (TX_BIT_ERROR, TX_HEADER_ARB_LOST, TX_OVERFLOW_ERROR, TX_UNDERFLOW_ERROR, TX_DATA_LENGTH_ERROR, and TX_FRAME_ERROR)"]
pub type RETRIES_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HEADER_RESPONSE` reader - Frame header/response identifier (only valid when TX_BUSY or RX_BUSY is '1') '0' - Frame header being transferred. '1' - Frame response being transferred."]
pub type HEADER_RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `TX_BUSY` reader - Transmitter busy. - Set to '1' on the start of the following commands: TX_HEADER, TX_RESPONSE. - Set to '0' on successful completion of previous commands or when an error is detected."]
pub type TX_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `RX_BUSY` reader - Receiver busy. - Set to '1' on the start of the following commands: RX_HEADER, RX_RESPONSE. - Set to '0' on successful completion of previous commands or when an error is detected."]
pub type RX_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `TX_DONE` reader - Transmitter done: - Set to '0' on the start of a new command. - Set to '1' on successful completion of the following command sequences: - TX_HEADER. - TX_HEADER, TX_RESPONSE. - RX_HEADER, TX_RESPONSE."]
pub type TX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_DONE` reader - Receiver done: - Set to '0' on the start of a new command. - Set to '1' on successful completion of the following commmand sequences: - RX_HEADER, RX_RESPONSE. - TX_HEADER, RX_RESPONSE."]
pub type RX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` reader - Copy of INTR.TIMEOUT"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_ARB_LOST` reader - Copy of INTR.TX_HEADER_ARB_LOST"]
pub type TX_HEADER_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `TX_BIT_ERROR` reader - Copy of INTR.TX_BIT_ERROR."]
pub type TX_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_CRC_ERROR` reader - Copy of INTR.RX_CRC_ERROR."]
pub type RX_CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` reader - Copy of INTR.RX_HEADER_PARITY_ERROR."]
pub type RX_HEADER_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_LENGTH_ERROR` reader - Copy of INTR.RX_DATA_LENGTH_ERROR."]
pub type RX_DATA_LENGTH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_LENGTH_ERROR` reader - Copy of INTR.TX_DATA_LENGTH_ERROR."]
pub type TX_DATA_LENGTH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERFLOW_ERROR` reader - Copy of INTR.RX_OVERFLOW_ERROR."]
pub type RX_OVERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_OVERFLOW_ERROR` reader - Copy of INTR.TX_OVERFLOW_ERROR."]
pub type TX_OVERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_UNDERFLOW_ERROR` reader - Copy of INTR.RX_UNDERFLOW_ERROR."]
pub type RX_UNDERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_UNDERFLOW_ERROR` reader - Copy of INTR.TX_UNDERFLOW_ERROR."]
pub type TX_UNDERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_FRAME_ERROR` reader - Copy of INTR.RX_FRAME_ERROR."]
pub type RX_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_FRAME_ERROR` reader - Copy of INTR.TX_FRAME_ERROR."]
pub type TX_FRAME_ERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - Retries count. The value reflects the number of retries that HW tries to transmit a header/response. HW will reset counter (either case below): 1. after successfully transmit the retry attempt 2. SW clears the CMD.TX_HEADER='0' 3. HW clearing CMD.TX_HEADER=0 due to errors (TX_BIT_ERROR, TX_HEADER_ARB_LOST, TX_OVERFLOW_ERROR, TX_UNDERFLOW_ERROR, TX_DATA_LENGTH_ERROR, and TX_FRAME_ERROR)"]
    #[inline(always)]
    pub fn retries_count(&self) -> RETRIES_COUNT_R {
        RETRIES_COUNT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Frame header/response identifier (only valid when TX_BUSY or RX_BUSY is '1') '0' - Frame header being transferred. '1' - Frame response being transferred."]
    #[inline(always)]
    pub fn header_response(&self) -> HEADER_RESPONSE_R {
        HEADER_RESPONSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitter busy. - Set to '1' on the start of the following commands: TX_HEADER, TX_RESPONSE. - Set to '0' on successful completion of previous commands or when an error is detected."]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receiver busy. - Set to '1' on the start of the following commands: RX_HEADER, RX_RESPONSE. - Set to '0' on successful completion of previous commands or when an error is detected."]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmitter done: - Set to '0' on the start of a new command. - Set to '1' on successful completion of the following command sequences: - TX_HEADER. - TX_HEADER, TX_RESPONSE. - RX_HEADER, TX_RESPONSE."]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receiver done: - Set to '0' on the start of a new command. - Set to '1' on successful completion of the following commmand sequences: - RX_HEADER, RX_RESPONSE. - TX_HEADER, RX_RESPONSE."]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - Copy of INTR.TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Copy of INTR.TX_HEADER_ARB_LOST"]
    #[inline(always)]
    pub fn tx_header_arb_lost(&self) -> TX_HEADER_ARB_LOST_R {
        TX_HEADER_ARB_LOST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Copy of INTR.TX_BIT_ERROR."]
    #[inline(always)]
    pub fn tx_bit_error(&self) -> TX_BIT_ERROR_R {
        TX_BIT_ERROR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Copy of INTR.RX_CRC_ERROR."]
    #[inline(always)]
    pub fn rx_crc_error(&self) -> RX_CRC_ERROR_R {
        RX_CRC_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Copy of INTR.RX_HEADER_PARITY_ERROR."]
    #[inline(always)]
    pub fn rx_header_parity_error(&self) -> RX_HEADER_PARITY_ERROR_R {
        RX_HEADER_PARITY_ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Copy of INTR.RX_DATA_LENGTH_ERROR."]
    #[inline(always)]
    pub fn rx_data_length_error(&self) -> RX_DATA_LENGTH_ERROR_R {
        RX_DATA_LENGTH_ERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Copy of INTR.TX_DATA_LENGTH_ERROR."]
    #[inline(always)]
    pub fn tx_data_length_error(&self) -> TX_DATA_LENGTH_ERROR_R {
        TX_DATA_LENGTH_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Copy of INTR.RX_OVERFLOW_ERROR."]
    #[inline(always)]
    pub fn rx_overflow_error(&self) -> RX_OVERFLOW_ERROR_R {
        RX_OVERFLOW_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Copy of INTR.TX_OVERFLOW_ERROR."]
    #[inline(always)]
    pub fn tx_overflow_error(&self) -> TX_OVERFLOW_ERROR_R {
        TX_OVERFLOW_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Copy of INTR.RX_UNDERFLOW_ERROR."]
    #[inline(always)]
    pub fn rx_underflow_error(&self) -> RX_UNDERFLOW_ERROR_R {
        RX_UNDERFLOW_ERROR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Copy of INTR.TX_UNDERFLOW_ERROR."]
    #[inline(always)]
    pub fn tx_underflow_error(&self) -> TX_UNDERFLOW_ERROR_R {
        TX_UNDERFLOW_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Copy of INTR.RX_FRAME_ERROR."]
    #[inline(always)]
    pub fn rx_frame_error(&self) -> RX_FRAME_ERROR_R {
        RX_FRAME_ERROR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Copy of INTR.TX_FRAME_ERROR."]
    #[inline(always)]
    pub fn tx_frame_error(&self) -> TX_FRAME_ERROR_R {
        TX_FRAME_ERROR_R::new(((self.bits >> 30) & 1) != 0)
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
