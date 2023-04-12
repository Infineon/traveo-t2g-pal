#[doc = "Register `ERRORCODE` reader"]
pub struct R(crate::R<ERRORCODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRORCODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRORCODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRORCODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERRORCODE` reader - This field indicates the cause for the error interrupt for markers (INTR_DEC.ERRORMARKER)."]
pub type ERRORCODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - This field indicates the cause for the error interrupt for markers (INTR_DEC.ERRORMARKER)."]
    #[inline(always)]
    pub fn errorcode(&self) -> ERRORCODE_R {
        ERRORCODE_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Error code for INTR_DEC.ErrorMarker.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorcode](index.html) module"]
pub struct ERRORCODE_SPEC;
impl crate::RegisterSpec for ERRORCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errorcode::R](R) reader structure"]
impl crate::Readable for ERRORCODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERRORCODE to value 0"]
impl crate::Resettable for ERRORCODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
