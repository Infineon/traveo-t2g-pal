#[doc = "Register `PWR_REGHC_CTL` reader"]
pub struct R(crate::R<PWR_REGHC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_REGHC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_REGHC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_REGHC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_REGHC_CTL` writer"]
pub struct W(crate::W<PWR_REGHC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_REGHC_CTL_SPEC>;
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
impl From<crate::W<PWR_REGHC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_REGHC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGHC_MODE` reader - REGHC control mode: 0: external transistor connected, 1: external PMIC connected"]
pub type REGHC_MODE_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_MODE` writer - REGHC control mode: 0: external transistor connected, 1: external PMIC connected"]
pub type REGHC_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
#[doc = "Field `REGHC_PMIC_DRV_VOUT` reader - Setting for DRV_VOUT pin for PMIC mode. See REGHC_VADJ for calculation of vadj. 2'b00: DRV_VOUT=vccd*0.9/vadj; 2'b01: DRV_VOUT=vccd*0.8/vadj; 2'b10: DRV_VOUT=vccd*0.6/vadj; 2'b11: DRV_VOUT=vccd"]
pub type REGHC_PMIC_DRV_VOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGHC_PMIC_DRV_VOUT` writer - Setting for DRV_VOUT pin for PMIC mode. See REGHC_VADJ for calculation of vadj. 2'b00: DRV_VOUT=vccd*0.9/vadj; 2'b01: DRV_VOUT=vccd*0.8/vadj; 2'b10: DRV_VOUT=vccd*0.6/vadj; 2'b11: DRV_VOUT=vccd"]
pub type REGHC_PMIC_DRV_VOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_REGHC_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `REGHC_VADJ` reader - Regulator output trim according to the formula vadj=(1.020V + REGHC_VADJ*0.005V) plus an offset described below. For example, 0x1A=>1.15V (nominal). For transistor mode, REGHC will dynamically adjust DRV_VOUT so the supply targets the vadj voltage. Adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.REGHC_TRANS_VADJ_OFFSET. For PMIC mode, adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.PMIC_VADJ_OFFSET to the nominal calculated code. For PMIC mode, also see REGHC_PMIC_DRV_VOUT."]
pub type REGHC_VADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGHC_VADJ` writer - Regulator output trim according to the formula vadj=(1.020V + REGHC_VADJ*0.005V) plus an offset described below. For example, 0x1A=>1.15V (nominal). For transistor mode, REGHC will dynamically adjust DRV_VOUT so the supply targets the vadj voltage. Adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.REGHC_TRANS_VADJ_OFFSET. For PMIC mode, adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.PMIC_VADJ_OFFSET to the nominal calculated code. For PMIC mode, also see REGHC_PMIC_DRV_VOUT."]
pub type REGHC_VADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_REGHC_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `REGHC_PMIC_USE_LINREG` reader - For REGHC external PMIC mode, controls whether hardware sequencer keeps the internal Active Linear Regulator enabled to improve supply supervision of vccd. When using this feature, if the PMIC fails to keep vccd above the internal regulator target, then the internal regulator will attempt to recover vccd. If the regulator current is too high, the regulator triggers an over-current detector (OCD) reset. 0: Internal Active Linear Regulator disabled after PMIC enabled. OCD is disabled.; 1: Internal Active Linear Regulator kept enabled. See datasheet for minimum PMIC vccd input to prevent OCD."]
pub type REGHC_PMIC_USE_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_USE_LINREG` writer - For REGHC external PMIC mode, controls whether hardware sequencer keeps the internal Active Linear Regulator enabled to improve supply supervision of vccd. When using this feature, if the PMIC fails to keep vccd above the internal regulator target, then the internal regulator will attempt to recover vccd. If the regulator current is too high, the regulator triggers an over-current detector (OCD) reset. 0: Internal Active Linear Regulator disabled after PMIC enabled. OCD is disabled.; 1: Internal Active Linear Regulator kept enabled. See datasheet for minimum PMIC vccd input to prevent OCD."]
pub type REGHC_PMIC_USE_LINREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
#[doc = "Field `REGHC_PMIC_USE_RADJ` reader - Controls whether hardware sequencer enables reset voltage adjustment circuit when enabling a PMIC."]
pub type REGHC_PMIC_USE_RADJ_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_USE_RADJ` writer - Controls whether hardware sequencer enables reset voltage adjustment circuit when enabling a PMIC."]
pub type REGHC_PMIC_USE_RADJ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
#[doc = "Field `REGHC_PMIC_RADJ` reader - Reset voltage adjustment for PMIC as a factor (Vfbk/Vref) where Vfbk is the feedback voltage and Vref is the PMIC internal reference. The reset voltage adjustment circuit is enabled by the hardware sequencer if REGHC_PMIC_USE_RADJ=1. PMIC have Vref of 0.8V or 0.9V, and the resulting reset voltage (Vreset) are precalculated in the table below: 3'b000: Vfbk/Vref=1.0000, Vreset=.800V@(Vref=0.8V), .900V@(Vref=0.9V); 3'b001: Vfbk/Vref=1.0556, Vreset=.844V@(Vref=0.8V), .950V@(Vref=0.9V); 3'b010: Vfbk/Vref=1.1111, Vreset=.889V@(Vref=0.8V), 1.000V@(Vref=0.9V); 3'b011: Vfbk/Vref=1.1250, Vreset=.900V@(Vref=0.8V), 1.013V@(Vref=0.9V); 3'b100: Vfbk/Vref=1.1667, Vreset=.933V@(Vref=0.8V), 1.050V@(Vref=0.9V); 3'b101: Vfbk/Vref=1.1875, Vreset=.950V@(Vref=0.8V), 1.069V@(Vref=0.9V); 3'b110: Vfbk/Vref=1.2500, Vreset=1.000V@(Vref=0.8V), 1.125V@(Vref=0.9V); 3'b111: Vfbk/Vref=1.3125, Vreset=1.050V@(Vref=0.8V), 1.181V@(Vref=0.9V);"]
pub type REGHC_PMIC_RADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGHC_PMIC_RADJ` writer - Reset voltage adjustment for PMIC as a factor (Vfbk/Vref) where Vfbk is the feedback voltage and Vref is the PMIC internal reference. The reset voltage adjustment circuit is enabled by the hardware sequencer if REGHC_PMIC_USE_RADJ=1. PMIC have Vref of 0.8V or 0.9V, and the resulting reset voltage (Vreset) are precalculated in the table below: 3'b000: Vfbk/Vref=1.0000, Vreset=.800V@(Vref=0.8V), .900V@(Vref=0.9V); 3'b001: Vfbk/Vref=1.0556, Vreset=.844V@(Vref=0.8V), .950V@(Vref=0.9V); 3'b010: Vfbk/Vref=1.1111, Vreset=.889V@(Vref=0.8V), 1.000V@(Vref=0.9V); 3'b011: Vfbk/Vref=1.1250, Vreset=.900V@(Vref=0.8V), 1.013V@(Vref=0.9V); 3'b100: Vfbk/Vref=1.1667, Vreset=.933V@(Vref=0.8V), 1.050V@(Vref=0.9V); 3'b101: Vfbk/Vref=1.1875, Vreset=.950V@(Vref=0.8V), 1.069V@(Vref=0.9V); 3'b110: Vfbk/Vref=1.2500, Vreset=1.000V@(Vref=0.8V), 1.125V@(Vref=0.9V); 3'b111: Vfbk/Vref=1.3125, Vreset=1.050V@(Vref=0.8V), 1.181V@(Vref=0.9V);"]
pub type REGHC_PMIC_RADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_REGHC_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `REGHC_PMIC_CTL_OUTEN` reader - Output enable for PMIC enable pin. Set this bit high to enable the driver on this pin."]
pub type REGHC_PMIC_CTL_OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_CTL_OUTEN` writer - Output enable for PMIC enable pin. Set this bit high to enable the driver on this pin."]
pub type REGHC_PMIC_CTL_OUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
#[doc = "Field `REGHC_PMIC_CTL_POLARITY` reader - Polarity used to enable the PMIC. The sequencer uses REGHC_PMIC_CTL_POLARITY to enable the PMIC, and it uses the complement to disable the PMIC."]
pub type REGHC_PMIC_CTL_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_CTL_POLARITY` writer - Polarity used to enable the PMIC. The sequencer uses REGHC_PMIC_CTL_POLARITY to enable the PMIC, and it uses the complement to disable the PMIC."]
pub type REGHC_PMIC_CTL_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
#[doc = "Field `REGHC_PMIC_STATUS_INEN` reader - Input buffer enable for PMIC status input. Set this bit high to enable the input receiver."]
pub type REGHC_PMIC_STATUS_INEN_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_STATUS_INEN` writer - Input buffer enable for PMIC status input. Set this bit high to enable the input receiver."]
pub type REGHC_PMIC_STATUS_INEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
#[doc = "Field `REGHC_PMIC_STATUS_POLARITY` reader - The polarity used to trigger a reset action based on the PMIC status input. The reset system triggers a reset when the unmasked PMIC status matches this value."]
pub type REGHC_PMIC_STATUS_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_STATUS_POLARITY` writer - The polarity used to trigger a reset action based on the PMIC status input. The reset system triggers a reset when the unmasked PMIC status matches this value."]
pub type REGHC_PMIC_STATUS_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
#[doc = "Field `REGHC_PMIC_STATUS_WAIT` reader - Wait count in 4us steps after PMIC status ok. This is used by the hardware sequencer to allow additional settling time before disabling the internal regulator. The LSB is 32 IMO periods which results in a nominal LSB step of 4us."]
pub type REGHC_PMIC_STATUS_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REGHC_PMIC_STATUS_WAIT` writer - Wait count in 4us steps after PMIC status ok. This is used by the hardware sequencer to allow additional settling time before disabling the internal regulator. The LSB is 32 IMO periods which results in a nominal LSB step of 4us."]
pub type REGHC_PMIC_STATUS_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_REGHC_CTL_SPEC, u16, u16, 10, O>;
#[doc = "Field `REGHC_TRANS_USE_OCD` reader - N/A"]
pub type REGHC_TRANS_USE_OCD_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_TRANS_USE_OCD` writer - N/A"]
pub type REGHC_TRANS_USE_OCD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
#[doc = "Field `REGHC_CONFIGURED` reader - Indicates the REGHC has been configured. This is used to know if REGHC should be enabled in response to a debug power up request. It is recommended to not change REGHC settings after REGHC_CONFIGURED is set high, because it can cause failure if REGHC is enabled or transitioning."]
pub type REGHC_CONFIGURED_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_CONFIGURED` writer - Indicates the REGHC has been configured. This is used to know if REGHC should be enabled in response to a debug power up request. It is recommended to not change REGHC settings after REGHC_CONFIGURED is set high, because it can cause failure if REGHC is enabled or transitioning."]
pub type REGHC_CONFIGURED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - REGHC control mode: 0: external transistor connected, 1: external PMIC connected"]
    #[inline(always)]
    pub fn reghc_mode(&self) -> REGHC_MODE_R {
        REGHC_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Setting for DRV_VOUT pin for PMIC mode. See REGHC_VADJ for calculation of vadj. 2'b00: DRV_VOUT=vccd*0.9/vadj; 2'b01: DRV_VOUT=vccd*0.8/vadj; 2'b10: DRV_VOUT=vccd*0.6/vadj; 2'b11: DRV_VOUT=vccd"]
    #[inline(always)]
    pub fn reghc_pmic_drv_vout(&self) -> REGHC_PMIC_DRV_VOUT_R {
        REGHC_PMIC_DRV_VOUT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:8 - Regulator output trim according to the formula vadj=(1.020V + REGHC_VADJ*0.005V) plus an offset described below. For example, 0x1A=>1.15V (nominal). For transistor mode, REGHC will dynamically adjust DRV_VOUT so the supply targets the vadj voltage. Adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.REGHC_TRANS_VADJ_OFFSET. For PMIC mode, adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.PMIC_VADJ_OFFSET to the nominal calculated code. For PMIC mode, also see REGHC_PMIC_DRV_VOUT."]
    #[inline(always)]
    pub fn reghc_vadj(&self) -> REGHC_VADJ_R {
        REGHC_VADJ_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - For REGHC external PMIC mode, controls whether hardware sequencer keeps the internal Active Linear Regulator enabled to improve supply supervision of vccd. When using this feature, if the PMIC fails to keep vccd above the internal regulator target, then the internal regulator will attempt to recover vccd. If the regulator current is too high, the regulator triggers an over-current detector (OCD) reset. 0: Internal Active Linear Regulator disabled after PMIC enabled. OCD is disabled.; 1: Internal Active Linear Regulator kept enabled. See datasheet for minimum PMIC vccd input to prevent OCD."]
    #[inline(always)]
    pub fn reghc_pmic_use_linreg(&self) -> REGHC_PMIC_USE_LINREG_R {
        REGHC_PMIC_USE_LINREG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls whether hardware sequencer enables reset voltage adjustment circuit when enabling a PMIC."]
    #[inline(always)]
    pub fn reghc_pmic_use_radj(&self) -> REGHC_PMIC_USE_RADJ_R {
        REGHC_PMIC_USE_RADJ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Reset voltage adjustment for PMIC as a factor (Vfbk/Vref) where Vfbk is the feedback voltage and Vref is the PMIC internal reference. The reset voltage adjustment circuit is enabled by the hardware sequencer if REGHC_PMIC_USE_RADJ=1. PMIC have Vref of 0.8V or 0.9V, and the resulting reset voltage (Vreset) are precalculated in the table below: 3'b000: Vfbk/Vref=1.0000, Vreset=.800V@(Vref=0.8V), .900V@(Vref=0.9V); 3'b001: Vfbk/Vref=1.0556, Vreset=.844V@(Vref=0.8V), .950V@(Vref=0.9V); 3'b010: Vfbk/Vref=1.1111, Vreset=.889V@(Vref=0.8V), 1.000V@(Vref=0.9V); 3'b011: Vfbk/Vref=1.1250, Vreset=.900V@(Vref=0.8V), 1.013V@(Vref=0.9V); 3'b100: Vfbk/Vref=1.1667, Vreset=.933V@(Vref=0.8V), 1.050V@(Vref=0.9V); 3'b101: Vfbk/Vref=1.1875, Vreset=.950V@(Vref=0.8V), 1.069V@(Vref=0.9V); 3'b110: Vfbk/Vref=1.2500, Vreset=1.000V@(Vref=0.8V), 1.125V@(Vref=0.9V); 3'b111: Vfbk/Vref=1.3125, Vreset=1.050V@(Vref=0.8V), 1.181V@(Vref=0.9V);"]
    #[inline(always)]
    pub fn reghc_pmic_radj(&self) -> REGHC_PMIC_RADJ_R {
        REGHC_PMIC_RADJ_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Output enable for PMIC enable pin. Set this bit high to enable the driver on this pin."]
    #[inline(always)]
    pub fn reghc_pmic_ctl_outen(&self) -> REGHC_PMIC_CTL_OUTEN_R {
        REGHC_PMIC_CTL_OUTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Polarity used to enable the PMIC. The sequencer uses REGHC_PMIC_CTL_POLARITY to enable the PMIC, and it uses the complement to disable the PMIC."]
    #[inline(always)]
    pub fn reghc_pmic_ctl_polarity(&self) -> REGHC_PMIC_CTL_POLARITY_R {
        REGHC_PMIC_CTL_POLARITY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input buffer enable for PMIC status input. Set this bit high to enable the input receiver."]
    #[inline(always)]
    pub fn reghc_pmic_status_inen(&self) -> REGHC_PMIC_STATUS_INEN_R {
        REGHC_PMIC_STATUS_INEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The polarity used to trigger a reset action based on the PMIC status input. The reset system triggers a reset when the unmasked PMIC status matches this value."]
    #[inline(always)]
    pub fn reghc_pmic_status_polarity(&self) -> REGHC_PMIC_STATUS_POLARITY_R {
        REGHC_PMIC_STATUS_POLARITY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29 - Wait count in 4us steps after PMIC status ok. This is used by the hardware sequencer to allow additional settling time before disabling the internal regulator. The LSB is 32 IMO periods which results in a nominal LSB step of 4us."]
    #[inline(always)]
    pub fn reghc_pmic_status_wait(&self) -> REGHC_PMIC_STATUS_WAIT_R {
        REGHC_PMIC_STATUS_WAIT_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn reghc_trans_use_ocd(&self) -> REGHC_TRANS_USE_OCD_R {
        REGHC_TRANS_USE_OCD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates the REGHC has been configured. This is used to know if REGHC should be enabled in response to a debug power up request. It is recommended to not change REGHC settings after REGHC_CONFIGURED is set high, because it can cause failure if REGHC is enabled or transitioning."]
    #[inline(always)]
    pub fn reghc_configured(&self) -> REGHC_CONFIGURED_R {
        REGHC_CONFIGURED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REGHC control mode: 0: external transistor connected, 1: external PMIC connected"]
    #[inline(always)]
    #[must_use]
    pub fn reghc_mode(&mut self) -> REGHC_MODE_W<0> {
        REGHC_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Setting for DRV_VOUT pin for PMIC mode. See REGHC_VADJ for calculation of vadj. 2'b00: DRV_VOUT=vccd*0.9/vadj; 2'b01: DRV_VOUT=vccd*0.8/vadj; 2'b10: DRV_VOUT=vccd*0.6/vadj; 2'b11: DRV_VOUT=vccd"]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_drv_vout(&mut self) -> REGHC_PMIC_DRV_VOUT_W<2> {
        REGHC_PMIC_DRV_VOUT_W::new(self)
    }
    #[doc = "Bits 4:8 - Regulator output trim according to the formula vadj=(1.020V + REGHC_VADJ*0.005V) plus an offset described below. For example, 0x1A=>1.15V (nominal). For transistor mode, REGHC will dynamically adjust DRV_VOUT so the supply targets the vadj voltage. Adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.REGHC_TRANS_VADJ_OFFSET. For PMIC mode, adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.PMIC_VADJ_OFFSET to the nominal calculated code. For PMIC mode, also see REGHC_PMIC_DRV_VOUT."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_vadj(&mut self) -> REGHC_VADJ_W<4> {
        REGHC_VADJ_W::new(self)
    }
    #[doc = "Bit 10 - For REGHC external PMIC mode, controls whether hardware sequencer keeps the internal Active Linear Regulator enabled to improve supply supervision of vccd. When using this feature, if the PMIC fails to keep vccd above the internal regulator target, then the internal regulator will attempt to recover vccd. If the regulator current is too high, the regulator triggers an over-current detector (OCD) reset. 0: Internal Active Linear Regulator disabled after PMIC enabled. OCD is disabled.; 1: Internal Active Linear Regulator kept enabled. See datasheet for minimum PMIC vccd input to prevent OCD."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_use_linreg(&mut self) -> REGHC_PMIC_USE_LINREG_W<10> {
        REGHC_PMIC_USE_LINREG_W::new(self)
    }
    #[doc = "Bit 11 - Controls whether hardware sequencer enables reset voltage adjustment circuit when enabling a PMIC."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_use_radj(&mut self) -> REGHC_PMIC_USE_RADJ_W<11> {
        REGHC_PMIC_USE_RADJ_W::new(self)
    }
    #[doc = "Bits 12:14 - Reset voltage adjustment for PMIC as a factor (Vfbk/Vref) where Vfbk is the feedback voltage and Vref is the PMIC internal reference. The reset voltage adjustment circuit is enabled by the hardware sequencer if REGHC_PMIC_USE_RADJ=1. PMIC have Vref of 0.8V or 0.9V, and the resulting reset voltage (Vreset) are precalculated in the table below: 3'b000: Vfbk/Vref=1.0000, Vreset=.800V@(Vref=0.8V), .900V@(Vref=0.9V); 3'b001: Vfbk/Vref=1.0556, Vreset=.844V@(Vref=0.8V), .950V@(Vref=0.9V); 3'b010: Vfbk/Vref=1.1111, Vreset=.889V@(Vref=0.8V), 1.000V@(Vref=0.9V); 3'b011: Vfbk/Vref=1.1250, Vreset=.900V@(Vref=0.8V), 1.013V@(Vref=0.9V); 3'b100: Vfbk/Vref=1.1667, Vreset=.933V@(Vref=0.8V), 1.050V@(Vref=0.9V); 3'b101: Vfbk/Vref=1.1875, Vreset=.950V@(Vref=0.8V), 1.069V@(Vref=0.9V); 3'b110: Vfbk/Vref=1.2500, Vreset=1.000V@(Vref=0.8V), 1.125V@(Vref=0.9V); 3'b111: Vfbk/Vref=1.3125, Vreset=1.050V@(Vref=0.8V), 1.181V@(Vref=0.9V);"]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_radj(&mut self) -> REGHC_PMIC_RADJ_W<12> {
        REGHC_PMIC_RADJ_W::new(self)
    }
    #[doc = "Bit 16 - Output enable for PMIC enable pin. Set this bit high to enable the driver on this pin."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_ctl_outen(&mut self) -> REGHC_PMIC_CTL_OUTEN_W<16> {
        REGHC_PMIC_CTL_OUTEN_W::new(self)
    }
    #[doc = "Bit 17 - Polarity used to enable the PMIC. The sequencer uses REGHC_PMIC_CTL_POLARITY to enable the PMIC, and it uses the complement to disable the PMIC."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_ctl_polarity(&mut self) -> REGHC_PMIC_CTL_POLARITY_W<17> {
        REGHC_PMIC_CTL_POLARITY_W::new(self)
    }
    #[doc = "Bit 18 - Input buffer enable for PMIC status input. Set this bit high to enable the input receiver."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_status_inen(&mut self) -> REGHC_PMIC_STATUS_INEN_W<18> {
        REGHC_PMIC_STATUS_INEN_W::new(self)
    }
    #[doc = "Bit 19 - The polarity used to trigger a reset action based on the PMIC status input. The reset system triggers a reset when the unmasked PMIC status matches this value."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_status_polarity(&mut self) -> REGHC_PMIC_STATUS_POLARITY_W<19> {
        REGHC_PMIC_STATUS_POLARITY_W::new(self)
    }
    #[doc = "Bits 20:29 - Wait count in 4us steps after PMIC status ok. This is used by the hardware sequencer to allow additional settling time before disabling the internal regulator. The LSB is 32 IMO periods which results in a nominal LSB step of 4us."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_status_wait(&mut self) -> REGHC_PMIC_STATUS_WAIT_W<20> {
        REGHC_PMIC_STATUS_WAIT_W::new(self)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn reghc_trans_use_ocd(&mut self) -> REGHC_TRANS_USE_OCD_W<30> {
        REGHC_TRANS_USE_OCD_W::new(self)
    }
    #[doc = "Bit 31 - Indicates the REGHC has been configured. This is used to know if REGHC should be enabled in response to a debug power up request. It is recommended to not change REGHC settings after REGHC_CONFIGURED is set high, because it can cause failure if REGHC is enabled or transitioning."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_configured(&mut self) -> REGHC_CONFIGURED_W<31> {
        REGHC_CONFIGURED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "REGHC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_reghc_ctl](index.html) module"]
pub struct PWR_REGHC_CTL_SPEC;
impl crate::RegisterSpec for PWR_REGHC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_reghc_ctl::R](R) reader structure"]
impl crate::Readable for PWR_REGHC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_reghc_ctl::W](W) writer structure"]
impl crate::Writable for PWR_REGHC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_REGHC_CTL to value 0x4000_0104"]
impl crate::Resettable for PWR_REGHC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0104;
}
