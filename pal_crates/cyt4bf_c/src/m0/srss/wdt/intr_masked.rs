#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDT` reader - Logical and of corresponding request and mask bits. Due to internal synchronization, it takes up to 8 SYSCLK cycles to read from this register. During this time AHB bus is stalled."]
pub type WDT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits. Due to internal synchronization, it takes up to 8 SYSCLK cycles to read from this register. During this time AHB bus is stalled."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "WDT Interrupt Masked Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
