#[doc = "Register `TSU_STROBE_SEC` reader"]
pub struct R(crate::R<TSU_STROBE_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_STROBE_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_STROBE_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_STROBE_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STROBE_SEC` reader - 1588 Timer Sync Strobe Seconds. The lowest significant 32-bit value of the Timer Seconds register captured when gem_tsu_ms and gem_tsu_inc_ctrl are zero."]
pub type STROBE_SEC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Sync Strobe Seconds. The lowest significant 32-bit value of the Timer Seconds register captured when gem_tsu_ms and gem_tsu_inc_ctrl are zero."]
    #[inline(always)]
    pub fn strobe_sec(&self) -> STROBE_SEC_R {
        STROBE_SEC_R::new(self.bits)
    }
}
#[doc = "1588 Timer Sync Strobe Seconds Register (31 to 0 bits)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_strobe_sec](index.html) module"]
pub struct TSU_STROBE_SEC_SPEC;
impl crate::RegisterSpec for TSU_STROBE_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_strobe_sec::R](R) reader structure"]
impl crate::Readable for TSU_STROBE_SEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSU_STROBE_SEC to value 0"]
impl crate::Resettable for TSU_STROBE_SEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
