#[doc = "Register `DEFERRED_FRAMES` reader"]
pub struct R(crate::R<DEFERRED_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEFERRED_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEFERRED_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEFERRED_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT18` reader - Deferred transmission frames - an 18 bit register counting the number of frames experiencing deferral due to carrier sense being active on their first attempt at transmission. Frames involved in any collision are not counted nor are frames that experienced a transmit under run."]
pub type COUNT18_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Deferred transmission frames - an 18 bit register counting the number of frames experiencing deferral due to carrier sense being active on their first attempt at transmission. Frames involved in any collision are not counted nor are frames that experienced a transmit under run."]
    #[inline(always)]
    pub fn count18(&self) -> COUNT18_R {
        COUNT18_R::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Deferred Transmission Frames. Presents in design but not support.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deferred_frames](index.html) module"]
pub struct DEFERRED_FRAMES_SPEC;
impl crate::RegisterSpec for DEFERRED_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deferred_frames::R](R) reader structure"]
impl crate::Readable for DEFERRED_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEFERRED_FRAMES to value 0"]
impl crate::Resettable for DEFERRED_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
