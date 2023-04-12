#[doc = "Register `CAL_STAT` reader"]
pub struct R(crate::R<CAL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALOUT` reader - Calibrator Outputs which internally control the HS-RX resistor calibration. Exposed as outputs for monitoring purposes. 2'b00: Expected Calibrator output for fast resistor process. 2'b01: Expected Calibrator output for typical resistor process. 2'b10: Expected Calibrator output for slow resistor process. 2'b11: Expected Calibrator output for slow resistor process. When read in case CTL.ENABLED = 0, a bus error is returned."]
pub type CALOUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Calibrator Outputs which internally control the HS-RX resistor calibration. Exposed as outputs for monitoring purposes. 2'b00: Expected Calibrator output for fast resistor process. 2'b01: Expected Calibrator output for typical resistor process. 2'b10: Expected Calibrator output for slow resistor process. 2'b11: Expected Calibrator output for slow resistor process. When read in case CTL.ENABLED = 0, a bus error is returned."]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Calibration Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_stat](index.html) module"]
pub struct CAL_STAT_SPEC;
impl crate::RegisterSpec for CAL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_stat::R](R) reader structure"]
impl crate::Readable for CAL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAL_STAT to value 0"]
impl crate::Resettable for CAL_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
