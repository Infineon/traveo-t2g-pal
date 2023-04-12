#[doc = "Register `TEST_CTL` reader"]
pub struct R(crate::R<TEST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CTL` writer"]
pub struct W(crate::W<TEST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CTL_SPEC>;
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
impl From<crate::W<TEST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_IDX` reader - Specifies the channel index of the channel to which test applies. The channel IO signals of channel indices CH_IDX and CH_NR-1 are connected as specified by MODE. CH_IDX should be in the range \\[0, CH_NR-2\\], as channel index CH_NR-1 is always involved in test and cannot be connected to itself. The test mode allows BOTH of the two connected channels to be tested. Note: this testing functionality simplifies SW development, but may also be used in the field to verify correct channel functionality."]
pub type CH_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_IDX` writer - Specifies the channel index of the channel to which test applies. The channel IO signals of channel indices CH_IDX and CH_NR-1 are connected as specified by MODE. CH_IDX should be in the range \\[0, CH_NR-2\\], as channel index CH_NR-1 is always involved in test and cannot be connected to itself. The test mode allows BOTH of the two connected channels to be tested. Note: this testing functionality simplifies SW development, but may also be used in the field to verify correct channel functionality."]
pub type CH_IDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEST_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `MODE` reader - Test mode: '0': Partial disconnect from IOSS. This mode's isolation allows for device test without relying on an external cxpi transceiver. The IOSS 'tx' IO cell can be used to observe messages outside of the device. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. '1': Full disconnect from IOSS (the IOSS/HSIOM should disconnect 'tx_out' from the 'tx' IO cell). This mode's isolation allows for device test without effecting an operational cxpi cluster. - tx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Test mode: '0': Partial disconnect from IOSS. This mode's isolation allows for device test without relying on an external cxpi transceiver. The IOSS 'tx' IO cell can be used to observe messages outside of the device. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. '1': Full disconnect from IOSS (the IOSS/HSIOM should disconnect 'tx_out' from the 'tx' IO cell). This mode's isolation allows for device test without effecting an operational cxpi cluster. - tx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Partial disconnect"]
    PARTIAL_DISCONNECT = 0,
    #[doc = "1: Full disconnect"]
    FULL_DISCONNECT = 1,
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
            false => MODE_A::PARTIAL_DISCONNECT,
            true => MODE_A::FULL_DISCONNECT,
        }
    }
    #[doc = "Checks if the value of the field is `PARTIAL_DISCONNECT`"]
    #[inline(always)]
    pub fn is_partial_disconnect(&self) -> bool {
        *self == MODE_A::PARTIAL_DISCONNECT
    }
    #[doc = "Checks if the value of the field is `FULL_DISCONNECT`"]
    #[inline(always)]
    pub fn is_full_disconnect(&self) -> bool {
        *self == MODE_A::FULL_DISCONNECT
    }
}
#[doc = "Field `MODE` writer - Test mode: '0': Partial disconnect from IOSS. This mode's isolation allows for device test without relying on an external cxpi transceiver. The IOSS 'tx' IO cell can be used to observe messages outside of the device. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. '1': Full disconnect from IOSS (the IOSS/HSIOM should disconnect 'tx_out' from the 'tx' IO cell). This mode's isolation allows for device test without effecting an operational cxpi cluster. - tx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Partial disconnect"]
    #[inline(always)]
    pub fn partial_disconnect(self) -> &'a mut W {
        self.variant(MODE_A::PARTIAL_DISCONNECT)
    }
    #[doc = "Full disconnect"]
    #[inline(always)]
    pub fn full_disconnect(self) -> &'a mut W {
        self.variant(MODE_A::FULL_DISCONNECT)
    }
}
#[doc = "Field `ENABLED` reader - Test enable: '0': Disabled. Functional mode. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_NR-1\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_rx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_rx_in\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_NR-1\\]. '1': Enabled. Test mode, specific test mode is specified by MODE."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "Test enable: '0': Disabled. Functional mode. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_NR-1\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_rx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_rx_in\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_NR-1\\]. '1': Enabled. Test mode, specific test mode is specified by MODE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: Functional mode"]
    FUNCTIONAL_MODE = 0,
    #[doc = "1: Test mode"]
    TEST_MODE = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::FUNCTIONAL_MODE,
            true => ENABLED_A::TEST_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL_MODE`"]
    #[inline(always)]
    pub fn is_functional_mode(&self) -> bool {
        *self == ENABLED_A::FUNCTIONAL_MODE
    }
    #[doc = "Checks if the value of the field is `TEST_MODE`"]
    #[inline(always)]
    pub fn is_test_mode(&self) -> bool {
        *self == ENABLED_A::TEST_MODE
    }
}
#[doc = "Field `ENABLED` writer - Test enable: '0': Disabled. Functional mode. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_NR-1\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_rx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_rx_in\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_NR-1\\]. '1': Enabled. Test mode, specific test mode is specified by MODE."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, ENABLED_A, O>;
impl<'a, const O: u8> ENABLED_W<'a, O> {
    #[doc = "Functional mode"]
    #[inline(always)]
    pub fn functional_mode(self) -> &'a mut W {
        self.variant(ENABLED_A::FUNCTIONAL_MODE)
    }
    #[doc = "Test mode"]
    #[inline(always)]
    pub fn test_mode(self) -> &'a mut W {
        self.variant(ENABLED_A::TEST_MODE)
    }
}
impl R {
    #[doc = "Bits 0:4 - Specifies the channel index of the channel to which test applies. The channel IO signals of channel indices CH_IDX and CH_NR-1 are connected as specified by MODE. CH_IDX should be in the range \\[0, CH_NR-2\\], as channel index CH_NR-1 is always involved in test and cannot be connected to itself. The test mode allows BOTH of the two connected channels to be tested. Note: this testing functionality simplifies SW development, but may also be used in the field to verify correct channel functionality."]
    #[inline(always)]
    pub fn ch_idx(&self) -> CH_IDX_R {
        CH_IDX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Test mode: '0': Partial disconnect from IOSS. This mode's isolation allows for device test without relying on an external cxpi transceiver. The IOSS 'tx' IO cell can be used to observe messages outside of the device. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. '1': Full disconnect from IOSS (the IOSS/HSIOM should disconnect 'tx_out' from the 'tx' IO cell). This mode's isolation allows for device test without effecting an operational cxpi cluster. - tx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Test enable: '0': Disabled. Functional mode. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_NR-1\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_rx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_rx_in\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_NR-1\\]. '1': Enabled. Test mode, specific test mode is specified by MODE."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Specifies the channel index of the channel to which test applies. The channel IO signals of channel indices CH_IDX and CH_NR-1 are connected as specified by MODE. CH_IDX should be in the range \\[0, CH_NR-2\\], as channel index CH_NR-1 is always involved in test and cannot be connected to itself. The test mode allows BOTH of the two connected channels to be tested. Note: this testing functionality simplifies SW development, but may also be used in the field to verify correct channel functionality."]
    #[inline(always)]
    #[must_use]
    pub fn ch_idx(&mut self) -> CH_IDX_W<0> {
        CH_IDX_W::new(self)
    }
    #[doc = "Bit 16 - Test mode: '0': Partial disconnect from IOSS. This mode's isolation allows for device test without relying on an external cxpi transceiver. The IOSS 'tx' IO cell can be used to observe messages outside of the device. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. '1': Full disconnect from IOSS (the IOSS/HSIOM should disconnect 'tx_out' from the 'tx' IO cell). This mode's isolation allows for device test without effecting an operational cxpi cluster. - tx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_IDX\\]
