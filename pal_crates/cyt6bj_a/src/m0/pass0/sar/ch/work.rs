#[doc = "Register `WORK` reader"]
pub struct R(crate::R<WORK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WORK` reader - SAR conversion working data of the channel. The data is written here right after sampling this channel."]
pub type WORK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ABOVE_HI_MIR` reader - mirror bit of the corresponding ABOVE_HI bit"]
pub type ABOVE_HI_MIR_R = crate::BitReader<bool>;
#[doc = "Field `RANGE_MIR` reader - mirror bit of corresponding bit in WORK_RANGE register"]
pub type RANGE_MIR_R = crate::BitReader<bool>;
#[doc = "Field `PULSE_MIR` reader - mirror bit of corresponding bit in WORK_PULSE register"]
pub type PULSE_MIR_R = crate::BitReader<bool>;
#[doc = "Field `VALID_MIR` reader - mirror bit of corresponding bit in WORK_VALID register"]
pub type VALID_MIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion working data of the channel. The data is written here right after sampling this channel."]
    #[inline(always)]
    pub fn work(&self) -> WORK_R {
        WORK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 28 - mirror bit of the corresponding ABOVE_HI bit"]
    #[inline(always)]
    pub fn above_hi_mir(&self) -> ABOVE_HI_MIR_R {
        ABOVE_HI_MIR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - mirror bit of corresponding bit in WORK_RANGE register"]
    #[inline(always)]
    pub fn range_mir(&self) -> RANGE_MIR_R {
        RANGE_MIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - mirror bit of corresponding bit in WORK_PULSE register"]
    #[inline(always)]
    pub fn pulse_mir(&self) -> PULSE_MIR_R {
        PULSE_MIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in WORK_VALID register"]
    #[inline(always)]
    pub fn valid_mir(&self) -> VALID_MIR_R {
        VALID_MIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Working data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [work](index.html) module"]
pub struct WORK_SPEC;
impl crate::RegisterSpec for WORK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [work::R](R) reader structure"]
impl crate::Readable for WORK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WORK to value 0"]
impl crate::Resettable for WORK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
