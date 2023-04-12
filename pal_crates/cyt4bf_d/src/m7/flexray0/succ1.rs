#[doc = "Register `SUCC1` reader"]
pub struct R(crate::R<SUCC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUCC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUCC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUCC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUCC1` writer"]
pub struct W(crate::W<SUCC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUCC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SUCC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUCC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - N/A"]
pub type CMD_R = crate::FieldReader<u8, CMD_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: N/A"]
    CMD_NOT_ACCEPTED = 0,
    #[doc = "1: N/A"]
    CONFIG = 1,
    #[doc = "2: N/A"]
    READY = 2,
    #[doc = "3: N/A"]
    WAKEUP = 3,
    #[doc = "4: N/A"]
    RUN = 4,
    #[doc = "5: N/A"]
    ALL_SLOTS = 5,
    #[doc = "6: N/A"]
    HALT = 6,
    #[doc = "7: N/A"]
    FREEZE = 7,
    #[doc = "8: N/A"]
    SEND_MTS = 8,
    #[doc = "9: N/A"]
    ALLOW_COLDSTART = 9,
    #[doc = "10: N/A"]
    RESET_STATUS_INDICATORS = 10,
    #[doc = "11: N/A"]
    MONITOR_MODE = 11,
    #[doc = "12: N/A"]
    CLEAR_RAMS = 12,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::CMD_NOT_ACCEPTED),
            1 => Some(CMD_A::CONFIG),
            2 => Some(CMD_A::READY),
            3 => Some(CMD_A::WAKEUP),
            4 => Some(CMD_A::RUN),
            5 => Some(CMD_A::ALL_SLOTS),
            6 => Some(CMD_A::HALT),
            7 => Some(CMD_A::FREEZE),
            8 => Some(CMD_A::SEND_MTS),
            9 => Some(CMD_A::ALLOW_COLDSTART),
            10 => Some(CMD_A::RESET_STATUS_INDICATORS),
            11 => Some(CMD_A::MONITOR_MODE),
            12 => Some(CMD_A::CLEAR_RAMS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMD_NOT_ACCEPTED`"]
    #[inline(always)]
    pub fn is_cmd_not_accepted(&self) -> bool {
        *self == CMD_A::CMD_NOT_ACCEPTED
    }
    #[doc = "Checks if the value of the field is `CONFIG`"]
    #[inline(always)]
    pub fn is_config(&self) -> bool {
        *self == CMD_A::CONFIG
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CMD_A::READY
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == CMD_A::WAKEUP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == CMD_A::RUN
    }
    #[doc = "Checks if the value of the field is `ALL_SLOTS`"]
    #[inline(always)]
    pub fn is_all_slots(&self) -> bool {
        *self == CMD_A::ALL_SLOTS
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == CMD_A::HALT
    }
    #[doc = "Checks if the value of the field is `FREEZE`"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == CMD_A::FREEZE
    }
    #[doc = "Checks if the value of the field is `SEND_MTS`"]
    #[inline(always)]
    pub fn is_send_mts(&self) -> bool {
        *self == CMD_A::SEND_MTS
    }
    #[doc = "Checks if the value of the field is `ALLOW_COLDSTART`"]
    #[inline(always)]
    pub fn is_allow_coldstart(&self) -> bool {
        *self == CMD_A::ALLOW_COLDSTART
    }
    #[doc = "Checks if the value of the field is `RESET_STATUS_INDICATORS`"]
    #[inline(always)]
    pub fn is_reset_status_indicators(&self) -> bool {
        *self == CMD_A::RESET_STATUS_INDICATORS
    }
    #[doc = "Checks if the value of the field is `MONITOR_MODE`"]
    #[inline(always)]
    pub fn is_monitor_mode(&self) -> bool {
        *self == CMD_A::MONITOR_MODE
    }
    #[doc = "Checks if the value of the field is `CLEAR_RAMS`"]
    #[inline(always)]
    pub fn is_clear_rams(&self) -> bool {
        *self == CMD_A::CLEAR_RAMS
    }
}
#[doc = "Field `CMD` writer - N/A"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUCC1_SPEC, u8, CMD_A, 4, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn cmd_not_accepted(self) -> &'a mut W {
        self.variant(CMD_A::CMD_NOT_ACCEPTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn config(self) -> &'a mut W {
        self.variant(CMD_A::CONFIG)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(CMD_A::READY)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(CMD_A::WAKEUP)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CMD_A::RUN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn all_slots(self) -> &'a mut W {
        self.variant(CMD_A::ALL_SLOTS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(CMD_A::HALT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut W {
        self.variant(CMD_A::FREEZE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn send_mts(self) -> &'a mut W {
        self.variant(CMD_A::SEND_MTS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn allow_coldstart(self) -> &'a mut W {
        self.variant(CMD_A::ALLOW_COLDSTART)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn reset_status_indicators(self) -> &'a mut W {
        self.variant(CMD_A::RESET_STATUS_INDICATORS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn monitor_mode(self) -> &'a mut W {
        self.variant(CMD_A::MONITOR_MODE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn clear_rams(self) -> &'a mut W {
        self.variant(CMD_A::CLEAR_RAMS)
    }
}
#[doc = "Field `PBSY` reader - POC Busy Signals that the POC is busy and cannot accept a command from the Host. CMD\\[3:0\\]
is locked against write accesses. Set to '1' after hard reset during initialization of internal RAM blocks. 1 = POC is busy, CMD\\[3:0\\]
locked 0 = POC not busy, CMD\\[3:0\\]
writeable"]
pub type PBSY_R = crate::BitReader<PBSY_A>;
#[doc = "POC Busy Signals that the POC is busy and cannot accept a command from the Host. CMD\\[3:0\\]
is locked against write accesses. Set to '1' after hard reset during initialization of internal RAM blocks. 1 = POC is busy, CMD\\[3:0\\]
locked 0 = POC not busy, CMD\\[3:0\\]
writeable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBSY_A {
    #[doc = "0: N/A"]
    POC_NOT_BUSY = 0,
    #[doc = "1: N/A"]
    POC_BUSY = 1,
}
impl From<PBSY_A> for bool {
    #[inline(always)]
    fn from(variant: PBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl PBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBSY_A {
        match self.bits {
            false => PBSY_A::POC_NOT_BUSY,
            true => PBSY_A::POC_BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `POC_NOT_BUSY`"]
    #[inline(always)]
    pub fn is_poc_not_busy(&self) -> bool {
        *self == PBSY_A::POC_NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `POC_BUSY`"]
    #[inline(always)]
    pub fn is_poc_busy(&self) -> bool {
        *self == PBSY_A::POC_BUSY
    }
}
#[doc = "Field `TXST` reader - Transmit Startup Frame in Key Slot (pKeySlotUsedForStartup) Defines whether the key slot is used to transmit startup frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit startup frame, node is leading or following coldstarter 0 = No startup frame transmission in key slot, node is non-coldstarter"]
pub type TXST_R = crate::BitReader<TXST_A>;
#[doc = "Transmit Startup Frame in Key Slot (pKeySlotUsedForStartup) Defines whether the key slot is used to transmit startup frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit startup frame, node is leading or following coldstarter 0 = No startup frame transmission in key slot, node is non-coldstarter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXST_A {
    #[doc = "0: N/A"]
    NO_STARTUP_FRAME_TXMN = 0,
    #[doc = "1: N/A"]
    STARTUP_FRAME_TXMN = 1,
}
impl From<TXST_A> for bool {
    #[inline(always)]
    fn from(variant: TXST_A) -> Self {
        variant as u8 != 0
    }
}
impl TXST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXST_A {
        match self.bits {
            false => TXST_A::NO_STARTUP_FRAME_TXMN,
            true => TXST_A::STARTUP_FRAME_TXMN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STARTUP_FRAME_TXMN`"]
    #[inline(always)]
    pub fn is_no_startup_frame_txmn(&self) -> bool {
        *self == TXST_A::NO_STARTUP_FRAME_TXMN
    }
    #[doc = "Checks if the value of the field is `STARTUP_FRAME_TXMN`"]
    #[inline(always)]
    pub fn is_startup_frame_txmn(&self) -> bool {
        *self == TXST_A::STARTUP_FRAME_TXMN
    }
}
#[doc = "Field `TXST` writer - Transmit Startup Frame in Key Slot (pKeySlotUsedForStartup) Defines whether the key slot is used to transmit startup frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit startup frame, node is leading or following coldstarter 0 = No startup frame transmission in key slot, node is non-coldstarter"]
pub type TXST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, TXST_A, O>;
impl<'a, const O: u8> TXST_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_startup_frame_txmn(self) -> &'a mut W {
        self.variant(TXST_A::NO_STARTUP_FRAME_TXMN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn startup_frame_txmn(self) -> &'a mut W {
        self.variant(TXST_A::STARTUP_FRAME_TXMN)
    }
}
#[doc = "Field `TXSY` reader - Transmit Sync Frame in Key Slot (pKeySlotUsedForSync) Defines whether the key slot is used to transmit sync frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit sync frame, node is sync node 0 = No sync frame transmission in key slot, node is neither sync nor coldstart node"]
pub type TXSY_R = crate::BitReader<TXSY_A>;
#[doc = "Transmit Sync Frame in Key Slot (pKeySlotUsedForSync) Defines whether the key slot is used to transmit sync frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit sync frame, node is sync node 0 = No sync frame transmission in key slot, node is neither sync nor coldstart node\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSY_A {
    #[doc = "0: N/A"]
    NO_SYNC_FRAME_TXMN = 0,
    #[doc = "1: N/A"]
    SYNC_FRAME_TXMN = 1,
}
impl From<TXSY_A> for bool {
    #[inline(always)]
    fn from(variant: TXSY_A) -> Self {
        variant as u8 != 0
    }
}
impl TXSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSY_A {
        match self.bits {
            false => TXSY_A::NO_SYNC_FRAME_TXMN,
            true => TXSY_A::SYNC_FRAME_TXMN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNC_FRAME_TXMN`"]
    #[inline(always)]
    pub fn is_no_sync_frame_txmn(&self) -> bool {
        *self == TXSY_A::NO_SYNC_FRAME_TXMN
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAME_TXMN`"]
    #[inline(always)]
    pub fn is_sync_frame_txmn(&self) -> bool {
        *self == TXSY_A::SYNC_FRAME_TXMN
    }
}
#[doc = "Field `TXSY` writer - Transmit Sync Frame in Key Slot (pKeySlotUsedForSync) Defines whether the key slot is used to transmit sync frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit sync frame, node is sync node 0 = No sync frame transmission in key slot, node is neither sync nor coldstart node"]
pub type TXSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, TXSY_A, O>;
impl<'a, const O: u8> TXSY_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_sync_frame_txmn(self) -> &'a mut W {
        self.variant(TXSY_A::NO_SYNC_FRAME_TXMN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sync_frame_txmn(self) -> &'a mut W {
        self.variant(TXSY_A::SYNC_FRAME_TXMN)
    }
}
#[doc = "Field `CSA` reader - Cold Start Attempts (gColdStartAttempts) Configures the maximum number of attempts that a cold starting node is permitted to try to start up the network without receiving any valid response from another node. It can be modified in DEFAULT_CONFIG or CONFIG state only. Must be identical in all nodes of a cluster. Valid values are 2 to 31."]
pub type CSA_R = crate::FieldReader<u8, CSA_A>;
#[doc = "Cold Start Attempts (gColdStartAttempts) Configures the maximum number of attempts that a cold starting node is permitted to try to start up the network without receiving any valid response from another node. It can be modified in DEFAULT_CONFIG or CONFIG state only. Must be identical in all nodes of a cluster. Valid values are 2 to 31.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSA_A {
    #[doc = "2: N/A"]
    MIN = 2,
}
impl From<CSA_A> for u8 {
    #[inline(always)]
    fn from(variant: CSA_A) -> Self {
        variant as _
    }
}
impl CSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSA_A> {
        match self.bits {
            2 => Some(CSA_A::MIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == CSA_A::MIN
    }
}
#[doc = "Field `CSA` writer - Cold Start Attempts (gColdStartAttempts) Configures the maximum number of attempts that a cold starting node is permitted to try to start up the network without receiving any valid response from another node. It can be modified in DEFAULT_CONFIG or CONFIG state only. Must be identical in all nodes of a cluster. Valid values are 2 to 31."]
pub type CSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUCC1_SPEC, u8, CSA_A, 5, O>;
impl<'a, const O: u8> CSA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(CSA_A::MIN)
    }
}
#[doc = "Field `PTA` reader - Passive to Active (pAllowPassiveToActive) Defines the number of consecutive even / odd cycle pairs that must have valid clock correction terms before the CC is allowed to transit from NORMAL_PASSIVE to NORMAL_ACTIVE state. If set to '00000' the CC is not allowed to transit from NORMAL_PASSIVE to NORMAL_ACTIVE state. It can be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 31 even / odd cycle pairs."]
pub type PTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTA` writer - Passive to Active (pAllowPassiveToActive) Defines the number of consecutive even / odd cycle pairs that must have valid clock correction terms before the CC is allowed to transit from NORMAL_PASSIVE to NORMAL_ACTIVE state. If set to '00000' the CC is not allowed to transit from NORMAL_PASSIVE to NORMAL_ACTIVE state. It can be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 31 even / odd cycle pairs."]
pub type PTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUCC1_SPEC, u8, u8, 5, O>;
#[doc = "Field `WUCS` reader - Wakeup Channel Select (pWakeupChannel) With this bit the Host selects the channel on which the CC sends the Wakeup pattern. The CC ignores any attempt to change the status of this bit when not in DEFAULT_CONFIG or CONFIG state. 1 = Send wakeup pattern on channel B 0 = Send wakeup pattern on channel A"]
pub type WUCS_R = crate::BitReader<WUCS_A>;
#[doc = "Wakeup Channel Select (pWakeupChannel) With this bit the Host selects the channel on which the CC sends the Wakeup pattern. The CC ignores any attempt to change the status of this bit when not in DEFAULT_CONFIG or CONFIG state. 1 = Send wakeup pattern on channel B 0 = Send wakeup pattern on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUCS_A {
    #[doc = "0: N/A"]
    SEND_WKUP_PATTERN_ON_CH_A = 0,
    #[doc = "1: N/A"]
    SEND_WKUP_PATTERN_ON_CH_B = 1,
}
impl From<WUCS_A> for bool {
    #[inline(always)]
    fn from(variant: WUCS_A) -> Self {
        variant as u8 != 0
    }
}
impl WUCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUCS_A {
        match self.bits {
            false => WUCS_A::SEND_WKUP_PATTERN_ON_CH_A,
            true => WUCS_A::SEND_WKUP_PATTERN_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `SEND_WKUP_PATTERN_ON_CH_A`"]
    #[inline(always)]
    pub fn is_send_wkup_pattern_on_ch_a(&self) -> bool {
        *self == WUCS_A::SEND_WKUP_PATTERN_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `SEND_WKUP_PATTERN_ON_CH_B`"]
    #[inline(always)]
    pub fn is_send_wkup_pattern_on_ch_b(&self) -> bool {
        *self == WUCS_A::SEND_WKUP_PATTERN_ON_CH_B
    }
}
#[doc = "Field `WUCS` writer - Wakeup Channel Select (pWakeupChannel) With this bit the Host selects the channel on which the CC sends the Wakeup pattern. The CC ignores any attempt to change the status of this bit when not in DEFAULT_CONFIG or CONFIG state. 1 = Send wakeup pattern on channel B 0 = Send wakeup pattern on channel A"]
pub type WUCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, WUCS_A, O>;
impl<'a, const O: u8> WUCS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn send_wkup_pattern_on_ch_a(self) -> &'a mut W {
        self.variant(WUCS_A::SEND_WKUP_PATTERN_ON_CH_A)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn send_wkup_pattern_on_ch_b(self) -> &'a mut W {
        self.variant(WUCS_A::SEND_WKUP_PATTERN_ON_CH_B)
    }
}
#[doc = "Field `TSM` reader - Transmission Slot Mode (pSingleSlotEnabled) Selects the initial transmission slot mode. In SINGLE slot mode the CC may only transmit in the preconfigured key slot. The key slot ID is configured in the header section of message buffer 0 respectively message buffers 0 and 1 depending on bit MRC.SPLM. In case TSM = '1', message buffer 0 respectively message buffers 0,1 can be (re)configured in DEFAULT_CONFIG or CONFIG state only. In ALL slot mode the CC may transmit in all slots. TSM is a configuration bit which can only be set / reset by the Host. The bit can be written in DEFAULT_CONFIG or CONFIG state only. The CC changes to ALL slot mode when the Host successfully applied the ALL_SLOTS command by writing CMD\\[3:0\\]
= '0101' in POC states NORMAL_ACTIVE or NORMAL_PASSIVE. The actual slot mode is monitored by CCSV.SLM\\[1:0\\]. 1 = SINGLE Slot Mode (default after hard reset) 0 = ALL Slot Mode"]
pub type TSM_R = crate::BitReader<TSM_A>;
#[doc = "Transmission Slot Mode (pSingleSlotEnabled) Selects the initial transmission slot mode. In SINGLE slot mode the CC may only transmit in the preconfigured key slot. The key slot ID is configured in the header section of message buffer 0 respectively message buffers 0 and 1 depending on bit MRC.SPLM. In case TSM = '1', message buffer 0 respectively message buffers 0,1 can be (re)configured in DEFAULT_CONFIG or CONFIG state only. In ALL slot mode the CC may transmit in all slots. TSM is a configuration bit which can only be set / reset by the Host. The bit can be written in DEFAULT_CONFIG or CONFIG state only. The CC changes to ALL slot mode when the Host successfully applied the ALL_SLOTS command by writing CMD\\[3:0\\]
= '0101' in POC states NORMAL_ACTIVE or NORMAL_PASSIVE. The actual slot mode is monitored by CCSV.SLM\\[1:0\\]. 1 = SINGLE Slot Mode (default after hard reset) 0 = ALL Slot Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSM_A {
    #[doc = "0: N/A"]
    ALL_SLOT_MODE = 0,
    #[doc = "1: N/A"]
    SINGLE_SLOT_MODE = 1,
}
impl From<TSM_A> for bool {
    #[inline(always)]
    fn from(variant: TSM_A) -> Self {
        variant as u8 != 0
    }
}
impl TSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSM_A {
        match self.bits {
            false => TSM_A::ALL_SLOT_MODE,
            true => TSM_A::SINGLE_SLOT_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_SLOT_MODE`"]
    #[inline(always)]
    pub fn is_all_slot_mode(&self) -> bool {
        *self == TSM_A::ALL_SLOT_MODE
    }
    #[doc = "Checks if the value of the field is `SINGLE_SLOT_MODE`"]
    #[inline(always)]
    pub fn is_single_slot_mode(&self) -> bool {
        *self == TSM_A::SINGLE_SLOT_MODE
    }
}
#[doc = "Field `TSM` writer - Transmission Slot Mode (pSingleSlotEnabled) Selects the initial transmission slot mode. In SINGLE slot mode the CC may only transmit in the preconfigured key slot. The key slot ID is configured in the header section of message buffer 0 respectively message buffers 0 and 1 depending on bit MRC.SPLM. In case TSM = '1', message buffer 0 respectively message buffers 0,1 can be (re)configured in DEFAULT_CONFIG or CONFIG state only. In ALL slot mode the CC may transmit in all slots. TSM is a configuration bit which can only be set / reset by the Host. The bit can be written in DEFAULT_CONFIG or CONFIG state only. The CC changes to ALL slot mode when the Host successfully applied the ALL_SLOTS command by writing CMD\\[3:0\\]
= '0101' in POC states NORMAL_ACTIVE or NORMAL_PASSIVE. The actual slot mode is monitored by CCSV.SLM\\[1:0\\]. 1 = SINGLE Slot Mode (default after hard reset) 0 = ALL Slot Mode"]
pub type TSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, TSM_A, O>;
impl<'a, const O: u8> TSM_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn all_slot_mode(self) -> &'a mut W {
        self.variant(TSM_A::ALL_SLOT_MODE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn single_slot_mode(self) -> &'a mut W {
        self.variant(TSM_A::SINGLE_SLOT_MODE)
    }
}
#[doc = "Field `HCSE` reader - Halt due to Clock Sync Error (pAllowHaltDueToClock) Controls the transition to HALT state due to a clock synchronization error. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = CC will enter HALT state 0 = CC will enter / remain in NORMAL_PASSIVE"]
pub type HCSE_R = crate::BitReader<HCSE_A>;
#[doc = "Halt due to Clock Sync Error (pAllowHaltDueToClock) Controls the transition to HALT state due to a clock synchronization error. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = CC will enter HALT state 0 = CC will enter / remain in NORMAL_PASSIVE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCSE_A {
    #[doc = "0: N/A"]
    CC_TO_ENTER_NORMAL_PASSIVE = 0,
    #[doc = "1: N/A"]
    CC_TO_ENTER_HALT = 1,
}
impl From<HCSE_A> for bool {
    #[inline(always)]
    fn from(variant: HCSE_A) -> Self {
        variant as u8 != 0
    }
}
impl HCSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCSE_A {
        match self.bits {
            false => HCSE_A::CC_TO_ENTER_NORMAL_PASSIVE,
            true => HCSE_A::CC_TO_ENTER_HALT,
        }
    }
    #[doc = "Checks if the value of the field is `CC_TO_ENTER_NORMAL_PASSIVE`"]
    #[inline(always)]
    pub fn is_cc_to_enter_normal_passive(&self) -> bool {
        *self == HCSE_A::CC_TO_ENTER_NORMAL_PASSIVE
    }
    #[doc = "Checks if the value of the field is `CC_TO_ENTER_HALT`"]
    #[inline(always)]
    pub fn is_cc_to_enter_halt(&self) -> bool {
        *self == HCSE_A::CC_TO_ENTER_HALT
    }
}
#[doc = "Field `HCSE` writer - Halt due to Clock Sync Error (pAllowHaltDueToClock) Controls the transition to HALT state due to a clock synchronization error. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = CC will enter HALT state 0 = CC will enter / remain in NORMAL_PASSIVE"]
pub type HCSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, HCSE_A, O>;
impl<'a, const O: u8> HCSE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn cc_to_enter_normal_passive(self) -> &'a mut W {
        self.variant(HCSE_A::CC_TO_ENTER_NORMAL_PASSIVE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn cc_to_enter_halt(self) -> &'a mut W {
        self.variant(HCSE_A::CC_TO_ENTER_HALT)
    }
}
#[doc = "Field `MTSA_` reader - Select Channel A for MTS Transmission The bit selects channel A for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel A selected for MTS transmission 0 = Channel A disabled for MTS transmission"]
pub type MTSA__R = crate::BitReader<MTSA__A>;
#[doc = "Select Channel A for MTS Transmission The bit selects channel A for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel A selected for MTS transmission 0 = Channel A disabled for MTS transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTSA__A {
    #[doc = "0: N/A"]
    CH_A_DISABLED_FOR_MTS_TXMN = 0,
    #[doc = "1: N/A"]
    CH_A_SELECTED_FOR_MTS_TXMN = 1,
}
impl From<MTSA__A> for bool {
    #[inline(always)]
    fn from(variant: MTSA__A) -> Self {
        variant as u8 != 0
    }
}
impl MTSA__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSA__A {
        match self.bits {
            false => MTSA__A::CH_A_DISABLED_FOR_MTS_TXMN,
            true => MTSA__A::CH_A_SELECTED_FOR_MTS_TXMN,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_DISABLED_FOR_MTS_TXMN`"]
    #[inline(always)]
    pub fn is_ch_a_disabled_for_mts_txmn(&self) -> bool {
        *self == MTSA__A::CH_A_DISABLED_FOR_MTS_TXMN
    }
    #[doc = "Checks if the value of the field is `CH_A_SELECTED_FOR_MTS_TXMN`"]
    #[inline(always)]
    pub fn is_ch_a_selected_for_mts_txmn(&self) -> bool {
        *self == MTSA__A::CH_A_SELECTED_FOR_MTS_TXMN
    }
}
#[doc = "Field `MTSA_` writer - Select Channel A for MTS Transmission The bit selects channel A for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel A selected for MTS transmission 0 = Channel A disabled for MTS transmission"]
pub type MTSA__W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, MTSA__A, O>;
impl<'a, const O: u8> MTSA__W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_disabled_for_mts_txmn(self) -> &'a mut W {
        self.variant(MTSA__A::CH_A_DISABLED_FOR_MTS_TXMN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_selected_for_mts_txmn(self) -> &'a mut W {
        self.variant(MTSA__A::CH_A_SELECTED_FOR_MTS_TXMN)
    }
}
#[doc = "Field `MTSB_` reader - Select Channel B for MTS Transmission The bit selects channel B for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel B selected for MTS transmission 0 = Channel B disabled for MTS transmission"]
pub type MTSB__R = crate::BitReader<MTSB__A>;
#[doc = "Select Channel B for MTS Transmission The bit selects channel B for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel B selected for MTS transmission 0 = Channel B disabled for MTS transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTSB__A {
    #[doc = "0: N/A"]
    CH_B_DISABLED_FOR_MTS_TXMN = 0,
    #[doc = "1: N/A"]
    CH_B_SELECTED_FOR_MTS_TXMN = 1,
}
impl From<MTSB__A> for bool {
    #[inline(always)]
    fn from(variant: MTSB__A) -> Self {
        variant as u8 != 0
    }
}
impl MTSB__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSB__A {
        match self.bits {
            false => MTSB__A::CH_B_DISABLED_FOR_MTS_TXMN,
            true => MTSB__A::CH_B_SELECTED_FOR_MTS_TXMN,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_DISABLED_FOR_MTS_TXMN`"]
    #[inline(always)]
    pub fn is_ch_b_disabled_for_mts_txmn(&self) -> bool {
        *self == MTSB__A::CH_B_DISABLED_FOR_MTS_TXMN
    }
    #[doc = "Checks if the value of the field is `CH_B_SELECTED_FOR_MTS_TXMN`"]
    #[inline(always)]
    pub fn is_ch_b_selected_for_mts_txmn(&self) -> bool {
        *self == MTSB__A::CH_B_SELECTED_FOR_MTS_TXMN
    }
}
#[doc = "Field `MTSB_` writer - Select Channel B for MTS Transmission The bit selects channel B for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel B selected for MTS transmission 0 = Channel B disabled for MTS transmission"]
pub type MTSB__W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, MTSB__A, O>;
impl<'a, const O: u8> MTSB__W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_disabled_for_mts_txmn(self) -> &'a mut W {
        self.variant(MTSB__A::CH_B_DISABLED_FOR_MTS_TXMN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_selected_for_mts_txmn(self) -> &'a mut W {
        self.variant(MTSB__A::CH_B_SELECTED_FOR_MTS_TXMN)
    }
}
#[doc = "Field `CCHA` reader - Connected to Channel A (pChannels) Configures whether the node is connected to channel A. 1 = Node connected to channel A (default after hard reset) 0 = Not connected to channel A"]
pub type CCHA_R = crate::BitReader<CCHA_A>;
#[doc = "Connected to Channel A (pChannels) Configures whether the node is connected to channel A. 1 = Node connected to channel A (default after hard reset) 0 = Not connected to channel A\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCHA_A {
    #[doc = "0: N/A"]
    CH_A_NOT_CONNECTED = 0,
    #[doc = "1: N/A"]
    CH_A_CONNECTED = 1,
}
impl From<CCHA_A> for bool {
    #[inline(always)]
    fn from(variant: CCHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CCHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCHA_A {
        match self.bits {
            false => CCHA_A::CH_A_NOT_CONNECTED,
            true => CCHA_A::CH_A_CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_NOT_CONNECTED`"]
    #[inline(always)]
    pub fn is_ch_a_not_connected(&self) -> bool {
        *self == CCHA_A::CH_A_NOT_CONNECTED
    }
    #[doc = "Checks if the value of the field is `CH_A_CONNECTED`"]
    #[inline(always)]
    pub fn is_ch_a_connected(&self) -> bool {
        *self == CCHA_A::CH_A_CONNECTED
    }
}
#[doc = "Field `CCHA` writer - Connected to Channel A (pChannels) Configures whether the node is connected to channel A. 1 = Node connected to channel A (default after hard reset) 0 = Not connected to channel A"]
pub type CCHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, CCHA_A, O>;
impl<'a, const O: u8> CCHA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_not_connected(self) -> &'a mut W {
        self.variant(CCHA_A::CH_A_NOT_CONNECTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_connected(self) -> &'a mut W {
        self.variant(CCHA_A::CH_A_CONNECTED)
    }
}
#[doc = "Field `CCHB` reader - Connected to Channel B (pChannels) Configures whether the node is connected to channel B. 1 = Node connected to channel B (default after hard reset) 0 = Not connected to channel B"]
pub type CCHB_R = crate::BitReader<CCHB_A>;
#[doc = "Connected to Channel B (pChannels) Configures whether the node is connected to channel B. 1 = Node connected to channel B (default after hard reset) 0 = Not connected to channel B\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCHB_A {
    #[doc = "0: N/A"]
    CH_B_NOT_CONNECTED = 0,
    #[doc = "1: N/A"]
    CH_B_CONNECTED = 1,
}
impl From<CCHB_A> for bool {
    #[inline(always)]
    fn from(variant: CCHB_A) -> Self {
        variant as u8 != 0
    }
}
impl CCHB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCHB_A {
        match self.bits {
            false => CCHB_A::CH_B_NOT_CONNECTED,
            true => CCHB_A::CH_B_CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_NOT_CONNECTED`"]
    #[inline(always)]
    pub fn is_ch_b_not_connected(&self) -> bool {
        *self == CCHB_A::CH_B_NOT_CONNECTED
    }
    #[doc = "Checks if the value of the field is `CH_B_CONNECTED`"]
    #[inline(always)]
    pub fn is_ch_b_connected(&self) -> bool {
        *self == CCHB_A::CH_B_CONNECTED
    }
}
#[doc = "Field `CCHB` writer - Connected to Channel B (pChannels) Configures whether the node is connected to channel B. 1 = Node connected to channel B (default after hard reset) 0 = Not connected to channel B"]
pub type CCHB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SUCC1_SPEC, CCHB_A, O>;
impl<'a, const O: u8> CCHB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_not_connected(self) -> &'a mut W {
        self.variant(CCHB_A::CH_B_NOT_CONNECTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_connected(self) -> &'a mut W {
        self.variant(CCHB_A::CH_B_CONNECTED)
    }
}
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - POC Busy Signals that the POC is busy and cannot accept a command from the Host. CMD\\[3:0\\]
is locked against write accesses. Set to '1' after hard reset during initialization of internal RAM blocks. 1 = POC is busy, CMD\\[3:0\\]
locked 0 = POC not busy, CMD\\[3:0\\]
writeable"]
    #[inline(always)]
    pub fn pbsy(&self) -> PBSY_R {
        PBSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Startup Frame in Key Slot (pKeySlotUsedForStartup) Defines whether the key slot is used to transmit startup frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit startup frame, node is leading or following coldstarter 0 = No startup frame transmission in key slot, node is non-coldstarter"]
    #[inline(always)]
    pub fn txst(&self) -> TXST_R {
        TXST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Sync Frame in Key Slot (pKeySlotUsedForSync) Defines whether the key slot is used to transmit sync frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit sync frame, node is sync node 0 = No sync frame transmission in key slot, node is neither sync nor coldstart node"]
    #[inline(always)]
    pub fn txsy(&self) -> TXSY_R {
        TXSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:15 - Cold Start Attempts (gColdStartAttempts) Configures the maximum number of attempts that a cold starting node is permitted to try to start up the network without receiving any valid response from another node. It can be modified in DEFAULT_CONFIG or CONFIG state only. Must be identical in all nodes of a cluster. Valid values are 2 to 31."]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Passive to Active (pAllowPassiveToActive) Defines the number of consecutive even / odd cycle pairs that must have valid clock correction terms before the CC is allowed to transit from NORMAL_PASSIVE to NORMAL_ACTIVE state. If set to '00000' the CC is not allowed to transit from NORMAL_PASSIVE to NORMAL_ACTIVE state. It can be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 31 even / odd cycle pairs."]
    #[inline(always)]
    pub fn pta(&self) -> PTA_R {
        PTA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Wakeup Channel Select (pWakeupChannel) With this bit the Host selects the channel on which the CC sends the Wakeup pattern. The CC ignores any attempt to change the status of this bit when not in DEFAULT_CONFIG or CONFIG state. 1 = Send wakeup pattern on channel B 0 = Send wakeup pattern on channel A"]
    #[inline(always)]
    pub fn wucs(&self) -> WUCS_R {
        WUCS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Slot Mode (pSingleSlotEnabled) Selects the initial transmission slot mode. In SINGLE slot mode the CC may only transmit in the preconfigured key slot. The key slot ID is configured in the header section of message buffer 0 respectively message buffers 0 and 1 depending on bit MRC.SPLM. In case TSM = '1', message buffer 0 respectively message buffers 0,1 can be (re)configured in DEFAULT_CONFIG or CONFIG state only. In ALL slot mode the CC may transmit in all slots. TSM is a configuration bit which can only be set / reset by the Host. The bit can be written in DEFAULT_CONFIG or CONFIG state only. The CC changes to ALL slot mode when the Host successfully applied the ALL_SLOTS command by writing CMD\\[3:0\\]
= '0101' in POC states NORMAL_ACTIVE or NORMAL_PASSIVE. The actual slot mode is monitored by CCSV.SLM\\[1:0\\]. 1 = SINGLE Slot Mode (default after hard reset) 0 = ALL Slot Mode"]
    #[inline(always)]
    pub fn tsm(&self) -> TSM_R {
        TSM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Halt due to Clock Sync Error (pAllowHaltDueToClock) Controls the transition to HALT state due to a clock synchronization error. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = CC will enter HALT state 0 = CC will enter / remain in NORMAL_PASSIVE"]
    #[inline(always)]
    pub fn hcse(&self) -> HCSE_R {
        HCSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select Channel A for MTS Transmission The bit selects channel A for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel A selected for MTS transmission 0 = Channel A disabled for MTS transmission"]
    #[inline(always)]
    pub fn mtsa_(&self) -> MTSA__R {
        MTSA__R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select Channel B for MTS Transmission The bit selects channel B for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel B selected for MTS transmission 0 = Channel B disabled for MTS transmission"]
    #[inline(always)]
    pub fn mtsb_(&self) -> MTSB__R {
        MTSB__R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Connected to Channel A (pChannels) Configures whether the node is connected to channel A. 1 = Node connected to channel A (default after hard reset) 0 = Not connected to channel A"]
    #[inline(always)]
    pub fn ccha(&self) -> CCHA_R {
        CCHA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Connected to Channel B (pChannels) Configures whether the node is connected to channel B. 1 = Node connected to channel B (default after hard reset) 0 = Not connected to channel B"]
    #[inline(always)]
    pub fn cchb(&self) -> CCHB_R {
        CCHB_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Startup Frame in Key Slot (pKeySlotUsedForStartup) Defines whether the key slot is used to transmit startup frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit startup frame, node is leading or following coldstarter 0 = No startup frame transmission in key slot, node is non-coldstarter"]
    #[inline(always)]
    #[must_use]
    pub fn txst(&mut self) -> TXST_W<8> {
        TXST_W::new(self)
    }
    #[doc = "Bit 9 - Transmit Sync Frame in Key Slot (pKeySlotUsedForSync) Defines whether the key slot is used to transmit sync frames. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = Key slot used to transmit sync frame, node is sync node 0 = No sync frame transmission in key slot, node is neither sync nor coldstart node"]
    #[inline(always)]
    #[must_use]
    pub fn txsy(&mut self) -> TXSY_W<9> {
        TXSY_W::new(self)
    }
    #[doc = "Bits 11:15 - Cold Start Attempts (gColdStartAttempts) Configures the maximum number of attempts that a cold starting node is permitted to try to start up the network without receiving any valid response from another node. It can be modified in DEFAULT_CONFIG or CONFIG state only. Must be identical in all nodes of a cluster. Valid values are 2 to 31."]
    #[inline(always)]
    #[must_use]
    pub fn csa(&mut self) -> CSA_W<11> {
        CSA_W::new(self)
    }
    #[doc = "Bits 16:20 - Passive to Active (pAllowPassiveToActive) Defines the number of consecutive even / odd cycle pairs that must have valid clock correction terms before the CC is allowed to transit from NORMAL_PASSIVE to NORMAL_ACTIVE state. If set to '00000' the CC is not allowed to transit from NORMAL_PASSIVE to NORMAL_ACTIVE state. It can be modified in DEFAULT_CONFIG or CONFIG state only. Valid values are 0 to 31 even / odd cycle pairs."]
    #[inline(always)]
    #[must_use]
    pub fn pta(&mut self) -> PTA_W<16> {
        PTA_W::new(self)
    }
    #[doc = "Bit 21 - Wakeup Channel Select (pWakeupChannel) With this bit the Host selects the channel on which the CC sends the Wakeup pattern. The CC ignores any attempt to change the status of this bit when not in DEFAULT_CONFIG or CONFIG state. 1 = Send wakeup pattern on channel B 0 = Send wakeup pattern on channel A"]
    #[inline(always)]
    #[must_use]
    pub fn wucs(&mut self) -> WUCS_W<21> {
        WUCS_W::new(self)
    }
    #[doc = "Bit 22 - Transmission Slot Mode (pSingleSlotEnabled) Selects the initial transmission slot mode. In SINGLE slot mode the CC may only transmit in the preconfigured key slot. The key slot ID is configured in the header section of message buffer 0 respectively message buffers 0 and 1 depending on bit MRC.SPLM. In case TSM = '1', message buffer 0 respectively message buffers 0,1 can be (re)configured in DEFAULT_CONFIG or CONFIG state only. In ALL slot mode the CC may transmit in all slots. TSM is a configuration bit which can only be set / reset by the Host. The bit can be written in DEFAULT_CONFIG or CONFIG state only. The CC changes to ALL slot mode when the Host successfully applied the ALL_SLOTS command by writing CMD\\[3:0\\]
= '0101' in POC states NORMAL_ACTIVE or NORMAL_PASSIVE. The actual slot mode is monitored by CCSV.SLM\\[1:0\\]. 1 = SINGLE Slot Mode (default after hard reset) 0 = ALL Slot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tsm(&mut self) -> TSM_W<22> {
        TSM_W::new(self)
    }
    #[doc = "Bit 23 - Halt due to Clock Sync Error (pAllowHaltDueToClock) Controls the transition to HALT state due to a clock synchronization error. The bit can be modified in DEFAULT_CONFIG or CONFIG state only. 1 = CC will enter HALT state 0 = CC will enter / remain in NORMAL_PASSIVE"]
    #[inline(always)]
    #[must_use]
    pub fn hcse(&mut self) -> HCSE_W<23> {
        HCSE_W::new(self)
    }
    #[doc = "Bit 24 - Select Channel A for MTS Transmission The bit selects channel A for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel A selected for MTS transmission 0 = Channel A disabled for MTS transmission"]
    #[inline(always)]
    #[must_use]
    pub fn mtsa_(&mut self) -> MTSA__W<24> {
        MTSA__W::new(self)
    }
    #[doc = "Bit 25 - Select Channel B for MTS Transmission The bit selects channel B for MTS symbol transmission. The flag is reset by default and may be modified only in DEFAULT_CONFIG or CONFIG state. 1 = Channel B selected for MTS transmission 0 = Channel B disabled for MTS transmission"]
    #[inline(always)]
    #[must_use]
    pub fn mtsb_(&mut self) -> MTSB__W<25> {
        MTSB__W::new(self)
    }
    #[doc = "Bit 26 - Connected to Channel A (pChannels) Configures whether the node is connected to channel A. 1 = Node connected to channel A (default after hard reset) 0 = Not connected to channel A"]
    #[inline(always)]
    #[must_use]
    pub fn ccha(&mut self) -> CCHA_W<26> {
        CCHA_W::new(self)
    }
    #[doc = "Bit 27 - Connected to Channel B (pChannels) Configures whether the node is connected to channel B. 1 = Node connected to channel B (default after hard reset) 0 = Not connected to channel B"]
    #[inline(always)]
    #[must_use]
    pub fn cchb(&mut self) -> CCHB_W<27> {
        CCHB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SUC Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [succ1](index.html) module"]
pub struct SUCC1_SPEC;
impl crate::RegisterSpec for SUCC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [succ1::R](R) reader structure"]
impl crate::Readable for SUCC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [succ1::W](W) writer structure"]
impl crate::Writable for SUCC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SUCC1 to value 0x0c40_1080"]
impl crate::Resettable for SUCC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c40_1080;
}
