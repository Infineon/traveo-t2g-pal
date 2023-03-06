#[doc = "Register `SCV` reader"]
pub struct R(crate::R<SCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCCA` reader - Slot Counter Channel A (vSlotCounter\\[A\\]) Current slot counter value on channel A. The value is incremented by the CC and reset at the start of a communication cycle. Valid values are 0 to 2047."]
pub type SCCA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCCB` reader - Slot Counter Channel B (vSlotCounter\\[B\\]) Current slot counter value on channel B. The value is incremented by the CC and reset at the start of a communication cycle. Valid values are 0 to 2047."]
pub type SCCB_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Slot Counter Channel A (vSlotCounter\\[A\\]) Current slot counter value on channel A. The value is incremented by the CC and reset at the start of a communication cycle. Valid values are 0 to 2047."]
    #[inline(always)]
    pub fn scca(&self) -> SCCA_R {
        SCCA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Slot Counter Channel B (vSlotCounter\\[B\\]) Current slot counter value on channel B. The value is incremented by the CC and reset at the start of a communication cycle. Valid values are 0 to 2047."]
    #[inline(always)]
    pub fn sccb(&self) -> SCCB_R {
        SCCB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "Slot Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scv](index.html) module"]
pub struct SCV_SPEC;
impl crate::RegisterSpec for SCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scv::R](R) reader structure"]
impl crate::Readable for SCV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCV to value 0"]
impl crate::Resettable for SCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
