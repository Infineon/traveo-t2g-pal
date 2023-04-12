#[doc = "Register `TB0ACTIVECOUNTER` reader"]
pub struct R(crate::R<TB0ACTIVECOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TB0ACTIVECOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TB0ACTIVECOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TB0ACTIVECOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TB0ACTIVECOUNTER` reader - The internal counter is incremented with each 16th clock when there is any ongoing blit operation programmed via TB0. Only active when TBUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type TB0ACTIVECOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The internal counter is incremented with each 16th clock when there is any ongoing blit operation programmed via TB0. Only active when TBUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn tb0activecounter(&self) -> TB0ACTIVECOUNTER_R {
        TB0ACTIVECOUNTER_R::new(self.bits)
    }
}
#[doc = "One of the TB utilization measurement registers. Since TB operations may be overlapping the sum of all TB utilization measurement registers may be higher than clock counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0activecounter](index.html) module"]
pub struct TB0ACTIVECOUNTER_SPEC;
impl crate::RegisterSpec for TB0ACTIVECOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tb0activecounter::R](R) reader structure"]
impl crate::Readable for TB0ACTIVECOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TB0ACTIVECOUNTER to value 0"]
impl crate::Resettable for TB0ACTIVECOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
