#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STSSIGERROR` reader - Error status bits for all evaluation windows (bit index = window index). When a bit is set it means that measured signature value is different from reference value (SigCRCRef). The bit is set when EnCRC is active and a certain number of frames have a violation (ErrThres). It is reset when the reference check for evaluation window is disabled (EnEvalWin or EnCRC) or in case a certain number of consecutive frames have no violation (ErrThresReset). The value corresponds to the panic status of this evaluation window."]
pub type STSSIGERROR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STSSIGVALID` reader - Measured signature values are valid. This flag is set when the SigCRC fields have been updated for all enabled evaluation windows. In that case also the corresponding interrupt event is triggered. It is reset when SW writes a '1' to the 'Kick' field in order to start a new measurement."]
pub type STSSIGVALID_R = crate::BitReader<bool>;
#[doc = "Field `STSSIGIDLE` reader - StsSigIdle = 1: Signature is in Idle state. StsSigIdle = 0: Signature runs."]
pub type STSSIGIDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Error status bits for all evaluation windows (bit index = window index). When a bit is set it means that measured signature value is different from reference value (SigCRCRef). The bit is set when EnCRC is active and a certain number of frames have a violation (ErrThres). It is reset when the reference check for evaluation window is disabled (EnEvalWin or EnCRC) or in case a certain number of consecutive frames have no violation (ErrThresReset). The value corresponds to the panic status of this evaluation window."]
    #[inline(always)]
    pub fn stssigerror(&self) -> STSSIGERROR_R {
        STSSIGERROR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Measured signature values are valid. This flag is set when the SigCRC fields have been updated for all enabled evaluation windows. In that case also the corresponding interrupt event is triggered. It is reset when SW writes a '1' to the 'Kick' field in order to start a new measurement."]
    #[inline(always)]
    pub fn stssigvalid(&self) -> STSSIGVALID_R {
        STSSIGVALID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - StsSigIdle = 1: Signature is in Idle state. StsSigIdle = 0: Signature runs."]
    #[inline(always)]
    pub fn stssigidle(&self) -> STSSIGIDLE_R {
        STSSIGIDLE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Module status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x0010_0000"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}
