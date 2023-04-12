#[doc = "Register `WORK_PULSE` reader"]
pub struct R(crate::R<WORK_PULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORK_PULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORK_PULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORK_PULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PULSE` reader - N/A"]
pub type PULSE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn pulse(&self) -> PULSE_R {
        PULSE_R::new(self.bits)
    }
}
#[doc = "Pulse detected\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [work_pulse](index.html) module"]
pub struct WORK_PULSE_SPEC;
impl crate::RegisterSpec for WORK_PULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [work_pulse::R](R) reader structure"]
impl crate::Readable for WORK_PULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WORK_PULSE to value 0"]
impl crate::Resettable for WORK_PULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
