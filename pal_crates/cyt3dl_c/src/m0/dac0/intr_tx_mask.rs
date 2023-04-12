#[doc = "Register `INTR_TX_MASK` reader"]
pub struct R(crate::R<INTR_TX_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_TX_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_TX_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_TX_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_TX_MASK` writer"]
pub struct W(crate::W<INTR_TX_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_TX_MASK_SPEC>;
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
impl From<crate::W<INTR_TX_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_TX_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_TRIGGER` reader - Mask for corresponding field in INTR_TX register."]
pub type FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_TRIGGER` writer - Mask for corresponding field in INTR_TX register."]
pub type FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_TX_MASK_SPEC, bool, O>;
#[doc = "Field `FIFO_OVERFLOW` reader - Mask for corresponding field in INTR_TX register."]
pub type FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_OVERFLOW` writer - Mask for corresponding field in INTR_TX register."]
pub type FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_TX_MASK_SPEC, bool, O>;
#[doc = "Field `FIFO_UNDERFLOW` reader - Mask for corresponding field in INTR_TX register."]
pub type FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_UNDERFLOW` writer - Mask for corresponding field in INTR_TX register."]
pub type FIFO_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_TX_MASK_SPEC, bool, O>;
#[doc = "Field `FAST_RAMP_COMPLETE` reader - Mask for corresponding field in INTR_TX register."]
pub type FAST_RAMP_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `FAST_RAMP_COMPLETE` writer - Mask for corresponding field in INTR_TX register."]
pub type FAST_RAMP_COMPLETE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_TX_MASK_SPEC, bool, O>;
#[doc = "Field `RAMP_COMPLETE` reader - Mask for corresponding field in INTR_TX register."]
pub type RAMP_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `RAMP_COMPLETE` writer - Mask for corresponding field in INTR_TX register."]
pub type RAMP_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_TX_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    pub fn fifo_trigger(&self) -> FIFO_TRIGGER_R {
        FIFO_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    pub fn fifo_overflow(&self) -> FIFO_OVERFLOW_R {
        FIFO_OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    pub fn fifo_underflow(&self) -> FIFO_UNDERFLOW_R {
        FIFO_UNDERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    pub fn fast_ramp_complete(&self) -> FAST_RAMP_COMPLETE_R {
        FAST_RAMP_COMPLETE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    pub fn ramp_complete(&self) -> RAMP_COMPLETE_R {
        RAMP_COMPLETE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_trigger(&mut self) -> FIFO_TRIGGER_W<0> {
        FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 1 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow(&mut self) -> FIFO_OVERFLOW_W<1> {
        FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 2 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underflow(&mut self) -> FIFO_UNDERFLOW_W<2> {
        FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 16 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ramp_complete(&mut self) -> FAST_RAMP_COMPLETE_W<16> {
        FAST_RAMP_COMPLETE_W::new(self)
    }
    #[doc = "Bit 17 - Mask for corresponding field in INTR_TX register."]
    #[inline(always)]
    #[must_use]
    pub fn ramp_complete(&mut self) -> RAMP_COMPLETE_W<17> {
        RAMP_COMPLETE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_tx_mask](index.html) module"]
pub struct INTR_TX_MASK_SPEC;
impl crate::RegisterSpec for INTR_TX_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_tx_mask::R](R) reader structure"]
impl crate::Readable for INTR_TX_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_tx_mask::W](W) writer structure"]
impl crate::Writable for INTR_TX_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_TX_MASK to value 0"]
impl crate::Resettable for INTR_TX_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
