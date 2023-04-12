#[doc = "Register `DESCR_Y_SIZE` reader"]
pub struct R(crate::R<DESCR_Y_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_Y_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_Y_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_Y_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Y_COUNT` reader - N/A"]
pub type Y_COUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn y_count(&self) -> Y_COUNT_R {
        Y_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor Y size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_y_size](index.html) module"]
pub struct DESCR_Y_SIZE_SPEC;
impl crate::RegisterSpec for DESCR_Y_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_y_size::R](R) reader structure"]
impl crate::Readable for DESCR_Y_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_Y_SIZE to value 0"]
impl crate::Resettable for DESCR_Y_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
