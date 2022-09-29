#[doc = "Register `PWR_CTL2` reader"]
pub struct R(crate::R<PWR_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTL2` writer"]
pub struct W(crate::W<PWR_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTL2_SPEC>;
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
impl From<crate::W<PWR_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINREG_DIS` reader - Explicitly disable the linear Core Regulator. Write zero for Traveo II devices. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Linear Core Regulator is not explicitly disabled. Hardware disables it automatically for internal sequences, including for DEEPSLEEP, HIBERNATE, and XRES low power modes. 1: Linear Core Regulator is explicitly disabled. Only use this for special cases when another source supplies vccd during ACTIVE and SLEEP modes. This setting is only legal when another source supplies vccd, but there is no special hardware protection for this case."]
pub type LINREG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `LINREG_DIS` writer - Explicitly disable the linear Core Regulator. Write zero for Traveo II devices. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Linear Core Regulator is not explicitly disabled. Hardware disables it automatically for internal sequences, including for DEEPSLEEP, HIBERNATE, and XRES low power modes. 1: Linear Core Regulator is explicitly disabled. Only use this for special cases when another source supplies vccd during ACTIVE and SLEEP modes. This setting is only legal when another source supplies vccd, but there is no special hardware protection for this case."]
pub type LINREG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `LINREG_OK` reader - Status of the linear Core Regulator."]
pub type LINREG_OK_R = crate::BitReader<bool>;
#[doc = "Field `LINREG_LPMODE` reader - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
pub type LINREG_LPMODE_R = crate::BitReader<bool>;
#[doc = "Field `LINREG_LPMODE` writer - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
pub type LINREG_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `DPSLP_REG_DIS` reader - N/A"]
pub type DPSLP_REG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `DPSLP_REG_DIS` writer - N/A"]
pub type DPSLP_REG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `RET_REG_DIS` reader - Disable the Retention regulator. This is only legal when another source supplies vccret, but there is no special hardware protection for this case. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Retention Regulator is on. 1: Retention Regulator is off."]
pub type RET_REG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `RET_REG_DIS` writer - Disable the Retention regulator. This is only legal when another source supplies vccret, but there is no special hardware protection for this case. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Retention Regulator is on. 1: Retention Regulator is off."]
pub type RET_REG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `NWELL_REG_DIS` reader - Disable the Nwell regulator. This is only legal when another source supplies vnwell, but there is no special hardware protection for this case. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
pub type NWELL_REG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `NWELL_REG_DIS` writer - Disable the Nwell regulator. This is only legal when another source supplies vnwell, but there is no special hardware protection for this case. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
pub type NWELL_REG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `REFV_DIS` reader - N/A"]
pub type REFV_DIS_R = crate::BitReader<bool>;
#[doc = "Field `REFV_DIS` writer - N/A"]
pub type REFV_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `REFV_OK` reader - Indicates that the normal mode of the voltage reference is ready."]
pub type REFV_OK_R = crate::BitReader<bool>;
#[doc = "Field `REFVBUF_DIS` reader - Disable the voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset."]
pub type REFVBUF_DIS_R = crate::BitReader<bool>;
#[doc = "Field `REFVBUF_DIS` writer - Disable the voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset."]
pub type REFVBUF_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `REFVBUF_OK` reader - Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting REFVBUF_DIS=1."]
pub type REFVBUF_OK_R = crate::BitReader<bool>;
#[doc = "Field `REFVBUF_LPMODE` reader - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type REFVBUF_LPMODE_R = crate::BitReader<bool>;
#[doc = "Field `REFVBUF_LPMODE` writer - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type REFVBUF_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `REFI_DIS` reader - N/A"]
pub type REFI_DIS_R = crate::BitReader<bool>;
#[doc = "Field `REFI_DIS` writer - N/A"]
pub type REFI_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `REFI_OK` reader - Indicates that the current reference is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting REFI_DIS=1."]
pub type REFI_OK_R = crate::BitReader<bool>;
#[doc = "Field `REFI_LPMODE` reader - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
pub type REFI_LPMODE_R = crate::BitReader<bool>;
#[doc = "Field `REFI_LPMODE` writer - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
pub type REFI_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `PORBOD_LPMODE` reader - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type PORBOD_LPMODE_R = crate::BitReader<bool>;
#[doc = "Field `PORBOD_LPMODE` writer - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type PORBOD_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `BGREF_LPMODE` reader - Current is reduced using a sample&amp;hold feature. This requires ILO0 to be operating properly. This register will not set unless CLK_ILO0_CONFIG.ILO0_ENABLE==1. When changing back to continuous operation, keep ILO0 enabled for at least 5 cycles after clearing this bit to allow for internal synchronization. 0: Bandgap Reference circuits operate in higher current mode. 1: Bandgap Reference circuits operate in low power (see above for tradeoffs)."]
pub type BGREF_LPMODE_R = crate::BitReader<bool>;
#[doc = "Field `BGREF_LPMODE` writer - Current is reduced using a sample&amp;hold feature. This requires ILO0 to be operating properly. This register will not set unless CLK_ILO0_CONFIG.ILO0_ENABLE==1. When changing back to continuous operation, keep ILO0 enabled for at least 5 cycles after clearing this bit to allow for internal synchronization. 0: Bandgap Reference circuits operate in higher current mode. 1: Bandgap Reference circuits operate in low power (see above for tradeoffs)."]
pub type BGREF_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
#[doc = "Field `PLL_LS_BYPASS` reader - Bypass level shifter inside the PLL. Unused, if no PLL is present in the product. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
pub type PLL_LS_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `PLL_LS_BYPASS` writer - Bypass level shifter inside the PLL. Unused, if no PLL is present in the product. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
pub type PLL_LS_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Explicitly disable the linear Core Regulator. Write zero for Traveo II devices. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Linear Core Regulator is not explicitly disabled. Hardware disables it automatically for internal sequences, including for DEEPSLEEP, HIBERNATE, and XRES low power modes. 1: Linear Core Regulator is explicitly disabled. Only use this for special cases when another source supplies vccd during ACTIVE and SLEEP modes. This setting is only legal when another source supplies vccd, but there is no special hardware protection for this case."]
    #[inline(always)]
    pub fn linreg_dis(&self) -> LINREG_DIS_R {
        LINREG_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the linear Core Regulator."]
    #[inline(always)]
    pub fn linreg_ok(&self) -> LINREG_OK_R {
        LINREG_OK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    pub fn linreg_lpmode(&self) -> LINREG_LPMODE_R {
        LINREG_LPMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn dpslp_reg_dis(&self) -> DPSLP_REG_DIS_R {
        DPSLP_REG_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable the Retention regulator. This is only legal when another source supplies vccret, but there is no special hardware protection for this case. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    pub fn ret_reg_dis(&self) -> RET_REG_DIS_R {
        RET_REG_DIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable the Nwell regulator. This is only legal when another source supplies vnwell, but there is no special hardware protection for this case. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    pub fn nwell_reg_dis(&self) -> NWELL_REG_DIS_R {
        NWELL_REG_DIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn refv_dis(&self) -> REFV_DIS_R {
        REFV_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates that the normal mode of the voltage reference is ready."]
    #[inline(always)]
    pub fn refv_ok(&self) -> REFV_OK_R {
        REFV_OK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable the voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset."]
    #[inline(always)]
    pub fn refvbuf_dis(&self) -> REFVBUF_DIS_R {
        REFVBUF_DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting REFVBUF_DIS=1."]
    #[inline(always)]
    pub fn refvbuf_ok(&self) -> REFVBUF_OK_R {
        REFVBUF_OK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn refvbuf_lpmode(&self) -> REFVBUF_LPMODE_R {
        REFVBUF_LPMODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn refi_dis(&self) -> REFI_DIS_R {
        REFI_DIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicates that the current reference is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting REFI_DIS=1."]
    #[inline(always)]
    pub fn refi_ok(&self) -> REFI_OK_R {
        REFI_OK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn refi_lpmode(&self) -> REFI_LPMODE_R {
        REFI_LPMODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn porbod_lpmode(&self) -> PORBOD_LPMODE_R {
        PORBOD_LPMODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Current is reduced using a sample&amp;hold feature. This requires ILO0 to be operating properly. This register will not set unless CLK_ILO0_CONFIG.ILO0_ENABLE==1. When changing back to continuous operation, keep ILO0 enabled for at least 5 cycles after clearing this bit to allow for internal synchronization. 0: Bandgap Reference circuits operate in higher current mode. 1: Bandgap Reference circuits operate in low power (see above for tradeoffs)."]
    #[inline(always)]
    pub fn bgref_lpmode(&self) -> BGREF_LPMODE_R {
        BGREF_LPMODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Bypass level shifter inside the PLL. Unused, if no PLL is present in the product. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    pub fn pll_ls_bypass(&self) -> PLL_LS_BYPASS_R {
        PLL_LS_BYPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Explicitly disable the linear Core Regulator. Write zero for Traveo II devices. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Linear Core Regulator is not explicitly disabled. Hardware disables it automatically for internal sequences, including for DEEPSLEEP, HIBERNATE, and XRES low power modes. 1: Linear Core Regulator is explicitly disabled. Only use this for special cases when another source supplies vccd during ACTIVE and SLEEP modes. This setting is only legal when another source supplies vccd, but there is no special hardware protection for this case."]
    #[inline(always)]
    #[must_use]
    pub fn linreg_dis(&mut self) -> LINREG_DIS_W<0> {
        LINREG_DIS_W::new(self)
    }
    #[doc = "Bit 2 - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    #[must_use]
    pub fn linreg_lpmode(&mut self) -> LINREG_LPMODE_W<2> {
        LINREG_LPMODE_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_reg_dis(&mut self) -> DPSLP_REG_DIS_W<4> {
        DPSLP_REG_DIS_W::new(self)
    }
    #[doc = "Bit 8 - Disable the Retention regulator. This is only legal when another source supplies vccret, but there is no special hardware protection for this case. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn ret_reg_dis(&mut self) -> RET_REG_DIS_W<8> {
        RET_REG_DIS_W::new(self)
    }
    #[doc = "Bit 12 - Disable the Nwell regulator. This is only legal when another source supplies vnwell, but there is no special hardware protection for this case. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn nwell_reg_dis(&mut self) -> NWELL_REG_DIS_W<12> {
        NWELL_REG_DIS_W::new(self)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn refv_dis(&mut self) -> REFV_DIS_W<16> {
        REFV_DIS_W::new(self)
    }
    #[doc = "Bit 20 - Disable the voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset."]
    #[inline(always)]
    #[must_use]
    pub fn refvbuf_dis(&mut self) -> REFVBUF_DIS_W<20> {
        REFVBUF_DIS_W::new(self)
    }
    #[doc = "Bit 22 - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES, HIBERNATE wakeup, or supply supervision reset. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn refvbuf_lpmode(&mut self) -> REFVBUF_LPMODE_W<22> {
        REFVBUF_LPMODE_W::new(self)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn refi_dis(&mut self) -> REFI_DIS_W<24> {
        REFI_DIS_W::new(self)
    }
    #[doc = "Bit 26 - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn refi_lpmode(&mut self) -> REFI_LPMODE_W<26> {
        REFI_LPMODE_W::new(self)
    }
    #[doc = "Bit 27 - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn porbod_lpmode(&mut self) -> PORBOD_LPMODE_W<27> {
        PORBOD_LPMODE_W::new(self)
    }
    #[doc = "Bit 28 - Current is reduced using a sample&amp;hold feature. This requires ILO0 to be operating properly. This register will not set unless CLK_ILO0_CONFIG.ILO0_ENABLE==1. When changing back to continuous operation, keep ILO0 enabled for at least 5 cycles after clearing this bit to allow for internal synchronization. 0: Bandgap Reference circuits operate in higher current mode. 1: Bandgap Reference circuits operate in low power (see above for tradeoffs)."]
    #[inline(always)]
    #[must_use]
    pub fn bgref_lpmode(&mut self) -> BGREF_LPMODE_W<28> {
        BGREF_LPMODE_W::new(self)
    }
    #[doc = "Bit 31 - Bypass level shifter inside the PLL. Unused, if no PLL is present in the product. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    #[must_use]
    pub fn pll_ls_bypass(&mut self) -> PLL_LS_BYPASS_W<31> {
        PLL_LS_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctl2](index.html) module"]
pub struct PWR_CTL2_SPEC;
impl crate::RegisterSpec for PWR_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctl2::R](R) reader structure"]
impl crate::Readable for PWR_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctl2::W](W) writer structure"]
impl crate::Writable for PWR_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CTL2 to value 0"]
impl crate::Resettable for PWR_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
