#[doc = "Register `DVSTT1` reader"]
pub struct R(crate::R<DVSTT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVSTT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVSTT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVSTT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DVSTT1_DRAM_BL_ENC` reader - N/A"]
pub type DVSTT1_DRAM_BL_ENC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DVSTT1_DFI_FREQ_RATIO` reader - DFI frequency ratio. FREQ_RATIO_DIV4 = 2'b10 Always read 2'b10 as frequency ratio controller : PHY is fixed to 1:4"]
pub type DVSTT1_DFI_FREQ_RATIO_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn dvstt1_dram_bl_enc(&self) -> DVSTT1_DRAM_BL_ENC_R {
        DVSTT1_DRAM_BL_ENC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DFI frequency ratio. FREQ_RATIO_DIV4 = 2'b10 Always read 2'b10 as frequency ratio controller : PHY is fixed to 1:4"]
    #[inline(always)]
    pub fn dvstt1_dfi_freq_ratio(&self) -> DVSTT1_DFI_FREQ_RATIO_R {
        DVSTT1_DFI_FREQ_RATIO_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[doc = "Device Mode Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvstt1](index.html) module"]
pub struct DVSTT1_SPEC;
impl crate::RegisterSpec for DVSTT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvstt1::R](R) reader structure"]
impl crate::Readable for DVSTT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DVSTT1 to value 0"]
impl crate::Resettable for DVSTT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
