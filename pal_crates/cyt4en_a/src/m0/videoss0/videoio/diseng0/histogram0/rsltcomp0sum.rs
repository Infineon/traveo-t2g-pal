#[doc = "Register `RSLTCOMP0SUM` reader"]
pub struct R(crate::R<RSLTCOMP0SUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTCOMP0SUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTCOMP0SUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTCOMP0SUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP0_SUM` reader - Sum of all component 0 values. In the case of RGB pixels, component 0 means the red value, in the case of YUV pixels, component 0 means the Y value. If luma/luminance calculation is switched on by field lum_mode, component 0 is the calculated luma/luminance. If the read value is 0xFFFFFFFF a counter overflow has occurred and the read value should not be used (this can only happen for frame dimensions beyond 2^12 * 2^12). (format is unsigned integer)"]
pub type COMP0_SUM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sum of all component 0 values. In the case of RGB pixels, component 0 means the red value, in the case of YUV pixels, component 0 means the Y value. If luma/luminance calculation is switched on by field lum_mode, component 0 is the calculated luma/luminance. If the read value is 0xFFFFFFFF a counter overflow has occurred and the read value should not be used (this can only happen for frame dimensions beyond 2^12 * 2^12). (format is unsigned integer)"]
    #[inline(always)]
    pub fn comp0_sum(&self) -> COMP0_SUM_R {
        COMP0_SUM_R::new(self.bits)
    }
}
#[doc = "Sum of all component 0 values.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltcomp0sum](index.html) module"]
pub struct RSLTCOMP0SUM_SPEC;
impl crate::RegisterSpec for RSLTCOMP0SUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltcomp0sum::R](R) reader structure"]
impl crate::Readable for RSLTCOMP0SUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTCOMP0SUM to value 0"]
impl crate::Resettable for RSLTCOMP0SUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
