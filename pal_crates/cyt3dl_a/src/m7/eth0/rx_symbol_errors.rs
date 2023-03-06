#[doc = "Register `RX_SYMBOL_ERRORS` reader"]
pub struct R(crate::R<RX_SYMBOL_ERRORS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SYMBOL_ERRORS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SYMBOL_ERRORS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SYMBOL_ERRORS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_SYMBOL_ERR` reader - Receive symbol errors - a 10-bit register counting the number of frames that had rx_er asserted during reception. For 10/100 mode symbol errors are counted regardless of frame length checks. For gigabit mode the frame must satisfy slot time requirements in order to count a symbol error. Additionally, in gigabit half duplex mode, carrier extension errors are also recorded. Receive symbol errors will also be counted as an FCS or alignment error if the frame is between 64 and 1518 bytes (1536 bytes if bit 8 is set in the network configuration register, 10240 bytes if bit 3 is set in the network configuration register). If the frame is larger it will be recorded as a jabber error."]
pub type COUNT_SYMBOL_ERR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive symbol errors - a 10-bit register counting the number of frames that had rx_er asserted during reception. For 10/100 mode symbol errors are counted regardless of frame length checks. For gigabit mode the frame must satisfy slot time requirements in order to count a symbol error. Additionally, in gigabit half duplex mode, carrier extension errors are also recorded. Receive symbol errors will also be counted as an FCS or alignment error if the frame is between 64 and 1518 bytes (1536 bytes if bit 8 is set in the network configuration register, 10240 bytes if bit 3 is set in the network configuration register). If the frame is larger it will be recorded as a jabber error."]
    #[inline(always)]
    pub fn count_symbol_err(&self) -> COUNT_SYMBOL_ERR_R {
        COUNT_SYMBOL_ERR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receive Symbol Errors\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_symbol_errors](index.html) module"]
pub struct RX_SYMBOL_ERRORS_SPEC;
impl crate::RegisterSpec for RX_SYMBOL_ERRORS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_symbol_errors::R](R) reader structure"]
impl crate::Readable for RX_SYMBOL_ERRORS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_SYMBOL_ERRORS to value 0"]
impl crate::Resettable for RX_SYMBOL_ERRORS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
