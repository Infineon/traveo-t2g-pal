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
#[doc = "Field `FIFO_UNDERFLOW` reader - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
pub type FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `IF_UNDERFLOW` reader - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
pub type IF_UNDERFLOW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
    #[inline(always)]
    pub fn fifo_underflow(&self) -> FIFO_UNDERFLOW_R {
        FIFO_UNDERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical AND of corresponding INTR_TX and INTR_TX_MASK fields."]
    #[inline(always)]
    pub fn if_underflow(&self) -> IF_UNDERFLOW_R {
        IF_UNDERFLOW_R::new(((self.bits >> 8) & 1) != 0)
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
