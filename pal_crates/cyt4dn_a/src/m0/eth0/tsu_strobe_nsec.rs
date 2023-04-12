#[doc = "Register `TSU_STROBE_NSEC` reader"]
pub struct R(crate::R<TSU_STROBE_NSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_STROBE_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_STROBE_NSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_STROBE_NSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STROBE_NSEC` reader - 1588 Timer Sync Strobe Nanoseconds. The value of the Timer Nanoseconds register captured when gem_tsu_ms and gem_tsu_inc_ctrl are zero."]
pub type STROBE_NSEC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - 1588 Timer Sync Strobe Nanoseconds. The value of the Timer Nanoseconds register captured when gem_tsu_ms and gem_tsu_inc_ctrl are zero."]
    #[inline(always)]
    pub fn strobe_nsec(&self) -> STROBE_NSEC_R {
        STROBE_NSEC_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "1588 Timer Sync Strobe Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_strobe_nsec](index.html) module"]
pub struct TSU_STROBE_NSEC_SPEC;
impl crate::RegisterSpec for TSU_STROBE_NSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_strobe_nsec::R](R) reader structure"]
impl crate::Readable for TSU_STROBE_NSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSU_STROBE_NSEC to value 0"]
impl crate::Resettable for TSU_STROBE_NSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
