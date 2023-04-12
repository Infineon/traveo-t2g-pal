#[doc = "Register `CCSV` reader"]
pub struct R(crate::R<CCSV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCSV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCSV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCSV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POCS` reader - N/A"]
pub type POCS_R = crate::FieldReader<u8, POCS_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POCS_A {
    #[doc = "0: N/A"]
    DEFAULT_CONFIG = 0,
    #[doc = "1: N/A"]
    READY = 1,
    #[doc = "2: N/A"]
    NORMAL_ACTIVE = 2,
    #[doc = "3: N/A"]
    NORMAL_PASSIVE = 3,
    #[doc = "4: N/A"]
    HALT = 4,
    #[doc = "5: N/A"]
    MONITOR_MODE = 5,
    #[doc = "15: N/A"]
    CONFIG = 15,
    #[doc = "16: N/A"]
    WAKEUP_STANDBY = 16,
    #[doc = "17: N/A"]
    WAKEUP_LISTEN = 17,
    #[doc = "18: N/A"]
    WAKEUP_SEND = 18,
    #[doc = "19: N/A"]
    WAKEUP_DETECT = 19,
    #[doc = "32: N/A"]
    STARTUP_PREPARE = 32,
    #[doc = "33: N/A"]
    COLDSTART_LISTEN = 33,
    #[doc = "34: N/A"]
    COLDSTART_COLLISION_RESOLUTION = 34,
    #[doc = "35: N/A"]
    COLDSTART_CONSISTENCY_CHECK = 35,
    #[doc = "36: N/A"]
    COLDSTART_GAP = 36,
    #[doc = "37: N/A"]
    COLDSTART_JOIN = 37,
    #[doc = "38: N/A"]
    INTEGRATION_COLDSTART_CHECK = 38,
    #[doc = "39: N/A"]
    INTEGRATION_LISTEN = 39,
    #[doc = "40: N/A"]
    INTEGRATION_CONSISTENCY_CHECK = 40,
    #[doc = "41: N/A"]
    INITIALIZE_SCHEDULE = 41,
    #[doc = "42: N/A"]
    ABORT_STARTUP = 42,
    #[doc = "43: N/A"]
    STARTUP_SUCCESS = 43,
}
impl From<POCS_A> for u8 {
    #[inline(always)]
    fn from(variant: POCS_A) -> Self {
        variant as _
    }
}
impl POCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POCS_A> {
        match self.bits {
            0 => Some(POCS_A::DEFAULT_CONFIG),
            1 => Some(POCS_A::READY),
            2 => Some(POCS_A::NORMAL_ACTIVE),
            3 => Some(POCS_A::NORMAL_PASSIVE),
            4 => Some(POCS_A::HALT),
            5 => Some(POCS_A::MONITOR_MODE),
            15 => Some(POCS_A::CONFIG),
            16 => Some(POCS_A::WAKEUP_STANDBY),
            17 => Some(POCS_A::WAKEUP_LISTEN),
            18 => Some(POCS_A::WAKEUP_SEND),
            19 => Some(POCS_A::WAKEUP_DETECT),
            32 => Some(POCS_A::STARTUP_PREPARE),
            33 => Some(POCS_A::COLDSTART_LISTEN),
            34 => Some(POCS_A::COLDSTART_COLLISION_RESOLUTION),
            35 => Some(POCS_A::COLDSTART_CONSISTENCY_CHECK),
            36 => Some(POCS_A::COLDSTART_GAP),
            37 => Some(POCS_A::COLDSTART_JOIN),
            38 => Some(POCS_A::INTEGRATION_COLDSTART_CHECK),
            39 => Some(POCS_A::INTEGRATION_LISTEN),
            40 => Some(POCS_A::INTEGRATION_CONSISTENCY_CHECK),
            41 => Some(POCS_A::INITIALIZE_SCHEDULE),
            42 => Some(POCS_A::ABORT_STARTUP),
            43 => Some(POCS_A::STARTUP_SUCCESS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT_CONFIG`"]
    #[inline(always)]
    pub fn is_default_config(&self) -> bool {
        *self == POCS_A::DEFAULT_CONFIG
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == POCS_A::READY
    }
    #[doc = "Checks if the value of the field is `NORMAL_ACTIVE`"]
    #[inline(always)]
    pub fn is_normal_active(&self) -> bool {
        *self == POCS_A::NORMAL_ACTIVE
    }
    #[doc = "Checks if the value of the field is `NORMAL_PASSIVE`"]
    #[inline(always)]
    pub fn is_normal_passive(&self) -> bool {
        *self == POCS_A::NORMAL_PASSIVE
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == POCS_A::HALT
    }
    #[doc = "Checks if the value of the field is `MONITOR_MODE`"]
    #[inline(always)]
    pub fn is_monitor_mode(&self) -> bool {
        *self == POCS_A::MONITOR_MODE
    }
    #[doc = "Checks if the value of the field is `CONFIG`"]
    #[inline(always)]
    pub fn is_config(&self) -> bool {
        *self == POCS_A::CONFIG
    }
    #[doc = "Checks if the value of the field is `WAKEUP_STANDBY`"]
    #[inline(always)]
    pub fn is_wakeup_standby(&self) -> bool {
        *self == POCS_A::WAKEUP_STANDBY
    }
    #[doc = "Checks if the value of the field is `WAKEUP_LISTEN`"]
    #[inline(always)]
    pub fn is_wakeup_listen(&self) -> bool {
        *self == POCS_A::WAKEUP_LISTEN
    }
    #[doc = "Checks if the value of the field is `WAKEUP_SEND`"]
    #[inline(always)]
    pub fn is_wakeup_send(&self) -> bool {
        *self == POCS_A::WAKEUP_SEND
    }
    #[doc = "Checks if the value of the field is `WAKEUP_DETECT`"]
    #[inline(always)]
    pub fn is_wakeup_detect(&self) -> bool {
        *self == POCS_A::WAKEUP_DETECT
    }
    #[doc = "Checks if the value of the field is `STARTUP_PREPARE`"]
    #[inline(always)]
    pub fn is_startup_prepare(&self) -> bool {
        *self == POCS_A::STARTUP_PREPARE
    }
    #[doc = "Checks if the value of the field is `COLDSTART_LISTEN`"]
    #[inline(always)]
    pub fn is_coldstart_listen(&self) -> bool {
        *self == POCS_A::COLDSTART_LISTEN
    }
    #[doc = "Checks if the value of the field is `COLDSTART_COLLISION_RESOLUTION`"]
    #[inline(always)]
    pub fn is_coldstart_collision_resolution(&self) -> bool {
        *self == POCS_A::COLDSTART_COLLISION_RESOLUTION
    }
    #[doc = "Checks if the value of the field is `COLDSTART_CONSISTENCY_CHECK`"]
    #[inline(always)]
    pub fn is_coldstart_consistency_check(&self) -> bool {
        *self == POCS_A::COLDSTART_CONSISTENCY_CHECK
    }
    #[doc = "Checks if the value of the field is `COLDSTART_GAP`"]
    #[inline(always)]
    pub fn is_coldstart_gap(&self) -> bool {
        *self == POCS_A::COLDSTART_GAP
    }
    #[doc = "Checks if the value of the field is `COLDSTART_JOIN`"]
    #[inline(always)]
    pub fn is_coldstart_join(&self) -> bool {
        *self == POCS_A::COLDSTART_JOIN
    }
    #[doc = "Checks if the value of the field is `INTEGRATION_COLDSTART_CHECK`"]
    #[inline(always)]
    pub fn is_integration_coldstart_check(&self) -> bool {
        *self == POCS_A::INTEGRATION_COLDSTART_CHECK
    }
    #[doc = "Checks if the value of the field is `INTEGRATION_LISTEN`"]
    #[inline(always)]
    pub fn is_integration_listen(&self) -> bool {
        *self == POCS_A::INTEGRATION_LISTEN
    }
    #[doc = "Checks if the value of the field is `INTEGRATION_CONSISTENCY_CHECK`"]
    #[inline(always)]
    pub fn is_integration_consistency_check(&self) -> bool {
        *self == POCS_A::INTEGRATION_CONSISTENCY_CHECK
    }
    #[doc = "Checks if the value of the field is `INITIALIZE_SCHEDULE`"]
    #[inline(always)]
    pub fn is_initialize_schedule(&self) -> bool {
        *self == POCS_A::INITIALIZE_SCHEDULE
    }
    #[doc = "Checks if the value of the field is `ABORT_STARTUP`"]
    #[inline(always)]
    pub fn is_abort_startup(&self) -> bool {
        *self == POCS_A::ABORT_STARTUP
    }
    #[doc = "Checks if the value of the field is `STARTUP_SUCCESS`"]
    #[inline(always)]
    pub fn is_startup_success(&self) -> bool {
        *self == POCS_A::STARTUP_SUCCESS
    }
}
#[doc = "Field `FSI` reader - Freeze Status Indicator (vPOC!Freeze) Indicates that the POC has entered the HALT state due to CHI command FREEZE or due to an error condition requiring an immediate POC halt. Reset by transition from HALT to DEFAULT_CONFIG state."]
pub type FSI_R = crate::BitReader<bool>;
#[doc = "Field `HRQ` reader - Halt Request (vPOC!CHIHaltRequest) Indicates that a request from the Host has been received to halt the POC at the end of the communication cycle. Reset by transition from HALT to DEFAULT_CONFIG state or when entering READY state."]
pub type HRQ_R = crate::BitReader<bool>;
#[doc = "Field `SLM` reader - N/A"]
pub type SLM_R = crate::FieldReader<u8, SLM_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLM_A {
    #[doc = "0: N/A"]
    SINGLE = 0,
    #[doc = "2: N/A"]
    ALL_PENDING = 2,
    #[doc = "3: N/A"]
    ALL = 3,
}
impl From<SLM_A> for u8 {
    #[inline(always)]
    fn from(variant: SLM_A) -> Self {
        variant as _
    }
}
impl SLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLM_A> {
        match self.bits {
            0 => Some(SLM_A::SINGLE),
            2 => Some(SLM_A::ALL_PENDING),
            3 => Some(SLM_A::ALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == SLM_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `ALL_PENDING`"]
    #[inline(always)]
    pub fn is_all_pending(&self) -> bool {
        *self == SLM_A::ALL_PENDING
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == SLM_A::ALL
    }
}
#[doc = "Field `CSNI` reader - Coldstart Noise Indicator (vPOC!ColdstartNoise) Indicates that the cold start procedure occurred under noisy conditions. Reset by CHI command RESET_STATUS_INDICATORS or by transition from HALT to DEFAULT_CONFIG state or from READY to STARTUP state."]
pub type CSNI_R = crate::BitReader<bool>;
#[doc = "Field `CSAI` reader - Coldstart Abort Indicator Coldstart aborted. Reset by CHI command RESET_STATUS_INDICATORS or by transition from HALT to DEFAULT_CONFIG state or from READY to STARTUP state."]
pub type CSAI_R = crate::BitReader<bool>;
#[doc = "Field `CSI` reader - Cold Start Inhibit (vColdStartInhibit) Indicates that the node is disabled from cold starting. The flag is set whenever the POC enters READY state due to CHI command READY. The flag has to be reset under control of the Host by CHI command ALLOW_COLDSTART (SUCC1.CMD\\[3:0\\]
= '1001'). 1 = Cold starting of node disabled 0 = Cold starting of node enabled"]
pub type CSI_R = crate::BitReader<CSI_A>;
#[doc = "Cold Start Inhibit (vColdStartInhibit) Indicates that the node is disabled from cold starting. The flag is set whenever the POC enters READY state due to CHI command READY. The flag has to be reset under control of the Host by CHI command ALLOW_COLDSTART (SUCC1.CMD\\[3:0\\]
= '1001'). 1 = Cold starting of node disabled 0 = Cold starting of node enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSI_A {
    #[doc = "0: N/A"]
    NODE_COLD_START_ENABLED = 0,
    #[doc = "1: N/A"]
    NODE_COLD_START_DISABLED = 1,
}
impl From<CSI_A> for bool {
    #[inline(always)]
    fn from(variant: CSI_A) -> Self {
        variant as u8 != 0
    }
}
impl CSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSI_A {
        match self.bits {
            false => CSI_A::NODE_COLD_START_ENABLED,
            true => CSI_A::NODE_COLD_START_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NODE_COLD_START_ENABLED`"]
    #[inline(always)]
    pub fn is_node_cold_start_enabled(&self) -> bool {
        *self == CSI_A::NODE_COLD_START_ENABLED
    }
    #[doc = "Checks if the value of the field is `NODE_COLD_START_DISABLED`"]
    #[inline(always)]
    pub fn is_node_cold_start_disabled(&self) -> bool {
        *self == CSI_A::NODE_COLD_START_DISABLED
    }
}
#[doc = "Field `WSV` reader - N/A"]
pub type WSV_R = crate::FieldReader<u8, WSV_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WSV_A {
    #[doc = "0: N/A"]
    UNDEFINED = 0,
    #[doc = "1: N/A"]
    RECEIVED_HEADER = 1,
    #[doc = "2: N/A"]
    RECEIVED_WUP = 2,
    #[doc = "3: N/A"]
    COLLISION_HEADER = 3,
    #[doc = "4: N/A"]
    COLLISION_WUP = 4,
    #[doc = "5: N/A"]
    COLLISION_UNKNOWN = 5,
    #[doc = "6: N/A"]
    TRANSMITTED = 6,
}
impl From<WSV_A> for u8 {
    #[inline(always)]
    fn from(variant: WSV_A) -> Self {
        variant as _
    }
}
impl WSV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WSV_A> {
        match self.bits {
            0 => Some(WSV_A::UNDEFINED),
            1 => Some(WSV_A::RECEIVED_HEADER),
            2 => Some(WSV_A::RECEIVED_WUP),
            3 => Some(WSV_A::COLLISION_HEADER),
            4 => Some(WSV_A::COLLISION_WUP),
            5 => Some(WSV_A::COLLISION_UNKNOWN),
            6 => Some(WSV_A::TRANSMITTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        *self == WSV_A::UNDEFINED
    }
    #[doc = "Checks if the value of the field is `RECEIVED_HEADER`"]
    #[inline(always)]
    pub fn is_received_header(&self) -> bool {
        *self == WSV_A::RECEIVED_HEADER
    }
    #[doc = "Checks if the value of the field is `RECEIVED_WUP`"]
    #[inline(always)]
    pub fn is_received_wup(&self) -> bool {
        *self == WSV_A::RECEIVED_WUP
    }
    #[doc = "Checks if the value of the field is `COLLISION_HEADER`"]
    #[inline(always)]
    pub fn is_collision_header(&self) -> bool {
        *self == WSV_A::COLLISION_HEADER
    }
    #[doc = "Checks if the value of the field is `COLLISION_WUP`"]
    #[inline(always)]
    pub fn is_collision_wup(&self) -> bool {
        *self == WSV_A::COLLISION_WUP
    }
    #[doc = "Checks if the value of the field is `COLLISION_UNKNOWN`"]
    #[inline(always)]
    pub fn is_collision_unknown(&self) -> bool {
        *self == WSV_A::COLLISION_UNKNOWN
    }
    #[doc = "Checks if the value of the field is `TRANSMITTED`"]
    #[inline(always)]
    pub fn is_transmitted(&self) -> bool {
        *self == WSV_A::TRANSMITTED
    }
}
#[doc = "Field `RCA` reader - Remaining Coldstart Attempts (vRemainingColdstartAttempts) Indicates the number of remaining coldstart attempts. The RUN command resets this counter to the maximum number of coldstart attempts as configured by SUCC1.CSA\\[4:0\\]. The initial value of RCA\\[4:0\\]
during CONFIG and DEFAULT_CONFIG state is also SUCC1.CSA\\[4:0\\]."]
pub type RCA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSL` reader - POC Status Log Status of POCS\\[5:0\\]
immediately before entering HALT state. Set when entering HALT state. Set to HALT when FREEZE command is applied during HALT state and FSI is not already set i.e. the HALT state was not reached by FREEZE command. Reset to '00 0000' when leaving HALT state."]
pub type PSL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - N/A"]
    #[inline(always)]
    pub fn pocs(&self) -> POCS_R {
        POCS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Freeze Status Indicator (vPOC!Freeze) Indicates that the POC has entered the HALT state due to CHI command FREEZE or due to an error condition requiring an immediate POC halt. Reset by transition from HALT to DEFAULT_CONFIG state."]
    #[inline(always)]
    pub fn fsi(&self) -> FSI_R {
        FSI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Halt Request (vPOC!CHIHaltRequest) Indicates that a request from the Host has been received to halt the POC at the end of the communication cycle. Reset by transition from HALT to DEFAULT_CONFIG state or when entering READY state."]
    #[inline(always)]
    pub fn hrq(&self) -> HRQ_R {
        HRQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn slm(&self) -> SLM_R {
        SLM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Coldstart Noise Indicator (vPOC!ColdstartNoise) Indicates that the cold start procedure occurred under noisy conditions. Reset by CHI command RESET_STATUS_INDICATORS or by transition from HALT to DEFAULT_CONFIG state or from READY to STARTUP state."]
    #[inline(always)]
    pub fn csni(&self) -> CSNI_R {
        CSNI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Coldstart Abort Indicator Coldstart aborted. Reset by CHI command RESET_STATUS_INDICATORS or by transition from HALT to DEFAULT_CONFIG state or from READY to STARTUP state."]
    #[inline(always)]
    pub fn csai(&self) -> CSAI_R {
        CSAI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Cold Start Inhibit (vColdStartInhibit) Indicates that the node is disabled from cold starting. The flag is set whenever the POC enters READY state due to CHI command READY. The flag has to be reset under control of the Host by CHI command ALLOW_COLDSTART (SUCC1.CMD\\[3:0\\]
= '1001'). 1 = Cold starting of node disabled 0 = Cold starting of node enabled"]
    #[inline(always)]
    pub fn csi(&self) -> CSI_R {
        CSI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - N/A"]
    #[inline(always)]
    pub fn wsv(&self) -> WSV_R {
        WSV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23 - Remaining Coldstart Attempts (vRemainingColdstartAttempts) Indicates the number of remaining coldstart attempts. The RUN command resets this counter to the maximum number of coldstart attempts as configured by SUCC1.CSA\\[4:0\\]. The initial value of RCA\\[4:0\\]
during CONFIG and DEFAULT_CONFIG state is also SUCC1.CSA\\[4:0\\]."]
    #[inline(always)]
    pub fn rca(&self) -> RCA_R {
        RCA_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29 - POC Status Log Status of POCS\\[5:0\\]
immediately before entering HALT state. Set when entering HALT state. Set to HALT when FREEZE command is applied during HALT state and FSI is not already set i.e. the HALT state was not reached by FREEZE command. Reset to '00 0000' when leaving HALT state."]
    #[inline(always)]
    pub fn psl(&self) -> PSL_R {
        PSL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "CC Status Vector\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccsv](index.html) module"]
pub struct CCSV_SPEC;
impl crate::RegisterSpec for CCSV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccsv::R](R) reader structure"]
impl crate::Readable for CCSV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCSV to value 0x0010_4000"]
impl crate::Resettable for CCSV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_4000;
}
