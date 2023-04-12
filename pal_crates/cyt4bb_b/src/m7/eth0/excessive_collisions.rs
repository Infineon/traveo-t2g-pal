#[doc = "Register `EXCESSIVE_COLLISIONS` reader"]
pub struct R(crate::R<EXCESSIVE_COLLISIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXCESSIVE_COLLISIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXCESSIVE_COLLISIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXCESSIVE_COLLISIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT16` reader - Excessive collisions - a 10 bit register counting the number of frames that failed to be transmitted because they experienced 16 collisions."]
pub type COUNT16_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Excessive collisions - a 10 bit register counting the number of frames that failed to be transmitted because they experienced 16 collisions."]
    #[inline(always)]
    pub fn count16(&self) -> COUNT16_R {
        COUNT16_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Excessive Collisions. Presents in design but not support.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [excessive_collisions](index.html) module"]
pub struct EXCESSIVE_COLLISIONS_SPEC;
impl crate::RegisterSpec for EXCESSIVE_COLLISIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [excessive_collisions::R](R) reader structure"]
impl crate::Readable for EXCESSIVE_COLLISIONS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXCESSIVE_COLLISIONS to value 0"]
impl crate::Resettable for EXCESSIVE_COLLISIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
