#[doc = "Register `TTCPT` reader"]
pub struct R(crate::R<TTCPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTCPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTCPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTCPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCV` reader - Cycle Count Value Cycle count value captured together with SWV. 0x00-3F Captured cycle count value"]
pub type CCV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWV` reader - Stop Watch Value On a rising/falling edge (as configured via TTOCN.SWP) at the Stop Watch Trigger pin m_ttcan_swt, when TTOCN.SWS is != '00' and TTIR.SWE is '0', the actual time value as selected by TTOCN.SWS (cycle, local, global) is copied to SWV and TTIR.SWE will be set to '1'. Capturing of the next stop watch value is enabled by resetting TTIR.SWE. 0x0000-FFFF Captured Stop Watch value"]
pub type SWV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:5 - Cycle Count Value Cycle count value captured together with SWV. 0x00-3F Captured cycle count value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Stop Watch Value On a rising/falling edge (as configured via TTOCN.SWP) at the Stop Watch Trigger pin m_ttcan_swt, when TTOCN.SWS is != '00' and TTIR.SWE is '0', the actual time value as selected by TTOCN.SWS (cycle, local, global) is copied to SWV and TTIR.SWE will be set to '1'. Capturing of the next stop watch value is enabled by resetting TTIR.SWE. 0x0000-FFFF Captured Stop Watch value"]
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "TT Capture Time\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttcpt](index.html) module"]
pub struct TTCPT_SPEC;
impl crate::RegisterSpec for TTCPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttcpt::R](R) reader structure"]
impl crate::Readable for TTCPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTCPT to value 0"]
impl crate::Resettable for TTCPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
