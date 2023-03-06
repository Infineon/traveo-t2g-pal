#[doc = "Register `DESCR_X_SIZE` reader"]
pub struct R(crate::R<DESCR_X_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_X_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_X_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_X_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `X_COUNT` reader - N/A"]
pub type X_COUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn x_count(&self) -> X_COUNT_R {
        X_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor X size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_x_size](index.html) module"]
pub struct DESCR_X_SIZE_SPEC;
impl crate::RegisterSpec for DESCR_X_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_x_size::R](R) reader structure"]
impl crate::Readable for DESCR_X_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_X_SIZE to value 0"]
impl crate::Resettable for DESCR_X_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
