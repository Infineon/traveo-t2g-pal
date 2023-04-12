#[doc = "Register `RSLTCOMP1SUM` reader"]
pub struct R(crate::R<RSLTCOMP1SUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSLTCOMP1SUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSLTCOMP1SUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSLTCOMP1SUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP1_SUM` reader - Sum of all component 1 values. In the case of RGB pixels, component 1 means the green value. In the case of YUV pixels, component 1 means the Cb value. If the read value is 0xFFFFFFFF a counter overflow has occured and the read value should not be used (this can only happen for frame dimensions beyond 2^12 * 2^12)."]
pub type COMP1_SUM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sum of all component 1 values. In the case of RGB pixels, component 1 means the green value. In the case of YUV pixels, component 1 means the Cb value. If the read value is 0xFFFFFFFF a counter overflow has occured and the read value should not be used (this can only happen for frame dimensions beyond 2^12 * 2^12)."]
    #[inline(always)]
    pub fn comp1_sum(&self) -> COMP1_SUM_R {
        COMP1_SUM_R::new(self.bits)
    }
}
#[doc = "Sum of all component 1 values.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsltcomp1sum](index.html) module"]
pub struct RSLTCOMP1SUM_SPEC;
impl crate::RegisterSpec for RSLTCOMP1SUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsltcomp1sum::R](R) reader structure"]
impl crate::Readable for RSLTCOMP1SUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSLTCOMP1SUM to value 0"]
impl crate::Resettable for RSLTCOMP1SUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
