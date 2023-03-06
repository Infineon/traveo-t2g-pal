#[doc = "Register `SFS` reader"]
pub struct R(crate::R<SFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VSAE` reader - Valid Sync Frames Channel A, even communication cycle Holds the number of valid sync frames received on channel A in the even communication cycle. If transmission of sync frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the NIT of each even communication cycle."]
pub type VSAE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSAO` reader - Valid Sync Frames Channel A, odd communication cycle Holds the number of valid sync frames received on channel A in the odd communication cycle. If transmission of sync frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the NIT of each odd communication cycle."]
pub type VSAO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSBE` reader - Valid Sync Frames Channel B, even communication cycle Holds the number of valid sync frames received on channel B in the even communication cycle. If transmission of sync frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the NIT of each even communication cycle."]
pub type VSBE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSBO` reader - Valid Sync Frames Channel B, odd communication cycle Holds the number of valid sync frames received on channel B in the odd communication cycle. If transmission of sync frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the NIT of each odd communication cycle."]
pub type VSBO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOCS` reader - Missing Offset Correction Signal The Missing Offset Correction flag signals to the Host, that no offset correction calculation can be performed because no sync frames were received. The flag is updated by the CC at start of offset correction phase. 1 = Missing offset correction signal 0 = Offset correction signal valid"]
pub type MOCS_R = crate::BitReader<MOCS_A>;
#[doc = "Missing Offset Correction Signal The Missing Offset Correction flag signals to the Host, that no offset correction calculation can be performed because no sync frames were received. The flag is updated by the CC at start of offset correction phase. 1 = Missing offset correction signal 0 = Offset correction signal valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOCS_A {
    #[doc = "0: N/A"]
    OFFSET_CORRECTION_SIGNAL_VALID = 0,
    #[doc = "1: N/A"]
    OFFSET_CORRECTION_SIGNAL_INVALID = 1,
}
impl From<MOCS_A> for bool {
    #[inline(always)]
    fn from(variant: MOCS_A) -> Self {
        variant as u8 != 0
    }
}
impl MOCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOCS_A {
        match self.bits {
            false => MOCS_A::OFFSET_CORRECTION_SIGNAL_VALID,
            true => MOCS_A::OFFSET_CORRECTION_SIGNAL_INVALID,
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET_CORRECTION_SIGNAL_VALID`"]
    #[inline(always)]
    pub fn is_offset_correction_signal_valid(&self) -> bool {
        *self == MOCS_A::OFFSET_CORRECTION_SIGNAL_VALID
    }
    #[doc = "Checks if the value of the field is `OFFSET_CORRECTION_SIGNAL_INVALID`"]
    #[inline(always)]
    pub fn is_offset_correction_signal_invalid(&self) -> bool {
        *self == MOCS_A::OFFSET_CORRECTION_SIGNAL_INVALID
    }
}
#[doc = "Field `OCLR` reader - Offset Correction Limit Reached The Offset Correction Limit Reached flag signals to the Host, that the offset correction value has exceeded its limit as defined by GTUC10.MOC\\[13:0\\]. The flag is updated by the CC at start of offset correction phase. 1 = Offset correction limit reached 0 = Offset correction below limit"]
pub type OCLR_R = crate::BitReader<OCLR_A>;
#[doc = "Offset Correction Limit Reached The Offset Correction Limit Reached flag signals to the Host, that the offset correction value has exceeded its limit as defined by GTUC10.MOC\\[13:0\\]. The flag is updated by the CC at start of offset correction phase. 1 = Offset correction limit reached 0 = Offset correction below limit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCLR_A {
    #[doc = "0: N/A"]
    OFFSET_CORRECTION_BELOW_LIMIT = 0,
    #[doc = "1: N/A"]
    OFFSET_CORRECTION_LIMIT_REACHED = 1,
}
impl From<OCLR_A> for bool {
    #[inline(always)]
    fn from(variant: OCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl OCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCLR_A {
        match self.bits {
            false => OCLR_A::OFFSET_CORRECTION_BELOW_LIMIT,
            true => OCLR_A::OFFSET_CORRECTION_LIMIT_REACHED,
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET_CORRECTION_BELOW_LIMIT`"]
    #[inline(always)]
    pub fn is_offset_correction_below_limit(&self) -> bool {
        *self == OCLR_A::OFFSET_CORRECTION_BELOW_LIMIT
    }
    #[doc = "Checks if the value of the field is `OFFSET_CORRECTION_LIMIT_REACHED`"]
    #[inline(always)]
    pub fn is_offset_correction_limit_reached(&self) -> bool {
        *self == OCLR_A::OFFSET_CORRECTION_LIMIT_REACHED
    }
}
#[doc = "Field `MRCS` reader - Missing Rate Correction Signal The Missing Rate Correction flag signals to the Host, that no rate correction calculation can be performed because no pairs of even / odd sync frames were received. The flag is updated by the CC at start of offset correction phase. 1 = Missing rate correction signal 0 = Rate correction signal valid"]
pub type MRCS_R = crate::BitReader<MRCS_A>;
#[doc = "Missing Rate Correction Signal The Missing Rate Correction flag signals to the Host, that no rate correction calculation can be performed because no pairs of even / odd sync frames were received. The flag is updated by the CC at start of offset correction phase. 1 = Missing rate correction signal 0 = Rate correction signal valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRCS_A {
    #[doc = "0: N/A"]
    RATE_CORRECTION_SIGNAL_VALID = 0,
    #[doc = "1: N/A"]
    RATE_CORRECTION_SIGNAL_INVALID = 1,
}
impl From<MRCS_A> for bool {
    #[inline(always)]
    fn from(variant: MRCS_A) -> Self {
        variant as u8 != 0
    }
}
impl MRCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRCS_A {
        match self.bits {
            false => MRCS_A::RATE_CORRECTION_SIGNAL_VALID,
            true => MRCS_A::RATE_CORRECTION_SIGNAL_INVALID,
        }
    }
    #[doc = "Checks if the value of the field is `RATE_CORRECTION_SIGNAL_VALID`"]
    #[inline(always)]
    pub fn is_rate_correction_signal_valid(&self) -> bool {
        *self == MRCS_A::RATE_CORRECTION_SIGNAL_VALID
    }
    #[doc = "Checks if the value of the field is `RATE_CORRECTION_SIGNAL_INVALID`"]
    #[inline(always)]
    pub fn is_rate_correction_signal_invalid(&self) -> bool {
        *self == MRCS_A::RATE_CORRECTION_SIGNAL_INVALID
    }
}
#[doc = "Field `RCLR` reader - Rate Correction Limit Reached The Rate Correction Limit Reached flag signals to the Host, that the rate correction value has exceeded its limit as defined by GTUC10.MRC\\[10:0\\]. The flag is updated by the CC at start of offset correction phase. 1 = Rate correction limit reached 0 = Rate correction below limit"]
pub type RCLR_R = crate::BitReader<RCLR_A>;
#[doc = "Rate Correction Limit Reached The Rate Correction Limit Reached flag signals to the Host, that the rate correction value has exceeded its limit as defined by GTUC10.MRC\\[10:0\\]. The flag is updated by the CC at start of offset correction phase. 1 = Rate correction limit reached 0 = Rate correction below limit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCLR_A {
    #[doc = "0: N/A"]
    RATE_CORRECTION_BELOW_LIMIT = 0,
    #[doc = "1: N/A"]
    RATE_CORRECTION_LIMIT_REACHED = 1,
}
impl From<RCLR_A> for bool {
    #[inline(always)]
    fn from(variant: RCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl RCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCLR_A {
        match self.bits {
            false => RCLR_A::RATE_CORRECTION_BELOW_LIMIT,
            true => RCLR_A::RATE_CORRECTION_LIMIT_REACHED,
        }
    }
    #[doc = "Checks if the value of the field is `RATE_CORRECTION_BELOW_LIMIT`"]
    #[inline(always)]
    pub fn is_rate_correction_below_limit(&self) -> bool {
        *self == RCLR_A::RATE_CORRECTION_BELOW_LIMIT
    }
    #[doc = "Checks if the value of the field is `RATE_CORRECTION_LIMIT_REACHED`"]
    #[inline(always)]
    pub fn is_rate_correction_limit_reached(&self) -> bool {
        *self == RCLR_A::RATE_CORRECTION_LIMIT_REACHED
    }
}
impl R {
    #[doc = "Bits 0:3 - Valid Sync Frames Channel A, even communication cycle Holds the number of valid sync frames received on channel A in the even communication cycle. If transmission of sync frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the NIT of each even communication cycle."]
    #[inline(always)]
    pub fn vsae(&self) -> VSAE_R {
        VSAE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Valid Sync Frames Channel A, odd communication cycle Holds the number of valid sync frames received on channel A in the odd communication cycle. If transmission of sync frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the NIT of each odd communication cycle."]
    #[inline(always)]
    pub fn vsao(&self) -> VSAO_R {
        VSAO_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Valid Sync Frames Channel B, even communication cycle Holds the number of valid sync frames received on channel B in the even communication cycle. If transmission of sync frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the NIT of each even communication cycle."]
    #[inline(always)]
    pub fn vsbe(&self) -> VSBE_R {
        VSBE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Valid Sync Frames Channel B, odd communication cycle Holds the number of valid sync frames received on channel B in the odd communication cycle. If transmission of sync frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the NIT of each odd communication cycle."]
    #[inline(always)]
    pub fn vsbo(&self) -> VSBO_R {
        VSBO_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Missing Offset Correction Signal The Missing Offset Correction flag signals to the Host, that no offset correction calculation can be performed because no sync frames were received. The flag is updated by the CC at start of offset correction phase. 1 = Missing offset correction signal 0 = Offset correction signal valid"]
    #[inline(always)]
    pub fn mocs(&self) -> MOCS_R {
        MOCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Offset Correction Limit Reached The Offset Correction Limit Reached flag signals to the Host, that the offset correction value has exceeded its limit as defined by GTUC10.MOC\\[13:0\\]. The flag is updated by the CC at start of offset correction phase. 1 = Offset correction limit reached 0 = Offset correction below limit"]
    #[inline(always)]
    pub fn oclr(&self) -> OCLR_R {
        OCLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Missing Rate Correction Signal The Missing Rate Correction flag signals to the Host, that no rate correction calculation can be performed because no pairs of even / odd sync frames were received. The flag is updated by the CC at start of offset correction phase. 1 = Missing rate correction signal 0 = Rate correction signal valid"]
    #[inline(always)]
    pub fn mrcs(&self) -> MRCS_R {
        MRCS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rate Correction Limit Reached The Rate Correction Limit Reached flag signals to the Host, that the rate correction value has exceeded its limit as defined by GTUC10.MRC\\[10:0\\]. The flag is updated by the CC at start of offset correction phase. 1 = Rate correction limit reached 0 = Rate correction below limit"]
    #[inline(always)]
    pub fn rclr(&self) -> RCLR_R {
        RCLR_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Sync Frame Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfs](index.html) module"]
pub struct SFS_SPEC;
impl crate::RegisterSpec for SFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfs::R](R) reader structure"]
impl crate::Readable for SFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SFS to value 0"]
impl crate::Resettable for SFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
