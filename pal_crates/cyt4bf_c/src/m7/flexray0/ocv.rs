#[doc = "Register `OCV` reader"]
pub struct R(crate::R<OCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OCV` reader - Offset Correction Value (vOffsetCorrection) Offset correction value (two's complement). Calculated internal offset correction value before limitation. If the OCV value exceeds the limits defined by GTUC10.MOC\\[13:0\\], flag SFS.OCLR is set to '1'."]
pub type OCV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:18 - Offset Correction Value (vOffsetCorrection) Offset correction value (two's complement). Calculated internal offset correction value before limitation. If the OCV value exceeds the limits defined by GTUC10.MOC\\[13:0\\], flag SFS.OCLR is set to '1'."]
    #[inline(always)]
    pub fn ocv(&self) -> OCV_R {
        OCV_R::new(self.bits & 0x0007_ffff)
    }
}
#[doc = "Offset Correction Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocv](index.html) module"]
pub struct OCV_SPEC;
impl crate::RegisterSpec for OCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocv::R](R) reader structure"]
impl crate::Readable for OCV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OCV to value 0"]
impl crate::Resettable for OCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
