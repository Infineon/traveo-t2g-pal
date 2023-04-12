#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_HEADER` reader - SW sets this field to '1' to transmit a header. HW sets this field to '0' on successful completion of ANY of the following legal command sequences (also set to '0' when an error is detected): - TX_HEADER - TX_HEADER, TX_RESPONSE. - TX_HEADER, RX_RESPONSE. - RX_HEADER, TX_RESPONSE. - RX_HEADER, RX_RESPONSE. - TX_WAKEUP. The header is transmitted when the PID field STOP bits are transmitted (INTR.TX_HEADER_DONE). HW sets this field to '1', when the 'tr_cmd_tx_header' input trigger is activated. This allows for time triggered LIN message transfer. HW driven time triggered transfer eliminates the jitter that is typically associated with SW driven transfer. In UART mode, a single data field (DATA0.DATA1) is transmitted."]
pub type TX_HEADER_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER` writer - SW sets this field to '1' to transmit a header. HW sets this field to '0' on successful completion of ANY of the following legal command sequences (also set to '0' when an error is detected): - TX_HEADER - TX_HEADER, TX_RESPONSE. - TX_HEADER, RX_RESPONSE. - RX_HEADER, TX_RESPONSE. - RX_HEADER, RX_RESPONSE. - TX_WAKEUP. The header is transmitted when the PID field STOP bits are transmitted (INTR.TX_HEADER_DONE). HW sets this field to '1', when the 'tr_cmd_tx_header' input trigger is activated. This allows for time triggered LIN message transfer. HW driven time triggered transfer eliminates the jitter that is typically associated with SW driven transfer. In UART mode, a single data field (DATA0.DATA1) is transmitted."]
pub type TX_HEADER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TX_RESPONSE` reader - SW sets this field to '1' to transmit a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). The response is transmitted when the checksum field STOP bits are transmitted (INTR.TX_RESPONSE_DONE)."]
pub type TX_RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE` writer - SW sets this field to '1' to transmit a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). The response is transmitted when the checksum field STOP bits are transmitted (INTR.TX_RESPONSE_DONE)."]
pub type TX_RESPONSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TX_WAKEUP` reader - SW sets this field to '1' to transmit a wakeup signal. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). The command generates CTL.BREAK_WAKEUP_LENGTH bit periods in the dominant state (low/'0') and transitions to the recessive state (high/'1') (INTR.TX_WAKEUP_DONE)."]
pub type TX_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `TX_WAKEUP` writer - SW sets this field to '1' to transmit a wakeup signal. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). The command generates CTL.BREAK_WAKEUP_LENGTH bit periods in the dominant state (low/'0') and transitions to the recessive state (high/'1') (INTR.TX_WAKEUP_DONE)."]
pub type TX_WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RX_HEADER` reader - SW sets this field to '1' to receive a header. HW sets this field to '0' on successful completion of the ANY of the legal command sequences (NOT set to '0' when an error is detected in LIN mode). The header is received when the PID field STOP bits are received (INTR.RX_HEADER_DONE). Typically, a slave node SW sets both RX_HEADER and RX_RESPONSE to '1', anticipating a transfer of a response from the master node to this slave node. After receipt of the header PID field (INTR.RX_HEADER_PID_DONE is activated), the slave node may decide to set TX_RESPONSE to '1' (which has a higher priority than RX_RESPONSE) to transmit a response. the Break detection is performed regardless of CMD.RX_HEADER. INTR.RX_BREAK_WAKEUP_DONE will trigger at LIN_RX rising edge, when the low pulse meet CTL0.BREAK_WAKEUP_LENGTH. when Break is detected, HW check CMD.RX_HEADER before entering SYNC byte processing state. when RX_HEADER is cleared, SW has at least 11 bit times to set RX_HEADER again, before next Break is detected (RX_BREAK_WAKEUP_DONE). in this case, there is no gap, Break will never be missed. In UART mode, a single data field in received (in DATA0.DATA1). HW set this field to '0' when the data field is received, or when an error is detected."]
pub type RX_HEADER_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER` writer - SW sets this field to '1' to receive a header. HW sets this field to '0' on successful completion of the ANY of the legal command sequences (NOT set to '0' when an error is detected in LIN mode). The header is received when the PID field STOP bits are received (INTR.RX_HEADER_DONE). Typically, a slave node SW sets both RX_HEADER and RX_RESPONSE to '1', anticipating a transfer of a response from the master node to this slave node. After receipt of the header PID field (INTR.RX_HEADER_PID_DONE is activated), the slave node may decide to set TX_RESPONSE to '1' (which has a higher priority than RX_RESPONSE) to transmit a response. the Break detection is performed regardless of CMD.RX_HEADER. INTR.RX_BREAK_WAKEUP_DONE will trigger at LIN_RX rising edge, when the low pulse meet CTL0.BREAK_WAKEUP_LENGTH. when Break is detected, HW check CMD.RX_HEADER before entering SYNC byte processing state. when RX_HEADER is cleared, SW has at least 11 bit times to set RX_HEADER again, before next Break is detected (RX_BREAK_WAKEUP_DONE). in this case, there is no gap, Break will never be missed. In UART mode, a single data field in received (in DATA0.DATA1). HW set this field to '0' when the data field is received, or when an error is detected."]
pub type RX_HEADER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE` reader - SW sets this field to '1' to receive a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (NOT set to '0' when an error is detected). The response is received when the checksum field STOP bits are received (INTR.RX_RESPONSE_DONE)."]
pub type RX_RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE` writer - SW sets this field to '1' to receive a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (NOT set to '0' when an error is detected). The response is received when the checksum field STOP bits are received (INTR.RX_RESPONSE_DONE)."]
pub type RX_RESPONSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SW sets this field to '1' to transmit a header. HW sets this field to '0' on successful completion of ANY of the following legal command sequences (also set to '0' when an error is detected): - TX_HEADER - TX_HEADER, TX_RESPONSE. - TX_HEADER, RX_RESPONSE. - RX_HEADER, TX_RESPONSE. - RX_HEADER, RX_RESPONSE. - TX_WAKEUP. The header is transmitted when the PID field STOP bits are transmitted (INTR.TX_HEADER_DONE). HW sets this field to '1', when the 'tr_cmd_tx_header' input trigger is activated. This allows for time triggered LIN message transfer. HW driven time triggered transfer eliminates the jitter that is typically associated with SW driven transfer. In UART mode, a single data field (DATA0.DATA1) is transmitted."]
    #[inline(always)]
    pub fn tx_header(&self) -> TX_HEADER_R {
        TX_HEADER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SW sets this field to '1' to transmit a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). The response is transmitted when the checksum field STOP bits are transmitted (INTR.TX_RESPONSE_DONE)."]
    #[inline(always)]
    pub fn tx_response(&self) -> TX_RESPONSE_R {
        TX_RESPONSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SW sets this field to '1' to transmit a wakeup signal. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). The command generates CTL.BREAK_WAKEUP_LENGTH bit periods in the dominant state (low/'0') and transitions to the recessive state (high/'1') (INTR.TX_WAKEUP_DONE)."]
    #[inline(always)]
    pub fn tx_wakeup(&self) -> TX_WAKEUP_R {
        TX_WAKEUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SW sets this field to '1' to receive a header. HW sets this field to '0' on successful completion of the ANY of the legal command sequences (NOT set to '0' when an error is detected in LIN mode). The header is received when the PID field STOP bits are received (INTR.RX_HEADER_DONE). Typically, a slave node SW sets both RX_HEADER and RX_RESPONSE to '1', anticipating a transfer of a response from the master node to this slave node. After receipt of the header PID field (INTR.RX_HEADER_PID_DONE is activated), the slave node may decide to set TX_RESPONSE to '1' (which has a higher priority than RX_RESPONSE) to transmit a response. the Break detection is performed regardless of CMD.RX_HEADER. INTR.RX_BREAK_WAKEUP_DONE will trigger at LIN_RX rising edge, when the low pulse meet CTL0.BREAK_WAKEUP_LENGTH. when Break is detected, HW check CMD.RX_HEADER before entering SYNC byte processing state. when RX_HEADER is cleared, SW has at least 11 bit times to set RX_HEADER again, before next Break is detected (RX_BREAK_WAKEUP_DONE). in this case, there is no gap, Break will never be missed. In UART mode, a single data field in received (in DATA0.DATA1). HW set this field to '0' when the data field is received, or when an error is detected."]
    #[inline(always)]
    pub fn rx_header(&self) -> RX_HEADER_R {
        RX_HEADER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SW sets this field to '1' to receive a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (NOT set to '0' when an error is detected). The response is received when the checksum field STOP bits are received (INTR.RX_RESPONSE_DONE)."]
    #[inline(always)]
    pub fn rx_response(&self) -> RX_RESPONSE_R {
        RX_RESPONSE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW sets this field to '1' to transmit a header. HW sets this field to '0' on successful completion of ANY of the following legal command sequences (also set to '0' when an error is detected): - TX_HEADER - TX_HEADER, TX_RESPONSE. - TX_HEADER, RX_RESPONSE. - RX_HEADER, TX_RESPONSE. - RX_HEADER, RX_RESPONSE. - TX_WAKEUP. The header is transmitted when the PID field STOP bits are transmitted (INTR.TX_HEADER_DONE). HW sets this field to '1', when the 'tr_cmd_tx_header' input trigger is activated. This allows for time triggered LIN message transfer. HW driven time triggered transfer eliminates the jitter that is typically associated with SW driven transfer. In UART mode, a single data field (DATA0.DATA1) is transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header(&mut self) -> TX_HEADER_W<0> {
        TX_HEADER_W::new(self)
    }
    #[doc = "Bit 1 - SW sets this field to '1' to transmit a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). The response is transmitted when the checksum field STOP bits are transmitted (INTR.TX_RESPONSE_DONE)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_response(&mut self) -> TX_RESPONSE_W<1> {
        TX_RESPONSE_W::new(self)
    }
    #[doc = "Bit 2 - SW sets this field to '1' to transmit a wakeup signal. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). The command generates CTL.BREAK_WAKEUP_LENGTH bit periods in the dominant state (low/'0') and transitions to the recessive state (high/'1') (INTR.TX_WAKEUP_DONE)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wakeup(&mut self) -> TX_WAKEUP_W<2> {
        TX_WAKEUP_W::new(self)
    }
    #[doc = "Bit 8 - SW sets this field to '1' to receive a header. HW sets this field to '0' on successful completion of the ANY of the legal command sequences (NOT set to '0' when an error is detected in LIN mode). The header is received when the PID field STOP bits are received (INTR.RX_HEADER_DONE). Typically, a slave node SW sets both RX_HEADER and RX_RESPONSE to '1', anticipating a transfer of a response from the master node to this slave node. After receipt of the header PID field (INTR.RX_HEADER_PID_DONE is activated), the slave node may decide to set TX_RESPONSE to '1' (which has a higher priority than RX_RESPONSE) to transmit a response. the Break detection is performed regardless of CMD.RX_HEADER. INTR.RX_BREAK_WAKEUP_DONE will trigger at LIN_RX rising edge, when the low pulse meet CTL0.BREAK_WAKEUP_LENGTH. when Break is detected, HW check CMD.RX_HEADER before entering SYNC byte processing state. when RX_HEADER is cleared, SW has at least 11 bit times to set RX_HEADER again, before next Break is detected (RX_BREAK_WAKEUP_DONE). in this case, there is no gap, Break will never be missed. In UART mode, a single data field in received (in DATA0.DATA1). HW set this field to '0' when the data field is received, or when an error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header(&mut self) -> RX_HEADER_W<8> {
        RX_HEADER_W::new(self)
    }
    #[doc = "Bit 9 - SW sets this field to '1' to receive a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (NOT set to '0' when an error is detected). The response is received when the checksum field STOP bits are received (INTR.RX_RESPONSE_DONE)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response(&mut self) -> RX_RESPONSE_W<9> {
        RX_RESPONSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
