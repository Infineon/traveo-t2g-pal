#[doc = "Register `RESULT` reader"]
pub struct R(crate::R<RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
pub type RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ABOVE_HI_MIR` reader - mirror bit of the corresponding ABOVE_HI bit"]
pub type ABOVE_HI_MIR_R = crate::BitReader<bool>;
#[doc = "Field `RANGE_INTR_MIR` reader - mirror bit of INTR.CH_RANGE bit"]
pub type RANGE_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `PULSE_INTR_MIR` reader - mirror bit of INTR.CH_PULSE bit"]
pub type PULSE_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `VALID_MIR` reader - mirror bit of the corresponding bit in RESULT_VALID register"]
pub type VALID_MIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 28 - mirror bit of the corresponding ABOVE_HI bit"]
    #[inline(always)]
    pub fn above_hi_mir(&self) -> ABOVE_HI_MIR_R {
        ABOVE_HI_MIR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - mirror bit of INTR.CH_RANGE bit"]
    #[inline(always)]
    pub fn range_intr_mir(&self) -> RANGE_INTR_MIR_R {
        RANGE_INTR_MIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - mirror bit of INTR.CH_PULSE bit"]
    #[inline(always)]
    pub fn pulse_intr_mir(&self) -> PULSE_INTR_MIR_R {
        PULSE_INTR_MIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - mirror bit of the corresponding bit in RESULT_VALID register"]
    #[inline(always)]
    pub fn valid_mir(&self) -> VALID_MIR_R {
        VALID_MIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Result data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](index.html) module"]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result::R](R) reader structure"]
impl crate::Readable for RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
