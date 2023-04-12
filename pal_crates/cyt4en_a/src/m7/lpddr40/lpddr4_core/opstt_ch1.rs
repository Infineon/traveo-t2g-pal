#[doc = "Register `OPSTT_CH1` reader"]
pub struct R(crate::R<OPSTT_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPSTT_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPSTT_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPSTT_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DRAM_PAUSE` reader - DRAM Paused for PD - Channel 1"]
pub type DRAM_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `USER_CMD_READY` reader - User Command Ready - Channel 1"]
pub type USER_CMD_READY_R = crate::BitReader<bool>;
#[doc = "Field `BANK_IDLE` reader - Bank Controller Idle (8 banks) - Channel 1"]
pub type BANK_IDLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XQR_EMPTY` reader - Read Path Cross-over Queue Empty - Channel 1"]
pub type XQR_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `XQR_FULL` reader - Read Path Cross-over Queue Full - Channel 1"]
pub type XQR_FULL_R = crate::BitReader<bool>;
#[doc = "Field `XQW_EMPTY` reader - Write Path Cross-over Queue Empty - Channel 1"]
pub type XQW_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `XQW_FULL` reader - Write Path Cross-over Queue Full - Channel 1"]
pub type XQW_FULL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DRAM Paused for PD - Channel 1"]
    #[inline(always)]
    pub fn dram_pause(&self) -> DRAM_PAUSE_R {
        DRAM_PAUSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User Command Ready - Channel 1"]
    #[inline(always)]
    pub fn user_cmd_ready(&self) -> USER_CMD_READY_R {
        USER_CMD_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Bank Controller Idle (8 banks) - Channel 1"]
    #[inline(always)]
    pub fn bank_idle(&self) -> BANK_IDLE_R {
        BANK_IDLE_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - Read Path Cross-over Queue Empty - Channel 1"]
    #[inline(always)]
    pub fn xqr_empty(&self) -> XQR_EMPTY_R {
        XQR_EMPTY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read Path Cross-over Queue Full - Channel 1"]
    #[inline(always)]
    pub fn xqr_full(&self) -> XQR_FULL_R {
        XQR_FULL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write Path Cross-over Queue Empty - Channel 1"]
    #[inline(always)]
    pub fn xqw_empty(&self) -> XQW_EMPTY_R {
        XQW_EMPTY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Path Cross-over Queue Full - Channel 1"]
    #[inline(always)]
    pub fn xqw_full(&self) -> XQW_FULL_R {
        XQW_FULL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Operation Status Register 1 - Channel 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opstt_ch1](index.html) module"]
pub struct OPSTT_CH1_SPEC;
impl crate::RegisterSpec for OPSTT_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opstt_ch1::R](R) reader structure"]
impl crate::Readable for OPSTT_CH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OPSTT_CH1 to value 0x17ff"]
impl crate::Resettable for OPSTT_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0x17ff;
}
