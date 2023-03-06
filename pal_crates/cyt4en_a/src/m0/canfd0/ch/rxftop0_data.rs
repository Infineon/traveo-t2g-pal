#[doc = "Register `RXFTOP0_DATA` reader"]
pub struct R(crate::R<RXFTOP0_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFTOP0_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFTOP0_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFTOP0_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F0TD` reader - When enabled (F0TPE=1) read data from MRAM at location FnTA. This register can have a read side effect if the following conditions are met: - M_TTCAN not being reconfigured (CCCR.CCE=0) - FIFO Top Pointer logic is enabled (FnTPE=1) - FIFO is not empty (FnFL!=0) The read side effect is as follows: - if FnMWC pointed to the last word of the message (as indicated by FnDS) then the corresponding message index (FnGI) is automatically acknowledge by a write to FnAI - FnMWC is incremented (or restarted if FnMWC pointed to the last word of the message) - the FIFO top address FnTA is incremented (with FIFO wrap around) When this logic is disabled (F0TPE=0) a Read from this register returns undefined data."]
pub type F0TD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - When enabled (F0TPE=1) read data from MRAM at location FnTA. This register can have a read side effect if the following conditions are met: - M_TTCAN not being reconfigured (CCCR.CCE=0) - FIFO Top Pointer logic is enabled (FnTPE=1) - FIFO is not empty (FnFL!=0) The read side effect is as follows: - if FnMWC pointed to the last word of the message (as indicated by FnDS) then the corresponding message index (FnGI) is automatically acknowledge by a write to FnAI - FnMWC is incremented (or restarted if FnMWC pointed to the last word of the message) - the FIFO top address FnTA is incremented (with FIFO wrap around) When this logic is disabled (F0TPE=0) a Read from this register returns undefined data."]
    #[inline(always)]
    pub fn f0td(&self) -> F0TD_R {
        F0TD_R::new(self.bits)
    }
}
#[doc = "Receive FIFO 0 Top Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop0_data](index.html) module"]
pub struct RXFTOP0_DATA_SPEC;
impl crate::RegisterSpec for RXFTOP0_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxftop0_data::R](R) reader structure"]
impl crate::Readable for RXFTOP0_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFTOP0_DATA to value 0"]
impl crate::Resettable for RXFTOP0_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
