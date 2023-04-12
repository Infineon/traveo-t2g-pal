#[doc = "Register `RCV` reader"]
pub struct R(crate::R<RCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCV` reader - Rate Correction Value (vRateCorrection) Rate correction value (two's complement). Calculated internal rate correction value before limitation. If the RCV value exceeds the limits defined by GTUC10.MRC\\[10:0\\], flag SFS.RCLR is set to '1'."]
pub type RCV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Rate Correction Value (vRateCorrection) Rate correction value (two's complement). Calculated internal rate correction value before limitation. If the RCV value exceeds the limits defined by GTUC10.MRC\\[10:0\\], flag SFS.RCLR is set to '1'."]
    #[inline(always)]
    pub fn rcv(&self) -> RCV_R {
        RCV_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Rate Correction Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcv](index.html) module"]
pub struct RCV_SPEC;
impl crate::RegisterSpec for RCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcv::R](R) reader structure"]
impl crate::Readable for RCV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCV to value 0"]
impl crate::Resettable for RCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
