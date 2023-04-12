#[doc = "Register `DDRBISTSTT_CH0` reader"]
pub struct R(crate::R<DDRBISTSTT_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRBISTSTT_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRBISTSTT_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRBISTSTT_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERROR` reader - BIST error - Channel 0"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ENDTEST` reader - BIST endtest - Channel 0"]
pub type ENDTEST_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_NEW` reader - New BIST error in diagnosis mode - Channel 0"]
pub type ERROR_NEW_R = crate::BitReader<bool>;
#[doc = "Field `BANK_FAIL` reader - Failed bank address in diagnosis mode (Width = DRAM_BANK_WIDTH) - Channel 0"]
pub type BANK_FAIL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_FAIL` reader - Failed row address in diagnosis mode (Width = DRAM_ROW_WIDTH) - Channel 0"]
pub type ROW_FAIL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - BIST error - Channel 0"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BIST endtest - Channel 0"]
    #[inline(always)]
    pub fn endtest(&self) -> ENDTEST_R {
        ENDTEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - New BIST error in diagnosis mode - Channel 0"]
    #[inline(always)]
    pub fn error_new(&self) -> ERROR_NEW_R {
        ERROR_NEW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Failed bank address in diagnosis mode (Width = DRAM_BANK_WIDTH) - Channel 0"]
    #[inline(always)]
    pub fn bank_fail(&self) -> BANK_FAIL_R {
        BANK_FAIL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:20 - Failed row address in diagnosis mode (Width = DRAM_ROW_WIDTH) - Channel 0"]
    #[inline(always)]
    pub fn row_fail(&self) -> ROW_FAIL_R {
        ROW_FAIL_R::new(((self.bits >> 6) & 0x7fff) as u16)
    }
}
#[doc = "DDR BIST Status Register - Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrbiststt_ch0](index.html) module"]
pub struct DDRBISTSTT_CH0_SPEC;
impl crate::RegisterSpec for DDRBISTSTT_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrbiststt_ch0::R](R) reader structure"]
impl crate::Readable for DDRBISTSTT_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRBISTSTT_CH0 to value 0"]
impl crate::Resettable for DDRBISTSTT_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
