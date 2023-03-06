#[doc = "Register `CM7_0_CTL` reader"]
pub struct R(crate::R<CM7_0_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_0_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_0_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_0_CTL` writer"]
pub struct W(crate::W<CM7_0_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_0_CTL_SPEC>;
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
impl From<crate::W<CM7_0_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_0_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPB_LOCK` reader - Write disable for specific CPU registers: Bit 0: ITCMR register. '0': enabled; '1': disabled. Bit 1: DTCMR register. '0': enabled; '1': disabled. Bit 2: AHBPCR register. '0': enabled; '1': disabled. Bit 3: VTOR register. '0': enabled; '1': disabled."]
pub type PPB_LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPB_LOCK` writer - Write disable for specific CPU registers: Bit 0: ITCMR register. '0': enabled; '1': disabled. Bit 1: DTCMR register. '0': enabled; '1': disabled. Bit 2: AHBPCR register. '0': enabled; '1': disabled. Bit 3: VTOR register. '0': enabled; '1': disabled."]
pub type PPB_LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM7_0_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPU_WAIT` reader - When this signal is '1' out of reset, it forces the CPU into a quiescent state that delays its boot-up sequence and instruction execution until this signal is driven '0'. This allows the TCMs to be loaded by the system before the CPU performs any TCM accesses. The CPU AHBS interface functions while CPU_WAIT is '1' and services transactions initiated by the system (e.g. DMA controller) to load the TCMs."]
pub type CPU_WAIT_R = crate::BitReader<bool>;
#[doc = "Field `CPU_WAIT` writer - When this signal is '1' out of reset, it forces the CPU into a quiescent state that delays its boot-up sequence and instruction execution until this signal is driven '0'. This allows the TCMs to be loaded by the system before the CPU performs any TCM accesses. The CPU AHBS interface functions while CPU_WAIT is '1' and services transactions initiated by the system (e.g. DMA controller) to load the TCMs."]
pub type CPU_WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `INIT_TCM_EN` reader - TCM enable initialization after reset: Bit 0: ITCM. '0': disabled; '1': enabled. Bit 1: DTCM. '0': disabled; '1': enabled."]
pub type INIT_TCM_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INIT_TCM_EN` writer - TCM enable initialization after reset: Bit 0: ITCM. '0': disabled; '1': enabled. Bit 1: DTCM. '0': disabled; '1': enabled."]
pub type INIT_TCM_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM7_0_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `INIT_RMW_EN` reader - TCM read-modify-write enable initialization after reset: Bit 0: ITCM. '0': disabled; '1': enabled. Bit 1: DTCM. '0': disabled; '1': enabled. Note: When TCM ECC is enabled, the read-modify-write functionality should be enabled. This prevents partial (sub-word) writes to the TCM."]
pub type INIT_RMW_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INIT_RMW_EN` writer - TCM read-modify-write enable initialization after reset: Bit 0: ITCM. '0': disabled; '1': enabled. Bit 1: DTCM. '0': disabled; '1': enabled. Note: When TCM ECC is enabled, the read-modify-write functionality should be enabled. This prevents partial (sub-word) writes to the TCM."]
pub type INIT_RMW_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM7_0_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ITCM_ECC_EN` reader - ITCM ECC enable: '0': Disabled. '1': Enabled."]
pub type ITCM_ECC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ITCM_ECC_EN` writer - ITCM ECC enable: '0': Disabled. '1': Enabled."]
pub type ITCM_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `ITCM_ECC_INJ_EN` reader - ITCM ECC error injection enable: '0': Disabled. '1': Enabled."]
pub type ITCM_ECC_INJ_EN_R = crate::BitReader<bool>;
#[doc = "Field `ITCM_ECC_INJ_EN` writer - ITCM ECC error injection enable: '0': Disabled. '1': Enabled."]
pub type ITCM_ECC_INJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `ITCM_READ_WS` reader - ITCM read wait states (writes have no wait states). '0': 0 wait states. '1': 1 wait state."]
pub type ITCM_READ_WS_R = crate::BitReader<bool>;
#[doc = "Field `ITCM_READ_WS` writer - ITCM read wait states (writes have no wait states). '0': 0 wait states. '1': 1 wait state."]
pub type ITCM_READ_WS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `ITCM_ECC_CHECK_DIS` reader - Disable ECC checking and thus fault reports. This also disables ECC correction (required to enable initialization). Intended usage is initialization. This bit is ignored when TCM_ECC_EN=0."]
pub type ITCM_ECC_CHECK_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ITCM_ECC_CHECK_DIS` writer - Disable ECC checking and thus fault reports. This also disables ECC correction (required to enable initialization). Intended usage is initialization. This bit is ignored when TCM_ECC_EN=0."]
pub type ITCM_ECC_CHECK_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `DTCM_ECC_EN` reader - DTCM ECC enable: '0': Disabled. '1': Enabled."]
pub type DTCM_ECC_EN_R = crate::BitReader<bool>;
#[doc = "Field `DTCM_ECC_EN` writer - DTCM ECC enable: '0': Disabled. '1': Enabled."]
pub type DTCM_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `DTCM_ECC_INJ_EN` reader - DTCM ECC error injection enable: '0': Disabled. '1': Enabled."]
pub type DTCM_ECC_INJ_EN_R = crate::BitReader<bool>;
#[doc = "Field `DTCM_ECC_INJ_EN` writer - DTCM ECC error injection enable: '0': Disabled. '1': Enabled."]
pub type DTCM_ECC_INJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `DTCM_READ_WS` reader - DTCM read wait states (writes have no wait states). '0': 0 wait states. '1': 1 wait state."]
pub type DTCM_READ_WS_R = crate::BitReader<bool>;
#[doc = "Field `DTCM_READ_WS` writer - DTCM read wait states (writes have no wait states). '0': 0 wait states. '1': 1 wait state."]
pub type DTCM_READ_WS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `TCMC_EN` reader - CM7 TCMC access control: '0': Disable access to the CM7 I/D-TCM slave port (AHBS). Access attempts will get a bus error response. '1': Enable access to the CM7 I/D-TCM slave port (AHBS). Before switching the CM7 power off, set this field to 0, and confirm that there are no outstanding AXI transactions and no ongoing AHB transactions to the CM7 TCMs, by checking that bits TCMC_* in CM7_x_STATUS are '0'."]
pub type TCMC_EN_R = crate::BitReader<bool>;
#[doc = "Field `TCMC_EN` writer - CM7 TCMC access control: '0': Disable access to the CM7 I/D-TCM slave port (AHBS). Access attempts will get a bus error response. '1': Enable access to the CM7 I/D-TCM slave port (AHBS). Before switching the CM7 power off, set this field to 0, and confirm that there are no outstanding AXI transactions and no ongoing AHB transactions to the CM7 TCMs, by checking that bits TCMC_* in CM7_x_STATUS are '0'."]
pub type TCMC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `IOC_MASK` reader - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
pub type IOC_MASK_R = crate::BitReader<bool>;
#[doc = "Field `IOC_MASK` writer - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
pub type IOC_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `DZC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type DZC_MASK_R = crate::BitReader<bool>;
#[doc = "Field `DZC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type DZC_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `OFC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type OFC_MASK_R = crate::BitReader<bool>;
#[doc = "Field `OFC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type OFC_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `UFC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type UFC_MASK_R = crate::BitReader<bool>;
#[doc = "Field `UFC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
pub type UFC_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `IXC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
pub type IXC_MASK_R = crate::BitReader<bool>;
#[doc = "Field `IXC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
pub type IXC_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
#[doc = "Field `IDC_MASK` reader - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
pub type IDC_MASK_R = crate::BitReader<bool>;
#[doc = "Field `IDC_MASK` writer - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
pub type IDC_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_0_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Write disable for specific CPU registers: Bit 0: ITCMR register. '0': enabled; '1': disabled. Bit 1: DTCMR register. '0': enabled; '1': disabled. Bit 2: AHBPCR register. '0': enabled; '1': disabled. Bit 3: VTOR register. '0': enabled; '1': disabled."]
    #[inline(always)]
    pub fn ppb_lock(&self) -> PPB_LOCK_R {
        PPB_LOCK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - When this signal is '1' out of reset, it forces the CPU into a quiescent state that delays its boot-up sequence and instruction execution until this signal is driven '0'. This allows the TCMs to be loaded by the system before the CPU performs any TCM accesses. The CPU AHBS interface functions while CPU_WAIT is '1' and services transactions initiated by the system (e.g. DMA controller) to load the TCMs."]
    #[inline(always)]
    pub fn cpu_wait(&self) -> CPU_WAIT_R {
        CPU_WAIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - TCM enable initialization after reset: Bit 0: ITCM. '0': disabled; '1': enabled. Bit 1: DTCM. '0': disabled; '1': enabled."]
    #[inline(always)]
    pub fn init_tcm_en(&self) -> INIT_TCM_EN_R {
        INIT_TCM_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TCM read-modify-write enable initialization after reset: Bit 0: ITCM. '0': disabled; '1': enabled. Bit 1: DTCM. '0': disabled; '1': enabled. Note: When TCM ECC is enabled, the read-modify-write functionality should be enabled. This prevents partial (sub-word) writes to the TCM."]
    #[inline(always)]
    pub fn init_rmw_en(&self) -> INIT_RMW_EN_R {
        INIT_RMW_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - ITCM ECC enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn itcm_ecc_en(&self) -> ITCM_ECC_EN_R {
        ITCM_ECC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ITCM ECC error injection enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn itcm_ecc_inj_en(&self) -> ITCM_ECC_INJ_EN_R {
        ITCM_ECC_INJ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ITCM read wait states (writes have no wait states). '0': 0 wait states. '1': 1 wait state."]
    #[inline(always)]
    pub fn itcm_read_ws(&self) -> ITCM_READ_WS_R {
        ITCM_READ_WS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable ECC checking and thus fault reports. This also disables ECC correction (required to enable initialization). Intended usage is initialization. This bit is ignored when TCM_ECC_EN=0."]
    #[inline(always)]
    pub fn itcm_ecc_check_dis(&self) -> ITCM_ECC_CHECK_DIS_R {
        ITCM_ECC_CHECK_DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DTCM ECC enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn dtcm_ecc_en(&self) -> DTCM_ECC_EN_R {
        DTCM_ECC_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DTCM ECC error injection enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn dtcm_ecc_inj_en(&self) -> DTCM_ECC_INJ_EN_R {
        DTCM_ECC_INJ_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DTCM read wait states (writes have no wait states). '0': 0 wait states. '1': 1 wait state."]
    #[inline(always)]
    pub fn dtcm_read_ws(&self) -> DTCM_READ_WS_R {
        DTCM_READ_WS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CM7 TCMC access control: '0': Disable access to the CM7 I/D-TCM slave port (AHBS). Access attempts will get a bus error response. '1': Enable access to the CM7 I/D-TCM slave port (AHBS). Before switching the CM7 power off, set this field to 0, and confirm that there are no outstanding AXI transactions and no ongoing AHB transactions to the CM7 TCMs, by checking that bits TCMC_* in CM7_x_STATUS are '0'."]
    #[inline(always)]
    pub fn tcmc_en(&self) -> TCMC_EN_R {
        TCMC_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ioc_mask(&self) -> IOC_MASK_R {
        IOC_MASK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn dzc_mask(&self) -> DZC_MASK_R {
        DZC_MASK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ofc_mask(&self) -> OFC_MASK_R {
        OFC_MASK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    pub fn ufc_mask(&self) -> UFC_MASK_R {
        UFC_MASK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
    #[inline(always)]
    pub fn ixc_mask(&self) -> IXC_MASK_R {
        IXC_MASK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
    #[inline(always)]
    pub fn idc_mask(&self) -> IDC_MASK_R {
        IDC_MASK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write disable for specific CPU registers: Bit 0: ITCMR register. '0': enabled; '1': disabled. Bit 1: DTCMR register. '0': enabled; '1': disabled. Bit 2: AHBPCR register. '0': enabled; '1': disabled. Bit 3: VTOR register. '0': enabled; '1': disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ppb_lock(&mut self) -> PPB_LOCK_W<0> {
        PPB_LOCK_W::new(self)
    }
    #[doc = "Bit 4 - When this signal is '1' out of reset, it forces the CPU into a quiescent state that delays its boot-up sequence and instruction execution until this signal is driven '0'. This allows the TCMs to be loaded by the system before the CPU performs any TCM accesses. The CPU AHBS interface functions while CPU_WAIT is '1' and services transactions initiated by the system (e.g. DMA controller) to load the TCMs."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_wait(&mut self) -> CPU_WAIT_W<4> {
        CPU_WAIT_W::new(self)
    }
    #[doc = "Bits 8:9 - TCM enable initialization after reset: Bit 0: ITCM. '0': disabled; '1': enabled. Bit 1: DTCM. '0': disabled; '1': enabled."]
    #[inline(always)]
    #[must_use]
    pub fn init_tcm_en(&mut self) -> INIT_TCM_EN_W<8> {
        INIT_TCM_EN_W::new(self)
    }
    #[doc = "Bits 10:11 - TCM read-modify-write enable initialization after reset: Bit 0: ITCM. '0': disabled; '1': enabled. Bit 1: DTCM. '0': disabled; '1': enabled. Note: When TCM ECC is enabled, the read-modify-write functionality should be enabled. This prevents partial (sub-word) writes to the TCM."]
    #[inline(always)]
    #[must_use]
    pub fn init_rmw_en(&mut self) -> INIT_RMW_EN_W<10> {
        INIT_RMW_EN_W::new(self)
    }
    #[doc = "Bit 16 - ITCM ECC enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn itcm_ecc_en(&mut self) -> ITCM_ECC_EN_W<16> {
        ITCM_ECC_EN_W::new(self)
    }
    #[doc = "Bit 17 - ITCM ECC error injection enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn itcm_ecc_inj_en(&mut self) -> ITCM_ECC_INJ_EN_W<17> {
        ITCM_ECC_INJ_EN_W::new(self)
    }
    #[doc = "Bit 18 - ITCM read wait states (writes have no wait states). '0': 0 wait states. '1': 1 wait state."]
    #[inline(always)]
    #[must_use]
    pub fn itcm_read_ws(&mut self) -> ITCM_READ_WS_W<18> {
        ITCM_READ_WS_W::new(self)
    }
    #[doc = "Bit 19 - Disable ECC checking and thus fault reports. This also disables ECC correction (required to enable initialization). Intended usage is initialization. This bit is ignored when TCM_ECC_EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn itcm_ecc_check_dis(&mut self) -> ITCM_ECC_CHECK_DIS_W<19> {
        ITCM_ECC_CHECK_DIS_W::new(self)
    }
    #[doc = "Bit 20 - DTCM ECC enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dtcm_ecc_en(&mut self) -> DTCM_ECC_EN_W<20> {
        DTCM_ECC_EN_W::new(self)
    }
    #[doc = "Bit 21 - DTCM ECC error injection enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dtcm_ecc_inj_en(&mut self) -> DTCM_ECC_INJ_EN_W<21> {
        DTCM_ECC_INJ_EN_W::new(self)
    }
    #[doc = "Bit 22 - DTCM read wait states (writes have no wait states). '0': 0 wait states. '1': 1 wait state."]
    #[inline(always)]
    #[must_use]
    pub fn dtcm_read_ws(&mut self) -> DTCM_READ_WS_W<22> {
        DTCM_READ_WS_W::new(self)
    }
    #[doc = "Bit 23 - CM7 TCMC access control: '0': Disable access to the CM7 I/D-TCM slave port (AHBS). Access attempts will get a bus error response. '1': Enable access to the CM7 I/D-TCM slave port (AHBS). Before switching the CM7 power off, set this field to 0, and confirm that there are no outstanding AXI transactions and no ongoing AHB transactions to the CM7 TCMs, by checking that bits TCMC_* in CM7_x_STATUS are '0'."]
    #[inline(always)]
    #[must_use]
    pub fn tcmc_en(&mut self) -> TCMC_EN_W<23> {
        TCMC_EN_W::new(self)
    }
    #[doc = "Bit 24 - CPU floating point unit (FPU) exception mask for the CPU's FPCSR.IOC 'invalid operation' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the ARM architecture does NOT support FPU exceptions; i.e. there is no precise FPU exception handler. Instead, FPU conditions are captured in the CPU's FPCSR register and the conditions are provided as CPU interface signals. The interface signals are 'masked' with the fields a provide by this register (CM7_0_CTL). The 'masked' signals are reduced/OR-ed into a single CPU floating point interrupt signal. The associated CPU interrupt handler allows for imprecise handling of FPU exception conditions. Note: the CPU's FPCSR exception conditions are 'sticky'. Typically, the CPU FPU interrupt handler will clear the exception condition(s) to '0'. Note: by default, the FPU exception masks are '0'. Therefore, FPU exception conditions will NOT activate the CPU's floating point interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ioc_mask(&mut self) -> IOC_MASK_W<24> {
        IOC_MASK_W::new(self)
    }
    #[doc = "Bit 25 - CPU FPU exception mask for the CPU's FPCSR.DZC 'divide by zero' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dzc_mask(&mut self) -> DZC_MASK_W<25> {
        DZC_MASK_W::new(self)
    }
    #[doc = "Bit 26 - CPU FPU exception mask for the CPU's FPCSR.OFC 'overflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ofc_mask(&mut self) -> OFC_MASK_W<26> {
        OFC_MASK_W::new(self)
    }
    #[doc = "Bit 27 - CPU FPU exception mask for the CPU's FPCSR.UFC 'underflow' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ufc_mask(&mut self) -> UFC_MASK_W<27> {
        UFC_MASK_W::new(self)
    }
    #[doc = "Bit 28 - CPU FPU exception mask for the CPU's FPCSR.IXC 'inexact' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: the 'inexact' condition is set as a result of rounding. Rounding may occur frequently and is typically not an error condition. To prevent frequent CPU FPU interrupts as a result of rounding, this field is typically set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ixc_mask(&mut self) -> IXC_MASK_W<28> {
        IXC_MASK_W::new(self)
    }
    #[doc = "Bit 31 - CPU FPU exception mask for the CPU's FPCSR.IDC 'input denormalized' exception condition: '0': The CPU's exception condition does NOT activate the CPU's floating point interrupt. '1': the CPU's exception condition activates the CPU's floating point interrupt. Note: if the CPU FPCSR.FZ field is set to '1', denormalized inputs are 'flushed to zero'. Dependent on the FPU algorithm, this may or may not occur frequently. To prevent frequent CPU FPU interrupts as a result of denormalized inputs, this field may be set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn idc_mask(&mut self) -> IDC_MASK_W<31> {
        IDC_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM7 0 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_0_ctl](index.html) module"]
pub struct CM7_0_CTL_SPEC;
impl crate::RegisterSpec for CM7_0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_0_ctl::R](R) reader structure"]
impl crate::Readable for CM7_0_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_0_ctl::W](W) writer structure"]
impl crate::Writable for CM7_0_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_0_CTL to value 0x1f"]
impl crate::Resettable for CM7_0_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
