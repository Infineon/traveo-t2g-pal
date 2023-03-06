#[doc = "Register `TX_ROUTE_CTL` reader"]
pub struct R(crate::R<TX_ROUTE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_ROUTE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_ROUTE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_ROUTE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_ROUTE_CTL` writer"]
pub struct W(crate::W<TX_ROUTE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_ROUTE_CTL_SPEC>;
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
impl From<crate::W<TX_ROUTE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_ROUTE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Controls routing to the TX slave signalling inputs (FSYNC/SCK): '0': TX slave signaling indipendent from RX signaling: - Transmitter tx_sck_in = IOSS tdm_tx_sck_in - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in '1': TX slave signalling inputs driven by RX Slave: - Transmitter tx_sck_in = IOSS tdm_rx_sck_in - Transmitter tx_fsync_in = IOSS tdm_rx_fsync_in '2': TX slave signalling inputs driven by RX Master: - Transmitter tx_sck_in = receiver tdm_rx_sck_out - Transmitter tx_fsync_in = receiver tdm_rx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the TX slave to share the same signaling used by the RX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Controls routing to the TX slave signalling inputs (FSYNC/SCK): '0': TX slave signaling indipendent from RX signaling: - Transmitter tx_sck_in = IOSS tdm_tx_sck_in - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in '1': TX slave signalling inputs driven by RX Slave: - Transmitter tx_sck_in = IOSS tdm_rx_sck_in - Transmitter tx_fsync_in = IOSS tdm_rx_fsync_in '2': TX slave signalling inputs driven by RX Master: - Transmitter tx_sck_in = receiver tdm_rx_sck_out - Transmitter tx_fsync_in = receiver tdm_rx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the TX slave to share the same signaling used by the RX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: N/A"]
    TX_IN_DRIVEN_BY_IOSS_TX_IN = 0,
    #[doc = "1: N/A"]
    TX_IN_DRIVEN_BY_IOSS_RX_IN = 1,
    #[doc = "2: N/A"]
    TX_IN_DRIVEN_BY_RX_OUT = 2,
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
            0 => Some(MODE_A::TX_IN_DRIVEN_BY_IOSS_TX_IN),
            1 => Some(MODE_A::TX_IN_DRIVEN_BY_IOSS_RX_IN),
            2 => Some(MODE_A::TX_IN_DRIVEN_BY_RX_OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TX_IN_DRIVEN_BY_IOSS_TX_IN`"]
    #[inline(always)]
    pub fn is_tx_in_driven_by_ioss_tx_in(&self) -> bool {
        *self == MODE_A::TX_IN_DRIVEN_BY_IOSS_TX_IN
    }
    #[doc = "Checks if the value of the field is `TX_IN_DRIVEN_BY_IOSS_RX_IN`"]
    #[inline(always)]
    pub fn is_tx_in_driven_by_ioss_rx_in(&self) -> bool {
        *self == MODE_A::TX_IN_DRIVEN_BY_IOSS_RX_IN
    }
    #[doc = "Checks if the value of the field is `TX_IN_DRIVEN_BY_RX_OUT`"]
    #[inline(always)]
    pub fn is_tx_in_driven_by_rx_out(&self) -> bool {
        *self == MODE_A::TX_IN_DRIVEN_BY_RX_OUT
    }
}
#[doc = "Field `MODE` writer - Controls routing to the TX slave signalling inputs (FSYNC/SCK): '0': TX slave signaling indipendent from RX signaling: - Transmitter tx_sck_in = IOSS tdm_tx_sck_in - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in '1': TX slave signalling inputs driven by RX Slave: - Transmitter tx_sck_in = IOSS tdm_rx_sck_in - Transmitter tx_fsync_in = IOSS tdm_rx_fsync_in '2': TX slave signalling inputs driven by RX Master: - Transmitter tx_sck_in = receiver tdm_rx_sck_out - Transmitter tx_fsync_in = receiver tdm_rx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the TX slave to share the same signaling used by the RX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_ROUTE_CTL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tx_in_driven_by_ioss_tx_in(self) -> &'a mut W {
        self.variant(MODE_A::TX_IN_DRIVEN_BY_IOSS_TX_IN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tx_in_driven_by_ioss_rx_in(self) -> &'a mut W {
        self.variant(MODE_A::TX_IN_DRIVEN_BY_IOSS_RX_IN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tx_in_driven_by_rx_out(self) -> &'a mut W {
        self.variant(MODE_A::TX_IN_DRIVEN_BY_RX_OUT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Controls routing to the TX slave signalling inputs (FSYNC/SCK): '0': TX slave signaling indipendent from RX signaling: - Transmitter tx_sck_in = IOSS tdm_tx_sck_in - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in '1': TX slave signalling inputs driven by RX Slave: - Transmitter tx_sck_in = IOSS tdm_rx_sck_in - Transmitter tx_fsync_in = IOSS tdm_rx_fsync_in '2': TX slave signalling inputs driven by RX Master: - Transmitter tx_sck_in = receiver tdm_rx_sck_out - Transmitter tx_fsync_in = receiver tdm_rx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the TX slave to share the same signaling used by the RX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls routing to the TX slave signalling inputs (FSYNC/SCK): '0': TX slave signaling indipendent from RX signaling: - Transmitter tx_sck_in = IOSS tdm_tx_sck_in - Transmitter tx_fsync_in = IOSS tdm_tx_fsync_in '1': TX slave signalling inputs driven by RX Slave: - Transmitter tx_sck_in = IOSS tdm_rx_sck_in - Transmitter tx_fsync_in = IOSS tdm_rx_fsync_in '2': TX slave signalling inputs driven by RX Master: - Transmitter tx_sck_in = receiver tdm_rx_sck_out - Transmitter tx_fsync_in = receiver tdm_rx_fsync_out Note: MODE=0 is the default behaviour. MODE=1 or 2 is intended to allow the TX slave to share the same signaling used by the RX. This feature can be used to reduce the number of IO pins necessary to connect to an external codec supporting common TX/RX signaling. Note: when MODE=1 or 2, TX_TEST_CTL.ENABLED and RX_TEST_CTL.ENABLED should not be set to '1'."]
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
#[doc = "TX route control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_route_ctl](index.html) module"]
pub struct TX_ROUTE_CTL_SPEC;
impl crate::RegisterSpec for TX_ROUTE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_route_ctl::R](R) reader structure"]
impl crate::Readable for TX_ROUTE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_route_ctl::W](W) writer structure"]
impl crate::Writable for TX_ROUTE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_ROUTE_CTL to value 0"]
impl crate::Resettable for TX_ROUTE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
