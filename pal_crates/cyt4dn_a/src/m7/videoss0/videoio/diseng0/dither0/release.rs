#[doc = "Register `RELEASE` reader"]
pub struct R(crate::R<RELEASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RELEASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RELEASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RELEASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUBVERSION` reader - Dither Unit subversion."]
pub type SUBVERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VERSION` reader - Dither Unit version."]
pub type VERSION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Dither Unit subversion."]
    #[inline(always)]
    pub fn subversion(&self) -> SUBVERSION_R {
        SUBVERSION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Dither Unit version."]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Dither Unit release.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [release](index.html) module"]
pub struct RELEASE_SPEC;
impl crate::RegisterSpec for RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [release::R](R) reader structure"]
impl crate::Readable for RELEASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RELEASE to value 0"]
impl crate::Resettable for RELEASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
