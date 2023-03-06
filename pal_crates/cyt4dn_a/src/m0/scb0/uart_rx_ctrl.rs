#[doc = "Register `UART_RX_CTRL` reader"]
pub struct R(crate::R<UART_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_RX_CTRL` writer"]
pub struct W(crate::W<UART_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_RX_CTRL_SPEC>;
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
impl From<crate::W<UART_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_BITS` reader - N/A"]
pub type STOP_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_BITS` writer - N/A"]
pub type STOP_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_RX_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PARITY` reader - N/A"]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - N/A"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `PARITY_ENABLED` reader - N/A"]
pub type PARITY_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ENABLED` writer - N/A"]
pub type PARITY_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `POLARITY` reader - Inverts incoming RX line signal. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - Inverts incoming RX line signal. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `DROP_ON_PARITY_ERROR` reader - Behavior when a parity check fails. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost. Only applicable in standard UART and SmartCard submodes (negatively acknowledged SmartCard data frames may be dropped with this field)."]
pub type DROP_ON_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `DROP_ON_PARITY_ERROR` writer - Behavior when a parity check fails. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost. Only applicable in standard UART and SmartCard submodes (negatively acknowledged SmartCard data frames may be dropped with this field)."]
pub type DROP_ON_PARITY_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `DROP_ON_FRAME_ERROR` reader - Behavior when an error is detected in a start or stop period. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost."]
pub type DROP_ON_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `DROP_ON_FRAME_ERROR` writer - Behavior when an error is detected in a start or stop period. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost."]
pub type DROP_ON_FRAME_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `MP_MODE` reader - N/A"]
pub type MP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MP_MODE` writer - N/A"]
pub type MP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `LIN_MODE` reader - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minimum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
pub type LIN_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LIN_MODE` writer - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minimum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
pub type LIN_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `SKIP_START` reader - N/A"]
pub type SKIP_START_R = crate::BitReader<bool>;
#[doc = "Field `SKIP_START` writer - N/A"]
pub type SKIP_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `BREAK_WIDTH` reader - N/A"]
pub type BREAK_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREAK_WIDTH` writer - N/A"]
pub type BREAK_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_RX_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `BREAK_LEVEL` reader - N/A"]
pub type BREAK_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `BREAK_LEVEL` writer - N/A"]
pub type BREAK_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn parity_enabled(&self) -> PARITY_ENABLED_R {
        PARITY_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Inverts incoming RX line signal. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Behavior when a parity check fails. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost. Only applicable in standard UART and SmartCard submodes (negatively acknowledged SmartCard data frames may be dropped with this field)."]
    #[inline(always)]
    pub fn drop_on_parity_error(&self) -> DROP_ON_PARITY_ERROR_R {
        DROP_ON_PARITY_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Behavior when an error is detected in a start or stop period. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub fn drop_on_frame_error(&self) -> DROP_ON_FRAME_ERROR_R {
        DROP_ON_FRAME_ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn mp_mode(&self) -> MP_MODE_R {
        MP_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minimum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
    #[inline(always)]
    pub fn lin_mode(&self) -> LIN_MODE_R {
        LIN_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn skip_start(&self) -> SKIP_START_R {
        SKIP_START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - N/A"]
    #[inline(always)]
    pub fn break_width(&self) -> BREAK_WIDTH_R {
        BREAK_WIDTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn break_level(&self) -> BREAK_LEVEL_R {
        BREAK_LEVEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<0> {
        STOP_BITS_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<4> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn parity_enabled(&mut self) -> PARITY_ENABLED_W<5> {
        PARITY_ENABLED_W::new(self)
    }
    #[doc = "Bit 6 - Inverts incoming RX line signal. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<6> {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 8 - Behavior when a parity check fails. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost. Only applicable in standard UART and SmartCard submodes (negatively acknowledged SmartCard data frames may be dropped with this field)."]
    #[inline(always)]
    #[must_use]
    pub fn drop_on_parity_error(&mut self) -> DROP_ON_PARITY_ERROR_W<8> {
        DROP_ON_PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 9 - Behavior when an error is detected in a start or stop period. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    #[must_use]
    pub fn drop_on_frame_error(&mut self) -> DROP_ON_FRAME_ERROR_W<9> {
        DROP_ON_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mp_mode(&mut self) -> MP_MODE_W<10> {
        MP_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minimum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
    #[inline(always)]
    #[must_use]
    pub fn lin_mode(&mut self) -> LIN_MODE_W<12> {
        LIN_MODE_W::new(self)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn skip_start(&mut self) -> SKIP_START_W<13> {
        SKIP_START_W::new(self)
    }
    #[doc = "Bits 16:19 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn break_width(&mut self) -> BREAK_WIDTH_W<16> {
        BREAK_WIDTH_W::new(self)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn break_level(&mut self) -> BREAK_LEVEL_W<24> {
        BREAK_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART receiver control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rx_ctrl](index.html) module"]
pub struct UART_RX_CTRL_SPEC;
impl crate::RegisterSpec for UART_RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rx_ctrl::R](R) reader structure"]
impl crate::Readable for UART_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_rx_ctrl::W](W) writer structure"]
impl crate::Writable for UART_RX_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_RX_CTRL to value 0x000a_0002"]
impl crate::Resettable for UART_RX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_0002;
}
