#[doc = "Register `TMR_STATUS` reader"]
pub struct R(crate::R<TMR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TMR` reader - While MEAS is 1, this field reads 0. After the end of the measurement window (MEAS = 0), this field reads the count of the timer at the end of the measurement window, unless the timer is off (TMR_CTL.TMR = 0); in this case, this field reads 0. The duration of the measurement window is TMR_STATUS.TMR * 2**TMR_CTL.PSC clock periods of the selected AXI port."]
pub type TMR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MEAS` reader - This field indicates the measurement window. This field is set by writing 1 to TMR_CMD.START. It is cleared by a timer match (after TMR_STATUS.TMR * 2**TMR_CTL.PSC AXI clock cycles), or by writing 1 to TMR_CMD.STOP, at the end of the current prescaler clock period. It should be noted that the measurment takes place in a different clock domain than the MMIO clock domain, so MEAS is only a rough indication of the measurment window. The actual measurement window starts as soon as TMR_CMD.START has been synchronized to the AXI port clock domain (after MEAS goes to 1). MEAS goes to 0 after the end of the measurement window has been synchronized from the AXI port clock domain to the MMIO clock domain. Hence, MEAS is 1 for slighly longer than the real measurement domain. After writing TMR_CMD.START, MEAS can be polled immediately. When MEAS is read as 0, the timer and counter registers can be read."]
pub type MEAS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:27 - While MEAS is 1, this field reads 0. After the end of the measurement window (MEAS = 0), this field reads the count of the timer at the end of the measurement window, unless the timer is off (TMR_CTL.TMR = 0); in this case, this field reads 0. The duration of the measurement window is TMR_STATUS.TMR * 2**TMR_CTL.PSC clock periods of the selected AXI port."]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - This field indicates the measurement window. This field is set by writing 1 to TMR_CMD.START. It is cleared by a timer match (after TMR_STATUS.TMR * 2**TMR_CTL.PSC AXI clock cycles), or by writing 1 to TMR_CMD.STOP, at the end of the current prescaler clock period. It should be noted that the measurment takes place in a different clock domain than the MMIO clock domain, so MEAS is only a rough indication of the measurment window. The actual measurement window starts as soon as TMR_CMD.START has been synchronized to the AXI port clock domain (after MEAS goes to 1). MEAS goes to 0 after the end of the measurement window has been synchronized from the AXI port clock domain to the MMIO clock domain. Hence, MEAS is 1 for slighly longer than the real measurement domain. After writing TMR_CMD.START, MEAS can be polled immediately. When MEAS is read as 0, the timer and counter registers can be read."]
    #[inline(always)]
    pub fn meas(&self) -> MEAS_R {
        MEAS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Timer status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr_status](index.html) module"]
pub struct TMR_STATUS_SPEC;
impl crate::RegisterSpec for TMR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr_status::R](R) reader structure"]
impl crate::Readable for TMR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TMR_STATUS to value 0"]
impl crate::Resettable for TMR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
