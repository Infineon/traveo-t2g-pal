#[doc = "Register `PAUSE_TIME` reader"]
pub struct R(crate::R<PAUSE_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUSE_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUSE_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUSE_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUANTUM` reader - Received pause quantum - stores the current value of the received pause quantum register which is decremented every 512 bit times."]
pub type QUANTUM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received pause quantum - stores the current value of the received pause quantum register which is decremented every 512 bit times."]
    #[inline(always)]
    pub fn quantum(&self) -> QUANTUM_R {
        QUANTUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pause_time](index.html) module"]
pub struct PAUSE_TIME_SPEC;
impl crate::RegisterSpec for PAUSE_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pause_time::R](R) reader structure"]
impl crate::Readable for PAUSE_TIME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PAUSE_TIME to value 0"]
impl crate::Resettable for PAUSE_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
