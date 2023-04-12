#[doc = "Register `DESCR_M_SIZE` reader"]
pub struct R(crate::R<DESCR_M_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_M_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_M_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_M_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M_COUNT` reader - N/A"]
pub type M_COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - N/A"]
    #[inline(always)]
    pub fn m_count(&self) -> M_COUNT_R {
        M_COUNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Channel descriptor M size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_m_size](index.html) module"]
pub struct DESCR_M_SIZE_SPEC;
impl crate::RegisterSpec for DESCR_M_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_m_size::R](R) reader structure"]
impl crate::Readable for DESCR_M_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_M_SIZE to value 0"]
impl crate::Resettable for DESCR_M_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
