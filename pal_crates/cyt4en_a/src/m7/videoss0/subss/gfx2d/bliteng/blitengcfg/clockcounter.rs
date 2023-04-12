#[doc = "Register `CLOCKCOUNTER` reader"]
pub struct R(crate::R<CLOCKCOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCKCOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCKCOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCKCOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLOCKCOUNTER` reader - The internal clock counter is incremented with each 16th clock cycle if any of the measurements are activated. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type CLOCKCOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The internal clock counter is incremented with each 16th clock cycle if any of the measurements are activated. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn clockcounter(&self) -> CLOCKCOUNTER_R {
        CLOCKCOUNTER_R::new(self.bits)
    }
}
#[doc = "One of Clock Counter registers.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockcounter](index.html) module"]
pub struct CLOCKCOUNTER_SPEC;
impl crate::RegisterSpec for CLOCKCOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clockcounter::R](R) reader structure"]
impl crate::Readable for CLOCKCOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLOCKCOUNTER to value 0"]
impl crate::Resettable for CLOCKCOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
