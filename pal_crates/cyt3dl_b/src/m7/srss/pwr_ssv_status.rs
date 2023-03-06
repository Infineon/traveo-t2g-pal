#[doc = "Register `PWR_SSV_STATUS` reader"]
pub struct R(crate::R<PWR_SSV_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SSV_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SSV_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SSV_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BODVDDD_OK` reader - BOD indicates vddd is ok. This will always read 1, because a detected brownout will reset the chip."]
pub type BODVDDD_OK_R = crate::BitReader<bool>;
#[doc = "Field `BODVDDA_OK` reader - BOD indicates vdda is ok."]
pub type BODVDDA_OK_R = crate::BitReader<bool>;
#[doc = "Field `BODVCCD_OK` reader - BOD indicates vccd is ok. This will always read 1, because a detected brownout will reset the chip."]
pub type BODVCCD_OK_R = crate::BitReader<bool>;
#[doc = "Field `OVDVDDD_OK` reader - OVD indicates vddd is ok. This will always read 1, because a detected over-voltage condition will reset the chip."]
pub type OVDVDDD_OK_R = crate::BitReader<bool>;
#[doc = "Field `OVDVDDA_OK` reader - OVD indicates vdda is ok."]
pub type OVDVDDA_OK_R = crate::BitReader<bool>;
#[doc = "Field `OVDVCCD_OK` reader - OVD indicates vccd is ok. This will always read 1, because a detected over-over-voltage condition will reset the chip."]
pub type OVDVCCD_OK_R = crate::BitReader<bool>;
#[doc = "Field `OCD_ACT_LINREG_OK` reader - OCD indicates the current drawn from the linear Active Regulator is ok. This will always read 1, because a detected over-current condition will reset the chip."]
pub type OCD_ACT_LINREG_OK_R = crate::BitReader<bool>;
#[doc = "Field `OCD_DPSLP_REG_OK` reader - OCD indicates the current drawn from the linear DeepSleep Regulator is ok. This will always read 1, because a detected over-current condition will reset the chip."]
pub type OCD_DPSLP_REG_OK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - BOD indicates vddd is ok. This will always read 1, because a detected brownout will reset the chip."]
    #[inline(always)]
    pub fn bodvddd_ok(&self) -> BODVDDD_OK_R {
        BODVDDD_OK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD indicates vdda is ok."]
    #[inline(always)]
    pub fn bodvdda_ok(&self) -> BODVDDA_OK_R {
        BODVDDA_OK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD indicates vccd is ok. This will always read 1, because a detected brownout will reset the chip."]
    #[inline(always)]
    pub fn bodvccd_ok(&self) -> BODVCCD_OK_R {
        BODVCCD_OK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - OVD indicates vddd is ok. This will always read 1, because a detected over-voltage condition will reset the chip."]
    #[inline(always)]
    pub fn ovdvddd_ok(&self) -> OVDVDDD_OK_R {
        OVDVDDD_OK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OVD indicates vdda is ok."]
    #[inline(always)]
    pub fn ovdvdda_ok(&self) -> OVDVDDA_OK_R {
        OVDVDDA_OK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OVD indicates vccd is ok. This will always read 1, because a detected over-over-voltage condition will reset the chip."]
    #[inline(always)]
    pub fn ovdvccd_ok(&self) -> OVDVCCD_OK_R {
        OVDVCCD_OK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - OCD indicates the current drawn from the linear Active Regulator is ok. This will always read 1, because a detected over-current condition will reset the chip."]
    #[inline(always)]
    pub fn ocd_act_linreg_ok(&self) -> OCD_ACT_LINREG_OK_R {
        OCD_ACT_LINREG_OK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OCD indicates the current drawn from the linear DeepSleep Regulator is ok. This will always read 1, because a detected over-current condition will reset the chip."]
    #[inline(always)]
    pub fn ocd_dpslp_reg_ok(&self) -> OCD_DPSLP_REG_OK_R {
        OCD_DPSLP_REG_OK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Supply Supervision Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ssv_status](index.html) module"]
pub struct PWR_SSV_STATUS_SPEC;
impl crate::RegisterSpec for PWR_SSV_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ssv_status::R](R) reader structure"]
impl crate::Readable for PWR_SSV_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_SSV_STATUS to value 0x0003_0505"]
impl crate::Resettable for PWR_SSV_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0505;
}
