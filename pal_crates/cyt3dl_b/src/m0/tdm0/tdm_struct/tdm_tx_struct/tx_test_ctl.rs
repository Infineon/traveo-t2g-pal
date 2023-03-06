#[doc = "Register `TX_TEST_CTL` reader"]
pub struct R(crate::R<TX_TEST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_TEST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_TEST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_TEST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_TEST_CTL` writer"]
pub struct W(crate::W<TX_TEST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_TEST_CTL_SPEC>;
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
impl From<crate::W<TX_TEST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_TEST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLED` reader - Test mode enable. '0': Disabled. Functional mode. - Transmitter tx_sck_in = IOSS tdm_tx_sck_in. - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in. - Receiver rx_sd_in = IOSS tdm_rx_sd_in. '1': Enabled. Test mode (intended to be used with (slave transmitter, master receiver) configuration). - Transmitter tx_sck_in = Receiver tdm_rx_sck_out. - Transmitter tx_fsync_in = Receiver tdm_rx_fsync_out. - Receiver rx_sd_in = Transmitter tdm_tx_sd_out. Note: TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1' simultaneously."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "Test mode enable. '0': Disabled. Functional mode. - Transmitter tx_sck_in = IOSS tdm_tx_sck_in. - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in. - Receiver rx_sd_in = IOSS tdm_rx_sd_in. '1': Enabled. Test mode (intended to be used with (slave transmitter, master receiver) configuration). - Transmitter tx_sck_in = Receiver tdm_rx_sck_out. - Transmitter tx_fsync_in = Receiver tdm_rx_fsync_out. - Receiver rx_sd_in = Transmitter tdm_tx_sd_out. Note: TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1' simultaneously.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: N/A"]
    FUNCTIONAL = 0,
    #[doc = "1: N/A"]
    TEST = 1,
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
            false => ENABLED_A::FUNCTIONAL,
            true => ENABLED_A::TEST,
        }
    }
    #[doc = "Checks if the value of the field is `FUNCTIONAL`"]
    #[inline(always)]
    pub fn is_functional(&self) -> bool {
        *self == ENABLED_A::FUNCTIONAL
    }
    #[doc = "Checks if the value of the field is `TEST`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == ENABLED_A::TEST
    }
}
#[doc = "Field `ENABLED` writer - Test mode enable. '0': Disabled. Functional mode. - Transmitter tx_sck_in = IOSS tdm_tx_sck_in. - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in. - Receiver rx_sd_in = IOSS tdm_rx_sd_in. '1': Enabled. Test mode (intended to be used with (slave transmitter, master receiver) configuration). - Transmitter tx_sck_in = Receiver tdm_rx_sck_out. - Transmitter tx_fsync_in = Receiver tdm_rx_fsync_out. - Receiver rx_sd_in = Transmitter tdm_tx_sd_out. Note: TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1' simultaneously."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_TEST_CTL_SPEC, ENABLED_A, O>;
impl<'a, const O: u8> ENABLED_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn functional(self) -> &'a mut W {
        self.variant(ENABLED_A::FUNCTIONAL)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn test(self) -> &'a mut W {
        self.variant(ENABLED_A::TEST)
    }
}
impl R {
    #[doc = "Bit 31 - Test mode enable. '0': Disabled. Functional mode. - Transmitter tx_sck_in = IOSS tdm_tx_sck_in. - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in. - Receiver rx_sd_in = IOSS tdm_rx_sd_in. '1': Enabled. Test mode (intended to be used with (slave transmitter, master receiver) configuration). - Transmitter tx_sck_in = Receiver tdm_rx_sck_out. - Transmitter tx_fsync_in = Receiver tdm_rx_fsync_out. - Receiver rx_sd_in = Transmitter tdm_tx_sd_out. Note: TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1' simultaneously."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Test mode enable. '0': Disabled. Functional mode. - Transmitter tx_sck_in = IOSS tdm_tx_sck_in. - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in. - Receiver rx_sd_in = IOSS tdm_rx_sd_in. '1': Enabled. Test mode (intended to be used with (slave transmitter, master receiver) configuration). - Transmitter tx_sck_in = Receiver tdm_rx_sck_out. - Transmitter tx_fsync_in = Receiver tdm_rx_fsync_out. - Receiver rx_sd_in = Transmitter tdm_tx_sd_out. Note: TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1' simultaneously."]
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
#[doc = "TX test control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_test_ctl](index.html) module"]
pub struct TX_TEST_CTL_SPEC;
impl crate::RegisterSpec for TX_TEST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_test_ctl::R](R) reader structure"]
impl crate::Readable for TX_TEST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_test_ctl::W](W) writer structure"]
impl crate::Writable for TX_TEST_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_TEST_CTL to value 0"]
impl crate::Resettable for TX_TEST_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
