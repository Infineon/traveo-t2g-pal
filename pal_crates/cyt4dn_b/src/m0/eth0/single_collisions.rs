#[doc = "Register `SINGLE_COLLISIONS` reader"]
pub struct R(crate::R<SINGLE_COLLISIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLE_COLLISIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLE_COLLISIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLE_COLLISIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT14` reader - Single collision frames - an 18 bit register counting the number of frames experiencing a single collision before being successfully transmitted, i.e. no under run."]
pub type COUNT14_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Single collision frames - an 18 bit register counting the number of frames experiencing a single collision before being successfully transmitted, i.e. no under run."]
    #[inline(always)]
    pub fn count14(&self) -> COUNT14_R {
        COUNT14_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Single Collision Frames. Presents in design but not support.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [single_collisions](index.html) module"]
pub struct SINGLE_COLLISIONS_SPEC;
impl crate::RegisterSpec for SINGLE_COLLISIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [single_collisions::R](R) reader structure"]
impl crate::Readable for SINGLE_COLLISIONS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SINGLE_COLLISIONS to value 0"]
impl crate::Resettable for SINGLE_COLLISIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
