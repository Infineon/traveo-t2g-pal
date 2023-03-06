#[doc = "Register `INTR_DPSLP_MASKED` reader"]
pub struct R(crate::R<INTR_DPSLP_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_DPSLP_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_DPSLP_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_DPSLP_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP1` reader - Logical and of corresponding INTR and INTR_MASK fields."]
pub type COMP1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DeepSleep interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_dpslp_masked](index.html) module"]
pub struct INTR_DPSLP_MASKED_SPEC;
impl crate::RegisterSpec for INTR_DPSLP_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_dpslp_masked::R](R) reader structure"]
impl crate::Readable for INTR_DPSLP_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_DPSLP_MASKED to value 0"]
impl crate::Resettable for INTR_DPSLP_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
