#[doc = "Register `PCSR_CH0` reader"]
pub struct R(crate::R<PCSR_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSR_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSR_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSR_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRSTC` reader - Compensation soft-restart completed - Channel 0"]
pub type SRSTC_R = crate::BitReader<bool>;
#[doc = "Field `UPDC` reader - Indicates if settings of the compensation were updated - Channel 0"]
pub type UPDC_R = crate::BitReader<bool>;
#[doc = "Field `NBC` reader - Impedance pull-down settings observation bits - Channel 0"]
pub type NBC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBC` reader - Impedance pull-up settings observation bits - Channel 0"]
pub type PBC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Compensation soft-restart completed - Channel 0"]
    #[inline(always)]
    pub fn srstc(&self) -> SRSTC_R {
        SRSTC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if settings of the compensation were updated - Channel 0"]
    #[inline(always)]
    pub fn updc(&self) -> UPDC_R {
        UPDC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Impedance pull-down settings observation bits - Channel 0"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Impedance pull-up settings observation bits - Channel 0"]
    #[inline(always)]
    pub fn pbc(&self) -> PBC_R {
        PBC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[doc = "PHY Compensation Status Register - Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsr_ch0](index.html) module"]
pub struct PCSR_CH0_SPEC;
impl crate::RegisterSpec for PCSR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsr_ch0::R](R) reader structure"]
impl crate::Readable for PCSR_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCSR_CH0 to value 0"]
impl crate::Resettable for PCSR_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
