#[doc = "Register `OSID[%s]` reader"]
pub struct R(crate::R<OSID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OID` reader - Odd Sync ID (vsSyncIDListA,B odd) Sync frame ID odd communication cycle."]
pub type OID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXOA` reader - Received / Configured Odd Sync ID on Channel A Signals that a sync frame corresponding to the stored odd sync ID was received on channel A or that the node is configured to be a sync node with key slot = OID\\[9:0\\]
(OSID1 only). 1 = Sync frame received on channel A / node configured to transmit sync frames 0 = No sync frame received on channel A / node not configured to transmit sync frames"]
pub type RXOA_R = crate::BitReader<RXOA_A>;
#[doc = "Received / Configured Odd Sync ID on Channel A Signals that a sync frame corresponding to the stored odd sync ID was received on channel A or that the node is configured to be a sync node with key slot = OID\\[9:0\\]
(OSID1 only). 1 = Sync frame received on channel A / node configured to transmit sync frames 0 = No sync frame received on channel A / node not configured to transmit sync frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOA_A {
    #[doc = "0: N/A"]
    NO_SYNC_FRAME_RXD_ON_CH_A = 0,
    #[doc = "1: N/A"]
    SYNC_FRAME_RXD_ON_CH_A = 1,
}
impl From<RXOA_A> for bool {
    #[inline(always)]
    fn from(variant: RXOA_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOA_A {
        match self.bits {
            false => RXOA_A::NO_SYNC_FRAME_RXD_ON_CH_A,
            true => RXOA_A::SYNC_FRAME_RXD_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNC_FRAME_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_no_sync_frame_rxd_on_ch_a(&self) -> bool {
        *self == RXOA_A::NO_SYNC_FRAME_RXD_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAME_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_sync_frame_rxd_on_ch_a(&self) -> bool {
        *self == RXOA_A::SYNC_FRAME_RXD_ON_CH_A
    }
}
#[doc = "Field `RXOB` reader - Received / Configured Odd Sync ID on Channel B Signals that a sync frame corresponding to the stored odd sync ID was received on channel B or that the node is configured to be a sync node with key slot = OID\\[9:0\\]
(OSID1 only). 1 = Sync frame received on channel B / node configured to transmit sync frames 0 = No sync frame received on channel B / node not configured to transmit sync frames"]
pub type RXOB_R = crate::BitReader<RXOB_A>;
#[doc = "Received / Configured Odd Sync ID on Channel B Signals that a sync frame corresponding to the stored odd sync ID was received on channel B or that the node is configured to be a sync node with key slot = OID\\[9:0\\]
(OSID1 only). 1 = Sync frame received on channel B / node configured to transmit sync frames 0 = No sync frame received on channel B / node not configured to transmit sync frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXOB_A {
    #[doc = "0: N/A"]
    NO_SYNC_FRAME_RXD_ON_CH_B = 0,
    #[doc = "1: N/A"]
    SYNC_FRAME_RXD_ON_CH_B = 1,
}
impl From<RXOB_A> for bool {
    #[inline(always)]
    fn from(variant: RXOB_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOB_A {
        match self.bits {
            false => RXOB_A::NO_SYNC_FRAME_RXD_ON_CH_B,
            true => RXOB_A::SYNC_FRAME_RXD_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNC_FRAME_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_no_sync_frame_rxd_on_ch_b(&self) -> bool {
        *self == RXOB_A::NO_SYNC_FRAME_RXD_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAME_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_sync_frame_rxd_on_ch_b(&self) -> bool {
        *self == RXOB_A::SYNC_FRAME_RXD_ON_CH_B
    }
}
impl R {
    #[doc = "Bits 0:9 - Odd Sync ID (vsSyncIDListA,B odd) Sync frame ID odd communication cycle."]
    #[inline(always)]
    pub fn oid(&self) -> OID_R {
        OID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Received / Configured Odd Sync ID on Channel A Signals that a sync frame corresponding to the stored odd sync ID was received on channel A or that the node is configured to be a sync node with key slot = OID\\[9:0\\]
(OSID1 only). 1 = Sync frame received on channel A / node configured to transmit sync frames 0 = No sync frame received on channel A / node not configured to transmit sync frames"]
    #[inline(always)]
    pub fn rxoa(&self) -> RXOA_R {
        RXOA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Received / Configured Odd Sync ID on Channel B Signals that a sync frame corresponding to the stored odd sync ID was received on channel B or that the node is configured to be a sync node with key slot = OID\\[9:0\\]
(OSID1 only). 1 = Sync frame received on channel B / node configured to transmit sync frames 0 = No sync frame received on channel B / node not configured to transmit sync frames"]
    #[inline(always)]
    pub fn rxob(&self) -> RXOB_R {
        RXOB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Odd Sync ID \\[1...15\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osid](index.html) module"]
pub struct OSID_SPEC;
impl crate::RegisterSpec for OSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osid::R](R) reader structure"]
impl crate::Readable for OSID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSID[%s]
to value 0"]
impl crate::Resettable for OSID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
