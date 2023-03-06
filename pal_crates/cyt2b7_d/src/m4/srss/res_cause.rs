#[doc = "Register `RES_CAUSE` reader"]
pub struct R(crate::R<RES_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES_CAUSE` writer"]
pub struct W(crate::W<RES_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_CAUSE_SPEC>;
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
impl From<crate::W<RES_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_WDT` reader - A basic WatchDog Timer (WDT) reset has occurred since last power cycle. ULP products: This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above). For products that support high-voltage cause detection, this bit blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
pub type RESET_WDT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_WDT` writer - A basic WatchDog Timer (WDT) reset has occurred since last power cycle. ULP products: This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above). For products that support high-voltage cause detection, this bit blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
pub type RESET_WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_ACT_FAULT` reader - Fault logging system requested a reset from its Active logic. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_ACT_FAULT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_ACT_FAULT` writer - Fault logging system requested a reset from its Active logic. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_ACT_FAULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_DPSLP_FAULT` reader - Fault logging system requested a reset from its DeepSleep logic. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_DPSLP_FAULT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_DPSLP_FAULT` writer - Fault logging system requested a reset from its DeepSleep logic. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_DPSLP_FAULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_TC_DBGRESET` reader - Test controller or debugger asserted reset. Only resets debug domain. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_TC_DBGRESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET_TC_DBGRESET` writer - Test controller or debugger asserted reset. Only resets debug domain. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_TC_DBGRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_SOFT` reader - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_SOFT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_SOFT` writer - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_SOFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_MCWDT0` reader - Multi-Counter Watchdog timer reset #0. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_MCWDT0_R = crate::BitReader<bool>;
#[doc = "Field `RESET_MCWDT0` writer - Multi-Counter Watchdog timer reset #0. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_MCWDT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_MCWDT1` reader - Multi-Counter Watchdog timer reset #1. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_MCWDT1_R = crate::BitReader<bool>;
#[doc = "Field `RESET_MCWDT1` writer - Multi-Counter Watchdog timer reset #1. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_MCWDT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_MCWDT2` reader - Multi-Counter Watchdog timer reset #2. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_MCWDT2_R = crate::BitReader<bool>;
#[doc = "Field `RESET_MCWDT2` writer - Multi-Counter Watchdog timer reset #2. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_MCWDT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_MCWDT3` reader - Multi-Counter Watchdog timer reset #3. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_MCWDT3_R = crate::BitReader<bool>;
#[doc = "Field `RESET_MCWDT3` writer - Multi-Counter Watchdog timer reset #3. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
pub type RESET_MCWDT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_XRES` reader - External XRES pin was asserted. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
pub type RESET_XRES_R = crate::BitReader<bool>;
#[doc = "Field `RESET_XRES` writer - External XRES pin was asserted. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
pub type RESET_XRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_BODVDDD` reader - External VDDD supply crossed brown-out limit. Note that this cause will only be observable as long as the VDDD supply does not go below the POR (power on reset) detection limit. Below this limit it is not possible to reliably retain information in the device. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_BODVDDD_R = crate::BitReader<bool>;
#[doc = "Field `RESET_BODVDDD` writer - External VDDD supply crossed brown-out limit. Note that this cause will only be observable as long as the VDDD supply does not go below the POR (power on reset) detection limit. Below this limit it is not possible to reliably retain information in the device. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_BODVDDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_BODVDDA` reader - External VDDA supply crossed the brown-out limit. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_BODVDDA_R = crate::BitReader<bool>;
#[doc = "Field `RESET_BODVDDA` writer - External VDDA supply crossed the brown-out limit. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_BODVDDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_BODVCCD` reader - Internal VCCD core supply crossed the brown-out limit. Note that this detector will detect gross issues with the internal core supply, but may not catch all brown-out conditions. Functional and timing supervision (CSV, WDT) is provided to create fully failsafe internal crash detection. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_BODVCCD_R = crate::BitReader<bool>;
#[doc = "Field `RESET_BODVCCD` writer - Internal VCCD core supply crossed the brown-out limit. Note that this detector will detect gross issues with the internal core supply, but may not catch all brown-out conditions. Functional and timing supervision (CSV, WDT) is provided to create fully failsafe internal crash detection. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_BODVCCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_OVDVDDD` reader - Overvoltage detection on the external VDDD supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OVDVDDD_R = crate::BitReader<bool>;
#[doc = "Field `RESET_OVDVDDD` writer - Overvoltage detection on the external VDDD supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OVDVDDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_OVDVDDA` reader - Overvoltage detection on the external VDDA supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OVDVDDA_R = crate::BitReader<bool>;
#[doc = "Field `RESET_OVDVDDA` writer - Overvoltage detection on the external VDDA supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OVDVDDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_OVDVCCD` reader - Overvoltage detection on the internal core VCCD supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OVDVCCD_R = crate::BitReader<bool>;
#[doc = "Field `RESET_OVDVCCD` writer - Overvoltage detection on the internal core VCCD supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OVDVCCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_OCD_ACT_LINREG` reader - Overcurrent detection on the internal VCCD supply when supplied by the ACTIVE power mode linear regulator. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OCD_ACT_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `RESET_OCD_ACT_LINREG` writer - Overcurrent detection on the internal VCCD supply when supplied by the ACTIVE power mode linear regulator. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OCD_ACT_LINREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_OCD_DPSLP_LINREG` reader - Overcurrent detection on the internal VCCD supply when supplied by the DEEPSLEEP power mode linear regulator. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OCD_DPSLP_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `RESET_OCD_DPSLP_LINREG` writer - Overcurrent detection on the internal VCCD supply when supplied by the DEEPSLEEP power mode linear regulator. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
pub type RESET_OCD_DPSLP_LINREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_PXRES` reader - PXRES triggered. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
pub type RESET_PXRES_R = crate::BitReader<bool>;
#[doc = "Field `RESET_PXRES` writer - PXRES triggered. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
pub type RESET_PXRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_STRUCT_XRES` reader - Structural reset was asserted. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
pub type RESET_STRUCT_XRES_R = crate::BitReader<bool>;
#[doc = "Field `RESET_STRUCT_XRES` writer - Structural reset was asserted. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
pub type RESET_STRUCT_XRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
#[doc = "Field `RESET_PORVDDD` reader - Indicator that a POR occurred. This is a high-voltage cause bit, and hardware clears the other bits when this one is set. It does not block further recording of other high-voltage causes."]
pub type RESET_PORVDDD_R = crate::BitReader<bool>;
#[doc = "Field `RESET_PORVDDD` writer - Indicator that a POR occurred. This is a high-voltage cause bit, and hardware clears the other bits when this one is set. It does not block further recording of other high-voltage causes."]
pub type RESET_PORVDDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle. ULP products: This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above). For products that support high-voltage cause detection, this bit blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
    #[inline(always)]
    pub fn reset_wdt(&self) -> RESET_WDT_R {
        RESET_WDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    pub fn reset_act_fault(&self) -> RESET_ACT_FAULT_R {
        RESET_ACT_FAULT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    pub fn reset_dpslp_fault(&self) -> RESET_DPSLP_FAULT_R {
        RESET_DPSLP_FAULT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test controller or debugger asserted reset. Only resets debug domain. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    pub fn reset_tc_dbgreset(&self) -> RESET_TC_DBGRESET_R {
        RESET_TC_DBGRESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    pub fn reset_soft(&self) -> RESET_SOFT_R {
        RESET_SOFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    pub fn reset_mcwdt0(&self) -> RESET_MCWDT0_R {
        RESET_MCWDT0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    pub fn reset_mcwdt1(&self) -> RESET_MCWDT1_R {
        RESET_MCWDT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    pub fn reset_mcwdt2(&self) -> RESET_MCWDT2_R {
        RESET_MCWDT2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    pub fn reset_mcwdt3(&self) -> RESET_MCWDT3_R {
        RESET_MCWDT3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - External XRES pin was asserted. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
    #[inline(always)]
    pub fn reset_xres(&self) -> RESET_XRES_R {
        RESET_XRES_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External VDDD supply crossed brown-out limit. Note that this cause will only be observable as long as the VDDD supply does not go below the POR (power on reset) detection limit. Below this limit it is not possible to reliably retain information in the device. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    pub fn reset_bodvddd(&self) -> RESET_BODVDDD_R {
        RESET_BODVDDD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External VDDA supply crossed the brown-out limit. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    pub fn reset_bodvdda(&self) -> RESET_BODVDDA_R {
        RESET_BODVDDA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal VCCD core supply crossed the brown-out limit. Note that this detector will detect gross issues with the internal core supply, but may not catch all brown-out conditions. Functional and timing supervision (CSV, WDT) is provided to create fully failsafe internal crash detection. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    pub fn reset_bodvccd(&self) -> RESET_BODVCCD_R {
        RESET_BODVCCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Overvoltage detection on the external VDDD supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    pub fn reset_ovdvddd(&self) -> RESET_OVDVDDD_R {
        RESET_OVDVDDD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Overvoltage detection on the external VDDA supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    pub fn reset_ovdvdda(&self) -> RESET_OVDVDDA_R {
        RESET_OVDVDDA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Overvoltage detection on the internal core VCCD supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    pub fn reset_ovdvccd(&self) -> RESET_OVDVCCD_R {
        RESET_OVDVCCD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Overcurrent detection on the internal VCCD supply when supplied by the ACTIVE power mode linear regulator. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    pub fn reset_ocd_act_linreg(&self) -> RESET_OCD_ACT_LINREG_R {
        RESET_OCD_ACT_LINREG_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Overcurrent detection on the internal VCCD supply when supplied by the DEEPSLEEP power mode linear regulator. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    pub fn reset_ocd_dpslp_linreg(&self) -> RESET_OCD_DPSLP_LINREG_R {
        RESET_OCD_DPSLP_LINREG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - PXRES triggered. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
    #[inline(always)]
    pub fn reset_pxres(&self) -> RESET_PXRES_R {
        RESET_PXRES_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Structural reset was asserted. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
    #[inline(always)]
    pub fn reset_struct_xres(&self) -> RESET_STRUCT_XRES_R {
        RESET_STRUCT_XRES_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicator that a POR occurred. This is a high-voltage cause bit, and hardware clears the other bits when this one is set. It does not block further recording of other high-voltage causes."]
    #[inline(always)]
    pub fn reset_porvddd(&self) -> RESET_PORVDDD_R {
        RESET_PORVDDD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle. ULP products: This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above). For products that support high-voltage cause detection, this bit blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
    #[inline(always)]
    #[must_use]
    pub fn reset_wdt(&mut self) -> RESET_WDT_W<0> {
        RESET_WDT_W::new(self)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    #[must_use]
    pub fn reset_act_fault(&mut self) -> RESET_ACT_FAULT_W<1> {
        RESET_ACT_FAULT_W::new(self)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    #[must_use]
    pub fn reset_dpslp_fault(&mut self) -> RESET_DPSLP_FAULT_W<2> {
        RESET_DPSLP_FAULT_W::new(self)
    }
    #[doc = "Bit 3 - Test controller or debugger asserted reset. Only resets debug domain. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    #[must_use]
    pub fn reset_tc_dbgreset(&mut self) -> RESET_TC_DBGRESET_W<3> {
        RESET_TC_DBGRESET_W::new(self)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    #[must_use]
    pub fn reset_soft(&mut self) -> RESET_SOFT_W<4> {
        RESET_SOFT_W::new(self)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt0(&mut self) -> RESET_MCWDT0_W<5> {
        RESET_MCWDT0_W::new(self)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt1(&mut self) -> RESET_MCWDT1_W<6> {
        RESET_MCWDT1_W::new(self)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt2(&mut self) -> RESET_MCWDT2_W<7> {
        RESET_MCWDT2_W::new(self)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3. This is a low-voltage cause bit that hardware clears when the low-voltage supply is initialized (see comments above)."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt3(&mut self) -> RESET_MCWDT3_W<8> {
        RESET_MCWDT3_W::new(self)
    }
    #[doc = "Bit 16 - External XRES pin was asserted. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
    #[inline(always)]
    #[must_use]
    pub fn reset_xres(&mut self) -> RESET_XRES_W<16> {
        RESET_XRES_W::new(self)
    }
    #[doc = "Bit 17 - External VDDD supply crossed brown-out limit. Note that this cause will only be observable as long as the VDDD supply does not go below the POR (power on reset) detection limit. Below this limit it is not possible to reliably retain information in the device. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    #[must_use]
    pub fn reset_bodvddd(&mut self) -> RESET_BODVDDD_W<17> {
        RESET_BODVDDD_W::new(self)
    }
    #[doc = "Bit 18 - External VDDA supply crossed the brown-out limit. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    #[must_use]
    pub fn reset_bodvdda(&mut self) -> RESET_BODVDDA_W<18> {
        RESET_BODVDDA_W::new(self)
    }
    #[doc = "Bit 19 - Internal VCCD core supply crossed the brown-out limit. Note that this detector will detect gross issues with the internal core supply, but may not catch all brown-out conditions. Functional and timing supervision (CSV, WDT) is provided to create fully failsafe internal crash detection. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    #[must_use]
    pub fn reset_bodvccd(&mut self) -> RESET_BODVCCD_W<19> {
        RESET_BODVCCD_W::new(self)
    }
    #[doc = "Bit 20 - Overvoltage detection on the external VDDD supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    #[must_use]
    pub fn reset_ovdvddd(&mut self) -> RESET_OVDVDDD_W<20> {
        RESET_OVDVDDD_W::new(self)
    }
    #[doc = "Bit 21 - Overvoltage detection on the external VDDA supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    #[must_use]
    pub fn reset_ovdvdda(&mut self) -> RESET_OVDVDDA_W<21> {
        RESET_OVDVDDA_W::new(self)
    }
    #[doc = "Bit 22 - Overvoltage detection on the internal core VCCD supply. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    #[must_use]
    pub fn reset_ovdvccd(&mut self) -> RESET_OVDVCCD_W<22> {
        RESET_OVDVCCD_W::new(self)
    }
    #[doc = "Bit 23 - Overcurrent detection on the internal VCCD supply when supplied by the ACTIVE power mode linear regulator. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    #[must_use]
    pub fn reset_ocd_act_linreg(&mut self) -> RESET_OCD_ACT_LINREG_W<23> {
        RESET_OCD_ACT_LINREG_W::new(self)
    }
    #[doc = "Bit 24 - Overcurrent detection on the internal VCCD supply when supplied by the DEEPSLEEP power mode linear regulator. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR."]
    #[inline(always)]
    #[must_use]
    pub fn reset_ocd_dpslp_linreg(&mut self) -> RESET_OCD_DPSLP_LINREG_W<24> {
        RESET_OCD_DPSLP_LINREG_W::new(self)
    }
    #[doc = "Bit 28 - PXRES triggered. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
    #[inline(always)]
    #[must_use]
    pub fn reset_pxres(&mut self) -> RESET_PXRES_W<28> {
        RESET_PXRES_W::new(self)
    }
    #[doc = "Bit 29 - Structural reset was asserted. This is a high-voltage cause bit that blocks recording of other high-voltage cause bits, except RESET_PORVDDD. Hardware clears this bit during POR. This bit is not blocked by other HV cause bits."]
    #[inline(always)]
    #[must_use]
    pub fn reset_struct_xres(&mut self) -> RESET_STRUCT_XRES_W<29> {
        RESET_STRUCT_XRES_W::new(self)
    }
    #[doc = "Bit 30 - Indicator that a POR occurred. This is a high-voltage cause bit, and hardware clears the other bits when this one is set. It does not block further recording of other high-voltage causes."]
    #[inline(always)]
    #[must_use]
    pub fn reset_porvddd(&mut self) -> RESET_PORVDDD_W<30> {
        RESET_PORVDDD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Cause Observation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_cause](index.html) module"]
pub struct RES_CAUSE_SPEC;
impl crate::RegisterSpec for RES_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res_cause::R](R) reader structure"]
impl crate::Readable for RES_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res_cause::W](W) writer structure"]
impl crate::Writable for RES_CAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RES_CAUSE to value 0x4000_0000"]
impl crate::Resettable for RES_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
