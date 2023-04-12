#[doc = "Register `PWR_REGHC_STATUS` reader"]
pub struct R(crate::R<PWR_REGHC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_REGHC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_REGHC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_REGHC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REGHC_ENABLED` reader - Indicates the state of the REGHC enable/disable sequencer. This bit is only valid when REGHC_SEQ_BUSY==0."]
pub type REGHC_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_OCD_OK` reader - Indicates the over-current detector is operating and the current drawn from REGHC is within limits. OCD is only a choice for transistor mode, and it is disabled for PMIC mode. 0: Current measurement exceeds limit or detector is OFF, 1: Current measurement within limit"]
pub type REGHC_OCD_OK_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_CKT_OK` reader - Indicates the REGHC circuit is enabled and operating. It does not indicate that the voltage and current are within required limits for robust operation."]
pub type REGHC_CKT_OK_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_UV_OUT` reader - N/A"]
pub type REGHC_UV_OUT_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_OV_OUT` reader - N/A"]
pub type REGHC_OV_OUT_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_STATUS_OK` reader - Indicates the PMIC status is ok. This includes polarity adjustment according to REGHC_PMIC_STATUS_POLARITY. 0: PMIC status is not ok or PMIC input buffer is disabled (REGHC_PMIC_STATUS_INEN==0); 1: PMIC status input buffer is enabled and indicates ok"]
pub type REGHC_PMIC_STATUS_OK_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_SEQ_BUSY` reader - Indicates the REGHC enable/disable sequencer is busy transitioning to/from REGHC."]
pub type REGHC_SEQ_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates the state of the REGHC enable/disable sequencer. This bit is only valid when REGHC_SEQ_BUSY==0."]
    #[inline(always)]
    pub fn reghc_enabled(&self) -> REGHC_ENABLED_R {
        REGHC_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the over-current detector is operating and the current drawn from REGHC is within limits. OCD is only a choice for transistor mode, and it is disabled for PMIC mode. 0: Current measurement exceeds limit or detector is OFF, 1: Current measurement within limit"]
    #[inline(always)]
    pub fn reghc_ocd_ok(&self) -> REGHC_OCD_OK_R {
        REGHC_OCD_OK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the REGHC circuit is enabled and operating. It does not indicate that the voltage and current are within required limits for robust operation."]
    #[inline(always)]
    pub fn reghc_ckt_ok(&self) -> REGHC_CKT_OK_R {
        REGHC_CKT_OK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn reghc_uv_out(&self) -> REGHC_UV_OUT_R {
        REGHC_UV_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn reghc_ov_out(&self) -> REGHC_OV_OUT_R {
        REGHC_OV_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates the PMIC status is ok. This includes polarity adjustment according to REGHC_PMIC_STATUS_POLARITY. 0: PMIC status is not ok or PMIC input buffer is disabled (REGHC_PMIC_STATUS_INEN==0); 1: PMIC status input buffer is enabled and indicates ok"]
    #[inline(always)]
    pub fn reghc_pmic_status_ok(&self) -> REGHC_PMIC_STATUS_OK_R {
        REGHC_PMIC_STATUS_OK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates the REGHC enable/disable sequencer is busy transitioning to/from REGHC."]
    #[inline(always)]
    pub fn reghc_seq_busy(&self) -> REGHC_SEQ_BUSY_R {
        REGHC_SEQ_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "REGHC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_reghc_status](index.html) module"]
pub struct PWR_REGHC_STATUS_SPEC;
impl crate::RegisterSpec for PWR_REGHC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_reghc_status::R](R) reader structure"]
impl crate::Readable for PWR_REGHC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_REGHC_STATUS to value 0"]
impl crate::Resettable for PWR_REGHC_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
