#[doc = "Register `PERFMEASUREMENTIDLECOUNTER` reader"]
pub struct R(crate::R<PERFMEASUREMENTIDLECOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFMEASUREMENTIDLECOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFMEASUREMENTIDLECOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFMEASUREMENTIDLECOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDLECOUNTER` reader - Idle counter is incremented with each clock cycle when blitEng is in idle state. When ClockCounter has overrun or MeasurementSave has been triggered idle counter is written to this register."]
pub type IDLECOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Idle counter is incremented with each clock cycle when blitEng is in idle state. When ClockCounter has overrun or MeasurementSave has been triggered idle counter is written to this register."]
    #[inline(always)]
    pub fn idlecounter(&self) -> IDLECOUNTER_R {
        IDLECOUNTER_R::new(self.bits)
    }
}
#[doc = "Performance measurement register for BlitEng containing idle cycles. Measurement is active only if PerfMeasurementEn is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfmeasurementidlecounter](index.html) module"]
pub struct PERFMEASUREMENTIDLECOUNTER_SPEC;
impl crate::RegisterSpec for PERFMEASUREMENTIDLECOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfmeasurementidlecounter::R](R) reader structure"]
impl crate::Readable for PERFMEASUREMENTIDLECOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERFMEASUREMENTIDLECOUNTER to value 0"]
impl crate::Resettable for PERFMEASUREMENTIDLECOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
