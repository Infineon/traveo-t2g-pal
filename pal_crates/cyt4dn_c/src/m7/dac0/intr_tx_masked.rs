#[doc = "Register `INTR_TX_MASKED` reader"]
pub struct R(crate::R<INTR_TX_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_TX_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_TX_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_TX_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_TRIGGER` reader - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
pub type FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_OVERFLOW` reader - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
pub type FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_UNDERFLOW` reader - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
pub type FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FAST_RAMP_COMPLETE` reader - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
pub type FAST_RAMP_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `RAMP_COMPLETE` reader - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
pub type RAMP_COMPLETE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
    #[inline(always)]
    pub fn fifo_trigger(&self) -> FIFO_TRIGGER_R {
        FIFO_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
    #[inline(always)]
    pub fn fifo_overflow(&self) -> FIFO_OVERFLOW_R {
        FIFO_OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
    #[inline(always)]
    pub fn fifo_underflow(&self) -> FIFO_UNDERFLOW_R {
        FIFO_UNDERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
    #[inline(always)]
    pub fn fast_ramp_complete(&self) -> FAST_RAMP_COMPLETE_R {
        FAST_RAMP_COMPLETE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
    #[inline(always)]
    pub fn ramp_complete(&self) -> RAMP_COMPLETE_R {
        RAMP_COMPLETE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_tx_masked](index.html) module"]
pub struct INTR_TX_MASKED_SPEC;
impl crate::RegisterSpec for INTR_TX_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_tx_masked::R](R) reader structure"]
impl crate::Readable for INTR_TX_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_TX_MASKED to value 0"]
impl crate::Resettable for INTR_TX_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
