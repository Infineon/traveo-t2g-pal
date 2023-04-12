#[doc = "Register `RSLTCOMP2SUM` reader"]
pub struct R(crate::R<RSLTCOMP2SUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTCOMP2SUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTCOMP2SUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTCOMP2SUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP2_SUM` reader - Sum of all component 2 values. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cr value. If the read value is 0xFFFFFFFF a counter overflow has occurred and the read value should not be used (this can only happen for frame dimensions beyond 2^12 * 2^12). (format is unsigned integer)"]
pub type COMP2_SUM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sum of all component 2 values. In the case of RGB pixels, component 2 means the blue value. In the case of YUV pixels, component 2 means the Cr value. If the read value is 0xFFFFFFFF a counter overflow has occurred and the read value should not be used (this can only happen for frame dimensions beyond 2^12 * 2^12). (format is unsigned integer)"]
    #[inline(always)]
    pub fn comp2_sum(&self) -> COMP2_SUM_R {
        COMP2_SUM_R::new(self.bits)
    }
}
#[doc = "Sum of all component 2 values.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltcomp2sum](index.html) module"]
pub struct RSLTCOMP2SUM_SPEC;
impl crate::RegisterSpec for RSLTCOMP2SUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltcomp2sum::R](R) reader structure"]
impl crate::Readable for RSLTCOMP2SUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTCOMP2SUM to value 0"]
impl crate::Resettable for RSLTCOMP2SUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
