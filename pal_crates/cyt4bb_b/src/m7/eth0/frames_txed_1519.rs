#[doc = "Register `FRAMES_TXED_1519` reader"]
pub struct R(crate::R<FRAMES_TXED_1519_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMES_TXED_1519_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMES_TXED_1519_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMES_TXED_1519_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_1519` reader - Greater than 1518 byte frames transmitted without error. A 32 bit register counting the number of 1518 or above byte frames successfully transmitted without error, i.e. no under run and not too many retries."]
pub type COUNT_1519_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Greater than 1518 byte frames transmitted without error. A 32 bit register counting the number of 1518 or above byte frames successfully transmitted without error, i.e. no under run and not too many retries."]
    #[inline(always)]
    pub fn count_1519(&self) -> COUNT_1519_R {
        COUNT_1519_R::new(self.bits)
    }
}
#[doc = "Greater Than 1518 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frames_txed_1519](index.html) module"]
pub struct FRAMES_TXED_1519_SPEC;
impl crate::RegisterSpec for FRAMES_TXED_1519_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frames_txed_1519::R](R) reader structure"]
impl crate::Readable for FRAMES_TXED_1519_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAMES_TXED_1519 to value 0"]
impl crate::Resettable for FRAMES_TXED_1519_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
