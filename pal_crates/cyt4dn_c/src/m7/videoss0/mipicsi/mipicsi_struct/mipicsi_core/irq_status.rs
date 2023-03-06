#[doc = "Register `IRQ_STATUS` reader"]
pub struct R(crate::R<IRQ_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IRQ_STATUS` reader - N/A"]
pub type IRQ_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn irq_status(&self) -> IRQ_STATUS_R {
        IRQ_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IRQ_STATUS is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_status](index.html) module"]
pub struct IRQ_STATUS_SPEC;
impl crate::RegisterSpec for IRQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_status::R](R) reader structure"]
impl crate::Readable for IRQ_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IRQ_STATUS to value 0"]
impl crate::Resettable for IRQ_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
