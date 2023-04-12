#[doc = "Register `RDHS3_MIR` reader"]
pub struct R(crate::R<RDHS3_MIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDHS3_MIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDHS3_MIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDHS3_MIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DP` reader - Data Pointer Pointer to the first 32-bit word of the data section of the addressed message buffer in the Message RAM."]
pub type DP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RCC` reader - Receive Cycle Count (vRF!Header!CycleCount) Cycle counter value updated from received data frame."]
pub type RCC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCI` reader - Received on Channel Indicator (vSS!Channel) Indicates the channel from which the received data frame was taken to update the respective receive buffer. 1 = Frame received on channel A 0 = Frame received on channel B"]
pub type RCI_R = crate::BitReader<RCI_A>;
#[doc = "Received on Channel Indicator (vSS!Channel) Indicates the channel from which the received data frame was taken to update the respective receive buffer. 1 = Frame received on channel A 0 = Frame received on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCI_A {
    #[doc = "0: N/A"]
    CH_B_FRAME_RECEIVED = 0,
    #[doc = "1: N/A"]
    CH_A_FRAME_RECEIVED = 1,
}
impl From<RCI_A> for bool {
    #[inline(always)]
    fn from(variant: RCI_A) -> Self {
        variant as u8 != 0
    }
}
impl RCI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCI_A {
        match self.bits {
            false => RCI_A::CH_B_FRAME_RECEIVED,
            true => RCI_A::CH_A_FRAME_RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_ch_b_frame_received(&self) -> bool {
        *self == RCI_A::CH_B_FRAME_RECEIVED
    }
    #[doc = "Checks if the value of the field is `CH_A_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_ch_a_frame_received(&self) -> bool {
        *self == RCI_A::CH_A_FRAME_RECEIVED
    }
}
#[doc = "Field `SFI` reader - Startup Frame Indicator (vRF!Header!SuFIndicator) A startup frame is marked by the startup frame indicator. 1 = The received frame is a startup frame 0 = The received frame is not a startup frame"]
pub type SFI_R = crate::BitReader<SFI_A>;
#[doc = "Startup Frame Indicator (vRF!Header!SuFIndicator) A startup frame is marked by the startup frame indicator. 1 = The received frame is a startup frame 0 = The received frame is not a startup frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFI_A {
    #[doc = "0: N/A"]
    RXD_FRAME_NOT_STARTUP_FRAME = 0,
    #[doc = "1: N/A"]
    RXD_FRAME_STARTUP_FRAME = 1,
}
impl From<SFI_A> for bool {
    #[inline(always)]
    fn from(variant: SFI_A) -> Self {
        variant as u8 != 0
    }
}
impl SFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFI_A {
        match self.bits {
            false => SFI_A::RXD_FRAME_NOT_STARTUP_FRAME,
            true => SFI_A::RXD_FRAME_STARTUP_FRAME,
        }
    }
    #[doc = "Checks if the value of the field is `RXD_FRAME_NOT_STARTUP_FRAME`"]
    #[inline(always)]
    pub fn is_rxd_frame_not_startup_frame(&self) -> bool {
        *self == SFI_A::RXD_FRAME_NOT_STARTUP_FRAME
    }
    #[doc = "Checks if the value of the field is `RXD_FRAME_STARTUP_FRAME`"]
    #[inline(always)]
    pub fn is_rxd_frame_startup_frame(&self) -> bool {
        *self == SFI_A::RXD_FRAME_STARTUP_FRAME
    }
}
#[doc = "Field `SYN` reader - Sync Frame Indicator (vRF!Header!SyFIndicator) A sync frame is marked by the sync frame indicator. 1 = The received frame is a sync frame 0 = The received frame is not a sync frame"]
pub type SYN_R = crate::BitReader<SYN_A>;
#[doc = "Sync Frame Indicator (vRF!Header!SyFIndicator) A sync frame is marked by the sync frame indicator. 1 = The received frame is a sync frame 0 = The received frame is not a sync frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYN_A {
    #[doc = "0: N/A"]
    RXD_FRAME_NOT_SYNC_FRAME = 0,
    #[doc = "1: N/A"]
    RXD_FRAME_SYNC_FRAME = 1,
}
impl From<SYN_A> for bool {
    #[inline(always)]
    fn from(variant: SYN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYN_A {
        match self.bits {
            false => SYN_A::RXD_FRAME_NOT_SYNC_FRAME,
            true => SYN_A::RXD_FRAME_SYNC_FRAME,
        }
    }
    #[doc = "Checks if the value of the field is `RXD_FRAME_NOT_SYNC_FRAME`"]
    #[inline(always)]
    pub fn is_rxd_frame_not_sync_frame(&self) -> bool {
        *self == SYN_A::RXD_FRAME_NOT_SYNC_FRAME
    }
    #[doc = "Checks if the value of the field is `RXD_FRAME_SYNC_FRAME`"]
    #[inline(always)]
    pub fn is_rxd_frame_sync_frame(&self) -> bool {
        *self == SYN_A::RXD_FRAME_SYNC_FRAME
    }
}
#[doc = "Field `NFI` reader - Null Frame Indicator (vRF!Header!NFIndicator) Is set to '1' after storage of the first received data frame. 1 = At least one data frame has been stored into the respective message buffer 0 = Up to now no data frame has been stored into the respective message buffer"]
pub type NFI_R = crate::BitReader<NFI_A>;
#[doc = "Null Frame Indicator (vRF!Header!NFIndicator) Is set to '1' after storage of the first received data frame. 1 = At least one data frame has been stored into the respective message buffer 0 = Up to now no data frame has been stored into the respective message buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFI_A {
    #[doc = "0: N/A"]
    DATA_FRAME_NOT_STORED = 0,
    #[doc = "1: N/A"]
    DATA_FRAME_STORED = 1,
}
impl From<NFI_A> for bool {
    #[inline(always)]
    fn from(variant: NFI_A) -> Self {
        variant as u8 != 0
    }
}
impl NFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFI_A {
        match self.bits {
            false => NFI_A::DATA_FRAME_NOT_STORED,
            true => NFI_A::DATA_FRAME_STORED,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_FRAME_NOT_STORED`"]
    #[inline(always)]
    pub fn is_data_frame_not_stored(&self) -> bool {
        *self == NFI_A::DATA_FRAME_NOT_STORED
    }
    #[doc = "Checks if the value of the field is `DATA_FRAME_STORED`"]
    #[inline(always)]
    pub fn is_data_frame_stored(&self) -> bool {
        *self == NFI_A::DATA_FRAME_STORED
    }
}
#[doc = "Field `PPI` reader - Payload Preamble Indicator (vRF!Header!PPIndicator) The payload preamble indicator defines whether a network management vector or message ID is contained within the payload segment of the received frame. 1 = Static segment: Network management vector in the first part of the payload Dynamic segment:Message ID in the first part of the payload 0 = The payload segment of the received frame does not contain a network management vector nor a message ID"]
pub type PPI_R = crate::BitReader<PPI_A>;
#[doc = "Payload Preamble Indicator (vRF!Header!PPIndicator) The payload preamble indicator defines whether a network management vector or message ID is contained within the payload segment of the received frame. 1 = Static segment: Network management vector in the first part of the payload Dynamic segment:Message ID in the first part of the payload 0 = The payload segment of the received frame does not contain a network management vector nor a message ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPI_A {
    #[doc = "0: N/A"]
    PAYLOAD_SEGMENT_HAS_NO_NMV_MID = 0,
    #[doc = "1: N/A"]
    PAYLOAD_SEGMENT_WITH_NMV_MID = 1,
}
impl From<PPI_A> for bool {
    #[inline(always)]
    fn from(variant: PPI_A) -> Self {
        variant as u8 != 0
    }
}
impl PPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPI_A {
        match self.bits {
            false => PPI_A::PAYLOAD_SEGMENT_HAS_NO_NMV_MID,
            true => PPI_A::PAYLOAD_SEGMENT_WITH_NMV_MID,
        }
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_SEGMENT_HAS_NO_NMV_MID`"]
    #[inline(always)]
    pub fn is_payload_segment_has_no_nmv_mid(&self) -> bool {
        *self == PPI_A::PAYLOAD_SEGMENT_HAS_NO_NMV_MID
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_SEGMENT_WITH_NMV_MID`"]
    #[inline(always)]
    pub fn is_payload_segment_with_nmv_mid(&self) -> bool {
        *self == PPI_A::PAYLOAD_SEGMENT_WITH_NMV_MID
    }
}
#[doc = "Field `RES` reader - N/A"]
pub type RES_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:10 - Data Pointer Pointer to the first 32-bit word of the data section of the addressed message buffer in the Message RAM."]
    #[inline(always)]
    pub fn dp(&self) -> DP_R {
        DP_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:21 - Receive Cycle Count (vRF!Header!CycleCount) Cycle counter value updated from received data frame."]
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Received on Channel Indicator (vSS!Channel) Indicates the channel from which the received data frame was taken to update the respective receive buffer. 1 = Frame received on channel A 0 = Frame received on channel B"]
    #[inline(always)]
    pub fn rci(&self) -> RCI_R {
        RCI_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Startup Frame Indicator (vRF!Header!SuFIndicator) A startup frame is marked by the startup frame indicator. 1 = The received frame is a startup frame 0 = The received frame is not a startup frame"]
    #[inline(always)]
    pub fn sfi(&self) -> SFI_R {
        SFI_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Sync Frame Indicator (vRF!Header!SyFIndicator) A sync frame is marked by the sync frame indicator. 1 = The received frame is a sync frame 0 = The received frame is not a sync frame"]
    #[inline(always)]
    pub fn syn(&self) -> SYN_R {
        SYN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Null Frame Indicator (vRF!Header!NFIndicator) Is set to '1' after storage of the first received data frame. 1 = At least one data frame has been stored into the respective message buffer 0 = Up to now no data frame has been stored into the respective message buffer"]
    #[inline(always)]
    pub fn nfi(&self) -> NFI_R {
        NFI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Payload Preamble Indicator (vRF!Header!PPIndicator) The payload preamble indicator defines whether a network management vector or message ID is contained within the payload segment of the received frame. 1 = Static segment: Network management vector in the first part of the payload Dynamic segment:Message ID in the first part of the payload 0 = The payload segment of the received frame does not contain a network management vector nor a message ID"]
    #[inline(always)]
    pub fn ppi(&self) -> PPI_R {
        PPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Read Header Section 3 (mirror)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdhs3_mir](index.html) module"]
pub struct RDHS3_MIR_SPEC;
impl crate::RegisterSpec for RDHS3_MIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdhs3_mir::R](R) reader structure"]
impl crate::Readable for RDHS3_MIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDHS3_MIR to value 0"]
impl crate::Resettable for RDHS3_MIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
