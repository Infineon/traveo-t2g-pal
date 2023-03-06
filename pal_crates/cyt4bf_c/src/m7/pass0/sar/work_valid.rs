#[doc = "Register `WORK_VALID` reader"]
pub struct R(crate::R<WORK_VALID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORK_VALID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORK_VALID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORK_VALID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WORK_VALID` reader - If set the corresponding WORK register is valid, i.e. was already acquired during the current group scan. If this bit is low then either the channel is not enabled, not yet acquired or it is used as a pulse detect channel."]
pub type WORK_VALID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - If set the corresponding WORK register is valid, i.e. was already acquired during the current group scan. If this bit is low then either the channel is not enabled, not yet acquired or it is used as a pulse detect channel."]
    #[inline(always)]
    pub fn work_valid(&self) -> WORK_VALID_R {
        WORK_VALID_R::new(self.bits)
    }
}
#[doc = "Channel working data register 'valid' bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [work_valid](index.html) module"]
pub struct WORK_VALID_SPEC;
impl crate::RegisterSpec for WORK_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [work_valid::R](R) reader structure"]
impl crate::Readable for WORK_VALID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WORK_VALID to value 0"]
impl crate::Resettable for WORK_VALID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
