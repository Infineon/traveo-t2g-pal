#[doc = "Register `MBS_MIR` reader"]
pub struct R(crate::R<MBS_MIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBS_MIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBS_MIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBS_MIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VFRA` reader - Valid Frame Received on Channel A (vSS!ValidFrameA) A valid frame indication is set if a valid frame was received on channel A. 1 = Valid frame received on channel A 0 = No valid frame received on channel A"]
pub type VFRA_R = crate::BitReader<VFRA_A>;
#[doc = "Valid Frame Received on Channel A (vSS!ValidFrameA) A valid frame indication is set if a valid frame was received on channel A. 1 = Valid frame received on channel A 0 = No valid frame received on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VFRA_A {
    #[doc = "0: N/A"]
    CH_A_NO_VALID_FRAME = 0,
    #[doc = "1: N/A"]
    CH_A_WITH_VALID_FRAME = 1,
}
impl From<VFRA_A> for bool {
    #[inline(always)]
    fn from(variant: VFRA_A) -> Self {
        variant as u8 != 0
    }
}
impl VFRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VFRA_A {
        match self.bits {
            false => VFRA_A::CH_A_NO_VALID_FRAME,
            true => VFRA_A::CH_A_WITH_VALID_FRAME,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_NO_VALID_FRAME`"]
    #[inline(always)]
    pub fn is_ch_a_no_valid_frame(&self) -> bool {
        *self == VFRA_A::CH_A_NO_VALID_FRAME
    }
    #[doc = "Checks if the value of the field is `CH_A_WITH_VALID_FRAME`"]
    #[inline(always)]
    pub fn is_ch_a_with_valid_frame(&self) -> bool {
        *self == VFRA_A::CH_A_WITH_VALID_FRAME
    }
}
#[doc = "Field `VFRB` reader - Valid Frame Received on Channel B (vSS!ValidFrameB) A valid frame indication is set if a valid frame was received on channel B. 1 = Valid frame received on channel B 0 = No valid frame received on channel B"]
pub type VFRB_R = crate::BitReader<VFRB_A>;
#[doc = "Valid Frame Received on Channel B (vSS!ValidFrameB) A valid frame indication is set if a valid frame was received on channel B. 1 = Valid frame received on channel B 0 = No valid frame received on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VFRB_A {
    #[doc = "0: N/A"]
    CH_B_NO_VALID_FRAME = 0,
    #[doc = "1: N/A"]
    CH_B_WITH_VALID_FRAME = 1,
}
impl From<VFRB_A> for bool {
    #[inline(always)]
    fn from(variant: VFRB_A) -> Self {
        variant as u8 != 0
    }
}
impl VFRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VFRB_A {
        match self.bits {
            false => VFRB_A::CH_B_NO_VALID_FRAME,
            true => VFRB_A::CH_B_WITH_VALID_FRAME,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_NO_VALID_FRAME`"]
    #[inline(always)]
    pub fn is_ch_b_no_valid_frame(&self) -> bool {
        *self == VFRB_A::CH_B_NO_VALID_FRAME
    }
    #[doc = "Checks if the value of the field is `CH_B_WITH_VALID_FRAME`"]
    #[inline(always)]
    pub fn is_ch_b_with_valid_frame(&self) -> bool {
        *self == VFRB_A::CH_B_WITH_VALID_FRAME
    }
}
#[doc = "Field `SEOA` reader - Syntax Error Observed on Channel A (vSS!SyntaxErrorA) A syntax error was observed in the assigned slot on channel A. 1 = Syntax error observed on channel A 0 = No syntax error observed on channel A"]
pub type SEOA_R = crate::BitReader<SEOA_A>;
#[doc = "Syntax Error Observed on Channel A (vSS!SyntaxErrorA) A syntax error was observed in the assigned slot on channel A. 1 = Syntax error observed on channel A 0 = No syntax error observed on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEOA_A {
    #[doc = "0: N/A"]
    CH_A_NO_SYNTAX_ERROR = 0,
    #[doc = "1: N/A"]
    CH_A_HAS_SYNTAX_ERROR = 1,
}
impl From<SEOA_A> for bool {
    #[inline(always)]
    fn from(variant: SEOA_A) -> Self {
        variant as u8 != 0
    }
}
impl SEOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEOA_A {
        match self.bits {
            false => SEOA_A::CH_A_NO_SYNTAX_ERROR,
            true => SEOA_A::CH_A_HAS_SYNTAX_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_NO_SYNTAX_ERROR`"]
    #[inline(always)]
    pub fn is_ch_a_no_syntax_error(&self) -> bool {
        *self == SEOA_A::CH_A_NO_SYNTAX_ERROR
    }
    #[doc = "Checks if the value of the field is `CH_A_HAS_SYNTAX_ERROR`"]
    #[inline(always)]
    pub fn is_ch_a_has_syntax_error(&self) -> bool {
        *self == SEOA_A::CH_A_HAS_SYNTAX_ERROR
    }
}
#[doc = "Field `SEOB` reader - Syntax Error Observed on Channel B (vSS!SyntaxErrorB) A syntax error was observed in the assigned slot on channel B. 1 = Syntax error observed on channel B 0 = No syntax error observed on channel B"]
pub type SEOB_R = crate::BitReader<SEOB_A>;
#[doc = "Syntax Error Observed on Channel B (vSS!SyntaxErrorB) A syntax error was observed in the assigned slot on channel B. 1 = Syntax error observed on channel B 0 = No syntax error observed on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEOB_A {
    #[doc = "0: N/A"]
    CH_B_NO_SYNTAX_ERROR = 0,
    #[doc = "1: N/A"]
    CH_B_HAS_SYNTAX_ERROR = 1,
}
impl From<SEOB_A> for bool {
    #[inline(always)]
    fn from(variant: SEOB_A) -> Self {
        variant as u8 != 0
    }
}
impl SEOB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEOB_A {
        match self.bits {
            false => SEOB_A::CH_B_NO_SYNTAX_ERROR,
            true => SEOB_A::CH_B_HAS_SYNTAX_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_NO_SYNTAX_ERROR`"]
    #[inline(always)]
    pub fn is_ch_b_no_syntax_error(&self) -> bool {
        *self == SEOB_A::CH_B_NO_SYNTAX_ERROR
    }
    #[doc = "Checks if the value of the field is `CH_B_HAS_SYNTAX_ERROR`"]
    #[inline(always)]
    pub fn is_ch_b_has_syntax_error(&self) -> bool {
        *self == SEOB_A::CH_B_HAS_SYNTAX_ERROR
    }
}
#[doc = "Field `CEOA` reader - Content Error Observed on Channel A (vSS!ContentErrorA) A content error was observed in the assigned slot on channel A. 1 = Content error observed on channel A 0 = No content error observed on channel A"]
pub type CEOA_R = crate::BitReader<CEOA_A>;
#[doc = "Content Error Observed on Channel A (vSS!ContentErrorA) A content error was observed in the assigned slot on channel A. 1 = Content error observed on channel A 0 = No content error observed on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEOA_A {
    #[doc = "0: N/A"]
    CH_A_NO_CONTENT_ERROR = 0,
    #[doc = "1: N/A"]
    CH_A_HAS_CONTENT_ERROR = 1,
}
impl From<CEOA_A> for bool {
    #[inline(always)]
    fn from(variant: CEOA_A) -> Self {
        variant as u8 != 0
    }
}
impl CEOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEOA_A {
        match self.bits {
            false => CEOA_A::CH_A_NO_CONTENT_ERROR,
            true => CEOA_A::CH_A_HAS_CONTENT_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_NO_CONTENT_ERROR`"]
    #[inline(always)]
    pub fn is_ch_a_no_content_error(&self) -> bool {
        *self == CEOA_A::CH_A_NO_CONTENT_ERROR
    }
    #[doc = "Checks if the value of the field is `CH_A_HAS_CONTENT_ERROR`"]
    #[inline(always)]
    pub fn is_ch_a_has_content_error(&self) -> bool {
        *self == CEOA_A::CH_A_HAS_CONTENT_ERROR
    }
}
#[doc = "Field `CEOB` reader - Content Error Observed on Channel B (vSS!ContentErrorB) A content error was observed in the assigned slot on channel B. 1 = Content error observed on channel B 0 = No content error observed on channel B"]
pub type CEOB_R = crate::BitReader<CEOB_A>;
#[doc = "Content Error Observed on Channel B (vSS!ContentErrorB) A content error was observed in the assigned slot on channel B. 1 = Content error observed on channel B 0 = No content error observed on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEOB_A {
    #[doc = "0: N/A"]
    CH_B_NO_CONTENT_ERROR = 0,
    #[doc = "1: N/A"]
    CH_B_HAS_CONTENT_ERROR = 1,
}
impl From<CEOB_A> for bool {
    #[inline(always)]
    fn from(variant: CEOB_A) -> Self {
        variant as u8 != 0
    }
}
impl CEOB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEOB_A {
        match self.bits {
            false => CEOB_A::CH_B_NO_CONTENT_ERROR,
            true => CEOB_A::CH_B_HAS_CONTENT_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_NO_CONTENT_ERROR`"]
    #[inline(always)]
    pub fn is_ch_b_no_content_error(&self) -> bool {
        *self == CEOB_A::CH_B_NO_CONTENT_ERROR
    }
    #[doc = "Checks if the value of the field is `CH_B_HAS_CONTENT_ERROR`"]
    #[inline(always)]
    pub fn is_ch_b_has_content_error(&self) -> bool {
        *self == CEOB_A::CH_B_HAS_CONTENT_ERROR
    }
}
#[doc = "Field `SVOA` reader - Slot Boundary Violation Observed on Channel A (vSS!BViolationA) A slot boundary violation (channel active at the start or at the end of the assigned slot) was observed on channel A. 1 = Slot boundary violation observed on channel A 0 = No slot boundary violation observed on channel A"]
pub type SVOA_R = crate::BitReader<SVOA_A>;
#[doc = "Slot Boundary Violation Observed on Channel A (vSS!BViolationA) A slot boundary violation (channel active at the start or at the end of the assigned slot) was observed on channel A. 1 = Slot boundary violation observed on channel A 0 = No slot boundary violation observed on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVOA_A {
    #[doc = "0: N/A"]
    CH_A_NO_SLOT_BNDRY_VIOLATION = 0,
    #[doc = "1: N/A"]
    CH_A_HAS_SLOT_BNDRY_VIOLATION = 1,
}
impl From<SVOA_A> for bool {
    #[inline(always)]
    fn from(variant: SVOA_A) -> Self {
        variant as u8 != 0
    }
}
impl SVOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVOA_A {
        match self.bits {
            false => SVOA_A::CH_A_NO_SLOT_BNDRY_VIOLATION,
            true => SVOA_A::CH_A_HAS_SLOT_BNDRY_VIOLATION,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_NO_SLOT_BNDRY_VIOLATION`"]
    #[inline(always)]
    pub fn is_ch_a_no_slot_bndry_violation(&self) -> bool {
        *self == SVOA_A::CH_A_NO_SLOT_BNDRY_VIOLATION
    }
    #[doc = "Checks if the value of the field is `CH_A_HAS_SLOT_BNDRY_VIOLATION`"]
    #[inline(always)]
    pub fn is_ch_a_has_slot_bndry_violation(&self) -> bool {
        *self == SVOA_A::CH_A_HAS_SLOT_BNDRY_VIOLATION
    }
}
#[doc = "Field `SVOB` reader - Slot Boundary Violation Observed on Channel B (vSS!BViolationB) A slot boundary violation (channel active at the start or at the end of the assigned slot) was observed on channel B. 1 = Slot boundary violation observed on channel B 0 = No slot boundary violation observed on channel B"]
pub type SVOB_R = crate::BitReader<SVOB_A>;
#[doc = "Slot Boundary Violation Observed on Channel B (vSS!BViolationB) A slot boundary violation (channel active at the start or at the end of the assigned slot) was observed on channel B. 1 = Slot boundary violation observed on channel B 0 = No slot boundary violation observed on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVOB_A {
    #[doc = "0: N/A"]
    CH_B_NO_SLOT_BNDRY_VIOLATION = 0,
    #[doc = "1: N/A"]
    CH_B_HAS_SLOT_BNDRY_VIOLATION = 1,
}
impl From<SVOB_A> for bool {
    #[inline(always)]
    fn from(variant: SVOB_A) -> Self {
        variant as u8 != 0
    }
}
impl SVOB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVOB_A {
        match self.bits {
            false => SVOB_A::CH_B_NO_SLOT_BNDRY_VIOLATION,
            true => SVOB_A::CH_B_HAS_SLOT_BNDRY_VIOLATION,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_NO_SLOT_BNDRY_VIOLATION`"]
    #[inline(always)]
    pub fn is_ch_b_no_slot_bndry_violation(&self) -> bool {
        *self == SVOB_A::CH_B_NO_SLOT_BNDRY_VIOLATION
    }
    #[doc = "Checks if the value of the field is `CH_B_HAS_SLOT_BNDRY_VIOLATION`"]
    #[inline(always)]
    pub fn is_ch_b_has_slot_bndry_violation(&self) -> bool {
        *self == SVOB_A::CH_B_HAS_SLOT_BNDRY_VIOLATION
    }
}
#[doc = "Field `TCIA` reader - Transmission Conflict Indication Channel A (vSS!TxConflictA) A transmission conflict indication is set if a transmission conflict has occurred on channel A. 1 = Transmission conflict occurred on channel A 0 = No transmission conflict occurred on channel A"]
pub type TCIA_R = crate::BitReader<TCIA_A>;
#[doc = "Transmission Conflict Indication Channel A (vSS!TxConflictA) A transmission conflict indication is set if a transmission conflict has occurred on channel A. 1 = Transmission conflict occurred on channel A 0 = No transmission conflict occurred on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIA_A {
    #[doc = "0: N/A"]
    CH_A_NO_TXMN_CONFLICT = 0,
    #[doc = "1: N/A"]
    CH_A_HAS_TXMN_CONFLICT = 1,
}
impl From<TCIA_A> for bool {
    #[inline(always)]
    fn from(variant: TCIA_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIA_A {
        match self.bits {
            false => TCIA_A::CH_A_NO_TXMN_CONFLICT,
            true => TCIA_A::CH_A_HAS_TXMN_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_NO_TXMN_CONFLICT`"]
    #[inline(always)]
    pub fn is_ch_a_no_txmn_conflict(&self) -> bool {
        *self == TCIA_A::CH_A_NO_TXMN_CONFLICT
    }
    #[doc = "Checks if the value of the field is `CH_A_HAS_TXMN_CONFLICT`"]
    #[inline(always)]
    pub fn is_ch_a_has_txmn_conflict(&self) -> bool {
        *self == TCIA_A::CH_A_HAS_TXMN_CONFLICT
    }
}
#[doc = "Field `TCIB` reader - Transmission Conflict Indication Channel B (vSS!TxConflictB) A transmission conflict indication is set if a transmission conflict has occurred on channel B. 1 = Transmission conflict occurred on channel B 0 = No transmission conflict occurred on channel B"]
pub type TCIB_R = crate::BitReader<TCIB_A>;
#[doc = "Transmission Conflict Indication Channel B (vSS!TxConflictB) A transmission conflict indication is set if a transmission conflict has occurred on channel B. 1 = Transmission conflict occurred on channel B 0 = No transmission conflict occurred on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIB_A {
    #[doc = "0: N/A"]
    CH_B_NO_TXMN_CONFLICT = 0,
    #[doc = "1: N/A"]
    CH_B_HAS_TXMN_CONFLICT = 1,
}
impl From<TCIB_A> for bool {
    #[inline(always)]
    fn from(variant: TCIB_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIB_A {
        match self.bits {
            false => TCIB_A::CH_B_NO_TXMN_CONFLICT,
            true => TCIB_A::CH_B_HAS_TXMN_CONFLICT,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_NO_TXMN_CONFLICT`"]
    #[inline(always)]
    pub fn is_ch_b_no_txmn_conflict(&self) -> bool {
        *self == TCIB_A::CH_B_NO_TXMN_CONFLICT
    }
    #[doc = "Checks if the value of the field is `CH_B_HAS_TXMN_CONFLICT`"]
    #[inline(always)]
    pub fn is_ch_b_has_txmn_conflict(&self) -> bool {
        *self == TCIB_A::CH_B_HAS_TXMN_CONFLICT
    }
}
#[doc = "Field `ESA` reader - Empty Slot Channel A In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots. 1 = No bus activity detected in the assigned slot on channel A 0 = Bus activity detected in the assigned slot on channel A"]
pub type ESA_R = crate::BitReader<ESA_A>;
#[doc = "Empty Slot Channel A In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots. 1 = No bus activity detected in the assigned slot on channel A 0 = Bus activity detected in the assigned slot on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESA_A {
    #[doc = "0: N/A"]
    CH_A_HAS_BUS_ACTIVITY = 0,
    #[doc = "1: N/A"]
    CH_A_NO_BUS_ACTIVITY = 1,
}
impl From<ESA_A> for bool {
    #[inline(always)]
    fn from(variant: ESA_A) -> Self {
        variant as u8 != 0
    }
}
impl ESA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESA_A {
        match self.bits {
            false => ESA_A::CH_A_HAS_BUS_ACTIVITY,
            true => ESA_A::CH_A_NO_BUS_ACTIVITY,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_HAS_BUS_ACTIVITY`"]
    #[inline(always)]
    pub fn is_ch_a_has_bus_activity(&self) -> bool {
        *self == ESA_A::CH_A_HAS_BUS_ACTIVITY
    }
    #[doc = "Checks if the value of the field is `CH_A_NO_BUS_ACTIVITY`"]
    #[inline(always)]
    pub fn is_ch_a_no_bus_activity(&self) -> bool {
        *self == ESA_A::CH_A_NO_BUS_ACTIVITY
    }
}
#[doc = "Field `ESB` reader - Empty Slot Channel B In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots. 1 = No bus activity detected in the assigned slot on channel B 0 = Bus activity detected in the assigned slot on channel B"]
pub type ESB_R = crate::BitReader<ESB_A>;
#[doc = "Empty Slot Channel B In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots. 1 = No bus activity detected in the assigned slot on channel B 0 = Bus activity detected in the assigned slot on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESB_A {
    #[doc = "0: N/A"]
    CH_B_HAS_BUS_ACTIVITY = 0,
    #[doc = "1: N/A"]
    CH_B_NO_BUS_ACTIVITY = 1,
}
impl From<ESB_A> for bool {
    #[inline(always)]
    fn from(variant: ESB_A) -> Self {
        variant as u8 != 0
    }
}
impl ESB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESB_A {
        match self.bits {
            false => ESB_A::CH_B_HAS_BUS_ACTIVITY,
            true => ESB_A::CH_B_NO_BUS_ACTIVITY,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_HAS_BUS_ACTIVITY`"]
    #[inline(always)]
    pub fn is_ch_b_has_bus_activity(&self) -> bool {
        *self == ESB_A::CH_B_HAS_BUS_ACTIVITY
    }
    #[doc = "Checks if the value of the field is `CH_B_NO_BUS_ACTIVITY`"]
    #[inline(always)]
    pub fn is_ch_b_no_bus_activity(&self) -> bool {
        *self == ESB_A::CH_B_NO_BUS_ACTIVITY
    }
}
#[doc = "Field `MLST` reader - Message Lost The flag is set in case the Host did not read the message before the message buffer was updated from a received data frame. Not affected by reception of null frames except for message buffers belonging to the receive FIFO. The flag is reset by a Host write to the message buffer via IBF or when a new message is stored into the message buffer after the message buffers ND flag was reset by reading out the message buffer via OBF. 1 = Unprocessed message was overwritten 0 = No message lost"]
pub type MLST_R = crate::BitReader<MLST_A>;
#[doc = "Message Lost The flag is set in case the Host did not read the message before the message buffer was updated from a received data frame. Not affected by reception of null frames except for message buffers belonging to the receive FIFO. The flag is reset by a Host write to the message buffer via IBF or when a new message is stored into the message buffer after the message buffers ND flag was reset by reading out the message buffer via OBF. 1 = Unprocessed message was overwritten 0 = No message lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MLST_A {
    #[doc = "0: N/A"]
    NO_MSG_LOST = 0,
    #[doc = "1: N/A"]
    MSG_LOST = 1,
}
impl From<MLST_A> for bool {
    #[inline(always)]
    fn from(variant: MLST_A) -> Self {
        variant as u8 != 0
    }
}
impl MLST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MLST_A {
        match self.bits {
            false => MLST_A::NO_MSG_LOST,
            true => MLST_A::MSG_LOST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MSG_LOST`"]
    #[inline(always)]
    pub fn is_no_msg_lost(&self) -> bool {
        *self == MLST_A::NO_MSG_LOST
    }
    #[doc = "Checks if the value of the field is `MSG_LOST`"]
    #[inline(always)]
    pub fn is_msg_lost(&self) -> bool {
        *self == MLST_A::MSG_LOST
    }
}
#[doc = "Field `FTA` reader - Frame Transmitted on Channel A Indicates that this node has transmitted a data frame in the configured slot on channel A. 1 = Data frame transmitted on channel A 0 = No data frame transmitted on channel A"]
pub type FTA_R = crate::BitReader<FTA_A>;
#[doc = "Frame Transmitted on Channel A Indicates that this node has transmitted a data frame in the configured slot on channel A. 1 = Data frame transmitted on channel A 0 = No data frame transmitted on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTA_A {
    #[doc = "0: N/A"]
    DATA_FRAME_NOT_TXD_ON_CH_A = 0,
    #[doc = "1: N/A"]
    DATA_FRAME_TXD_ON_CH_A = 1,
}
impl From<FTA_A> for bool {
    #[inline(always)]
    fn from(variant: FTA_A) -> Self {
        variant as u8 != 0
    }
}
impl FTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTA_A {
        match self.bits {
            false => FTA_A::DATA_FRAME_NOT_TXD_ON_CH_A,
            true => FTA_A::DATA_FRAME_TXD_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_FRAME_NOT_TXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_data_frame_not_txd_on_ch_a(&self) -> bool {
        *self == FTA_A::DATA_FRAME_NOT_TXD_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `DATA_FRAME_TXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_data_frame_txd_on_ch_a(&self) -> bool {
        *self == FTA_A::DATA_FRAME_TXD_ON_CH_A
    }
}
#[doc = "Field `FTB` reader - Frame Transmitted on Channel B Indicates that this node has transmitted a data frame in the configured slot on channel B. 1 = Data frame transmitted on channel B 0 = No data frame transmitted on channel B"]
pub type FTB_R = crate::BitReader<FTB_A>;
#[doc = "Frame Transmitted on Channel B Indicates that this node has transmitted a data frame in the configured slot on channel B. 1 = Data frame transmitted on channel B 0 = No data frame transmitted on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTB_A {
    #[doc = "0: N/A"]
    DATA_FRAME_NOT_TXD_ON_CH_B = 0,
    #[doc = "1: N/A"]
    DATA_FRAME_TXD_ON_CH_B = 1,
}
impl From<FTB_A> for bool {
    #[inline(always)]
    fn from(variant: FTB_A) -> Self {
        variant as u8 != 0
    }
}
impl FTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTB_A {
        match self.bits {
            false => FTB_A::DATA_FRAME_NOT_TXD_ON_CH_B,
            true => FTB_A::DATA_FRAME_TXD_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_FRAME_NOT_TXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_data_frame_not_txd_on_ch_b(&self) -> bool {
        *self == FTB_A::DATA_FRAME_NOT_TXD_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `DATA_FRAME_TXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_data_frame_txd_on_ch_b(&self) -> bool {
        *self == FTB_A::DATA_FRAME_TXD_ON_CH_B
    }
}
#[doc = "Field `CCS` reader - Cycle Count Status Actual cycle count when status was updated. For receive buffers (CFG = '0') the following status bits are updated from both valid data and null frames. If no valid frame was received, the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
pub type CCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCIS` reader - Received on Channel Indicator Status (vSS!Channel) Indicates the channel on which the frame was received. 1 = Frame received on channel A 0 = Frame received on channel B"]
pub type RCIS_R = crate::BitReader<RCIS_A>;
#[doc = "Received on Channel Indicator Status (vSS!Channel) Indicates the channel on which the frame was received. 1 = Frame received on channel A 0 = Frame received on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCIS_A {
    #[doc = "0: N/A"]
    CH_B_FRAME_RECEIVED = 0,
    #[doc = "1: N/A"]
    CH_A_FRAME_RECEIVED = 1,
}
impl From<RCIS_A> for bool {
    #[inline(always)]
    fn from(variant: RCIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RCIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCIS_A {
        match self.bits {
            false => RCIS_A::CH_B_FRAME_RECEIVED,
            true => RCIS_A::CH_A_FRAME_RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_ch_b_frame_received(&self) -> bool {
        *self == RCIS_A::CH_B_FRAME_RECEIVED
    }
    #[doc = "Checks if the value of the field is `CH_A_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_ch_a_frame_received(&self) -> bool {
        *self == RCIS_A::CH_A_FRAME_RECEIVED
    }
}
#[doc = "Field `SFIS` reader - Startup Frame Indicator Status (vRF!Header!SuFIndicator) A startup frame is marked by the startup frame indicator. 1 = The received frame is a startup frame 0 = No startup frame received"]
pub type SFIS_R = crate::BitReader<SFIS_A>;
#[doc = "Startup Frame Indicator Status (vRF!Header!SuFIndicator) A startup frame is marked by the startup frame indicator. 1 = The received frame is a startup frame 0 = No startup frame received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFIS_A {
    #[doc = "0: N/A"]
    NO_STARTUP_FRAME_RECEIVED = 0,
    #[doc = "1: N/A"]
    STARTUP_FRAME_RECEIVED = 1,
}
impl From<SFIS_A> for bool {
    #[inline(always)]
    fn from(variant: SFIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SFIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFIS_A {
        match self.bits {
            false => SFIS_A::NO_STARTUP_FRAME_RECEIVED,
            true => SFIS_A::STARTUP_FRAME_RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STARTUP_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_no_startup_frame_received(&self) -> bool {
        *self == SFIS_A::NO_STARTUP_FRAME_RECEIVED
    }
    #[doc = "Checks if the value of the field is `STARTUP_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_startup_frame_received(&self) -> bool {
        *self == SFIS_A::STARTUP_FRAME_RECEIVED
    }
}
#[doc = "Field `SYNS` reader - Sync Frame Indicator Status (vRF!Header!SyFIndicator) A sync frame is marked by the sync frame indicator. 1 = The received frame is a sync frame 0 = No sync frame received"]
pub type SYNS_R = crate::BitReader<SYNS_A>;
#[doc = "Sync Frame Indicator Status (vRF!Header!SyFIndicator) A sync frame is marked by the sync frame indicator. 1 = The received frame is a sync frame 0 = No sync frame received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNS_A {
    #[doc = "0: N/A"]
    NO_SYNC_FRAME_RECEIVED = 0,
    #[doc = "1: N/A"]
    SYNC_FRAME_RECEIVED = 1,
}
impl From<SYNS_A> for bool {
    #[inline(always)]
    fn from(variant: SYNS_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNS_A {
        match self.bits {
            false => SYNS_A::NO_SYNC_FRAME_RECEIVED,
            true => SYNS_A::SYNC_FRAME_RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNC_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_no_sync_frame_received(&self) -> bool {
        *self == SYNS_A::NO_SYNC_FRAME_RECEIVED
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_sync_frame_received(&self) -> bool {
        *self == SYNS_A::SYNC_FRAME_RECEIVED
    }
}
#[doc = "Field `NFIS` reader - Null Frame Indicator Status (vRF!Header!NFIndicator) If set to '0' the payload segment of the received frame contains no usable data. 1 = Received frame is not a null frame 0 = Received frame is a null frame"]
pub type NFIS_R = crate::BitReader<NFIS_A>;
#[doc = "Null Frame Indicator Status (vRF!Header!NFIndicator) If set to '0' the payload segment of the received frame contains no usable data. 1 = Received frame is not a null frame 0 = Received frame is a null frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFIS_A {
    #[doc = "0: N/A"]
    NULL_FRAME_RECEIVED = 0,
    #[doc = "1: N/A"]
    NO_NULL_FRAME_RECEIVED = 1,
}
impl From<NFIS_A> for bool {
    #[inline(always)]
    fn from(variant: NFIS_A) -> Self {
        variant as u8 != 0
    }
}
impl NFIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFIS_A {
        match self.bits {
            false => NFIS_A::NULL_FRAME_RECEIVED,
            true => NFIS_A::NO_NULL_FRAME_RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NULL_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_null_frame_received(&self) -> bool {
        *self == NFIS_A::NULL_FRAME_RECEIVED
    }
    #[doc = "Checks if the value of the field is `NO_NULL_FRAME_RECEIVED`"]
    #[inline(always)]
    pub fn is_no_null_frame_received(&self) -> bool {
        *self == NFIS_A::NO_NULL_FRAME_RECEIVED
    }
}
#[doc = "Field `PPIS` reader - Payload Preamble Indicator Status (vRF!Header!PPIndicator) The payload preamble indicator defines whether a network management vector or message ID is contained within the payload segment of the received frame. 1 = Static segment: Network management vector at the beginning of the payload Dynamic segment: Message ID at the beginning of the payload 0 = The payload segment of the received frame does not contain a network management vector or a message ID"]
pub type PPIS_R = crate::BitReader<PPIS_A>;
#[doc = "Payload Preamble Indicator Status (vRF!Header!PPIndicator) The payload preamble indicator defines whether a network management vector or message ID is contained within the payload segment of the received frame. 1 = Static segment: Network management vector at the beginning of the payload Dynamic segment: Message ID at the beginning of the payload 0 = The payload segment of the received frame does not contain a network management vector or a message ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPIS_A {
    #[doc = "0: N/A"]
    PAYLOAD_SEGMENT_HAS_NO_NMV_MID = 0,
    #[doc = "1: N/A"]
    PAYLOAD_SEGMENT_WITH_NMV_MID = 1,
}
impl From<PPIS_A> for bool {
    #[inline(always)]
    fn from(variant: PPIS_A) -> Self {
        variant as u8 != 0
    }
}
impl PPIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPIS_A {
        match self.bits {
            false => PPIS_A::PAYLOAD_SEGMENT_HAS_NO_NMV_MID,
            true => PPIS_A::PAYLOAD_SEGMENT_WITH_NMV_MID,
        }
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_SEGMENT_HAS_NO_NMV_MID`"]
    #[inline(always)]
    pub fn is_payload_segment_has_no_nmv_mid(&self) -> bool {
        *self == PPIS_A::PAYLOAD_SEGMENT_HAS_NO_NMV_MID
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_SEGMENT_WITH_NMV_MID`"]
    #[inline(always)]
    pub fn is_payload_segment_with_nmv_mid(&self) -> bool {
        *self == PPIS_A::PAYLOAD_SEGMENT_WITH_NMV_MID
    }
}
#[doc = "Field `RESS` reader - N/A"]
pub type RESS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Valid Frame Received on Channel A (vSS!ValidFrameA) A valid frame indication is set if a valid frame was received on channel A. 1 = Valid frame received on channel A 0 = No valid frame received on channel A"]
    #[inline(always)]
    pub fn vfra(&self) -> VFRA_R {
        VFRA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Valid Frame Received on Channel B (vSS!ValidFrameB) A valid frame indication is set if a valid frame was received on channel B. 1 = Valid frame received on channel B 0 = No valid frame received on channel B"]
    #[inline(always)]
    pub fn vfrb(&self) -> VFRB_R {
        VFRB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Syntax Error Observed on Channel A (vSS!SyntaxErrorA) A syntax error was observed in the assigned slot on channel A. 1 = Syntax error observed on channel A 0 = No syntax error observed on channel A"]
    #[inline(always)]
    pub fn seoa(&self) -> SEOA_R {
        SEOA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Syntax Error Observed on Channel B (vSS!SyntaxErrorB) A syntax error was observed in the assigned slot on channel B. 1 = Syntax error observed on channel B 0 = No syntax error observed on channel B"]
    #[inline(always)]
    pub fn seob(&self) -> SEOB_R {
        SEOB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Content Error Observed on Channel A (vSS!ContentErrorA) A content error was observed in the assigned slot on channel A. 1 = Content error observed on channel A 0 = No content error observed on channel A"]
    #[inline(always)]
    pub fn ceoa(&self) -> CEOA_R {
        CEOA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Content Error Observed on Channel B (vSS!ContentErrorB) A content error was observed in the assigned slot on channel B. 1 = Content error observed on channel B 0 = No content error observed on channel B"]
    #[inline(always)]
    pub fn ceob(&self) -> CEOB_R {
        CEOB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slot Boundary Violation Observed on Channel A (vSS!BViolationA) A slot boundary violation (channel active at the start or at the end of the assigned slot) was observed on channel A. 1 = Slot boundary violation observed on channel A 0 = No slot boundary violation observed on channel A"]
    #[inline(always)]
    pub fn svoa(&self) -> SVOA_R {
        SVOA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slot Boundary Violation Observed on Channel B (vSS!BViolationB) A slot boundary violation (channel active at the start or at the end of the assigned slot) was observed on channel B. 1 = Slot boundary violation observed on channel B 0 = No slot boundary violation observed on channel B"]
    #[inline(always)]
    pub fn svob(&self) -> SVOB_R {
        SVOB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission Conflict Indication Channel A (vSS!TxConflictA) A transmission conflict indication is set if a transmission conflict has occurred on channel A. 1 = Transmission conflict occurred on channel A 0 = No transmission conflict occurred on channel A"]
    #[inline(always)]
    pub fn tcia(&self) -> TCIA_R {
        TCIA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Conflict Indication Channel B (vSS!TxConflictB) A transmission conflict indication is set if a transmission conflict has occurred on channel B. 1 = Transmission conflict occurred on channel B 0 = No transmission conflict occurred on channel B"]
    #[inline(always)]
    pub fn tcib(&self) -> TCIB_R {
        TCIB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Empty Slot Channel A In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots. 1 = No bus activity detected in the assigned slot on channel A 0 = Bus activity detected in the assigned slot on channel A"]
    #[inline(always)]
    pub fn esa(&self) -> ESA_R {
        ESA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Empty Slot Channel B In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots. 1 = No bus activity detected in the assigned slot on channel B 0 = Bus activity detected in the assigned slot on channel B"]
    #[inline(always)]
    pub fn esb(&self) -> ESB_R {
        ESB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Message Lost The flag is set in case the Host did not read the message before the message buffer was updated from a received data frame. Not affected by reception of null frames except for message buffers belonging to the receive FIFO. The flag is reset by a Host write to the message buffer via IBF or when a new message is stored into the message buffer after the message buffers ND flag was reset by reading out the message buffer via OBF. 1 = Unprocessed message was overwritten 0 = No message lost"]
    #[inline(always)]
    pub fn mlst(&self) -> MLST_R {
        MLST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Frame Transmitted on Channel A Indicates that this node has transmitted a data frame in the configured slot on channel A. 1 = Data frame transmitted on channel A 0 = No data frame transmitted on channel A"]
    #[inline(always)]
    pub fn fta(&self) -> FTA_R {
        FTA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Frame Transmitted on Channel B Indicates that this node has transmitted a data frame in the configured slot on channel B. 1 = Data frame transmitted on channel B 0 = No data frame transmitted on channel B"]
    #[inline(always)]
    pub fn ftb(&self) -> FTB_R {
        FTB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Cycle Count Status Actual cycle count when status was updated. For receive buffers (CFG = '0') the following status bits are updated from both valid data and null frames. If no valid frame was received, the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Received on Channel Indicator Status (vSS!Channel) Indicates the channel on which the frame was received. 1 = Frame received on channel A 0 = Frame received on channel B"]
    #[inline(always)]
    pub fn rcis(&self) -> RCIS_R {
        RCIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Startup Frame Indicator Status (vRF!Header!SuFIndicator) A startup frame is marked by the startup frame indicator. 1 = The received frame is a startup frame 0 = No startup frame received"]
    #[inline(always)]
    pub fn sfis(&self) -> SFIS_R {
        SFIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Sync Frame Indicator Status (vRF!Header!SyFIndicator) A sync frame is marked by the sync frame indicator. 1 = The received frame is a sync frame 0 = No sync frame received"]
    #[inline(always)]
    pub fn syns(&self) -> SYNS_R {
        SYNS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Null Frame Indicator Status (vRF!Header!NFIndicator) If set to '0' the payload segment of the received frame contains no usable data. 1 = Received frame is not a null frame 0 = Received frame is a null frame"]
    #[inline(always)]
    pub fn nfis(&self) -> NFIS_R {
        NFIS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Payload Preamble Indicator Status (vRF!Header!PPIndicator) The payload preamble indicator defines whether a network management vector or message ID is contained within the payload segment of the received frame. 1 = Static segment: Network management vector at the beginning of the payload Dynamic segment: Message ID at the beginning of the payload 0 = The payload segment of the received frame does not contain a network management vector or a message ID"]
    #[inline(always)]
    pub fn ppis(&self) -> PPIS_R {
        PPIS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn ress(&self) -> RESS_R {
        RESS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Message Buffer Status (mirror)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbs_mir](index.html) module"]
pub struct MBS_MIR_SPEC;
impl crate::RegisterSpec for MBS_MIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbs_mir::R](R) reader structure"]
impl crate::Readable for MBS_MIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MBS_MIR to value 0"]
impl crate::Resettable for MBS_MIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
