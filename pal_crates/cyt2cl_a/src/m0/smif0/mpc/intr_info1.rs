#[doc = "Register `INTR_INFO1` reader"]
pub struct R(crate::R<INTR_INFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_INFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_INFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_INFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - Full address of the access that caused violation"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Full address of the access that caused violation"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
#[doc = "Infor about violation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_info1](index.html) module"]
pub struct INTR_INFO1_SPEC;
impl crate::RegisterSpec for INTR_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_info1::R](R) reader structure"]
impl crate::Readable for INTR_INFO1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_INFO1 to value 0"]
impl crate::Resettable for INTR_INFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
