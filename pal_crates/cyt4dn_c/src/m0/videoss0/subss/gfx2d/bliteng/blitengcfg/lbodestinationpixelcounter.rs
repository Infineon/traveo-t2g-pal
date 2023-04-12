#[doc = "Register `LBODESTINATIONPIXELCOUNTER` reader"]
pub struct R(crate::R<LBODESTINATIONPIXELCOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBODESTINATIONPIXELCOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBODESTINATIONPIXELCOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBODESTINATIONPIXELCOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBODESTINATIONPIXELCOUNTER` reader - Provides the number of processed destination pixels which should be multiplied by 32. In the case of odd width this counter counts one more pixel per line compared to the actual number of pixels. Only active in LBO and when DestinationPixelMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type LBODESTINATIONPIXELCOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the number of processed destination pixels which should be multiplied by 32. In the case of odd width this counter counts one more pixel per line compared to the actual number of pixels. Only active in LBO and when DestinationPixelMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn lbodestinationpixelcounter(&self) -> LBODESTINATIONPIXELCOUNTER_R {
        LBODESTINATIONPIXELCOUNTER_R::new(self.bits)
    }
}
#[doc = "Destination pixel counter for LBO mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbodestinationpixelcounter](index.html) module"]
pub struct LBODESTINATIONPIXELCOUNTER_SPEC;
impl crate::RegisterSpec for LBODESTINATIONPIXELCOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbodestinationpixelcounter::R](R) reader structure"]
impl crate::Readable for LBODESTINATIONPIXELCOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBODESTINATIONPIXELCOUNTER to value 0"]
impl crate::Resettable for LBODESTINATIONPIXELCOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
