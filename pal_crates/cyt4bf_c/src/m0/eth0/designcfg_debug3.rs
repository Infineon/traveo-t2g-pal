#[doc = "Register `DESIGNCFG_DEBUG3` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_SPEC_ADD_FILTERS` reader - Takes the value of the `num_spec_add_filters DEFINE"]
pub type NUM_SPEC_ADD_FILTERS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 24:29 - Takes the value of the `num_spec_add_filters DEFINE"]
    #[inline(always)]
    pub fn num_spec_add_filters(&self) -> NUM_SPEC_ADD_FILTERS_R {
        NUM_SPEC_ADD_FILTERS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Design Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug3](index.html) module"]
pub struct DESIGNCFG_DEBUG3_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug3::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG3 to value 0x0400_0000"]
impl crate::Resettable for DESIGNCFG_DEBUG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0000;
}
