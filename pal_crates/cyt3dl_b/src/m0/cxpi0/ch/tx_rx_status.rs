#[doc = "Register `TX_RX_STATUS` reader"]
pub struct R(crate::R<TX_RX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_RX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_RX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_RX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_RX_STATUS` writer"]
pub struct W(crate::W<TX_RX_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_RX_STATUS_SPEC>;
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
impl From<crate::W<TX_RX_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_RX_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_IN` reader - CXPI transmitter input ('tx_in', 'cxpi_tx_in' in functional mode). TX_IN and RX_IN can be used to determine a wakeup source. Note that wakeup source detection relies on the external transceiver functionality."]
pub type TX_IN_R = crate::BitReader<bool>;
#[doc = "Field `RX_IN` reader - CXPI receiver input ('rx_in', 'cxpi_rx_in' in functional mode)."]
pub type RX_IN_R = crate::BitReader<bool>;
#[doc = "Field `TX_OUT` reader - CXPI transmitter output ('tx_out', 'cxpi_tx_out')."]
pub type TX_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EN_OUT` reader - CXPI transceiver enable ('en_out', 'cxpi_en_out'). This field controls the enable (or low active sleep enable) of the external transceiver: '0': Disabled. '1': Enabled. If CTL0.AUTO_EN is '0', SW controls this field to enable the external transceiver. If CTL0.AUTO_EN is '1', HW controls this field to enable the external transceiver: - HW sets this field to '1' when it is out of Sleep state. If it moves to Sleep state, HW will clear this field to '0'."]
pub type EN_OUT_R = crate::BitReader<bool>;
#[doc = "Field `EN_OUT` writer - CXPI transceiver enable ('en_out', 'cxpi_en_out'). This field controls the enable (or low active sleep enable) of the external transceiver: '0': Disabled. '1': Enabled. If CTL0.AUTO_EN is '0', SW controls this field to enable the external transceiver. If CTL0.AUTO_EN is '1', HW controls this field to enable the external transceiver: - HW sets this field to '1' when it is out of Sleep state. If it moves to Sleep state, HW will clear this field to '0'."]
pub type EN_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_RX_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - CXPI transmitter input ('tx_in', 'cxpi_tx_in' in functional mode). TX_IN and RX_IN can be used to determine a wakeup source. Note that wakeup source detection relies on the external transceiver functionality."]
    #[inline(always)]
    pub fn tx_in(&self) -> TX_IN_R {
        TX_IN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CXPI receiver input ('rx_in', 'cxpi_rx_in' in functional mode)."]
    #[inline(always)]
    pub fn rx_in(&self) -> RX_IN_R {
        RX_IN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - CXPI transmitter output ('tx_out', 'cxpi_tx_out')."]
    #[inline(always)]
    pub fn tx_out(&self) -> TX_OUT_R {
        TX_OUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - CXPI transceiver enable ('en_out', 'cxpi_en_out'). This field controls the enable (or low active sleep enable) of the external transceiver: '0': Disabled. '1': Enabled. If CTL0.AUTO_EN is '0', SW controls this field to enable the external transceiver. If CTL0.AUTO_EN is '1', HW controls this field to enable the external transceiver: - HW sets this field to '1' when it is out of Sleep state. If it moves to Sleep state, HW will clear this field to '0'."]
    #[inline(always)]
    pub fn en_out(&self) -> EN_OUT_R {
        EN_OUT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - CXPI transceiver enable ('en_out', 'cxpi_en_out'). This field controls the enable (or low active sleep enable) of the external transceiver: '0': Disabled. '1': Enabled. If CTL0.AUTO_EN is '0', SW controls this field to enable the external transceiver. If CTL0.AUTO_EN is '1', HW controls this field to enable the external transceiver: - HW sets this field to '1' when it is out of Sleep state. If it moves to Sleep state, HW will clear this field to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn en_out(&mut self) -> EN_OUT_W<26> {
        EN_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX/RX status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rx_status](index.html) module"]
pub struct TX_RX_STATUS_SPEC;
impl crate::RegisterSpec for TX_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_rx_status::R](R) reader structure"]
impl crate::Readable for TX_RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_rx_status::W](W) writer structure"]
impl crate::Writable for TX_RX_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_RX_STATUS to value 0x0500_0000"]
impl crate::Resettable for TX_RX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0500_0000;
}
