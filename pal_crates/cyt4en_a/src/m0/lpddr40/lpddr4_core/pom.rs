#[doc = "Register `POM` reader"]
pub struct R(crate::R<POM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POM` writer"]
pub struct W(crate::W<POM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POM_SPEC>;
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
impl From<crate::W<POM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POM_CHANEN` reader - LPDDR4 channel enable for training. CHAN_NO = 2'b00 Enable no channel CHAN_0 = 2'b01 Enable channel 0 CHAN_1 = 2'b10 Enable channel 1 CHAN_BOTH = 2'b11 Enable channel 0 and channel 1"]
pub type POM_CHANEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POM_CHANEN` writer - LPDDR4 channel enable for training. CHAN_NO = 2'b00 Enable no channel CHAN_0 = 2'b01 Enable channel 0 CHAN_1 = 2'b10 Enable channel 1 CHAN_BOTH = 2'b11 Enable channel 0 and channel 1"]
pub type POM_CHANEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POM_SPEC, u8, u8, 2, O>;
#[doc = "Field `POM_DFIEN` reader - DFI interface enable. After training, MC must write 1'b1 to this field for normal DFI operation. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DFIEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_DFIEN` writer - DFI interface enable. After training, MC must write 1'b1 to this field for normal DFI operation. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DFIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_PROC` reader - PHY operation proceed. MC must write 1'b1 to this field when request from the PHY is satisfied. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_PROC_R = crate::BitReader<bool>;
#[doc = "Field `POM_PROC` writer - PHY operation proceed. MC must write 1'b1 to this field when request from the PHY is satisfied. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_PROC_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_PHYSETEN` reader - PHY setting reload enable To reduce PHY start-up time, DTI PHY supports a process called 'PHY setting reload' to put the PHY to normal operation mode without performing any time-consumed training process. To enable PHY setting reload process, user must write 1'b1 to physeten field in POM register. The completion of this process is indicated through physetc field in POS register. After reload process is completed, user must write 1'b0 to physeten field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_PHYSETEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_PHYSETEN` writer - PHY setting reload enable To reduce PHY start-up time, DTI PHY supports a process called 'PHY setting reload' to put the PHY to normal operation mode without performing any time-consumed training process. To enable PHY setting reload process, user must write 1'b1 to physeten field in POM register. The completion of this process is indicated through physetc field in POS register. After reload process is completed, user must write 1'b0 to physeten field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_PHYSETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_PHYFSEN` reader - PHY frequency change enable To switch Frequency Set-point, user must write 1'b1 to phyfsen and phyinit field in POM register with target Frequency Set-point in fs field. fs value must be different from operating Frequency Set-point. User can obtain operating Frequency Set-point by reading ofs field in POS register. The completion of this process is indicated through phyfsc field in the POS. After this process is completed, user must write 1'b0 to phyfsen and phyinit. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_PHYFSEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_PHYFSEN` writer - PHY frequency change enable To switch Frequency Set-point, user must write 1'b1 to phyfsen and phyinit field in POM register with target Frequency Set-point in fs field. fs value must be different from operating Frequency Set-point. User can obtain operating Frequency Set-point by reading ofs field in POS register. The completion of this process is indicated through phyfsc field in the POS. After this process is completed, user must write 1'b0 to phyfsen and phyinit. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_PHYFSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_PHYINIT` reader - PHY initialization enable. PHY initialization includes DLL init, DRAM init, and PHY training. Software can do each step separately by writing other bits in this register individually.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_PHYINIT_R = crate::BitReader<bool>;
#[doc = "Field `POM_PHYINIT` writer - PHY initialization enable. PHY initialization includes DLL init, DRAM init, and PHY training. Software can do each step separately by writing other bits in this register individually.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_PHYINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_DLLRSTEN` reader - DLL reset enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DLLRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_DLLRSTEN` writer - DLL reset enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DLLRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_DRAMINITEN` reader - DRAM initialization enable The PHY built-in DRAM initialization is triggered using draminiten and phyinit fields in the POM - PHY Operation Mode Register. Its completion is indicated through bit draminitc in the POS - PHY Operation Status Register. The DRAM mode registers and timing registers must be configured correctly prior to triggering DRAM initialization, even if user chooses not to use the built-in initialization. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DRAMINITEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_DRAMINITEN` writer - DRAM initialization enable The PHY built-in DRAM initialization is triggered using draminiten and phyinit fields in the POM - PHY Operation Mode Register. Its completion is indicated through bit draminitc in the POS - PHY Operation Status Register. The DRAM mode registers and timing registers must be configured correctly prior to triggering DRAM initialization, even if user chooses not to use the built-in initialization. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DRAMINITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_VREFDQRDEN` reader - PHY VREF-DQ training enable. The PHY built-in VREF-DQ training is triggered using vrefdqrden and phyinit field in the POM. Its completion is indicated through bit vrefdqrdc in thePOS. The field vrefdqrderr in the PTS0 shows that the PHY cannot find correct VREF setting for its internal receiver after training. The error is asserted if the PHY doesn't assert VT_DONE. The PHY looks at the voltage high level during VREF-DQ training. If (voltage_high/2) is beyond the VREF generation specification then VT_DONE won't assert. After PHY VREF-DQ training is completed, user must write 1'b0 to verfdqrden and phyinit field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_VREFDQRDEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_VREFDQRDEN` writer - PHY VREF-DQ training enable. The PHY built-in VREF-DQ training is triggered using vrefdqrden and phyinit field in the POM. Its completion is indicated through bit vrefdqrdc in thePOS. The field vrefdqrderr in the PTS0 shows that the PHY cannot find correct VREF setting for its internal receiver after training. The error is asserted if the PHY doesn't assert VT_DONE. The PHY looks at the voltage high level during VREF-DQ training. If (voltage_high/2) is beyond the VREF generation specification then VT_DONE won't assert. After PHY VREF-DQ training is completed, user must write 1'b0 to verfdqrden and phyinit field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_VREFDQRDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_VREFCAEN` reader - DRAM VREF-CA training enable. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_VREFCAEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_VREFCAEN` writer - DRAM VREF-CA training enable. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_VREFCAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_GTEN` reader - Gate training enable. The PHY built-in gate training is triggered using POM_GTEN and POM_PHYINIT field in thePOM. Its completion is indicated through bit GTC in the POS. The field GTERR in the PTS0 shows gate training error for each Data Module in the PHY while training. After DQS gate training is completed, user must write 1'b0 to POM_GTE and POM_PHYINIT. The error is asserted when the PHY doesn't provide HIGH on GT_STATUS. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_GTEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_GTEN` writer - Gate training enable. The PHY built-in gate training is triggered using POM_GTEN and POM_PHYINIT field in thePOM. Its completion is indicated through bit GTC in the POS. The field GTERR in the PTS0 shows gate training error for each Data Module in the PHY while training. After DQS gate training is completed, user must write 1'b0 to POM_GTE and POM_PHYINIT. The error is asserted when the PHY doesn't provide HIGH on GT_STATUS. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_GTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_WRLVLEN` reader - Write leveling enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_WRLVLEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_WRLVLEN` writer - Write leveling enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_WRLVLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_RDLVLEN` reader - Read data eye DQ training enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_RDLVLEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_RDLVLEN` writer - Read data eye DQ training enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_RDLVLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_VREFDQWREN` reader - DRAM VREF-DQ training enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_VREFDQWREN_R = crate::BitReader<bool>;
#[doc = "Field `POM_VREFDQWREN` writer - DRAM VREF-DQ training enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_VREFDQWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_DLYEVALEN` reader - Delay evaluation enable. Delay evaluation process must be performed when there is at least one DDR chip with CK-DQS delay exceeding 1 clock cycle. If all DDR chips have CK-DQS delay smaller than 1 clock cycle, this process is not needed. This process can be enabled by writing 1'b1 to dlyevalen field and phyinit field in POM register. The completion of this process is identified by dlyevalc field in POS register. After evaluation is completed, user must write 1'b0 to dlyevalen field and phyinit. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DLYEVALEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_DLYEVALEN` writer - Delay evaluation enable. Delay evaluation process must be performed when there is at least one DDR chip with CK-DQS delay exceeding 1 clock cycle. If all DDR chips have CK-DQS delay smaller than 1 clock cycle, this process is not needed. This process can be enabled by writing 1'b1 to dlyevalen field and phyinit field in POM register. The completion of this process is identified by dlyevalc field in POS register. After evaluation is completed, user must write 1'b0 to dlyevalen field and phyinit. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DLYEVALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_SANCHKEN` reader - Write/read sanity check enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_SANCHKEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_SANCHKEN` writer - Write/read sanity check enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_SANCHKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_FS` reader - Target Frequency Set-point for PHY operations. Must be different from operating set point. FS0 = 0 Frequency Set Point 0 FS1 = 1 Frequency Set Point 1"]
pub type POM_FS_R = crate::BitReader<bool>;
#[doc = "Field `POM_FS` writer - Target Frequency Set-point for PHY operations. Must be different from operating set point. FS0 = 0 Frequency Set Point 0 FS1 = 1 Frequency Set Point 1"]
pub type POM_FS_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_CLKLOCKEN` reader - MC-PHY clock phase lock request. When running in 1:4 mode user must write 1'b1 to clklocken field in POM register along with dllrsten and phyinit, this enables phase lock mechanism inside the PHY. User must clear this field when writing 1'b0 to phyinit field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_CLKLOCKEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_CLKLOCKEN` writer - MC-PHY clock phase lock request. When running in 1:4 mode user must write 1'b1 to clklocken field in POM register along with dllrsten and phyinit, this enables phase lock mechanism inside the PHY. User must clear this field when writing 1'b0 to phyinit field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_CLKLOCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_CMDDLYEN` reader - Load PHY command bus delay To help PHY debugging, DTI PHY supports user to set timing delay for its memory control and address signals. This feature is also to meet CA/CS setup and hold time in slow frequency operation. To enable PHY command delay load, user must write 1'b1 to cmddlyen and phyinit field in POM register. The completion of this process is indicated via cmddlyc field in POS register. After loading process is completed, user must write 1'b0 to cmddlyen field. cmddlyen must be combined with physeten to reload trained setting since physeten does not set for command bus. In this case user must write PHY setting to PTSR* registers then write 1'b1 to cmddlyen, physeten and phyinit fields in POM register. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_CMDDLYEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_CMDDLYEN` writer - Load PHY command bus delay To help PHY debugging, DTI PHY supports user to set timing delay for its memory control and address signals. This feature is also to meet CA/CS setup and hold time in slow frequency operation. To enable PHY command delay load, user must write 1'b1 to cmddlyen and phyinit field in POM register. The completion of this process is indicated via cmddlyc field in POS register. After loading process is completed, user must write 1'b0 to cmddlyen field. cmddlyen must be combined with physeten to reload trained setting since physeten does not set for command bus. In this case user must write PHY setting to PTSR* registers then write 1'b1 to cmddlyen, physeten and phyinit fields in POM register. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_CMDDLYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_PWDEXIT` reader - Exit from power-down indicator flag"]
pub type POM_PWDEXIT_R = crate::BitReader<bool>;
#[doc = "Field `POM_PWDEXIT` writer - Exit from power-down indicator flag"]
pub type POM_PWDEXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_ODT` reader - On Die Termination (ODT) control flag.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_ODT_R = crate::BitReader<bool>;
#[doc = "Field `POM_ODT` writer - On Die Termination (ODT) control flag.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_ODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
#[doc = "Field `POM_DQSDQEN` reader - tDQS2DQ retraining enable. For handling tDQS2DQ shifting.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DQSDQEN_R = crate::BitReader<bool>;
#[doc = "Field `POM_DQSDQEN` writer - tDQS2DQ retraining enable. For handling tDQS2DQ shifting.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type POM_DQSDQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - LPDDR4 channel enable for training. CHAN_NO = 2'b00 Enable no channel CHAN_0 = 2'b01 Enable channel 0 CHAN_1 = 2'b10 Enable channel 1 CHAN_BOTH = 2'b11 Enable channel 0 and channel 1"]
    #[inline(always)]
    pub fn pom_chanen(&self) -> POM_CHANEN_R {
        POM_CHANEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DFI interface enable. After training, MC must write 1'b1 to this field for normal DFI operation. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_dfien(&self) -> POM_DFIEN_R {
        POM_DFIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PHY operation proceed. MC must write 1'b1 to this field when request from the PHY is satisfied. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_proc(&self) -> POM_PROC_R {
        POM_PROC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY setting reload enable To reduce PHY start-up time, DTI PHY supports a process called 'PHY setting reload' to put the PHY to normal operation mode without performing any time-consumed training process. To enable PHY setting reload process, user must write 1'b1 to physeten field in POM register. The completion of this process is indicated through physetc field in POS register. After reload process is completed, user must write 1'b0 to physeten field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_physeten(&self) -> POM_PHYSETEN_R {
        POM_PHYSETEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PHY frequency change enable To switch Frequency Set-point, user must write 1'b1 to phyfsen and phyinit field in POM register with target Frequency Set-point in fs field. fs value must be different from operating Frequency Set-point. User can obtain operating Frequency Set-point by reading ofs field in POS register. The completion of this process is indicated through phyfsc field in the POS. After this process is completed, user must write 1'b0 to phyfsen and phyinit. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_phyfsen(&self) -> POM_PHYFSEN_R {
        POM_PHYFSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHY initialization enable. PHY initialization includes DLL init, DRAM init, and PHY training. Software can do each step separately by writing other bits in this register individually.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_phyinit(&self) -> POM_PHYINIT_R {
        POM_PHYINIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DLL reset enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_dllrsten(&self) -> POM_DLLRSTEN_R {
        POM_DLLRSTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DRAM initialization enable The PHY built-in DRAM initialization is triggered using draminiten and phyinit fields in the POM - PHY Operation Mode Register. Its completion is indicated through bit draminitc in the POS - PHY Operation Status Register. The DRAM mode registers and timing registers must be configured correctly prior to triggering DRAM initialization, even if user chooses not to use the built-in initialization. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_draminiten(&self) -> POM_DRAMINITEN_R {
        POM_DRAMINITEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PHY VREF-DQ training enable. The PHY built-in VREF-DQ training is triggered using vrefdqrden and phyinit field in the POM. Its completion is indicated through bit vrefdqrdc in thePOS. The field vrefdqrderr in the PTS0 shows that the PHY cannot find correct VREF setting for its internal receiver after training. The error is asserted if the PHY doesn't assert VT_DONE. The PHY looks at the voltage high level during VREF-DQ training. If (voltage_high/2) is beyond the VREF generation specification then VT_DONE won't assert. After PHY VREF-DQ training is completed, user must write 1'b0 to verfdqrden and phyinit field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_vrefdqrden(&self) -> POM_VREFDQRDEN_R {
        POM_VREFDQRDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DRAM VREF-CA training enable. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_vrefcaen(&self) -> POM_VREFCAEN_R {
        POM_VREFCAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Gate training enable. The PHY built-in gate training is triggered using POM_GTEN and POM_PHYINIT field in thePOM. Its completion is indicated through bit GTC in the POS. The field GTERR in the PTS0 shows gate training error for each Data Module in the PHY while training. After DQS gate training is completed, user must write 1'b0 to POM_GTE and POM_PHYINIT. The error is asserted when the PHY doesn't provide HIGH on GT_STATUS. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_gten(&self) -> POM_GTEN_R {
        POM_GTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write leveling enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_wrlvlen(&self) -> POM_WRLVLEN_R {
        POM_WRLVLEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Read data eye DQ training enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_rdlvlen(&self) -> POM_RDLVLEN_R {
        POM_RDLVLEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DRAM VREF-DQ training enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_vrefdqwren(&self) -> POM_VREFDQWREN_R {
        POM_VREFDQWREN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Delay evaluation enable. Delay evaluation process must be performed when there is at least one DDR chip with CK-DQS delay exceeding 1 clock cycle. If all DDR chips have CK-DQS delay smaller than 1 clock cycle, this process is not needed. This process can be enabled by writing 1'b1 to dlyevalen field and phyinit field in POM register. The completion of this process is identified by dlyevalc field in POS register. After evaluation is completed, user must write 1'b0 to dlyevalen field and phyinit. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_dlyevalen(&self) -> POM_DLYEVALEN_R {
        POM_DLYEVALEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write/read sanity check enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_sanchken(&self) -> POM_SANCHKEN_R {
        POM_SANCHKEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Target Frequency Set-point for PHY operations. Must be different from operating set point. FS0 = 0 Frequency Set Point 0 FS1 = 1 Frequency Set Point 1"]
    #[inline(always)]
    pub fn pom_fs(&self) -> POM_FS_R {
        POM_FS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MC-PHY clock phase lock request. When running in 1:4 mode user must write 1'b1 to clklocken field in POM register along with dllrsten and phyinit, this enables phase lock mechanism inside the PHY. User must clear this field when writing 1'b0 to phyinit field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_clklocken(&self) -> POM_CLKLOCKEN_R {
        POM_CLKLOCKEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Load PHY command bus delay To help PHY debugging, DTI PHY supports user to set timing delay for its memory control and address signals. This feature is also to meet CA/CS setup and hold time in slow frequency operation. To enable PHY command delay load, user must write 1'b1 to cmddlyen and phyinit field in POM register. The completion of this process is indicated via cmddlyc field in POS register. After loading process is completed, user must write 1'b0 to cmddlyen field. cmddlyen must be combined with physeten to reload trained setting since physeten does not set for command bus. In this case user must write PHY setting to PTSR* registers then write 1'b1 to cmddlyen, physeten and phyinit fields in POM register. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_cmddlyen(&self) -> POM_CMDDLYEN_R {
        POM_CMDDLYEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Exit from power-down indicator flag"]
    #[inline(always)]
    pub fn pom_pwdexit(&self) -> POM_PWDEXIT_R {
        POM_PWDEXIT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - On Die Termination (ODT) control flag.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_odt(&self) -> POM_ODT_R {
        POM_ODT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - tDQS2DQ retraining enable. For handling tDQS2DQ shifting.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn pom_dqsdqen(&self) -> POM_DQSDQEN_R {
        POM_DQSDQEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LPDDR4 channel enable for training. CHAN_NO = 2'b00 Enable no channel CHAN_0 = 2'b01 Enable channel 0 CHAN_1 = 2'b10 Enable channel 1 CHAN_BOTH = 2'b11 Enable channel 0 and channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pom_chanen(&mut self) -> POM_CHANEN_W<0> {
        POM_CHANEN_W::new(self)
    }
    #[doc = "Bit 2 - DFI interface enable. After training, MC must write 1'b1 to this field for normal DFI operation. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_dfien(&mut self) -> POM_DFIEN_W<2> {
        POM_DFIEN_W::new(self)
    }
    #[doc = "Bit 3 - PHY operation proceed. MC must write 1'b1 to this field when request from the PHY is satisfied. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_proc(&mut self) -> POM_PROC_W<3> {
        POM_PROC_W::new(self)
    }
    #[doc = "Bit 4 - PHY setting reload enable To reduce PHY start-up time, DTI PHY supports a process called 'PHY setting reload' to put the PHY to normal operation mode without performing any time-consumed training process. To enable PHY setting reload process, user must write 1'b1 to physeten field in POM register. The completion of this process is indicated through physetc field in POS register. After reload process is completed, user must write 1'b0 to physeten field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_physeten(&mut self) -> POM_PHYSETEN_W<4> {
        POM_PHYSETEN_W::new(self)
    }
    #[doc = "Bit 5 - PHY frequency change enable To switch Frequency Set-point, user must write 1'b1 to phyfsen and phyinit field in POM register with target Frequency Set-point in fs field. fs value must be different from operating Frequency Set-point. User can obtain operating Frequency Set-point by reading ofs field in POS register. The completion of this process is indicated through phyfsc field in the POS. After this process is completed, user must write 1'b0 to phyfsen and phyinit. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_phyfsen(&mut self) -> POM_PHYFSEN_W<5> {
        POM_PHYFSEN_W::new(self)
    }
    #[doc = "Bit 6 - PHY initialization enable. PHY initialization includes DLL init, DRAM init, and PHY training. Software can do each step separately by writing other bits in this register individually.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_phyinit(&mut self) -> POM_PHYINIT_W<6> {
        POM_PHYINIT_W::new(self)
    }
    #[doc = "Bit 7 - DLL reset enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_dllrsten(&mut self) -> POM_DLLRSTEN_W<7> {
        POM_DLLRSTEN_W::new(self)
    }
    #[doc = "Bit 8 - DRAM initialization enable The PHY built-in DRAM initialization is triggered using draminiten and phyinit fields in the POM - PHY Operation Mode Register. Its completion is indicated through bit draminitc in the POS - PHY Operation Status Register. The DRAM mode registers and timing registers must be configured correctly prior to triggering DRAM initialization, even if user chooses not to use the built-in initialization. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_draminiten(&mut self) -> POM_DRAMINITEN_W<8> {
        POM_DRAMINITEN_W::new(self)
    }
    #[doc = "Bit 9 - PHY VREF-DQ training enable. The PHY built-in VREF-DQ training is triggered using vrefdqrden and phyinit field in the POM. Its completion is indicated through bit vrefdqrdc in thePOS. The field vrefdqrderr in the PTS0 shows that the PHY cannot find correct VREF setting for its internal receiver after training. The error is asserted if the PHY doesn't assert VT_DONE. The PHY looks at the voltage high level during VREF-DQ training. If (voltage_high/2) is beyond the VREF generation specification then VT_DONE won't assert. After PHY VREF-DQ training is completed, user must write 1'b0 to verfdqrden and phyinit field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_vrefdqrden(&mut self) -> POM_VREFDQRDEN_W<9> {
        POM_VREFDQRDEN_W::new(self)
    }
    #[doc = "Bit 10 - DRAM VREF-CA training enable. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_vrefcaen(&mut self) -> POM_VREFCAEN_W<10> {
        POM_VREFCAEN_W::new(self)
    }
    #[doc = "Bit 11 - Gate training enable. The PHY built-in gate training is triggered using POM_GTEN and POM_PHYINIT field in thePOM. Its completion is indicated through bit GTC in the POS. The field GTERR in the PTS0 shows gate training error for each Data Module in the PHY while training. After DQS gate training is completed, user must write 1'b0 to POM_GTE and POM_PHYINIT. The error is asserted when the PHY doesn't provide HIGH on GT_STATUS. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_gten(&mut self) -> POM_GTEN_W<11> {
        POM_GTEN_W::new(self)
    }
    #[doc = "Bit 12 - Write leveling enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_wrlvlen(&mut self) -> POM_WRLVLEN_W<12> {
        POM_WRLVLEN_W::new(self)
    }
    #[doc = "Bit 13 - Read data eye DQ training enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_rdlvlen(&mut self) -> POM_RDLVLEN_W<13> {
        POM_RDLVLEN_W::new(self)
    }
    #[doc = "Bit 14 - DRAM VREF-DQ training enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_vrefdqwren(&mut self) -> POM_VREFDQWREN_W<14> {
        POM_VREFDQWREN_W::new(self)
    }
    #[doc = "Bit 15 - Delay evaluation enable. Delay evaluation process must be performed when there is at least one DDR chip with CK-DQS delay exceeding 1 clock cycle. If all DDR chips have CK-DQS delay smaller than 1 clock cycle, this process is not needed. This process can be enabled by writing 1'b1 to dlyevalen field and phyinit field in POM register. The completion of this process is identified by dlyevalc field in POS register. After evaluation is completed, user must write 1'b0 to dlyevalen field and phyinit. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_dlyevalen(&mut self) -> POM_DLYEVALEN_W<15> {
        POM_DLYEVALEN_W::new(self)
    }
    #[doc = "Bit 16 - Write/read sanity check enable.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_sanchken(&mut self) -> POM_SANCHKEN_W<16> {
        POM_SANCHKEN_W::new(self)
    }
    #[doc = "Bit 17 - Target Frequency Set-point for PHY operations. Must be different from operating set point. FS0 = 0 Frequency Set Point 0 FS1 = 1 Frequency Set Point 1"]
    #[inline(always)]
    #[must_use]
    pub fn pom_fs(&mut self) -> POM_FS_W<17> {
        POM_FS_W::new(self)
    }
    #[doc = "Bit 18 - MC-PHY clock phase lock request. When running in 1:4 mode user must write 1'b1 to clklocken field in POM register along with dllrsten and phyinit, this enables phase lock mechanism inside the PHY. User must clear this field when writing 1'b0 to phyinit field. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_clklocken(&mut self) -> POM_CLKLOCKEN_W<18> {
        POM_CLKLOCKEN_W::new(self)
    }
    #[doc = "Bit 19 - Load PHY command bus delay To help PHY debugging, DTI PHY supports user to set timing delay for its memory control and address signals. This feature is also to meet CA/CS setup and hold time in slow frequency operation. To enable PHY command delay load, user must write 1'b1 to cmddlyen and phyinit field in POM register. The completion of this process is indicated via cmddlyc field in POS register. After loading process is completed, user must write 1'b0 to cmddlyen field. cmddlyen must be combined with physeten to reload trained setting since physeten does not set for command bus. In this case user must write PHY setting to PTSR* registers then write 1'b1 to cmddlyen, physeten and phyinit fields in POM register. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_cmddlyen(&mut self) -> POM_CMDDLYEN_W<19> {
        POM_CMDDLYEN_W::new(self)
    }
    #[doc = "Bit 20 - Exit from power-down indicator flag"]
    #[inline(always)]
    #[must_use]
    pub fn pom_pwdexit(&mut self) -> POM_PWDEXIT_W<20> {
        POM_PWDEXIT_W::new(self)
    }
    #[doc = "Bit 21 - On Die Termination (ODT) control flag.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_odt(&mut self) -> POM_ODT_W<21> {
        POM_ODT_W::new(self)
    }
    #[doc = "Bit 22 - tDQS2DQ retraining enable. For handling tDQS2DQ shifting.Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn pom_dqsdqen(&mut self) -> POM_DQSDQEN_W<22> {
        POM_DQSDQEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Operation Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pom](index.html) module"]
pub struct POM_SPEC;
impl crate::RegisterSpec for POM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pom::R](R) reader structure"]
impl crate::Readable for POM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pom::W](W) writer structure"]
impl crate::Writable for POM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POM to value 0"]
impl crate::Resettable for POM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
