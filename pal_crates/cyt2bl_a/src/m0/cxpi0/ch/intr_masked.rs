#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_HEADER_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_WAKEUP_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_TRIGGER` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_WAKEUP_DETECT` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_WAKEUP_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_TRIGGER` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PID_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_HEADER_PID_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TXRX_COMPLETE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TXRX_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_ARB_LOST` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_HEADER_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `TX_BIT_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_CRC_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_HEADER_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_LENGTH_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_DATA_LENGTH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_LENGTH_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_DATA_LENGTH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERFLOW_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_OVERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_OVERFLOW_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_OVERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_UNDERFLOW_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_UNDERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_UNDERFLOW_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_UNDERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_FRAME_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_FRAME_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_FRAME_ERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_header_done(&self) -> TX_HEADER_DONE_R {
        TX_HEADER_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_response_done(&self) -> TX_RESPONSE_DONE_R {
        TX_RESPONSE_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_wakeup_done(&self) -> TX_WAKEUP_DONE_R {
        TX_WAKEUP_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_fifo_trigger(&self) -> TX_FIFO_TRIGGER_R {
        TX_FIFO_TRIGGER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_header_done(&self) -> RX_HEADER_DONE_R {
        RX_HEADER_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_response_done(&self) -> RX_RESPONSE_DONE_R {
        RX_RESPONSE_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_wakeup_detect(&self) -> RX_WAKEUP_DETECT_R {
        RX_WAKEUP_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_fifo_trigger(&self) -> RX_FIFO_TRIGGER_R {
        RX_FIFO_TRIGGER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_header_pid_done(&self) -> RX_HEADER_PID_DONE_R {
        RX_HEADER_PID_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn txrx_complete(&self) -> TXRX_COMPLETE_R {
        TXRX_COMPLETE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_header_arb_lost(&self) -> TX_HEADER_ARB_LOST_R {
        TX_HEADER_ARB_LOST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_bit_error(&self) -> TX_BIT_ERROR_R {
        TX_BIT_ERROR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_crc_error(&self) -> RX_CRC_ERROR_R {
        RX_CRC_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_header_parity_error(&self) -> RX_HEADER_PARITY_ERROR_R {
        RX_HEADER_PARITY_ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_data_length_error(&self) -> RX_DATA_LENGTH_ERROR_R {
        RX_DATA_LENGTH_ERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_data_length_error(&self) -> TX_DATA_LENGTH_ERROR_R {
        TX_DATA_LENGTH_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_overflow_error(&self) -> RX_OVERFLOW_ERROR_R {
        RX_OVERFLOW_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_overflow_error(&self) -> TX_OVERFLOW_ERROR_R {
        TX_OVERFLOW_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_underflow_error(&self) -> RX_UNDERFLOW_ERROR_R {
        RX_UNDERFLOW_ERROR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_underflow_error(&self) -> TX_UNDERFLOW_ERROR_R {
        TX_UNDERFLOW_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_frame_error(&self) -> RX_FRAME_ERROR_R {
        RX_FRAME_ERROR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_frame_error(&self) -> TX_FRAME_ERROR_R {
        TX_FRAME_ERROR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
