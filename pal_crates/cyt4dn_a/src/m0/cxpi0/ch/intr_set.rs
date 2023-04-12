#[doc = "Register `INTR_SET` reader"]
pub struct R(crate::R<INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SET` writer"]
pub struct W(crate::W<INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SET_SPEC>;
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
impl From<crate::W<INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_HEADER_DONE` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_DONE` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_HEADER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_RESPONSE_DONE` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_DONE` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_RESPONSE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_WAKEUP_DONE` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_WAKEUP_DONE` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_WAKEUP_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_TRIGGER` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_TRIGGER` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_DONE` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_DONE` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_HEADER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE_DONE` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_DONE` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_RESPONSE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_WAKEUP_DETECT` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_WAKEUP_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RX_WAKEUP_DETECT` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_WAKEUP_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_TRIGGER` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_TRIGGER` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_PID_DONE` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_HEADER_PID_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PID_DONE` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_HEADER_PID_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TXRX_COMPLETE` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TXRX_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `TXRX_COMPLETE` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TXRX_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_HEADER_ARB_LOST` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_HEADER_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_ARB_LOST` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_HEADER_ARB_LOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_BIT_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_BIT_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_BIT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_CRC_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_CRC_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_CRC_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_HEADER_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_HEADER_PARITY_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_DATA_LENGTH_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_DATA_LENGTH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_LENGTH_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_DATA_LENGTH_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_DATA_LENGTH_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_DATA_LENGTH_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_LENGTH_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_DATA_LENGTH_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_OVERFLOW_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_OVERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERFLOW_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_OVERFLOW_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_OVERFLOW_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_OVERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_OVERFLOW_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_OVERFLOW_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_UNDERFLOW_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_UNDERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_UNDERFLOW_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_UNDERFLOW_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_UNDERFLOW_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_UNDERFLOW_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_UNDERFLOW_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_UNDERFLOW_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_FRAME_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_FRAME_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type RX_FRAME_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_FRAME_ERROR` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_FRAME_ERROR` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TX_FRAME_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_header_done(&self) -> TX_HEADER_DONE_R {
        TX_HEADER_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_response_done(&self) -> TX_RESPONSE_DONE_R {
        TX_RESPONSE_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_wakeup_done(&self) -> TX_WAKEUP_DONE_R {
        TX_WAKEUP_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_fifo_trigger(&self) -> TX_FIFO_TRIGGER_R {
        TX_FIFO_TRIGGER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_header_done(&self) -> RX_HEADER_DONE_R {
        RX_HEADER_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_response_done(&self) -> RX_RESPONSE_DONE_R {
        RX_RESPONSE_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_wakeup_detect(&self) -> RX_WAKEUP_DETECT_R {
        RX_WAKEUP_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_fifo_trigger(&self) -> RX_FIFO_TRIGGER_R {
        RX_FIFO_TRIGGER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_header_pid_done(&self) -> RX_HEADER_PID_DONE_R {
        RX_HEADER_PID_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn txrx_complete(&self) -> TXRX_COMPLETE_R {
        TXRX_COMPLETE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_header_arb_lost(&self) -> TX_HEADER_ARB_LOST_R {
        TX_HEADER_ARB_LOST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_bit_error(&self) -> TX_BIT_ERROR_R {
        TX_BIT_ERROR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_crc_error(&self) -> RX_CRC_ERROR_R {
        RX_CRC_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_header_parity_error(&self) -> RX_HEADER_PARITY_ERROR_R {
        RX_HEADER_PARITY_ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_data_length_error(&self) -> RX_DATA_LENGTH_ERROR_R {
        RX_DATA_LENGTH_ERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_data_length_error(&self) -> TX_DATA_LENGTH_ERROR_R {
        TX_DATA_LENGTH_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_overflow_error(&self) -> RX_OVERFLOW_ERROR_R {
        RX_OVERFLOW_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_overflow_error(&self) -> TX_OVERFLOW_ERROR_R {
        TX_OVERFLOW_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_underflow_error(&self) -> RX_UNDERFLOW_ERROR_R {
        RX_UNDERFLOW_ERROR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_underflow_error(&self) -> TX_UNDERFLOW_ERROR_R {
        TX_UNDERFLOW_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn rx_frame_error(&self) -> RX_FRAME_ERROR_R {
        RX_FRAME_ERROR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn tx_frame_error(&self) -> TX_FRAME_ERROR_R {
        TX_FRAME_ERROR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header_done(&mut self) -> TX_HEADER_DONE_W<0> {
        TX_HEADER_DONE_W::new(self)
    }
    #[doc = "Bit 1 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_response_done(&mut self) -> TX_RESPONSE_DONE_W<1> {
        TX_RESPONSE_DONE_W::new(self)
    }
    #[doc = "Bit 3 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wakeup_done(&mut self) -> TX_WAKEUP_DONE_W<3> {
        TX_WAKEUP_DONE_W::new(self)
    }
    #[doc = "Bit 4 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_trigger(&mut self) -> TX_FIFO_TRIGGER_W<4> {
        TX_FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 8 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_done(&mut self) -> RX_HEADER_DONE_W<8> {
        RX_HEADER_DONE_W::new(self)
    }
    #[doc = "Bit 9 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response_done(&mut self) -> RX_RESPONSE_DONE_W<9> {
        RX_RESPONSE_DONE_W::new(self)
    }
    #[doc = "Bit 10 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wakeup_detect(&mut self) -> RX_WAKEUP_DETECT_W<10> {
        RX_WAKEUP_DETECT_W::new(self)
    }
    #[doc = "Bit 11 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_trigger(&mut self) -> RX_FIFO_TRIGGER_W<11> {
        RX_FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 12 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_pid_done(&mut self) -> RX_HEADER_PID_DONE_W<12> {
        RX_HEADER_PID_DONE_W::new(self)
    }
    #[doc = "Bit 13 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn txrx_complete(&mut self) -> TXRX_COMPLETE_W<13> {
        TXRX_COMPLETE_W::new(self)
    }
    #[doc = "Bit 18 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<18> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 19 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header_arb_lost(&mut self) -> TX_HEADER_ARB_LOST_W<19> {
        TX_HEADER_ARB_LOST_W::new(self)
    }
    #[doc = "Bit 20 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bit_error(&mut self) -> TX_BIT_ERROR_W<20> {
        TX_BIT_ERROR_W::new(self)
    }
    #[doc = "Bit 21 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_error(&mut self) -> RX_CRC_ERROR_W<21> {
        RX_CRC_ERROR_W::new(self)
    }
    #[doc = "Bit 22 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header_parity_error(&mut self) -> RX_HEADER_PARITY_ERROR_W<22> {
        RX_HEADER_PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 23 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_length_error(&mut self) -> RX_DATA_LENGTH_ERROR_W<23> {
        RX_DATA_LENGTH_ERROR_W::new(self)
    }
    #[doc = "Bit 24 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_length_error(&mut self) -> TX_DATA_LENGTH_ERROR_W<24> {
        TX_DATA_LENGTH_ERROR_W::new(self)
    }
    #[doc = "Bit 25 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow_error(&mut self) -> RX_OVERFLOW_ERROR_W<25> {
        RX_OVERFLOW_ERROR_W::new(self)
    }
    #[doc = "Bit 26 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_overflow_error(&mut self) -> TX_OVERFLOW_ERROR_W<26> {
        TX_OVERFLOW_ERROR_W::new(self)
    }
    #[doc = "Bit 27 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_underflow_error(&mut self) -> RX_UNDERFLOW_ERROR_W<27> {
        RX_UNDERFLOW_ERROR_W::new(self)
    }
    #[doc = "Bit 28 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_underflow_error(&mut self) -> TX_UNDERFLOW_ERROR_W<28> {
        TX_UNDERFLOW_ERROR_W::new(self)
    }
    #[doc = "Bit 29 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_frame_error(&mut self) -> RX_FRAME_ERROR_W<29> {
        RX_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 30 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
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
#[doc = "Interrupt set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_set::R](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_set::W](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
