#[doc = "Register `RX_ROUTE_CTL` reader"]
pub struct R(crate::R<RX_ROUTE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ROUTE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ROUTE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ROUTE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_ROUTE_CTL` writer"]
pub struct W(crate::W<RX_ROUTE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_ROUTE_CTL_SPEC>;
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
impl From<crate::W<RX_ROUTE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_ROUTE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Controls routing to the RX slave signalling inputs (FSYNC/SCK): '0': RX slave signaling indipendent from TX signaling: - Receiver rx_sck_in = IOSS tdm_rx_sck_in - Receiver rx_fsync_in = IOSS tdm_rx_fsync_in '1': RX slave signalling inputs driven by TX Slave: - Receiver rx_sck_in = IOSS tdm_tx_sck_in - Receiver rx_fsync_in = IOSS tdm_tx_fsync_in '2': RX slave signalling inputs driven by TX Master: - Receiver rx_sck_in = transmitter tdm_tx_sck_out - Receiver rx_fsync_in = transmitter tdm_tx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the RX slave to share the same signaling used by the TX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Controls routing to the RX slave signalling inputs (FSYNC/SCK): '0': RX slave signaling indipendent from TX signaling: - Receiver rx_sck_in = IOSS tdm_rx_sck_in - Receiver rx_fsync_in = IOSS tdm_rx_fsync_in '1': RX slave signalling inputs driven by TX Slave: - Receiver rx_sck_in = IOSS tdm_tx_sck_in - Receiver rx_fsync_in = IOSS tdm_tx_fsync_in '2': RX slave signalling inputs driven by TX Master: - Receiver rx_sck_in = transmitter tdm_tx_sck_out - Receiver rx_fsync_in = transmitter tdm_tx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the RX slave to share the same signaling used by the TX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: N/A"]
    RX_IN_DRIVEN_BY_IOSS_RX_IN = 0,
    #[doc = "1: N/A"]
    RX_IN_DRIVEN_BY_IOSS_TX_IN = 1,
    #[doc = "2: N/A"]
    RX_IN_DRIVEN_BY_TX_OUT = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::RX_IN_DRIVEN_BY_IOSS_RX_IN),
            1 => Some(MODE_A::RX_IN_DRIVEN_BY_IOSS_TX_IN),
            2 => Some(MODE_A::RX_IN_DRIVEN_BY_TX_OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RX_IN_DRIVEN_BY_IOSS_RX_IN`"]
    #[inline(always)]
    pub fn is_rx_in_driven_by_ioss_rx_in(&self) -> bool {
        *self == MODE_A::RX_IN_DRIVEN_BY_IOSS_RX_IN
    }
    #[doc = "Checks if the value of the field is `RX_IN_DRIVEN_BY_IOSS_TX_IN`"]
    #[inline(always)]
    pub fn is_rx_in_driven_by_ioss_tx_in(&self) -> bool {
        *self == MODE_A::RX_IN_DRIVEN_BY_IOSS_TX_IN
    }
    #[doc = "Checks if the value of the field is `RX_IN_DRIVEN_BY_TX_OUT`"]
    #[inline(always)]
    pub fn is_rx_in_driven_by_tx_out(&self) -> bool {
        *self == MODE_A::RX_IN_DRIVEN_BY_TX_OUT
    }
}
#[doc = "Field `MODE` writer - Controls routing to the RX slave signalling inputs (FSYNC/SCK): '0': RX slave signaling indipendent from TX signaling: - Receiver rx_sck_in = IOSS tdm_rx_sck_in - Receiver rx_fsync_in = IOSS tdm_rx_fsync_in '1': RX slave signalling inputs driven by TX Slave: - Receiver rx_sck_in = IOSS tdm_tx_sck_in - Receiver rx_fsync_in = IOSS tdm_tx_fsync_in '2': RX slave signalling inputs driven by TX Master: - Receiver rx_sck_in = transmitter tdm_tx_sck_out - Receiver rx_fsync_in = transmitter tdm_tx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the RX slave to share the same signaling used by the TX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_ROUTE_CTL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rx_in_driven_by_ioss_rx_in(self) -> &'a mut W {
        self.variant(MODE_A::RX_IN_DRIVEN_BY_IOSS_RX_IN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rx_in_driven_by_ioss_tx_in(self) -> &'a mut W {
        self.variant(MODE_A::RX_IN_DRIVEN_BY_IOSS_TX_IN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rx_in_driven_by_tx_out(self) -> &'a mut W {
        self.variant(MODE_A::RX_IN_DRIVEN_BY_TX_OUT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Controls routing to the RX slave signalling inputs (FSYNC/SCK): '0': RX slave signaling indipendent from TX signaling: - Receiver rx_sck_in = IOSS tdm_rx_sck_in - Receiver rx_fsync_in = IOSS tdm_rx_fsync_in '1': RX slave signalling inputs driven by TX Slave: - Receiver rx_sck_in = IOSS tdm_tx_sck_in - Receiver rx_fsync_in = IOSS tdm_tx_fsync_in '2': RX slave signalling inputs driven by TX Master: - Receiver rx_sck_in = transmitter tdm_tx_sck_out - Receiver rx_fsync_in = transmitter tdm_tx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the RX slave to share the same signaling used by the TX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls routing to the RX slave signalling inputs (FSYNC/SCK): '0': RX slave signaling indipendent from TX signaling: - Receiver rx_sck_in = IOSS tdm_rx_sck_in - Receiver rx_fsync_in = IOSS tdm_rx_fsync_in '1': RX slave signalling inputs driven by TX Slave: - Receiver rx_sck_in = IOSS tdm_tx_sck_in - Receiver rx_fsync_in = IOSS tdm_tx_fsync_in '2': RX slave signalling inputs driven by TX Master: - Receiver rx_sck_in = transmitter tdm_tx_sck_out - Receiver rx_fsync_in = transmitter tdm_tx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the RX slave to share the same signaling used by the TX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX route control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_route_ctl](index.html) module"]
pub struct RX_ROUTE_CTL_SPEC;
impl crate::RegisterSpec for RX_ROUTE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_route_ctl::R](R) reader structure"]
impl crate::Readable for RX_ROUTE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_route_ctl::W](W) writer structure"]
impl crate::Writable for RX_ROUTE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_ROUTE_CTL to value 0"]
impl crate::Resettable for RX_ROUTE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