= cxpi_tx_out\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= cxpi_tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_IDX\\]
&amp; tx_out\\[CH_NR-1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<16> {
        MODE_W::new(self)
    }
    #[doc = "Bit 31 - Test enable: '0': Disabled. Functional mode. - tx_in\\[CH_IDX\\]
= IOSS cxpi_tx_in\\[CH_IDX\\]. - tx_in\\[CH_NR-1\\]
= IOSS cxpi_tx_in\\[CH_NR-1\\]. - rx_in\\[CH_IDX\\]
= IOSS cxpi_rx_in\\[CH_IDX\\]. - rx_in\\[CH_NR-1\\]
= IOSS cxpi_rx_in\\[CH_NR-1\\]. - cxpi_tx_out\\[CH_IDX\\]
= tx_out\\[CH_IDX\\]. - cxpi_tx_out\\[CH_NR-1\\]
= tx_out\\[CH_NR-1\\]. '1': Enabled. Test mode, specific test mode is specified by MODE."]
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
#[doc = "Test control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_ctl](index.html) module"]
pub struct TEST_CTL_SPEC;
impl crate::RegisterSpec for TEST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_ctl::R](R) reader structure"]
impl crate::Readable for TEST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_ctl::W](W) writer structure"]
impl crate::Writable for TEST_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST_CTL to value 0"]
impl crate::Resettable for TEST_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
