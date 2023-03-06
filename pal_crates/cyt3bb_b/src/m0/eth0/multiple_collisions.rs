#[doc = "Register `MULTIPLE_COLLISIONS` reader"]
pub struct R(crate::R<MULTIPLE_COLLISIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULTIPLE_COLLISIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULTIPLE_COLLISIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULTIPLE_COLLISIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT15` reader - Multiple collision frames - an 18 bit register counting the number of frames experiencing between two and fifteen collisions prior to being successfully transmitted, i.e. no under run and not too many retries."]
pub type COUNT15_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Multiple collision frames - an 18 bit register counting the number of frames experiencing between two and fifteen collisions prior to being successfully transmitted, i.e. no under run and not too many retries."]
    #[inline(always)]
    pub fn count15(&self) -> COUNT15_R {
        COUNT15_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Multiple Collision Frames. Presents in design but not support.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [multiple_collisions](index.html) module"]
pub struct MULTIPLE_COLLISIONS_SPEC;
impl crate::RegisterSpec for MULTIPLE_COLLISIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [multiple_collisions::R](R) reader structure"]
impl crate::Readable for MULTIPLE_COLLISIONS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MULTIPLE_COLLISIONS to value 0"]
impl crate::Resettable for MULTIPLE_COLLISIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
