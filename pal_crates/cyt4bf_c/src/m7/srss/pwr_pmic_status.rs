#[doc = "Register `PWR_PMIC_STATUS` reader"]
pub struct R(crate::R<PWR_PMIC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_PMIC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_PMIC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_PMIC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PMIC_ENABLED` reader - Indicates the state of the PMIC enable/disable sequencer. This bit is only valid when PMIC_SEQ_BUSY==0."]
pub type PMIC_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_STATUS_OK` reader - Indicates the PMIC status is ok. This includes polarity adjustment according to PMIC_STATUS_POLARITY. 0: PMIC status is not ok or PMIC input buffer is disabled (PMIC_STATUS_INEN==0); 1: PMIC status input buffer is enabled and indicates ok"]
pub type PMIC_STATUS_OK_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_SEQ_BUSY` reader - Indicates the PMIC enable/disable sequencer is busy transitioning to/from PMIC."]
pub type PMIC_SEQ_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates the state of the PMIC enable/disable sequencer. This bit is only valid when PMIC_SEQ_BUSY==0."]
    #[inline(always)]
    pub fn pmic_enabled(&self) -> PMIC_ENABLED_R {
        PMIC_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates the PMIC status is ok. This includes polarity adjustment according to PMIC_STATUS_POLARITY. 0: PMIC status is not ok or PMIC input buffer is disabled (PMIC_STATUS_INEN==0); 1: PMIC status input buffer is enabled and indicates ok"]
    #[inline(always)]
    pub fn pmic_status_ok(&self) -> PMIC_STATUS_OK_R {
        PMIC_STATUS_OK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates the PMIC enable/disable sequencer is busy transitioning to/from PMIC."]
    #[inline(always)]
    pub fn pmic_seq_busy(&self) -> PMIC_SEQ_BUSY_R {
        PMIC_SEQ_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "PMIC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_pmic_status](index.html) module"]
pub struct PWR_PMIC_STATUS_SPEC;
impl crate::RegisterSpec for PWR_PMIC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_pmic_status::R](R) reader structure"]
impl crate::Readable for PWR_PMIC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_PMIC_STATUS to value 0"]
impl crate::Resettable for PWR_PMIC_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
