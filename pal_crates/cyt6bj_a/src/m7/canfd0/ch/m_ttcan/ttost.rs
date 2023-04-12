#[doc = "Register `TTOST` reader"]
pub struct R(crate::R<TTOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EL` reader - Error Level 00= Severity 0 - No Error 01= Severity 1 - Warning 10= Severity 2 - Error 11= Severity 3 - Severe Error"]
pub type EL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MS` reader - Master State 00= Master_Off, no master properties relevant 01= Operating as Time Slave 10= Operating as Backup Time Master 11= Operating as current Time Master"]
pub type MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYS` reader - Synchronization State 00= Out of Synchronization 01= Synchronizing to TTCAN communication 10= Schedule suspended by Gap (In_Gap) 11= Synchronized to schedule (In_Schedule)"]
pub type SYS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QGTP` reader - Quality of Global Time Phase Only relevant in TTCAN Level 0 and Level 2, otherwise fixed to '0'. 0= Global time not valid 1= Global time in phase with Time Master"]
pub type QGTP_R = crate::BitReader<bool>;
#[doc = "Field `QCS` reader - Quality of Clock Speed Only relevant in TTCAN Level 0 and Level 2, otherwise fixed to '1'. 0= Local clock speed not synchronized to Time Master clock speed 1= Synchronization Deviation &lt;= SDL"]
pub type QCS_R = crate::BitReader<bool>;
#[doc = "Field `RTO` reader - Reference Trigger Offset The Reference Trigger Offset value is a signed integer with a range from -127 (0x81) to 127 (0x7F). There is no notification when the lower limit of -127 is reached. In case the M_TTCAN becomes Time Master (MS\\[1:0\\]
= '11'), the reset of RTO is delayed due to synchronization between Host and CAN clock domain. For time slaves the value configured by TTOCF.IRTO is read. 0x00-FF Actual Reference Trigger offset value"]
pub type RTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGTD` reader - Wait for Global Time Discontinuity 0= No global time preset pending 1= Node waits for the global time preset to take effect. The bit is reset when the node has transmitted a reference message with Disc_Bit = '1' or after it received a reference message."]
pub type WGTD_R = crate::BitReader<bool>;
#[doc = "Field `GFI` reader - Gap Finished Indicator Set when the CPU writes TTOCN.FGP, or by a time mark interrupt if TMG = '1', or via input pin m_ttcan_evt if TTOCN.GCS = '1'. Not set by Ref_Trigger_Gap or when Gap is finished by another node sending a reference message. 0= Reset at the end of each reference message 1= Gap finished by M_TTCAN"]
pub type GFI_R = crate::BitReader<bool>;
#[doc = "Field `TMP` reader - Time Master Priority 0x0-7 Priority of actual Time Master"]
pub type TMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GSI` reader - Gap Started Indicator 0= No Gap in schedule, reset by each reference message and for all time slaves 1= Gap time after Basic Cycle has started"]
pub type GSI_R = crate::BitReader<bool>;
#[doc = "Field `WFE` reader - Wait for Event 0= No Gap announced, reset by a reference message with Next_is_Gap = '0' 1= Reference message with Next_is_Gap = '1' received"]
pub type WFE_R = crate::BitReader<bool>;
#[doc = "Field `AWE` reader - Application Watchdog Event The application watchdog is served by reading TTOST. When the watchdog is not served in time, bit AWE is set, all TTCAN communication is stopped, and the M_TTCAN is set into Bus Monitoring Mode. 0= Application Watchdog served in time 1= Failed to serve Application Watchdog in time"]
pub type AWE_R = crate::BitReader<bool>;
#[doc = "Field `WECS` reader - Wait for External Clock Synchronization 0= No external clock synchronization pending 1= Node waits for external clock synchronization to take effect. The bit is reset at the start of the next basic cycle."]
pub type WECS_R = crate::BitReader<bool>;
#[doc = "Field `SPL` reader - Schedule Phase Lock The bit is valid only when external synchronization is enabled (TTOCN.ESCN = '1'). In this case it signals that the difference between cycle time configured by TTGTP.CTP and the cycle time at the rising edge at pin m_ttcan_evt is less or equal 9 NTU (see Section 4.11). 0= Phase outside range 1= Phase inside range"]
pub type SPL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - Error Level 00= Severity 0 - No Error 01= Severity 1 - Warning 10= Severity 2 - Error 11= Severity 3 - Severe Error"]
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Master State 00= Master_Off, no master properties relevant 01= Operating as Time Slave 10= Operating as Backup Time Master 11= Operating as current Time Master"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization State 00= Out of Synchronization 01= Synchronizing to TTCAN communication 10= Schedule suspended by Gap (In_Gap) 11= Synchronized to schedule (In_Schedule)"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase Only relevant in TTCAN Level 0 and Level 2, otherwise fixed to '0'. 0= Global time not valid 1= Global time in phase with Time Master"]
    #[inline(always)]
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Quality of Clock Speed Only relevant in TTCAN Level 0 and Level 2, otherwise fixed to '1'. 0= Local clock speed not synchronized to Time Master clock speed 1= Synchronization Deviation &lt;= SDL"]
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset The Reference Trigger Offset value is a signed integer with a range from -127 (0x81) to 127 (0x7F). There is no notification when the lower limit of -127 is reached. In case the M_TTCAN becomes Time Master (MS\\[1:0\\]
= '11'), the reset of RTO is delayed due to synchronization between Host and CAN clock domain. For time slaves the value configured by TTOCF.IRTO is read. 0x00-FF Actual Reference Trigger offset value"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity 0= No global time preset pending 1= Node waits for the global time preset to take effect. The bit is reset when the node has transmitted a reference message with Disc_Bit = '1' or after it received a reference message."]
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Gap Finished Indicator Set when the CPU writes TTOCN.FGP, or by a time mark interrupt if TMG = '1', or via input pin m_ttcan_evt if TTOCN.GCS = '1'. Not set by Ref_Trigger_Gap or when Gap is finished by another node sending a reference message. 0= Reset at the end of each reference message 1= Gap finished by M_TTCAN"]
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Time Master Priority 0x0-7 Priority of actual Time Master"]
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Gap Started Indicator 0= No Gap in schedule, reset by each reference message and for all time slaves 1= Gap time after Basic Cycle has started"]
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wait for Event 0= No Gap announced, reset by a reference message with Next_is_Gap = '0' 1= Reference message with Next_is_Gap = '1' received"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Application Watchdog Event The application watchdog is served by reading TTOST. When the watchdog is not served in time, bit AWE is set, all TTCAN communication is stopped, and the M_TTCAN is set into Bus Monitoring Mode. 0= Application Watchdog served in time 1= Failed to serve Application Watchdog in time"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization 0= No external clock synchronization pending 1= Node waits for external clock synchronization to take effect. The bit is reset at the start of the next basic cycle."]
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Schedule Phase Lock The bit is valid only when external synchronization is enabled (TTOCN.ESCN = '1'). In this case it signals that the difference between cycle time configured by TTGTP.CTP and the cycle time at the rising edge at pin m_ttcan_evt is less or equal 9 NTU (see Section 4.11). 0= Phase outside range 1= Phase inside range"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "TT Operation Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttost](index.html) module"]
pub struct TTOST_SPEC;
impl crate::RegisterSpec for TTOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttost::R](R) reader structure"]
impl crate::Readable for TTOST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTOST to value 0x80"]
impl crate::Resettable for TTOST_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
