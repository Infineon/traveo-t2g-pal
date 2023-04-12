#[doc = "Register `DESIGNCFG_DEBUG4` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSVD_31_0` reader - N/A"]
pub type RSVD_31_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn rsvd_31_0(&self) -> RSVD_31_0_R {
        RSVD_31_0_R::new(self.bits)
    }
}
#[doc = "Design Configuration Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug4](index.html) module"]
pub struct DESIGNCFG_DEBUG4_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug4::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG4 to value 0"]
impl crate::Resettable for DESIGNCFG_DEBUG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
