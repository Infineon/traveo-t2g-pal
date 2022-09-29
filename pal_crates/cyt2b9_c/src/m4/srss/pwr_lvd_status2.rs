#[doc = "Register `PWR_LVD_STATUS2` reader"]
pub struct R(crate::R<PWR_LVD_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_LVD_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_LVD_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_LVD_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HVLVD2_OUT` reader - HVLVD2 output. 0: below voltage threshold 1: above voltage threshold"]
pub type HVLVD2_OUT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - HVLVD2 output. 0: below voltage threshold 1: above voltage threshold"]
    #[inline(always)]
    pub fn hvlvd2_out(&self) -> HVLVD2_OUT_R {
        HVLVD2_OUT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "High Voltage / Low Voltage Detector (HVLVD) Status Register #2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_lvd_status2](index.html) module"]
pub struct PWR_LVD_STATUS2_SPEC;
impl crate::RegisterSpec for PWR_LVD_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_lvd_status2::R](R) reader structure"]
impl crate::Readable for PWR_LVD_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_LVD_STATUS2 to value 0"]
impl crate::Resettable for PWR_LVD_STATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
