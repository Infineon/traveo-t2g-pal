#[doc = "Register `SPEC_ADD36_TOP` reader"]
pub struct R(crate::R<SPEC_ADD36_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPEC_ADD36_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPEC_ADD36_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPEC_ADD36_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSVD_31_0` reader - Write ignore, read 0"]
pub type RSVD_31_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write ignore, read 0"]
    #[inline(always)]
    pub fn rsvd_31_0(&self) -> RSVD_31_0_R {
        RSVD_31_0_R::new(self.bits)
    }
}
#[doc = "Not presents.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spec_add36_top](index.html) module"]
pub struct SPEC_ADD36_TOP_SPEC;
impl crate::RegisterSpec for SPEC_ADD36_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spec_add36_top::R](R) reader structure"]
impl crate::Readable for SPEC_ADD36_TOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPEC_ADD36_TOP to value 0"]
impl crate::Resettable for SPEC_ADD36_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
