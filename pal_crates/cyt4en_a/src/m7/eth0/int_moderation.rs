#[doc = "Register `INT_MODERATION` reader"]
pub struct R(crate::R<INT_MODERATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_MODERATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_MODERATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_MODERATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_MODERATION` writer"]
pub struct W(crate::W<INT_MODERATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_MODERATION_SPEC>;
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
impl From<crate::W<INT_MODERATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_MODERATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_INT_MODERATION` reader - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received. A non-zero value indicates receive interrupt moderation will be performed."]
pub type RX_INT_MODERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_INT_MODERATION` writer - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received. A non-zero value indicates receive interrupt moderation will be performed."]
pub type RX_INT_MODERATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_MODERATION_SPEC, u8, u8, 8, O>;
#[doc = "Field `TX_INT_MODERATION` reader - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted. A non-zero value indicates transmit interrupt moderation will be performed."]
pub type TX_INT_MODERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_INT_MODERATION` writer - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted. A non-zero value indicates transmit interrupt moderation will be performed."]
pub type TX_INT_MODERATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_MODERATION_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received. A non-zero value indicates receive interrupt moderation will be performed."]
    #[inline(always)]
    pub fn rx_int_moderation(&self) -> RX_INT_MODERATION_R {
        RX_INT_MODERATION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted. A non-zero value indicates transmit interrupt moderation will be performed."]
    #[inline(always)]
    pub fn tx_int_moderation(&self) -> TX_INT_MODERATION_R {
        TX_INT_MODERATION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received. A non-zero value indicates receive interrupt moderation will be performed."]
    #[inline(always)]
    #[must_use]
    pub fn rx_int_moderation(&mut self) -> RX_INT_MODERATION_W<0> {
        RX_INT_MODERATION_W::new(self)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted. A non-zero value indicates transmit interrupt moderation will be performed."]
    #[inline(always)]
    #[must_use]
    pub fn tx_int_moderation(&mut self) -> TX_INT_MODERATION_W<16> {
        TX_INT_MODERATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used to moderate the number of transmit and receive complete interrupts issued. With interrupt moderation enabled receive and transmit interrupts are not generated immediately a frame is transmitted or received. Instead when a receive or transmit event occurs a timer is started and the interrupt is asserted after it times out. This limits the frequency with which the CPU receives interrupts. When interrupt moderation is enabled interrupt status bit one is always used for receive and bit 7 is always used for transmit even when priority queuing is enabled. With interrupt moderation 800ns periods are counted. GEM determines what constitutes an 800ns period by looking at the tbi (bit 11), gigabit bit (10) and speed (bit 0) bits in the network configuration register and counting tx_clk cycles. Bit 0 needs to be set to 1 for 100M operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_moderation](index.html) module"]
pub struct INT_MODERATION_SPEC;
impl crate::RegisterSpec for INT_MODERATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_moderation::R](R) reader structure"]
impl crate::Readable for INT_MODERATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_moderation::W](W) writer structure"]
impl crate::Writable for INT_MODERATION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_MODERATION to value 0"]
impl crate::Resettable for INT_MODERATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
