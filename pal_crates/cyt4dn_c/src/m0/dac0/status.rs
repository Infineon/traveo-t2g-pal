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
#[doc = "Field `DAC_BUSY` reader - DAC Busy. '0' - DAC is idle. '1' - Indicates that FS_SEL is in the middle of synchronizing to clk_if domain. SW must not change FS_SEL until DAC_BUSY=0."]
pub type DAC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `FAST_RAMP_DONE` reader - Fast Ramp Done. '0' - Fast ramping is not complete. '1' - Fast ramp circuit is complete. The fast ramp circuit is deemed complete based on time specified in FAST_RAMP_COUNT_IN_MS. Timer starts counting if FAST_RAMP_DONE=0 and IF_CTL.DAC_EN=1."]
pub type FAST_RAMP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `COMP_RAMP_DONE` reader - Complete Ramp Done. '0' - Complete Ramp is not complete. '1' - Complete Ramp is complete. The ramping of the circuit is deemed complete based on time specified in COMP_RAMP_COUNT_IN_MS. Timer starts counting if FAST_RAMP_DONE=1 and COMP_RAMP_DONE=0 and IF_CTL.DAC_EN=1 This bit transition from '0'->'1' will cause HW to set TX_INTR.RAMP_COMPLETE=1. SW clearing TX_INTR.RAMP_COMPLETE=0 will not have effect of this status bit."]
pub type COMP_RAMP_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DAC Busy. '0' - DAC is idle. '1' - Indicates that FS_SEL is in the middle of synchronizing to clk_if domain. SW must not change FS_SEL until DAC_BUSY=0."]
    #[inline(always)]
    pub fn dac_busy(&self) -> DAC_BUSY_R {
        DAC_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Ramp Done. '0' - Fast ramping is not complete. '1' - Fast ramp circuit is complete. The fast ramp circuit is deemed complete based on time specified in FAST_RAMP_COUNT_IN_MS. Timer starts counting if FAST_RAMP_DONE=0 and IF_CTL.DAC_EN=1."]
    #[inline(always)]
    pub fn fast_ramp_done(&self) -> FAST_RAMP_DONE_R {
        FAST_RAMP_DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Complete Ramp Done. '0' - Complete Ramp is not complete. '1' - Complete Ramp is complete. The ramping of the circuit is deemed complete based on time specified in COMP_RAMP_COUNT_IN_MS. Timer starts counting if FAST_RAMP_DONE=1 and COMP_RAMP_DONE=0 and IF_CTL.DAC_EN=1 This bit transition from '0'->'1' will cause HW to set TX_INTR.RAMP_COMPLETE=1. SW clearing TX_INTR.RAMP_COMPLETE=0 will not have effect of this status bit."]
    #[inline(always)]
    pub fn comp_ramp_done(&self) -> COMP_RAMP_DONE_R {
        COMP_RAMP_DONE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
