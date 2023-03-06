#[doc = "Register `ESID[%s]` reader"]
pub struct R(crate::R<ESID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EID` reader - Even Sync ID (vsSyncIDListA,B even) Sync frame ID even communication cycle."]
pub type EID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXEA` reader - Received / Configured Even Sync ID on Channel A Signals that a sync frame corresponding to the stored even sync ID was received on channel A or that the node is configured to be a sync node with key slot = EID\\[9:0\\]
(ESID1 only). 1 = Sync frame received on channel A / node configured to transmit sync frames 0 = No sync frame received on channel A / node not configured to transmit sync frames"]
pub type RXEA_R = crate::BitReader<RXEA_A>;
#[doc = "Received / Configured Even Sync ID on Channel A Signals that a sync frame corresponding to the stored even sync ID was received on channel A or that the node is configured to be a sync node with key slot = EID\\[9:0\\]
(ESID1 only). 1 = Sync frame received on channel A / node configured to transmit sync frames 0 = No sync frame received on channel A / node not configured to transmit sync frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEA_A {
    #[doc = "0: N/A"]
    NO_SYNC_FRAME_RXD_ON_CH_A = 0,
    #[doc = "1: N/A"]
    SYNC_FRAME_RXD_ON_CH_A = 1,
}
impl From<RXEA_A> for bool {
    #[inline(always)]
    fn from(variant: RXEA_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEA_A {
        match self.bits {
            false => RXEA_A::NO_SYNC_FRAME_RXD_ON_CH_A,
            true => RXEA_A::SYNC_FRAME_RXD_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNC_FRAME_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_no_sync_frame_rxd_on_ch_a(&self) -> bool {
        *self == RXEA_A::NO_SYNC_FRAME_RXD_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAME_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_sync_frame_rxd_on_ch_a(&self) -> bool {
        *self == RXEA_A::SYNC_FRAME_RXD_ON_CH_A
    }
}
#[doc = "Field `RXEB` reader - Received / Configured Even Sync ID on Channel B Signals that a sync frame corresponding to the stored even sync ID was received on channel B or that the node is configured to be a sync node with key slot = EID\\[9:0\\]
(ESID1 only). 1 = Sync frame received on channel B / node configured to transmit sync frames 0 = No sync frame received on channel B / node not configured to transmit sync frames"]
pub type RXEB_R = crate::BitReader<RXEB_A>;
#[doc = "Received / Configured Even Sync ID on Channel B Signals that a sync frame corresponding to the stored even sync ID was received on channel B or that the node is configured to be a sync node with key slot = EID\\[9:0\\]
(ESID1 only). 1 = Sync frame received on channel B / node configured to transmit sync frames 0 = No sync frame received on channel B / node not configured to transmit sync frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEB_A {
    #[doc = "0: N/A"]
    NO_SYNC_FRAME_RXD_ON_CH_B = 0,
    #[doc = "1: N/A"]
    SYNC_FRAME_RXD_ON_CH_B = 1,
}
impl From<RXEB_A> for bool {
    #[inline(always)]
    fn from(variant: RXEB_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEB_A {
        match self.bits {
            false => RXEB_A::NO_SYNC_FRAME_RXD_ON_CH_B,
            true => RXEB_A::SYNC_FRAME_RXD_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNC_FRAME_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_no_sync_frame_rxd_on_ch_b(&self) -> bool {
        *self == RXEB_A::NO_SYNC_FRAME_RXD_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAME_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_sync_frame_rxd_on_ch_b(&self) -> bool {
        *self == RXEB_A::SYNC_FRAME_RXD_ON_CH_B
    }
}
impl R {
    #[doc = "Bits 0:9 - Even Sync ID (vsSyncIDListA,B even) Sync frame ID even communication cycle."]
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Received / Configured Even Sync ID on Channel A Signals that a sync frame corresponding to the stored even sync ID was received on channel A or that the node is configured to be a sync node with key slot = EID\\[9:0\\]
(ESID1 only). 1 = Sync frame received on channel A / node configured to transmit sync frames 0 = No sync frame received on channel A / node not configured to transmit sync frames"]
    #[inline(always)]
    pub fn rxea(&self) -> RXEA_R {
        RXEA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Received / Configured Even Sync ID on Channel B Signals that a sync frame corresponding to the stored even sync ID was received on channel B or that the node is configured to be a sync node with key slot = EID\\[9:0\\]
(ESID1 only). 1 = Sync frame received on channel B / node configured to transmit sync frames 0 = No sync frame received on channel B / node not configured to transmit sync frames"]
    #[inline(always)]
    pub fn rxeb(&self) -> RXEB_R {
        RXEB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Even Sync ID \\[1...15\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esid](index.html) module"]
pub struct ESID_SPEC;
impl crate::RegisterSpec for ESID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esid::R](R) reader structure"]
impl crate::Readable for ESID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESID[%s]
to value 0"]
impl crate::Resettable for ESID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
