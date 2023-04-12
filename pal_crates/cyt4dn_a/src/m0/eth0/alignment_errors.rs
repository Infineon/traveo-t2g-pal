#[doc = "Register `ALIGNMENT_ERRORS` reader"]
pub struct R(crate::R<ALIGNMENT_ERRORS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALIGNMENT_ERRORS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALIGNMENT_ERRORS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALIGNMENT_ERRORS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_ALIGNMENT_ERROR` reader - Alignment errors - a 10 bit register counting frames that are not an integral number of bytes long and have bad CRC when their length is truncated to an integral number of bytes and are between 64 and 1518 bytes in length (1536 if bit 8 set in network configuration register, 10,240 bytes if bit 3 is set in the network configuration register). This register is also incremented if a symbol error is detected and the frame is of valid length and does not have an integral number of bytes."]
pub type COUNT_ALIGNMENT_ERROR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Alignment errors - a 10 bit register counting frames that are not an integral number of bytes long and have bad CRC when their length is truncated to an integral number of bytes and are between 64 and 1518 bytes in length (1536 if bit 8 set in network configuration register, 10,240 bytes if bit 3 is set in the network configuration register). This register is also incremented if a symbol error is detected and the frame is of valid length and does not have an integral number of bytes."]
    #[inline(always)]
    pub fn count_alignment_error(&self) -> COUNT_ALIGNMENT_ERROR_R {
        COUNT_ALIGNMENT_ERROR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Alignment Errors\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alignment_errors](index.html) module"]
pub struct ALIGNMENT_ERRORS_SPEC;
impl crate::RegisterSpec for ALIGNMENT_ERRORS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alignment_errors::R](R) reader structure"]
impl crate::Readable for ALIGNMENT_ERRORS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ALIGNMENT_ERRORS to value 0"]
impl crate::Resettable for ALIGNMENT_ERRORS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
