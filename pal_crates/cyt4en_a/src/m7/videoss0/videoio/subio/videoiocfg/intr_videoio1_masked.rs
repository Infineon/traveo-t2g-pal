#[doc = "Register `INTR_VIDEOIO1_MASKED` reader"]
pub struct R(crate::R<INTR_VIDEOIO1_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_VIDEOIO1_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_VIDEOIO1_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_VIDEOIO1_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR_VIDEOIO1_MASKED` reader - Status vector INTR_VIDEOIO1 masked with INTR_VIDEOIO1_MASK."]
pub type INTR_VIDEOIO1_MASKED_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Status vector INTR_VIDEOIO1 masked with INTR_VIDEOIO1_MASK."]
    #[inline(always)]
    pub fn intr_videoio1_masked(&self) -> INTR_VIDEOIO1_MASKED_R {
        INTR_VIDEOIO1_MASKED_R::new(self.bits)
    }
}
#[doc = "Interrupt masked register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_videoio1_masked](index.html) module"]
pub struct INTR_VIDEOIO1_MASKED_SPEC;
impl crate::RegisterSpec for INTR_VIDEOIO1_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_videoio1_masked::R](R) reader structure"]
impl crate::Readable for INTR_VIDEOIO1_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_VIDEOIO1_MASKED to value 0"]
impl crate::Resettable for INTR_VIDEOIO1_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
