#[doc = "Register `RESULT_VALID` reader"]
pub struct R(crate::R<RESULT_VALID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_VALID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_VALID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_VALID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT_VALID` reader - If set the corresponding RESULT register is valid, i.e. was acquired during the preceding group scan. If this bit is low, after a group scan completed, then either the channel is not enabled or is used as a pulse detect channel."]
pub type RESULT_VALID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - If set the corresponding RESULT register is valid, i.e. was acquired during the preceding group scan. If this bit is low, after a group scan completed, then either the channel is not enabled or is used as a pulse detect channel."]
    #[inline(always)]
    pub fn result_valid(&self) -> RESULT_VALID_R {
        RESULT_VALID_R::new(self.bits)
    }
}
#[doc = "Channel result data register 'valid' bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result_valid](index.html) module"]
pub struct RESULT_VALID_SPEC;
impl crate::RegisterSpec for RESULT_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result_valid::R](R) reader structure"]
impl crate::Readable for RESULT_VALID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT_VALID to value 0"]
impl crate::Resettable for RESULT_VALID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
