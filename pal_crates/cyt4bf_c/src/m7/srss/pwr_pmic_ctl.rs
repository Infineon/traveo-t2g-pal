#[doc = "Register `PWR_PMIC_CTL` reader"]
pub struct R(crate::R<PWR_PMIC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_PMIC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_PMIC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_PMIC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_PMIC_CTL` writer"]
pub struct W(crate::W<PWR_PMIC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_PMIC_CTL_SPEC>;
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
impl From<crate::W<PWR_PMIC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_PMIC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMIC_VREF` reader - PMIC reference voltage setting. This selects the scaling factor used to generate the output voltage (vout) given the feedback voltage (vfb) for the chosen PMIC. For a PMIC that compares vfb to an internal reference voltage (vref) according to the formula vout=vref/vfb, select that vref below. For a PMIC that contains an internal resistor divider and expects an unscaled feedback voltage, use the 'No scaling' choice. 2'b00: Scale for vref=0.9V, use PMIC_VADJ to set the vccd target; 2'b01: Scale for vref=0.8V, use PMIC_VADJ to set the vccd target; 2'b10: Scale for vref=0.6V, use PMIC_VADJ to set the vccd target; 2'b11: No scaling, PMIC_VADJ has no effect"]
pub type PMIC_VREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMIC_VREF` writer - PMIC reference voltage setting. This selects the scaling factor used to generate the output voltage (vout) given the feedback voltage (vfb) for the chosen PMIC. For a PMIC that compares vfb to an internal reference voltage (vref) according to the formula vout=vref/vfb, select that vref below. For a PMIC that contains an internal resistor divider and expects an unscaled feedback voltage, use the 'No scaling' choice. 2'b00: Scale for vref=0.9V, use PMIC_VADJ to set the vccd target; 2'b01: Scale for vref=0.8V, use PMIC_VADJ to set the vccd target; 2'b10: Scale for vref=0.6V, use PMIC_VADJ to set the vccd target; 2'b11: No scaling, PMIC_VADJ has no effect"]
pub type PMIC_VREF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_PMIC_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PMIC_VADJ` reader - Voltage adjustment output setting. The lookup table in this field requires the proper setting in PMIC_VREF for the chosen PMIC. This field has no effect when PMIC_VREF selects no scaling. The feedback tap point is at a vccd pad inside the chip, so the voltage may be a little higher at the PMIC output. Adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.PMIC_VADJ_OFFSET to desired code in this lookup table: 0x03: 1.040V, 0x04: 1.049V, 0x05: 1.057V, 0x06: 1.066V, 0x07: 1.074V, 0x08: 1.083V, 0x09: 1.091V, 0x0A: 1.099V, 0x0B: 1.108V, 0x0C: 1.116V, 0x0D: 1.125V, 0x0E: 1.133V, 0x0F: 1.142V, 0x10: 1.150V, 0x11: 1.158V, 0x12: 1.167V, 0x13: 1.175V, 0x14: 1.184V, 0x15: 1.192V, 0x16: 1.201V, 0x17: 1.209V, 0x18: 1.218V, 0x19: 1.226V, 0x1A: 1.234V, 0x1B: 1.243V, 0x1C: 1.251V, others: Illegal. Behavior is undefined."]
pub type PMIC_VADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMIC_VADJ` writer - Voltage adjustment output setting. The lookup table in this field requires the proper setting in PMIC_VREF for the chosen PMIC. This field has no effect when PMIC_VREF selects no scaling. The feedback tap point is at a vccd pad inside the chip, so the voltage may be a little higher at the PMIC output. Adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.PMIC_VADJ_OFFSET to desired code in this lookup table: 0x03: 1.040V, 0x04: 1.049V, 0x05: 1.057V, 0x06: 1.066V, 0x07: 1.074V, 0x08: 1.083V, 0x09: 1.091V, 0x0A: 1.099V, 0x0B: 1.108V, 0x0C: 1.116V, 0x0D: 1.125V, 0x0E: 1.133V, 0x0F: 1.142V, 0x10: 1.150V, 0x11: 1.158V, 0x12: 1.167V, 0x13: 1.175V, 0x14: 1.184V, 0x15: 1.192V, 0x16: 1.201V, 0x17: 1.209V, 0x18: 1.218V, 0x19: 1.226V, 0x1A: 1.234V, 0x1B: 1.243V, 0x1C: 1.251V, others: Illegal. Behavior is undefined."]
pub type PMIC_VADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_PMIC_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PMIC_USE_LINREG` reader - Controls whether hardware sequencer keeps the internal Active Linear Regulator enabled to improve supply supervision of vccd. When using this feature, if the PMIC fails to keep vccd above the internal regulator target, then the internal regulator will attempt to recover vccd. If the regulator current is too high, the regulator triggers an over-current detector (OCD) reset. 0: Internal Active Linear Regulator disabled after PMIC enabled. OCD is disabled.; 1: Internal Active Linear Regulator kept enabled. See datasheet for minimum PMIC vccd input to prevent OCD."]
pub type PMIC_USE_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_USE_LINREG` writer - Controls whether hardware sequencer keeps the internal Active Linear Regulator enabled to improve supply supervision of vccd. When using this feature, if the PMIC fails to keep vccd above the internal regulator target, then the internal regulator will attempt to recover vccd. If the regulator current is too high, the regulator triggers an over-current detector (OCD) reset. 0: Internal Active Linear Regulator disabled after PMIC enabled. OCD is disabled.; 1: Internal Active Linear Regulator kept enabled. See datasheet for minimum PMIC vccd input to prevent OCD."]
pub type PMIC_USE_LINREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_VADJ_BUF_EN` reader - Analog buffer enable on voltage adjust output. Write this bit depending on the type of PMIC connected: 0: Bypass buffer. This connects the resistor divider directly to the output pin. Use this setting for a PMIC with a high-impedance feedback input, such as those that support a resistor divider on the PCB. This setting can also be used with a low-impedance PMIC with PMIC_VREF=2'b11 (no scaling). 1: Use analog buffer. This enables an analog buffer between the resistor divider output and the pin. The buffer can drive a resistor divider on the PCB that feeds into the PMIC feedback input. This allows targeting a different PMIC reference voltage from PMIC_VREF choices, while still supporting voltage adjustment using the internal divider."]
pub type PMIC_VADJ_BUF_EN_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_VADJ_BUF_EN` writer - Analog buffer enable on voltage adjust output. Write this bit depending on the type of PMIC connected: 0: Bypass buffer. This connects the resistor divider directly to the output pin. Use this setting for a PMIC with a high-impedance feedback input, such as those that support a resistor divider on the PCB. This setting can also be used with a low-impedance PMIC with PMIC_VREF=2'b11 (no scaling). 1: Use analog buffer. This enables an analog buffer between the resistor divider output and the pin. The buffer can drive a resistor divider on the PCB that feeds into the PMIC feedback input. This allows targeting a different PMIC reference voltage from PMIC_VREF choices, while still supporting voltage adjustment using the internal divider."]
pub type PMIC_VADJ_BUF_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_CTL_OUTEN` reader - Output enable for PMIC enable pin. Set this bit high to enable the driver on this pin."]
pub type PMIC_CTL_OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_CTL_OUTEN` writer - Output enable for PMIC enable pin. Set this bit high to enable the driver on this pin."]
pub type PMIC_CTL_OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_CTL_POLARITY` reader - Polarity used to enable the PMIC. The sequencer uses PMIC_CTL_POLARITY to enable the PMIC, and it uses the complement to disable the PMIC."]
pub type PMIC_CTL_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_CTL_POLARITY` writer - Polarity used to enable the PMIC. The sequencer uses PMIC_CTL_POLARITY to enable the PMIC, and it uses the complement to disable the PMIC."]
pub type PMIC_CTL_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_STATUS_INEN` reader - Input buffer enable for PMIC status input. Set this bit high to enable the input receiver."]
pub type PMIC_STATUS_INEN_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_STATUS_INEN` writer - Input buffer enable for PMIC status input. Set this bit high to enable the input receiver."]
pub type PMIC_STATUS_INEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_STATUS_POLARITY` reader - The polarity used to trigger a reset action based on the PMIC status input. The reset system triggers a reset when the unmasked PMIC status matches this value."]
pub type PMIC_STATUS_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_STATUS_POLARITY` writer - The polarity used to trigger a reset action based on the PMIC status input. The reset system triggers a reset when the unmasked PMIC status matches this value."]
pub type PMIC_STATUS_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_STATUS_WAIT` reader - Wait count in 4us steps after PMIC status ok. This is used by the hardware sequencer to allow additional settling time before disabling the internal regulator. The LSB is 32 IMO periods which results in a nominal LSB step of 4us."]
pub type PMIC_STATUS_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PMIC_STATUS_WAIT` writer - Wait count in 4us steps after PMIC status ok. This is used by the hardware sequencer to allow additional settling time before disabling the internal regulator. The LSB is 32 IMO periods which results in a nominal LSB step of 4us."]
pub type PMIC_STATUS_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_PMIC_CTL_SPEC, u16, u16, 10, O>;
#[doc = "Field `PMIC_CONFIGURED` reader - Indicates the PMIC has been configured. This is used to know if PMIC should be enabled in response to a debug power up request. Do not change PMIC settings after this bit is set high."]
pub type PMIC_CONFIGURED_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_CONFIGURED` writer - Indicates the PMIC has been configured. This is used to know if PMIC should be enabled in response to a debug power up request. Do not change PMIC settings after this bit is set high."]
pub type PMIC_CONFIGURED_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_PMIC_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:3 - PMIC reference voltage setting. This selects the scaling factor used to generate the output voltage (vout) given the feedback voltage (vfb) for the chosen PMIC. For a PMIC that compares vfb to an internal reference voltage (vref) according to the formula vout=vref/vfb, select that vref below. For a PMIC that contains an internal resistor divider and expects an unscaled feedback voltage, use the 'No scaling' choice. 2'b00: Scale for vref=0.9V, use PMIC_VADJ to set the vccd target; 2'b01: Scale for vref=0.8V, use PMIC_VADJ to set the vccd target; 2'b10: Scale for vref=0.6V, use PMIC_VADJ to set the vccd target; 2'b11: No scaling, PMIC_VADJ has no effect"]
    #[inline(always)]
    pub fn pmic_vref(&self) -> PMIC_VREF_R {
        PMIC_VREF_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:8 - Voltage adjustment output setting. The lookup table in this field requires the proper setting in PMIC_VREF for the chosen PMIC. This field has no effect when PMIC_VREF selects no scaling. The feedback tap point is at a vccd pad inside the chip, so the voltage may be a little higher at the PMIC output. Adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.PMIC_VADJ_OFFSET to desired code in this lookup table: 0x03: 1.040V, 0x04: 1.049V, 0x05: 1.057V, 0x06: 1.066V, 0x07: 1.074V, 0x08: 1.083V, 0x09: 1.091V, 0x0A: 1.099V, 0x0B: 1.108V, 0x0C: 1.116V, 0x0D: 1.125V, 0x0E: 1.133V, 0x0F: 1.142V, 0x10: 1.150V, 0x11: 1.158V, 0x12: 1.167V, 0x13: 1.175V, 0x14: 1.184V, 0x15: 1.192V, 0x16: 1.201V, 0x17: 1.209V, 0x18: 1.218V, 0x19: 1.226V, 0x1A: 1.234V, 0x1B: 1.243V, 0x1C: 1.251V, others: Illegal. Behavior is undefined."]
    #[inline(always)]
    pub fn pmic_vadj(&self) -> PMIC_VADJ_R {
        PMIC_VADJ_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Controls whether hardware sequencer keeps the internal Active Linear Regulator enabled to improve supply supervision of vccd. When using this feature, if the PMIC fails to keep vccd above the internal regulator target, then the internal regulator will attempt to recover vccd. If the regulator current is too high, the regulator triggers an over-current detector (OCD) reset. 0: Internal Active Linear Regulator disabled after PMIC enabled. OCD is disabled.; 1: Internal Active Linear Regulator kept enabled. See datasheet for minimum PMIC vccd input to prevent OCD."]
    #[inline(always)]
    pub fn pmic_use_linreg(&self) -> PMIC_USE_LINREG_R {
        PMIC_USE_LINREG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog buffer enable on voltage adjust output. Write this bit depending on the type of PMIC connected: 0: Bypass buffer. This connects the resistor divider directly to the output pin. Use this setting for a PMIC with a high-impedance feedback input, such as those that support a resistor divider on the PCB. This setting can also be used with a low-impedance PMIC with PMIC_VREF=2'b11 (no scaling). 1: Use analog buffer. This enables an analog buffer between the resistor divider output and the pin. The buffer can drive a resistor divider on the PCB that feeds into the PMIC feedback input. This allows targeting a different PMIC reference voltage from PMIC_VREF choices, while still supporting voltage adjustment using the internal divider."]
    #[inline(always)]
    pub fn pmic_vadj_buf_en(&self) -> PMIC_VADJ_BUF_EN_R {
        PMIC_VADJ_BUF_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output enable for PMIC enable pin. Set this bit high to enable the driver on this pin."]
    #[inline(always)]
    pub fn pmic_ctl_outen(&self) -> PMIC_CTL_OUTEN_R {
        PMIC_CTL_OUTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Polarity used to enable the PMIC. The sequencer uses PMIC_CTL_POLARITY to enable the PMIC, and it uses the complement to disable the PMIC."]
    #[inline(always)]
    pub fn pmic_ctl_polarity(&self) -> PMIC_CTL_POLARITY_R {
        PMIC_CTL_POLARITY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input buffer enable for PMIC status input. Set this bit high to enable the input receiver."]
    #[inline(always)]
    pub fn pmic_status_inen(&self) -> PMIC_STATUS_INEN_R {
        PMIC_STATUS_INEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The polarity used to trigger a reset action based on the PMIC status input. The reset system triggers a reset when the unmasked PMIC status matches this value."]
    #[inline(always)]
    pub fn pmic_status_polarity(&self) -> PMIC_STATUS_POLARITY_R {
        PMIC_STATUS_POLARITY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29 - Wait count in 4us steps after PMIC status ok. This is used by the hardware sequencer to allow additional settling time before disabling the internal regulator. The LSB is 32 IMO periods which results in a nominal LSB step of 4us."]
    #[inline(always)]
    pub fn pmic_status_wait(&self) -> PMIC_STATUS_WAIT_R {
        PMIC_STATUS_WAIT_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Indicates the PMIC has been configured. This is used to know if PMIC should be enabled in response to a debug power up request. Do not change PMIC settings after this bit is set high."]
    #[inline(always)]
    pub fn pmic_configured(&self) -> PMIC_CONFIGURED_R {
        PMIC_CONFIGURED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - PMIC reference voltage setting. This selects the scaling factor used to generate the output voltage (vout) given the feedback voltage (vfb) for the chosen PMIC. For a PMIC that compares vfb to an internal reference voltage (vref) according to the formula vout=vref/vfb, select that vref below. For a PMIC that contains an internal resistor divider and expects an unscaled feedback voltage, use the 'No scaling' choice. 2'b00: Scale for vref=0.9V, use PMIC_VADJ to set the vccd target; 2'b01: Scale for vref=0.8V, use PMIC_VADJ to set the vccd target; 2'b10: Scale for vref=0.6V, use PMIC_VADJ to set the vccd target; 2'b11: No scaling, PMIC_VADJ has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_vref(&mut self) -> PMIC_VREF_W<2> {
        PMIC_VREF_W::new(self)
    }
    #[doc = "Bits 4:8 - Voltage adjustment output setting. The lookup table in this field requires the proper setting in PMIC_VREF for the chosen PMIC. This field has no effect when PMIC_VREF selects no scaling. The feedback tap point is at a vccd pad inside the chip, so the voltage may be a little higher at the PMIC output. Adjust for die to die variation by adding the signed offset value in SFLASH SRSS_PWR_OFFSET.PMIC_VADJ_OFFSET to desired code in this lookup table: 0x03: 1.040V, 0x04: 1.049V, 0x05: 1.057V, 0x06: 1.066V, 0x07: 1.074V, 0x08: 1.083V, 0x09: 1.091V, 0x0A: 1.099V, 0x0B: 1.108V, 0x0C: 1.116V, 0x0D: 1.125V, 0x0E: 1.133V, 0x0F: 1.142V, 0x10: 1.150V, 0x11: 1.158V, 0x12: 1.167V, 0x13: 1.175V, 0x14: 1.184V, 0x15: 1.192V, 0x16: 1.201V, 0x17: 1.209V, 0x18: 1.218V, 0x19: 1.226V, 0x1A: 1.234V, 0x1B: 1.243V, 0x1C: 1.251V, others: Illegal. Behavior is undefined."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_vadj(&mut self) -> PMIC_VADJ_W<4> {
        PMIC_VADJ_W::new(self)
    }
    #[doc = "Bit 10 - Controls whether hardware sequencer keeps the internal Active Linear Regulator enabled to improve supply supervision of vccd. When using this feature, if the PMIC fails to keep vccd above the internal regulator target, then the internal regulator will attempt to recover vccd. If the regulator current is too high, the regulator triggers an over-current detector (OCD) reset. 0: Internal Active Linear Regulator disabled after PMIC enabled. OCD is disabled.; 1: Internal Active Linear Regulator kept enabled. See datasheet for minimum PMIC vccd input to prevent OCD."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_use_linreg(&mut self) -> PMIC_USE_LINREG_W<10> {
        PMIC_USE_LINREG_W::new(self)
    }
    #[doc = "Bit 15 - Analog buffer enable on voltage adjust output. Write this bit depending on the type of PMIC connected: 0: Bypass buffer. This connects the resistor divider directly to the output pin. Use this setting for a PMIC with a high-impedance feedback input, such as those that support a resistor divider on the PCB. This setting can also be used with a low-impedance PMIC with PMIC_VREF=2'b11 (no scaling). 1: Use analog buffer. This enables an analog buffer between the resistor divider output and the pin. The buffer can drive a resistor divider on the PCB that feeds into the PMIC feedback input. This allows targeting a different PMIC reference voltage from PMIC_VREF choices, while still supporting voltage adjustment using the internal divider."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_vadj_buf_en(&mut self) -> PMIC_VADJ_BUF_EN_W<15> {
        PMIC_VADJ_BUF_EN_W::new(self)
    }
    #[doc = "Bit 16 - Output enable for PMIC enable pin. Set this bit high to enable the driver on this pin."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_ctl_outen(&mut self) -> PMIC_CTL_OUTEN_W<16> {
        PMIC_CTL_OUTEN_W::new(self)
    }
    #[doc = "Bit 17 - Polarity used to enable the PMIC. The sequencer uses PMIC_CTL_POLARITY to enable the PMIC, and it uses the complement to disable the PMIC."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_ctl_polarity(&mut self) -> PMIC_CTL_POLARITY_W<17> {
        PMIC_CTL_POLARITY_W::new(self)
    }
    #[doc = "Bit 18 - Input buffer enable for PMIC status input. Set this bit high to enable the input receiver."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_status_inen(&mut self) -> PMIC_STATUS_INEN_W<18> {
        PMIC_STATUS_INEN_W::new(self)
    }
    #[doc = "Bit 19 - The polarity used to trigger a reset action based on the PMIC status input. The reset system triggers a reset when the unmasked PMIC status matches this value."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_status_polarity(&mut self) -> PMIC_STATUS_POLARITY_W<19> {
        PMIC_STATUS_POLARITY_W::new(self)
    }
    #[doc = "Bits 20:29 - Wait count in 4us steps after PMIC status ok. This is used by the hardware sequencer to allow additional settling time before disabling the internal regulator. The LSB is 32 IMO periods which results in a nominal LSB step of 4us."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_status_wait(&mut self) -> PMIC_STATUS_WAIT_W<20> {
        PMIC_STATUS_WAIT_W::new(self)
    }
    #[doc = "Bit 31 - Indicates the PMIC has been configured. This is used to know if PMIC should be enabled in response to a debug power up request. Do not change PMIC settings after this bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_configured(&mut self) -> PMIC_CONFIGURED_W<31> {
        PMIC_CONFIGURED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMIC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_pmic_ctl](index.html) module"]
pub struct PWR_PMIC_CTL_SPEC;
impl crate::RegisterSpec for PWR_PMIC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_pmic_ctl::R](R) reader structure"]
impl crate::Readable for PWR_PMIC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_pmic_ctl::W](W) writer structure"]
impl crate::Writable for PWR_PMIC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_PMIC_CTL to value 0x0104"]
impl crate::Resettable for PWR_PMIC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0104;
}
