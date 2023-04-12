#[doc = "Register `FCS_ERRORS` reader"]
pub struct R(crate::R<FCS_ERRORS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCS_ERRORS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCS_ERRORS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCS_ERRORS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_FCS_ERR` reader - Frame check sequence errors - a 10 bit register counting frames that are an integral number of bytes, have bad CRC and are between 64 and 1518 bytes in length (1536 if bit 8 set in network configuration register, 10,240 bytes if bit 3 is set in the network configuration register). This register is also incremented if a symbol error is detected and the frame is of valid length and has an integral number of bytes. This register is incremented for a frame with bad FCS, regardless of whether it is copied to memory due to ignore FCS mode being enabled in bit 26 of the network configuration register."]
pub type COUNT_FCS_ERR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Frame check sequence errors - a 10 bit register counting frames that are an integral number of bytes, have bad CRC and are between 64 and 1518 bytes in length (1536 if bit 8 set in network configuration register, 10,240 bytes if bit 3 is set in the network configuration register). This register is also incremented if a symbol error is detected and the frame is of valid length and has an integral number of bytes. This register is incremented for a frame with bad FCS, regardless of whether it is copied to memory due to ignore FCS mode being enabled in bit 26 of the network configuration register."]
    #[inline(always)]
    pub fn count_fcs_err(&self) -> COUNT_FCS_ERR_R {
        COUNT_FCS_ERR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Frame Check Sequence Errors\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcs_errors](index.html) module"]
pub struct FCS_ERRORS_SPEC;
impl crate::RegisterSpec for FCS_ERRORS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcs_errors::R](R) reader structure"]
impl crate::Readable for FCS_ERRORS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCS_ERRORS to value 0"]
impl crate::Resettable for FCS_ERRORS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
