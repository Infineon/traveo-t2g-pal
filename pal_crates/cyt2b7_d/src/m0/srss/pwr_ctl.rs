#[doc = "Register `PWR_CTL` reader"]
pub struct R(crate::R<PWR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POWER_MODE` reader - Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
pub type POWER_MODE_R = crate::FieldReader<u8, POWER_MODE_A>;
#[doc = "Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POWER_MODE_A {
    #[doc = "0: System is resetting."]
    RESET = 0,
    #[doc = "1: At least one CPU is running."]
    ACTIVE = 1,
    #[doc = "2: No CPUs are running. Peripherals may be running."]
    SLEEP = 2,
    #[doc = "3: Main high-frequency clock is off; low speed clocks are available. Communication interface clocks may be present."]
    DEEPSLEEP = 3,
}
impl From<POWER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: POWER_MODE_A) -> Self {
        variant as _
    }
}
impl POWER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_MODE_A {
        match self.bits {
            0 => POWER_MODE_A::RESET,
            1 => POWER_MODE_A::ACTIVE,
            2 => POWER_MODE_A::SLEEP,
            3 => POWER_MODE_A::DEEPSLEEP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == POWER_MODE_A::RESET
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == POWER_MODE_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == POWER_MODE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == POWER_MODE_A::DEEPSLEEP
    }
}
#[doc = "Field `DEBUG_SESSION` reader - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
pub type DEBUG_SESSION_R = crate::BitReader<DEBUG_SESSION_A>;
#[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUG_SESSION_A {
    #[doc = "0: No debug session active"]
    NO_SESSION = 0,
    #[doc = "1: Debug session is active. Power modes behave differently to keep the debug session active."]
    SESSION_ACTIVE = 1,
}
impl From<DEBUG_SESSION_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_SESSION_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBUG_SESSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUG_SESSION_A {
        match self.bits {
            false => DEBUG_SESSION_A::NO_SESSION,
            true => DEBUG_SESSION_A::SESSION_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SESSION`"]
    #[inline(always)]
    pub fn is_no_session(&self) -> bool {
        *self == DEBUG_SESSION_A::NO_SESSION
    }
    #[doc = "Checks if the value of the field is `SESSION_ACTIVE`"]
    #[inline(always)]
    pub fn is_session_active(&self) -> bool {
        *self == DEBUG_SESSION_A::SESSION_ACTIVE
    }
}
#[doc = "Field `LPM_READY` reader - Indicates whether certain low power functions are ready. The low current circuits take longer to startup after XRES, HIBERNATE wakeup, or supply supervision reset than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
pub type LPM_READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub fn debug_session(&self) -> DEBUG_SESSION_R {
        DEBUG_SESSION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether certain low power functions are ready. The low current circuits take longer to startup after XRES, HIBERNATE wakeup, or supply supervision reset than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
    #[inline(always)]
    pub fn lpm_ready(&self) -> LPM_READY_R {
        LPM_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Power Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctl](index.html) module"]
pub struct PWR_CTL_SPEC;
impl crate::RegisterSpec for PWR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctl::R](R) reader structure"]
impl crate::Readable for PWR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_CTL to value 0"]
impl crate::Resettable for PWR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
