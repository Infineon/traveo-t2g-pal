#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_BITS` reader - STOP bit periods: '0': 1/2 bit period. '1': 1 bit period. '2': 1 1/2 bit period. '3': 2 bit periods. In LIN mode, this field should be set to '1' (the default value) . In UART mode, this field can be programmed as desired. Note: receiver STOP bit frame errors can only be detected if the number of STOP bit periods is 1 or more bit period."]
pub type STOP_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_BITS` writer - STOP bit periods: '0': 1/2 bit period. '1': 1 bit period. '2': 1 1/2 bit period. '3': 2 bit periods. In LIN mode, this field should be set to '1' (the default value) . In UART mode, this field can be programmed as desired. Note: receiver STOP bit frame errors can only be detected if the number of STOP bit periods is 1 or more bit period."]
pub type STOP_BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `AUTO_EN` reader - LIN transceiver auto enable: '0': Disabled. '1': Enabled. The TX_RX_STATUS.EN_OUT field is controlled by HW."]
pub type AUTO_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_EN` writer - LIN transceiver auto enable: '0': Disabled. '1': Enabled. The TX_RX_STATUS.EN_OUT field is controlled by HW."]
pub type AUTO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `BREAK_DELIMITER_LENGTH` reader - In LIN mode, this field specifies the break delimiter length: (used in header transmission, not used in header reception). '0': 1 bit period. '1': 2 bit periods (default value). '2': 3 bit periods. '3': 4 bit periods. In UART mode, this field specifies the data field size: '0': 5 bit data field. '1': 6 bit data field. '2': 7 bit data field. '3': 8 bit data field. When the data field size is less than 8 bits, the most significant (unused) bits of the DATAx.DATAy\\[7:0\\]
fields should be set to '0' for the transmitter."]
pub type BREAK_DELIMITER_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREAK_DELIMITER_LENGTH` writer - In LIN mode, this field specifies the break delimiter length: (used in header transmission, not used in header reception). '0': 1 bit period. '1': 2 bit periods (default value). '2': 3 bit periods. '3': 4 bit periods. In UART mode, this field specifies the data field size: '0': 5 bit data field. '1': 6 bit data field. '2': 7 bit data field. '3': 8 bit data field. When the data field size is less than 8 bits, the most significant (unused) bits of the DATAx.DATAy\\[7:0\\]
fields should be set to '0' for the transmitter."]
pub type BREAK_DELIMITER_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `BREAK_WAKEUP_LENGTH` reader - Break/wakeup length (minus 1) in bit periods: '0': 1 bit period. ... '10': 11 bit periods (break length for slave nodes) ... '12': 13 bit periods (break length for master nodes) ... '30': 31 bit periods. '31': Illegal (should NOT be used!!!) This field is used for transmission/reception of BOTH break and wakeup signals. Note that these functions are mutually exclusive: - When CMD.TX_HEADER is '1', the field specifies the transmitted break field. - When CMD.TX_WAKEUP is '1', the field specifies the transmitted wakeup field. - When CMD.RX_HEADER is '1', the field specifies the to be received break field. - Otherwise, the field specifies the to be received wakeup field. Per the standard, the master wakeup duration is between 250 us and 5 ms. To support uncalibrated slaves, a slave has a detection threshold of 150 us (3 bit periods at 20 kbps). After transmission of a break or wakeup signal, the INTR.TX_BREAK_WAKEUP_DONE interrupt cause is activated. After reception of a wakeup signal, the INTR.RX_BREAK_WAKEUP_DONE interrupt cause is activated. To specify longer wakeup signals in terms of absolute time (us/ms rather than bit periods), the associated PERI clock divider value can be (temporarily) increased to make the LIN bit period longer. Note: entering bus sleep mode is achieved with the 'go-to-sleep' command."]
pub type BREAK_WAKEUP_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREAK_WAKEUP_LENGTH` writer - Break/wakeup length (minus 1) in bit periods: '0': 1 bit period. ... '10': 11 bit periods (break length for slave nodes) ... '12': 13 bit periods (break length for master nodes) ... '30': 31 bit periods. '31': Illegal (should NOT be used!!!) This field is used for transmission/reception of BOTH break and wakeup signals. Note that these functions are mutually exclusive: - When CMD.TX_HEADER is '1', the field specifies the transmitted break field. - When CMD.TX_WAKEUP is '1', the field specifies the transmitted wakeup field. - When CMD.RX_HEADER is '1', the field specifies the to be received break field. - Otherwise, the field specifies the to be received wakeup field. Per the standard, the master wakeup duration is between 250 us and 5 ms. To support uncalibrated slaves, a slave has a detection threshold of 150 us (3 bit periods at 20 kbps). After transmission of a break or wakeup signal, the INTR.TX_BREAK_WAKEUP_DONE interrupt cause is activated. After reception of a wakeup signal, the INTR.RX_BREAK_WAKEUP_DONE interrupt cause is activated. To specify longer wakeup signals in terms of absolute time (us/ms rather than bit periods), the associated PERI clock divider value can be (temporarily) increased to make the LIN bit period longer. Note: entering bus sleep mode is achieved with the 'go-to-sleep' command."]
pub type BREAK_WAKEUP_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `MODE` reader - Mode of operation: '0': LIN mode. '1': UART mode."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Mode of operation: '0': LIN mode. '1': UART mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: LIN mode."]
    LIN = 0,
    #[doc = "1: UART mode."]
    UART = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::LIN,
            true => MODE_A::UART,
        }
    }
    #[doc = "Checks if the value of the field is `LIN`"]
    #[inline(always)]
    pub fn is_lin(&self) -> bool {
        *self == MODE_A::LIN
    }
    #[doc = "Checks if the value of the field is `UART`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == MODE_A::UART
    }
}
#[doc = "Field `MODE` writer - Mode of operation: '0': LIN mode. '1': UART mode."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "LIN mode."]
    #[inline(always)]
    pub fn lin(self) -> &'a mut W {
        self.variant(MODE_A::LIN)
    }
    #[doc = "UART mode."]
    #[inline(always)]
    pub fn uart(self) -> &'a mut W {
        self.variant(MODE_A::UART)
    }
}
#[doc = "Field `BIT_ERROR_IGNORE` reader - Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: this field does NOT effect the reporting of the bit error through INTR/STATUS.TX_HEADER/RESPONSE_BIT_ERROR; i.e. bit errors are always reported."]
pub type BIT_ERROR_IGNORE_R = crate::BitReader<bool>;
#[doc = "Field `BIT_ERROR_IGNORE` writer - Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: this field does NOT effect the reporting of the bit error through INTR/STATUS.TX_HEADER/RESPONSE_BIT_ERROR; i.e. bit errors are always reported."]
pub type BIT_ERROR_IGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `PARITY` reader - Parity mode: '0': Even parity: even number of '1' bits (including parity). '1': Odd parity. Note: Used in UART mode only."]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - Parity mode: '0': Even parity: even number of '1' bits (including parity). '1': Odd parity. Note: Used in UART mode only."]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `PARITY_EN` reader - Parity generation enable: '0': Disabled. No parity bit is transferred. '1': Enabled. The parity bit is transferred after the last (most significant) data field bit. Note: Used in UART mode only."]
pub type PARITY_EN_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_EN` writer - Parity generation enable: '0': Disabled. No parity bit is transferred. '1': Enabled. The parity bit is transferred after the last (most significant) data field bit. Note: Used in UART mode only."]
pub type PARITY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `FILTER_EN` reader - RX filter (for 'lin_rx_in'): '0': No filter. '1': Median 3 (default value) operates on the last three 'lin_rx_in' values. The sequences '000', '001', '010' and '100' result in a filtered value '0'. The sequences '111', '110', '101' and '011' result in a filtered value '1'."]
pub type FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_EN` writer - RX filter (for 'lin_rx_in'): '0': No filter. '1': Median 3 (default value) operates on the last three 'lin_rx_in' values. The sequences '000', '001', '010' and '100' result in a filtered value '0'. The sequences '111', '110', '101' and '011' result in a filtered value '1'."]
pub type FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Channel enable: '0': Disabled. If a channel is disabled, all non-retained MMIO registers (e.g. the CMD, STATUS, and INTR registers) have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Channel enable: '0': Disabled. If a channel is disabled, all non-retained MMIO registers (e.g. the CMD, STATUS, and INTR registers) have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - STOP bit periods: '0': 1/2 bit period. '1': 1 bit period. '2': 1 1/2 bit period. '3': 2 bit periods. In LIN mode, this field should be set to '1' (the default value) . In UART mode, this field can be programmed as desired. Note: receiver STOP bit frame errors can only be detected if the number of STOP bit periods is 1 or more bit period."]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - LIN transceiver auto enable: '0': Disabled. '1': Enabled. The TX_RX_STATUS.EN_OUT field is controlled by HW."]
    #[inline(always)]
    pub fn auto_en(&self) -> AUTO_EN_R {
        AUTO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - In LIN mode, this field specifies the break delimiter length: (used in header transmission, not used in header reception). '0': 1 bit period. '1': 2 bit periods (default value). '2': 3 bit periods. '3': 4 bit periods. In UART mode, this field specifies the data field size: '0': 5 bit data field. '1': 6 bit data field. '2': 7 bit data field. '3': 8 bit data field. When the data field size is less than 8 bits, the most significant (unused) bits of the DATAx.DATAy\\[7:0\\]
fields should be set to '0' for the transmitter."]
    #[inline(always)]
    pub fn break_delimiter_length(&self) -> BREAK_DELIMITER_LENGTH_R {
        BREAK_DELIMITER_LENGTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:20 - Break/wakeup length (minus 1) in bit periods: '0': 1 bit period. ... '10': 11 bit periods (break length for slave nodes) ... '12': 13 bit periods (break length for master nodes) ... '30': 31 bit periods. '31': Illegal (should NOT be used!!!) This field is used for transmission/reception of BOTH break and wakeup signals. Note that these functions are mutually exclusive: - When CMD.TX_HEADER is '1', the field specifies the transmitted break field. - When CMD.TX_WAKEUP is '1', the field specifies the transmitted wakeup field. - When CMD.RX_HEADER is '1', the field specifies the to be received break field. - Otherwise, the field specifies the to be received wakeup field. Per the standard, the master wakeup duration is between 250 us and 5 ms. To support uncalibrated slaves, a slave has a detection threshold of 150 us (3 bit periods at 20 kbps). After transmission of a break or wakeup signal, the INTR.TX_BREAK_WAKEUP_DONE interrupt cause is activated. After reception of a wakeup signal, the INTR.RX_BREAK_WAKEUP_DONE interrupt cause is activated. To specify longer wakeup signals in terms of absolute time (us/ms rather than bit periods), the associated PERI clock divider value can be (temporarily) increased to make the LIN bit period longer. Note: entering bus sleep mode is achieved with the 'go-to-sleep' command."]
    #[inline(always)]
    pub fn break_wakeup_length(&self) -> BREAK_WAKEUP_LENGTH_R {
        BREAK_WAKEUP_LENGTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Mode of operation: '0': LIN mode. '1': UART mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: this field does NOT effect the reporting of the bit error through INTR/STATUS.TX_HEADER/RESPONSE_BIT_ERROR; i.e. bit errors are always reported."]
    #[inline(always)]
    pub fn bit_error_ignore(&self) -> BIT_ERROR_IGNORE_R {
        BIT_ERROR_IGNORE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Parity mode: '0': Even parity: even number of '1' bits (including parity). '1': Odd parity. Note: Used in UART mode only."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Parity generation enable: '0': Disabled. No parity bit is transferred. '1': Enabled. The parity bit is transferred after the last (most significant) data field bit. Note: Used in UART mode only."]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RX filter (for 'lin_rx_in'): '0': No filter. '1': Median 3 (default value) operates on the last three 'lin_rx_in' values. The sequences '000', '001', '010' and '100' result in a filtered value '0'. The sequences '111', '110', '101' and '011' result in a filtered value '1'."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. If a channel is disabled, all non-retained MMIO registers (e.g. the CMD, STATUS, and INTR registers) have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - STOP bit periods: '0': 1/2 bit period. '1': 1 bit period. '2': 1 1/2 bit period. '3': 2 bit periods. In LIN mode, this field should be set to '1' (the default value) . In UART mode, this field can be programmed as desired. Note: receiver STOP bit frame errors can only be detected if the number of STOP bit periods is 1 or more bit period."]
    #[inline(always)]
    #[must_use]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<0> {
        STOP_BITS_W::new(self)
    }
    #[doc = "Bit 4 - LIN transceiver auto enable: '0': Disabled. '1': Enabled. The TX_RX_STATUS.EN_OUT field is controlled by HW."]
    #[inline(always)]
    #[must_use]
    pub fn auto_en(&mut self) -> AUTO_EN_W<4> {
        AUTO_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - In LIN mode, this field specifies the break delimiter length: (used in header transmission, not used in header reception). '0': 1 bit period. '1': 2 bit periods (default value). '2': 3 bit periods. '3': 4 bit periods. In UART mode, this field specifies the data field size: '0': 5 bit data field. '1': 6 bit data field. '2': 7 bit data field. '3': 8 bit data field. When the data field size is less than 8 bits, the most significant (unused) bits of the DATAx.DATAy\\[7:0\\]
fields should be set to '0' for the transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn break_delimiter_length(&mut self) -> BREAK_DELIMITER_LENGTH_W<8> {
        BREAK_DELIMITER_LENGTH_W::new(self)
    }
    #[doc = "Bits 16:20 - Break/wakeup length (minus 1) in bit periods: '0': 1 bit period. ... '10': 11 bit periods (break length for slave nodes) ... '12': 13 bit periods (break length for master nodes) ... '30': 31 bit periods. '31': Illegal (should NOT be used!!!) This field is used for transmission/reception of BOTH break and wakeup signals. Note that these functions are mutually exclusive: - When CMD.TX_HEADER is '1', the field specifies the transmitted break field. - When CMD.TX_WAKEUP is '1', the field specifies the transmitted wakeup field. - When CMD.RX_HEADER is '1', the field specifies the to be received break field. - Otherwise, the field specifies the to be received wakeup field. Per the standard, the master wakeup duration is between 250 us and 5 ms. To support uncalibrated slaves, a slave has a detection threshold of 150 us (3 bit periods at 20 kbps). After transmission of a break or wakeup signal, the INTR.TX_BREAK_WAKEUP_DONE interrupt cause is activated. After reception of a wakeup signal, the INTR.RX_BREAK_WAKEUP_DONE interrupt cause is activated. To specify longer wakeup signals in terms of absolute time (us/ms rather than bit periods), the associated PERI clock divider value can be (temporarily) increased to make the LIN bit period longer. Note: entering bus sleep mode is achieved with the 'go-to-sleep' command."]
    #[inline(always)]
    #[must_use]
    pub fn break_wakeup_length(&mut self) -> BREAK_WAKEUP_LENGTH_W<16> {
        BREAK_WAKEUP_LENGTH_W::new(self)
    }
    #[doc = "Bit 24 - Mode of operation: '0': LIN mode. '1': UART mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<24> {
        MODE_W::new(self)
    }
    #[doc = "Bit 27 - Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: this field does NOT effect the reporting of the bit error through INTR/STATUS.TX_HEADER/RESPONSE_BIT_ERROR; i.e. bit errors are always reported."]
    #[inline(always)]
    #[must_use]
    pub fn bit_error_ignore(&mut self) -> BIT_ERROR_IGNORE_W<27> {
        BIT_ERROR_IGNORE_W::new(self)
    }
    #[doc = "Bit 28 - Parity mode: '0': Even parity: even number of '1' bits (including parity). '1': Odd parity. Note: Used in UART mode only."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<28> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 29 - Parity generation enable: '0': Disabled. No parity bit is transferred. '1': Enabled. The parity bit is transferred after the last (most significant) data field bit. Note: Used in UART mode only."]
    #[inline(always)]
    #[must_use]
    pub fn parity_en(&mut self) -> PARITY_EN_W<29> {
        PARITY_EN_W::new(self)
    }
    #[doc = "Bit 30 - RX filter (for 'lin_rx_in'): '0': No filter. '1': Median 3 (default value) operates on the last three 'lin_rx_in' values. The sequences '000', '001', '010' and '100' result in a filtered value '0'. The sequences '111', '110', '101' and '011' result in a filtered value '1'."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<30> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. If a channel is disabled, all non-retained MMIO registers (e.g. the CMD, STATUS, and INTR registers) have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x400c_0101"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x400c_0101;
}
