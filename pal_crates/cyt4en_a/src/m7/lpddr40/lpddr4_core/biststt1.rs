#[doc = "Register `BISTSTT1` reader"]
pub struct R(crate::R<BISTSTT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BISTSTT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BISTSTT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BISTSTT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BISTERR_DQ` reader - BIST Error status of DQ Data Range A '1' indicates an error on the respective DQ line."]
pub type BISTERR_DQ_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - BIST Error status of DQ Data Range A '1' indicates an error on the respective DQ line."]
    #[inline(always)]
    pub fn bisterr_dq(&self) -> BISTERR_DQ_R {
        BISTERR_DQ_R::new(self.bits)
    }
}
#[doc = "BIST Status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biststt1](index.html) module"]
pub struct BISTSTT1_SPEC;
impl crate::RegisterSpec for BISTSTT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biststt1::R](R) reader structure"]
impl crate::Readable for BISTSTT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BISTSTT1 to value 0"]
impl crate::Resettable for BISTSTT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
