#[doc = "Register `IBODESTINATIONPIXELCOUNTER` reader"]
pub struct R(crate::R<IBODESTINATIONPIXELCOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBODESTINATIONPIXELCOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBODESTINATIONPIXELCOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBODESTINATIONPIXELCOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBODESTINATIONPIXELCOUNTER` reader - Provides the number of processed destination pixels which should be multiplied by 16. Only active in IBO and when DestinationPixelMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type IBODESTINATIONPIXELCOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the number of processed destination pixels which should be multiplied by 16. Only active in IBO and when DestinationPixelMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn ibodestinationpixelcounter(&self) -> IBODESTINATIONPIXELCOUNTER_R {
        IBODESTINATIONPIXELCOUNTER_R::new(self.bits)
    }
}
#[doc = "Destination pixel counter for IBO mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibodestinationpixelcounter](index.html) module"]
pub struct IBODESTINATIONPIXELCOUNTER_SPEC;
impl crate::RegisterSpec for IBODESTINATIONPIXELCOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibodestinationpixelcounter::R](R) reader structure"]
impl crate::Readable for IBODESTINATIONPIXELCOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IBODESTINATIONPIXELCOUNTER to value 0"]
impl crate::Resettable for IBODESTINATIONPIXELCOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
