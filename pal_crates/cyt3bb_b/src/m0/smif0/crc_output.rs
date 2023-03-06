#[doc = "Register `CRC_OUTPUT` reader"]
pub struct R(crate::R<CRC_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRC_OUTPUT` reader - CRC engine output."]
pub type CRC_OUTPUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CRC engine output."]
    #[inline(always)]
    pub fn crc_output(&self) -> CRC_OUTPUT_R {
        CRC_OUTPUT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CRC output\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_output](index.html) module"]
pub struct CRC_OUTPUT_SPEC;
impl crate::RegisterSpec for CRC_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_output::R](R) reader structure"]
impl crate::Readable for CRC_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRC_OUTPUT to value 0xff"]
impl crate::Resettable for CRC_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
