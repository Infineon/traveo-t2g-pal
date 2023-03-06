#[doc = "Register `COMP1_STATUS` reader"]
pub struct R(crate::R<COMP1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMP1_OUT` reader - DeepSleep comparator 'comp1_out_lf\\[\\]' outputs (synchronized from clk_lf to the IP clock)."]
pub type COMP1_OUT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DeepSleep comparator 'comp1_out_lf\\[\\]' outputs (synchronized from clk_lf to the IP clock)."]
    #[inline(always)]
    pub fn comp1_out(&self) -> COMP1_OUT_R {
        COMP1_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Comparator structures comparator 1 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_status](index.html) module"]
pub struct COMP1_STATUS_SPEC;
impl crate::RegisterSpec for COMP1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_status::R](R) reader structure"]
impl crate::Readable for COMP1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMP1_STATUS to value 0"]
impl crate::Resettable for COMP1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
