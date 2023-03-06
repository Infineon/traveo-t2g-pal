#[doc = "Register `INT_Q2_MASK` reader"]
pub struct R(crate::R<INT_Q2_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_Q2_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_Q2_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_Q2_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECEIVE_COMPLETE_INTERRUPT_MASK` reader - receive complete interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RECEIVE_COMPLETE_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RX_USED_INTERRUPT_MASK` reader - A read of this register returns the value of the RX Used interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RX_USED_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_MASK` reader - retry limit exceeded or late collision interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `AMBA_ERROR_INTERRUPT_MASK` reader - A read of this register returns the value of the AMBA (AXI) error interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type AMBA_ERROR_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `TRANSMIT_COMPLETE_INTERRUPT_MASK` reader - transmit complete interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type TRANSMIT_COMPLETE_INTERRUPT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RESP_NOT_OK_INTERRUPT_MASK` reader - bresp not OK interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
pub type RESP_NOT_OK_INTERRUPT_MASK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - receive complete interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn receive_complete_interrupt_mask(&self) -> RECEIVE_COMPLETE_INTERRUPT_MASK_R {
        RECEIVE_COMPLETE_INTERRUPT_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A read of this register returns the value of the RX Used interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn rx_used_interrupt_mask(&self) -> RX_USED_INTERRUPT_MASK_R {
        RX_USED_INTERRUPT_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - retry limit exceeded or late collision interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn retry_limit_exceeded_or_late_collision_interrupt_mask(
        &self,
    ) -> RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_MASK_R {
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A read of this register returns the value of the AMBA (AXI) error interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn amba_error_interrupt_mask(&self) -> AMBA_ERROR_INTERRUPT_MASK_R {
        AMBA_ERROR_INTERRUPT_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transmit complete interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn transmit_complete_interrupt_mask(&self) -> TRANSMIT_COMPLETE_INTERRUPT_MASK_R {
        TRANSMIT_COMPLETE_INTERRUPT_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - bresp not OK interrupt mask. 0: Interrupt is enabled. 1: Interrupt is disabled. A write to this register directly affects the state of the corresponding bit in the interrupt status register, causing an interrupt to be generated if a 1 is written."]
    #[inline(always)]
    pub fn resp_not_ok_interrupt_mask(&self) -> RESP_NOT_OK_INTERRUPT_MASK_R {
        RESP_NOT_OK_INTERRUPT_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_q2_mask](index.html) module"]
pub struct INT_Q2_MASK_SPEC;
impl crate::RegisterSpec for INT_Q2_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_q2_mask::R](R) reader structure"]
impl crate::Readable for INT_Q2_MASK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_Q2_MASK to value 0x08e6"]
impl crate::Resettable for INT_Q2_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x08e6;
}
