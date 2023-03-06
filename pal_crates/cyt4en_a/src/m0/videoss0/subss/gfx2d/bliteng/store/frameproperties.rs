#[doc = "Register `FRAMEPROPERTIES` reader"]
pub struct R(crate::R<FRAMEPROPERTIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEPROPERTIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEPROPERTIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEPROPERTIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIELDID` reader - Field identifier for interlaced video streams (0/1 = even/odd line indices of progressive frame). Status is updated with begin of a new field."]
pub type FIELDID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Field identifier for interlaced video streams (0/1 = even/odd line indices of progressive frame). Status is updated with begin of a new field."]
    #[inline(always)]
    pub fn fieldid(&self) -> FIELDID_R {
        FIELDID_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Ring buffer synchronization.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameproperties](index.html) module"]
pub struct FRAMEPROPERTIES_SPEC;
impl crate::RegisterSpec for FRAMEPROPERTIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frameproperties::R](R) reader structure"]
impl crate::Readable for FRAMEPROPERTIES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAMEPROPERTIES to value 0"]
impl crate::Resettable for FRAMEPROPERTIES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
