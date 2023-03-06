#[doc = "Register `LBOSOURCEPIXELRGBACOUNTER` reader"]
pub struct R(crate::R<LBOSOURCEPIXELRGBACOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBOSOURCEPIXELRGBACOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBOSOURCEPIXELRGBACOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBOSOURCEPIXELRGBACOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBOSOURCEPIXELRGBACOUNTER` reader - Provides the number of processed source RGBA pixels which should be multiplied by 256. Only active in LBO and when LBOSourcePixelMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type LBOSOURCEPIXELRGBACOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the number of processed source RGBA pixels which should be multiplied by 256. Only active in LBO and when LBOSourcePixelMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn lbosourcepixelrgbacounter(&self) -> LBOSOURCEPIXELRGBACOUNTER_R {
        LBOSOURCEPIXELRGBACOUNTER_R::new(self.bits)
    }
}
#[doc = "RGBA source pixel counter for LBO mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbosourcepixelrgbacounter](index.html) module"]
pub struct LBOSOURCEPIXELRGBACOUNTER_SPEC;
impl crate::RegisterSpec for LBOSOURCEPIXELRGBACOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbosourcepixelrgbacounter::R](R) reader structure"]
impl crate::Readable for LBOSOURCEPIXELRGBACOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBOSOURCEPIXELRGBACOUNTER to value 0"]
impl crate::Resettable for LBOSOURCEPIXELRGBACOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
