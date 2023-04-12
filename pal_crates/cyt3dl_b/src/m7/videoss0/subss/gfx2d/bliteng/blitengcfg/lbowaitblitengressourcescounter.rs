#[doc = "Register `LBOWAITBLITENGRESSOURCESCOUNTER` reader"]
pub struct R(crate::R<LBOWAITBLITENGRESSOURCESCOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBOWAITBLITENGRESSOURCESCOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBOWAITBLITENGRESSOURCESCOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBOWAITBLITENGRESSOURCESCOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBOWAITBLITENGRESSOURCESCOUNTER` reader - The internal counter is incremented with each 16th clock when the operation has been triggered but is waiting for BlitEng ressources. Only active in LBO and when LBOUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type LBOWAITBLITENGRESSOURCESCOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The internal counter is incremented with each 16th clock when the operation has been triggered but is waiting for BlitEng ressources. Only active in LBO and when LBOUtilizationMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn lbowaitblitengressourcescounter(&self) -> LBOWAITBLITENGRESSOURCESCOUNTER_R {
        LBOWAITBLITENGRESSOURCESCOUNTER_R::new(self.bits)
    }
}
#[doc = "One of the LBO utilization measurement registers.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbowaitblitengressourcescounter](index.html) module"]
pub struct LBOWAITBLITENGRESSOURCESCOUNTER_SPEC;
impl crate::RegisterSpec for LBOWAITBLITENGRESSOURCESCOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbowaitblitengressourcescounter::R](R) reader structure"]
impl crate::Readable for LBOWAITBLITENGRESSOURCESCOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBOWAITBLITENGRESSOURCESCOUNTER to value 0"]
impl crate::Resettable for LBOWAITBLITENGRESSOURCESCOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
