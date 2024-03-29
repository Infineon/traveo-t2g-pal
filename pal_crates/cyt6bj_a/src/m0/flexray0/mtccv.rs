#[doc = "Register `MTCCV` reader"]
pub struct R(crate::R<MTCCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTCCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTCCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTCCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MTV` reader - Macrotick Value (vMacrotick) Current macrotick value. The value is incremented by the CC and reset at the start of a communication cycle. Valid values are 0 to 15999."]
pub type MTV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCV` reader - Cycle Counter Value (vCycleCounter) Current cycle counter value. The value is incremented by the CC at the start of a communication cycle. Valid values are 0 to 63."]
pub type CCV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:13 - Macrotick Value (vMacrotick) Current macrotick value. The value is incremented by the CC and reset at the start of a communication cycle. Valid values are 0 to 15999."]
    #[inline(always)]
    pub fn mtv(&self) -> MTV_R {
        MTV_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Cycle Counter Value (vCycleCounter) Current cycle counter value. The value is incremented by the CC at the start of a communication cycle. Valid values are 0 to 63."]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Macrotick and Cycle Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtccv](index.html) module"]
pub struct MTCCV_SPEC;
impl crate::RegisterSpec for MTCCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtccv::R](R) reader structure"]
impl crate::Readable for MTCCV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTCCV to value 0"]
impl crate::Resettable for MTCCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
