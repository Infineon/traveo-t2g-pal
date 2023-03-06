#[doc = "Register `M_IDX` reader"]
pub struct R(crate::R<M_IDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M_IDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M_IDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M_IDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M` reader - Specifies the M loop index. In the range of \\[0, M_COUNT\\], with M_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
pub type M_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Specifies the M loop index. In the range of \\[0, M_COUNT\\], with M_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Channel current M index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m_idx](index.html) module"]
pub struct M_IDX_SPEC;
impl crate::RegisterSpec for M_IDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m_idx::R](R) reader structure"]
impl crate::Readable for M_IDX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M_IDX to value 0"]
impl crate::Resettable for M_IDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
