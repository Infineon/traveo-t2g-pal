#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWER_ACTION` reader - Action taken if this watchdog is serviced before LOWER_LIMIT is reached. LOWER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected and/or when the chip is in DEEPSLEEP/HIBERNATE modes. For LOWER_LIMIT >= 1: The action is triggered on same edge when it meets this condition. For LOWER_LIMIT == 0: No action is triggered."]
pub type LOWER_ACTION_R = crate::BitReader<LOWER_ACTION_A>;
#[doc = "Action taken if this watchdog is serviced before LOWER_LIMIT is reached. LOWER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected and/or when the chip is in DEEPSLEEP/HIBERNATE modes. For LOWER_LIMIT >= 1: The action is triggered on same edge when it meets this condition. For LOWER_LIMIT == 0: No action is triggered.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOWER_ACTION_A {
    #[doc = "0: Do nothing"]
    NOTHING = 0,
    #[doc = "1: Trigger a reset."]
    RESET = 1,
}
impl From<LOWER_ACTION_A> for bool {
    #[inline(always)]
    fn from(variant: LOWER_ACTION_A) -> Self {
        variant as u8 != 0
    }
}
impl LOWER_ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWER_ACTION_A {
        match self.bits {
            false => LOWER_ACTION_A::NOTHING,
            true => LOWER_ACTION_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHING`"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == LOWER_ACTION_A::NOTHING
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LOWER_ACTION_A::RESET
    }
}
#[doc = "Field `LOWER_ACTION` writer - Action taken if this watchdog is serviced before LOWER_LIMIT is reached. LOWER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected and/or when the chip is in DEEPSLEEP/HIBERNATE modes. For LOWER_LIMIT >= 1: The action is triggered on same edge when it meets this condition. For LOWER_LIMIT == 0: No action is triggered."]
pub type LOWER_ACTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, LOWER_ACTION_A, O>;
impl<'a, const O: u8> LOWER_ACTION_W<'a, O> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut W {
        self.variant(LOWER_ACTION_A::NOTHING)
    }
    #[doc = "Trigger a reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LOWER_ACTION_A::RESET)
    }
}
#[doc = "Field `UPPER_ACTION` reader - Action taken if this watchdog is not serviced before UPPER_LIMIT is reached. The counter stops counting when UPPER_LIMIT is reached, regardless of UPPER_ACTION setting. UPPER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected. For UPPER_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For UPPER_LIMIT &lt; 2: The action may take up to one extra clk_ilo0 cycle to trigger."]
pub type UPPER_ACTION_R = crate::BitReader<UPPER_ACTION_A>;
#[doc = "Action taken if this watchdog is not serviced before UPPER_LIMIT is reached. The counter stops counting when UPPER_LIMIT is reached, regardless of UPPER_ACTION setting. UPPER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected. For UPPER_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For UPPER_LIMIT &lt; 2: The action may take up to one extra clk_ilo0 cycle to trigger.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPPER_ACTION_A {
    #[doc = "0: Do nothing"]
    NOTHING = 0,
    #[doc = "1: Trigger a reset."]
    RESET = 1,
}
impl From<UPPER_ACTION_A> for bool {
    #[inline(always)]
    fn from(variant: UPPER_ACTION_A) -> Self {
        variant as u8 != 0
    }
}
impl UPPER_ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPPER_ACTION_A {
        match self.bits {
            false => UPPER_ACTION_A::NOTHING,
            true => UPPER_ACTION_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHING`"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == UPPER_ACTION_A::NOTHING
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == UPPER_ACTION_A::RESET
    }
}
#[doc = "Field `UPPER_ACTION` writer - Action taken if this watchdog is not serviced before UPPER_LIMIT is reached. The counter stops counting when UPPER_LIMIT is reached, regardless of UPPER_ACTION setting. UPPER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected. For UPPER_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For UPPER_LIMIT &lt; 2: The action may take up to one extra clk_ilo0 cycle to trigger."]
pub type UPPER_ACTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, UPPER_ACTION_A, O>;
impl<'a, const O: u8> UPPER_ACTION_W<'a, O> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut W {
        self.variant(UPPER_ACTION_A::NOTHING)
    }
    #[doc = "Trigger a reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UPPER_ACTION_A::RESET)
    }
}
#[doc = "Field `WARN_ACTION` reader - Action taken when the count value reaches WARN_LIMIT. The minimum setting to achieve a periodic interrupt is WARN_LIMIT==1. A setting of zero will trigger once but not periodically. For WARN_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For WARN_LIMIT &lt; 2 : The action may take up to one extra clk_ilo0 cycle to trigger."]
pub type WARN_ACTION_R = crate::BitReader<WARN_ACTION_A>;
#[doc = "Action taken when the count value reaches WARN_LIMIT. The minimum setting to achieve a periodic interrupt is WARN_LIMIT==1. A setting of zero will trigger once but not periodically. For WARN_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For WARN_LIMIT &lt; 2 : The action may take up to one extra clk_ilo0 cycle to trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WARN_ACTION_A {
    #[doc = "0: Do nothing"]
    NOTHING = 0,
    #[doc = "1: Trigger an interrupt."]
    INT = 1,
}
impl From<WARN_ACTION_A> for bool {
    #[inline(always)]
    fn from(variant: WARN_ACTION_A) -> Self {
        variant as u8 != 0
    }
}
impl WARN_ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARN_ACTION_A {
        match self.bits {
            false => WARN_ACTION_A::NOTHING,
            true => WARN_ACTION_A::INT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHING`"]
    #[inline(always)]
    pub fn is_nothing(&self) -> bool {
        *self == WARN_ACTION_A::NOTHING
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == WARN_ACTION_A::INT
    }
}
#[doc = "Field `WARN_ACTION` writer - Action taken when the count value reaches WARN_LIMIT. The minimum setting to achieve a periodic interrupt is WARN_LIMIT==1. A setting of zero will trigger once but not periodically. For WARN_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For WARN_LIMIT &lt; 2 : The action may take up to one extra clk_ilo0 cycle to trigger."]
pub type WARN_ACTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, WARN_ACTION_A, O>;
impl<'a, const O: u8> WARN_ACTION_W<'a, O> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn nothing(self) -> &'a mut W {
        self.variant(WARN_ACTION_A::NOTHING)
    }
    #[doc = "Trigger an interrupt."]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(WARN_ACTION_A::INT)
    }
}
#[doc = "Field `AUTO_SERVICE` reader - Automatically service when the count value reaches WARN_LIMIT. This allows creation of a periodic interrupt if this counter is not needed as a watchdog. This field is ignored when LOWER_ACTION&lt;>NOTHING or when UPPER_ACTION&lt;>NOTHING."]
pub type AUTO_SERVICE_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_SERVICE` writer - Automatically service when the count value reaches WARN_LIMIT. This allows creation of a periodic interrupt if this counter is not needed as a watchdog. This field is ignored when LOWER_ACTION&lt;>NOTHING or when UPPER_ACTION&lt;>NOTHING."]
pub type AUTO_SERVICE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `DEBUG_TRIGGER_EN` reader - Enables the trigger input for WDT to pause the counter during debug mode. To pause at a breakpoint while debugging, configure the trigger matrix to connect the related CPU halted signal to the trigger input for this WDT, and then set this bit. It takes up to two clk_ilo0 cycles for the trigger signal to be processed. Triggers that are less than two clk_ilo0 cycles may be missed. Synchronization error can accumulate each time it is halted. 0: Pauses the counter whenever a debug probe is connected. 1: Pauses the counter whenever a debug probe is connected and the trigger input is high."]
pub type DEBUG_TRIGGER_EN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG_TRIGGER_EN` writer - Enables the trigger input for WDT to pause the counter during debug mode. To pause at a breakpoint while debugging, configure the trigger matrix to connect the related CPU halted signal to the trigger input for this WDT, and then set this bit. It takes up to two clk_ilo0 cycles for the trigger signal to be processed. Triggers that are less than two clk_ilo0 cycles may be missed. Synchronization error can accumulate each time it is halted. 0: Pauses the counter whenever a debug probe is connected. 1: Pauses the counter whenever a debug probe is connected and the trigger input is high."]
pub type DEBUG_TRIGGER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `DPSLP_PAUSE` reader - Pauses/runs this counter when the system is in DEEPSLEEP. Note it may take up to two clk_ilo0 cycles for the counter to pause, due to internal synchronization. During DEEPSLEEP wakeup, the pause request is removed when clk_hf0 starts clocking, and then it may take up to two clk_ilo0 cycles for the counter to start. After wakeup, the LOWER_ACTION is ignored until after the first service. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. 0: Counter behaves normally during DEEPSLEEP. 1: Counter pauses during DEEPSLEEP."]
pub type DPSLP_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `DPSLP_PAUSE` writer - Pauses/runs this counter when the system is in DEEPSLEEP. Note it may take up to two clk_ilo0 cycles for the counter to pause, due to internal synchronization. During DEEPSLEEP wakeup, the pause request is removed when clk_hf0 starts clocking, and then it may take up to two clk_ilo0 cycles for the counter to start. After wakeup, the LOWER_ACTION is ignored until after the first service. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. 0: Counter behaves normally during DEEPSLEEP. 1: Counter pauses during DEEPSLEEP."]
pub type DPSLP_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `HIB_PAUSE` reader - Pauses/runs this counter when the system is in HIBERNATE. Note it may take up to two clk_ilo0 cycles for the counter to pause, due to internal synchronization. After wakeup, the LOWER_ACTION is ignored until after the first service. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. 0: Counter behaves normally during HIBERNATE. 1: Counter pauses during HIBERNATE."]
pub type HIB_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `HIB_PAUSE` writer - Pauses/runs this counter when the system is in HIBERNATE. Note it may take up to two clk_ilo0 cycles for the counter to pause, due to internal synchronization. After wakeup, the LOWER_ACTION is ignored until after the first service. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. 0: Counter behaves normally during HIBERNATE. 1: Counter pauses during HIBERNATE."]
pub type HIB_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `DEBUG_RUN` reader - Pauses/runs this counter while a debugger is connected. Other behaviors are unchanged during debugging, including service, configuration updates and enable/disable. Note it may take up to two clk_ilo0 cycles for the counter to pause and another two cycles to unpause, due to internal synchronization. If the debugger is connected for at least two clk_ilo0 cycles, the LOWER_ACTION is ignored until after the first service after the debugger is disconnected. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. If the debugger is disconnected before two clk_ilo0 cycles, the LOWER_ACTION may or may not be ignored. 0: When debugger connected, counter pauses incrementing as configured in DEBUG_TRIGGER_EN. 1: When debugger connected, counter increments normally, but reset generation is blocked."]
pub type DEBUG_RUN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG_RUN` writer - Pauses/runs this counter while a debugger is connected. Other behaviors are unchanged during debugging, including service, configuration updates and enable/disable. Note it may take up to two clk_ilo0 cycles for the counter to pause and another two cycles to unpause, due to internal synchronization. If the debugger is connected for at least two clk_ilo0 cycles, the LOWER_ACTION is ignored until after the first service after the debugger is disconnected. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. If the debugger is disconnected before two clk_ilo0 cycles, the LOWER_ACTION may or may not be ignored. 0: When debugger connected, counter pauses incrementing as configured in DEBUG_TRIGGER_EN. 1: When debugger connected, counter increments normally, but reset generation is blocked."]
pub type DEBUG_RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Action taken if this watchdog is serviced before LOWER_LIMIT is reached. LOWER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected and/or when the chip is in DEEPSLEEP/HIBERNATE modes. For LOWER_LIMIT >= 1: The action is triggered on same edge when it meets this condition. For LOWER_LIMIT == 0: No action is triggered."]
    #[inline(always)]
    pub fn lower_action(&self) -> LOWER_ACTION_R {
        LOWER_ACTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Action taken if this watchdog is not serviced before UPPER_LIMIT is reached. The counter stops counting when UPPER_LIMIT is reached, regardless of UPPER_ACTION setting. UPPER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected. For UPPER_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For UPPER_LIMIT &lt; 2: The action may take up to one extra clk_ilo0 cycle to trigger."]
    #[inline(always)]
    pub fn upper_action(&self) -> UPPER_ACTION_R {
        UPPER_ACTION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Action taken when the count value reaches WARN_LIMIT. The minimum setting to achieve a periodic interrupt is WARN_LIMIT==1. A setting of zero will trigger once but not periodically. For WARN_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For WARN_LIMIT &lt; 2 : The action may take up to one extra clk_ilo0 cycle to trigger."]
    #[inline(always)]
    pub fn warn_action(&self) -> WARN_ACTION_R {
        WARN_ACTION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Automatically service when the count value reaches WARN_LIMIT. This allows creation of a periodic interrupt if this counter is not needed as a watchdog. This field is ignored when LOWER_ACTION&lt;>NOTHING or when UPPER_ACTION&lt;>NOTHING."]
    #[inline(always)]
    pub fn auto_service(&self) -> AUTO_SERVICE_R {
        AUTO_SERVICE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 28 - Enables the trigger input for WDT to pause the counter during debug mode. To pause at a breakpoint while debugging, configure the trigger matrix to connect the related CPU halted signal to the trigger input for this WDT, and then set this bit. It takes up to two clk_ilo0 cycles for the trigger signal to be processed. Triggers that are less than two clk_ilo0 cycles may be missed. Synchronization error can accumulate each time it is halted. 0: Pauses the counter whenever a debug probe is connected. 1: Pauses the counter whenever a debug probe is connected and the trigger input is high."]
    #[inline(always)]
    pub fn debug_trigger_en(&self) -> DEBUG_TRIGGER_EN_R {
        DEBUG_TRIGGER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pauses/runs this counter when the system is in DEEPSLEEP. Note it may take up to two clk_ilo0 cycles for the counter to pause, due to internal synchronization. During DEEPSLEEP wakeup, the pause request is removed when clk_hf0 starts clocking, and then it may take up to two clk_ilo0 cycles for the counter to start. After wakeup, the LOWER_ACTION is ignored until after the first service. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. 0: Counter behaves normally during DEEPSLEEP. 1: Counter pauses during DEEPSLEEP."]
    #[inline(always)]
    pub fn dpslp_pause(&self) -> DPSLP_PAUSE_R {
        DPSLP_PAUSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Pauses/runs this counter when the system is in HIBERNATE. Note it may take up to two clk_ilo0 cycles for the counter to pause, due to internal synchronization. After wakeup, the LOWER_ACTION is ignored until after the first service. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. 0: Counter behaves normally during HIBERNATE. 1: Counter pauses during HIBERNATE."]
    #[inline(always)]
    pub fn hib_pause(&self) -> HIB_PAUSE_R {
        HIB_PAUSE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Pauses/runs this counter while a debugger is connected. Other behaviors are unchanged during debugging, including service, configuration updates and enable/disable. Note it may take up to two clk_ilo0 cycles for the counter to pause and another two cycles to unpause, due to internal synchronization. If the debugger is connected for at least two clk_ilo0 cycles, the LOWER_ACTION is ignored until after the first service after the debugger is disconnected. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. If the debugger is disconnected before two clk_ilo0 cycles, the LOWER_ACTION may or may not be ignored. 0: When debugger connected, counter pauses incrementing as configured in DEBUG_TRIGGER_EN. 1: When debugger connected, counter increments normally, but reset generation is blocked."]
    #[inline(always)]
    pub fn debug_run(&self) -> DEBUG_RUN_R {
        DEBUG_RUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Action taken if this watchdog is serviced before LOWER_LIMIT is reached. LOWER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected and/or when the chip is in DEEPSLEEP/HIBERNATE modes. For LOWER_LIMIT >= 1: The action is triggered on same edge when it meets this condition. For LOWER_LIMIT == 0: No action is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn lower_action(&mut self) -> LOWER_ACTION_W<0> {
        LOWER_ACTION_W::new(self)
    }
    #[doc = "Bit 4 - Action taken if this watchdog is not serviced before UPPER_LIMIT is reached. The counter stops counting when UPPER_LIMIT is reached, regardless of UPPER_ACTION setting. UPPER_ACTION is ignored (i.e. treated as NOTHING) when a debugger is connected. For UPPER_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For UPPER_LIMIT &lt; 2: The action may take up to one extra clk_ilo0 cycle to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn upper_action(&mut self) -> UPPER_ACTION_W<4> {
        UPPER_ACTION_W::new(self)
    }
    #[doc = "Bit 8 - Action taken when the count value reaches WARN_LIMIT. The minimum setting to achieve a periodic interrupt is WARN_LIMIT==1. A setting of zero will trigger once but not periodically. For WARN_LIMIT >= 2: The action is triggered on same edge when it meets this condition. For WARN_LIMIT &lt; 2 : The action may take up to one extra clk_ilo0 cycle to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn warn_action(&mut self) -> WARN_ACTION_W<8> {
        WARN_ACTION_W::new(self)
    }
    #[doc = "Bit 12 - Automatically service when the count value reaches WARN_LIMIT. This allows creation of a periodic interrupt if this counter is not needed as a watchdog. This field is ignored when LOWER_ACTION&lt;>NOTHING or when UPPER_ACTION&lt;>NOTHING."]
    #[inline(always)]
    #[must_use]
    pub fn auto_service(&mut self) -> AUTO_SERVICE_W<12> {
        AUTO_SERVICE_W::new(self)
    }
    #[doc = "Bit 28 - Enables the trigger input for WDT to pause the counter during debug mode. To pause at a breakpoint while debugging, configure the trigger matrix to connect the related CPU halted signal to the trigger input for this WDT, and then set this bit. It takes up to two clk_ilo0 cycles for the trigger signal to be processed. Triggers that are less than two clk_ilo0 cycles may be missed. Synchronization error can accumulate each time it is halted. 0: Pauses the counter whenever a debug probe is connected. 1: Pauses the counter whenever a debug probe is connected and the trigger input is high."]
    #[inline(always)]
    #[must_use]
    pub fn debug_trigger_en(&mut self) -> DEBUG_TRIGGER_EN_W<28> {
        DEBUG_TRIGGER_EN_W::new(self)
    }
    #[doc = "Bit 29 - Pauses/runs this counter when the system is in DEEPSLEEP. Note it may take up to two clk_ilo0 cycles for the counter to pause, due to internal synchronization. During DEEPSLEEP wakeup, the pause request is removed when clk_hf0 starts clocking, and then it may take up to two clk_ilo0 cycles for the counter to start. After wakeup, the LOWER_ACTION is ignored until after the first service. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. 0: Counter behaves normally during DEEPSLEEP. 1: Counter pauses during DEEPSLEEP."]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_pause(&mut self) -> DPSLP_PAUSE_W<29> {
        DPSLP_PAUSE_W::new(self)
    }
    #[doc = "Bit 30 - Pauses/runs this counter when the system is in HIBERNATE. Note it may take up to two clk_ilo0 cycles for the counter to pause, due to internal synchronization. After wakeup, the LOWER_ACTION is ignored until after the first service. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. 0: Counter behaves normally during HIBERNATE. 1: Counter pauses during HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn hib_pause(&mut self) -> HIB_PAUSE_W<30> {
        HIB_PAUSE_W::new(self)
    }
    #[doc = "Bit 31 - Pauses/runs this counter while a debugger is connected. Other behaviors are unchanged during debugging, including service, configuration updates and enable/disable. Note it may take up to two clk_ilo0 cycles for the counter to pause and another two cycles to unpause, due to internal synchronization. If the debugger is connected for at least two clk_ilo0 cycles, the LOWER_ACTION is ignored until after the first service after the debugger is disconnected. This prevents an unintentional trigger of the LOWER_ACTION before the firmware realigns the servicing period. After the first service, LOWER_ACTION behaves as configured. If the debugger is disconnected before two clk_ilo0 cycles, the LOWER_ACTION may or may not be ignored. 0: When debugger connected, counter pauses incrementing as configured in DEBUG_TRIGGER_EN. 1: When debugger connected, counter increments normally, but reset generation is blocked."]
    #[inline(always)]
    #[must_use]
    pub fn debug_run(&mut self) -> DEBUG_RUN_W<31> {
        DEBUG_RUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x10"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
