#[doc = "Register `PERFMEASUREMENTCLOCKCOUNTER` reader"]
pub struct R(crate::R<PERFMEASUREMENTCLOCKCOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFMEASUREMENTCLOCKCOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFMEASUREMENTCLOCKCOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFMEASUREMENTCLOCKCOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLOCKCOUNTER` reader - When ClockCounterRunning has overrun or MeasurementSave has been triggered the value of clock counter is written to this register."]
pub type CLOCKCOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - When ClockCounterRunning has overrun or MeasurementSave has been triggered the value of clock counter is written to this register."]
    #[inline(always)]
    pub fn clockcounter(&self) -> CLOCKCOUNTER_R {
        CLOCKCOUNTER_R::new(self.bits)
    }
}
#[doc = "Performance measurement register for BlitEng containing clock cycle counter. Measurement is active only if PerfMeasurementEn is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfmeasurementclockcounter](index.html) module"]
pub struct PERFMEASUREMENTCLOCKCOUNTER_SPEC;
impl crate::RegisterSpec for PERFMEASUREMENTCLOCKCOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfmeasurementclockcounter::R](R) reader structure"]
impl crate::Readable for PERFMEASUREMENTCLOCKCOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERFMEASUREMENTCLOCKCOUNTER to value 0"]
impl crate::Resettable for PERFMEASUREMENTCLOCKCOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
