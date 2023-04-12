#[doc = "Register `SAMPLE_CTL` reader"]
pub struct R(crate::R<SAMPLE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE_CTL` writer"]
pub struct W(crate::W<SAMPLE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_CTL_SPEC>;
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
impl From<crate::W<SAMPLE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_ADDR` reader - N/A"]
pub type PIN_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_ADDR` writer - N/A"]
pub type PIN_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPLE_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `PORT_ADDR` reader - Select the physical port. This field is only valid for ADC0. ADC0 can control and connect to the SARMUX of the neighboring ADC1-3. This requires the corresponding ADC to be off while the SARMUX is left on. When ADC0 controls another SARMUX it uses the PIN_ADDR, EXT_MUX_EN/SEL of this channel to control the other SARMUX."]
pub type PORT_ADDR_R = crate::FieldReader<u8, PORT_ADDR_A>;
#[doc = "Select the physical port. This field is only valid for ADC0. ADC0 can control and connect to the SARMUX of the neighboring ADC1-3. This requires the corresponding ADC to be off while the SARMUX is left on. When ADC0 controls another SARMUX it uses the PIN_ADDR, EXT_MUX_EN/SEL of this channel to control the other SARMUX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PORT_ADDR_A {
    #[doc = "0: ADC uses it's own SARMUX"]
    SARMUX0 = 0,
    #[doc = "1: ADC0 uses SARMUX1 (only valid for ADC0, undefined result if used for ADC1-3)"]
    SARMUX1 = 1,
    #[doc = "2: ADC0 uses SARMUX2 (only valid for ADC0, undefined result if used for ADC1-3)"]
    SARMUX2 = 2,
    #[doc = "3: ADC0 uses SARMUX3 (only valid for ADC0, undefined result if used for ADC1-3)"]
    SARMUX3 = 3,
}
impl From<PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT_ADDR_A) -> Self {
        variant as _
    }
}
impl PORT_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_ADDR_A {
        match self.bits {
            0 => PORT_ADDR_A::SARMUX0,
            1 => PORT_ADDR_A::SARMUX1,
            2 => PORT_ADDR_A::SARMUX2,
            3 => PORT_ADDR_A::SARMUX3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX0`"]
    #[inline(always)]
    pub fn is_sarmux0(&self) -> bool {
        *self == PORT_ADDR_A::SARMUX0
    }
    #[doc = "Checks if the value of the field is `SARMUX1`"]
    #[inline(always)]
    pub fn is_sarmux1(&self) -> bool {
        *self == PORT_ADDR_A::SARMUX1
    }
    #[doc = "Checks if the value of the field is `SARMUX2`"]
    #[inline(always)]
    pub fn is_sarmux2(&self) -> bool {
        *self == PORT_ADDR_A::SARMUX2
    }
    #[doc = "Checks if the value of the field is `SARMUX3`"]
    #[inline(always)]
    pub fn is_sarmux3(&self) -> bool {
        *self == PORT_ADDR_A::SARMUX3
    }
}
#[doc = "Field `PORT_ADDR` writer - Select the physical port. This field is only valid for ADC0. ADC0 can control and connect to the SARMUX of the neighboring ADC1-3. This requires the corresponding ADC to be off while the SARMUX is left on. When ADC0 controls another SARMUX it uses the PIN_ADDR, EXT_MUX_EN/SEL of this channel to control the other SARMUX."]
pub type PORT_ADDR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SAMPLE_CTL_SPEC, u8, PORT_ADDR_A, 2, O>;
impl<'a, const O: u8> PORT_ADDR_W<'a, O> {
    #[doc = "ADC uses it's own SARMUX"]
    #[inline(always)]
    pub fn sarmux0(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::SARMUX0)
    }
    #[doc = "ADC0 uses SARMUX1 (only valid for ADC0, undefined result if used for ADC1-3)"]
    #[inline(always)]
    pub fn sarmux1(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::SARMUX1)
    }
    #[doc = "ADC0 uses SARMUX2 (only valid for ADC0, undefined result if used for ADC1-3)"]
    #[inline(always)]
    pub fn sarmux2(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::SARMUX2)
    }
    #[doc = "ADC0 uses SARMUX3 (only valid for ADC0, undefined result if used for ADC1-3)"]
    #[inline(always)]
    pub fn sarmux3(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::SARMUX3)
    }
}
#[doc = "Field `EXT_MUX_SEL` reader - External analog mux select. This bit setting is related to EXT_MUX\\[x\\]_y on pin assignment. 0x0: Select EXT_MUX\\[x\\]_0 pin 0x1: Select EXT_MUX\\[x\\]_1 pin"]
pub type EXT_MUX_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXT_MUX_SEL` writer - External analog mux select. This bit setting is related to EXT_MUX\\[x\\]_y on pin assignment. 0x0: Select EXT_MUX\\[x\\]_0 pin 0x1: Select EXT_MUX\\[x\\]_1 pin"]
pub type EXT_MUX_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLE_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `EXT_MUX_EN` reader - External analog mux enable. This enable can be used as enable (chip select) for the external analog mux (this enable is not used as enable for the GPIO output driver). This enable also prevents unnecessary toggle activity on the select signals of the external analog mux. When this enable is low EXT_MUX_SEL value will be ignored and the previous value will be maintained. Note that an external analog mux can only be used in combination with a pin input, i.e. PIN_ADDR&lt;32 or Vmotor. If an internal signal is selected this enable should be 0."]
pub type EXT_MUX_EN_R = crate::BitReader<bool>;
#[doc = "Field `EXT_MUX_EN` writer - External analog mux enable. This enable can be used as enable (chip select) for the external analog mux (this enable is not used as enable for the GPIO output driver). This enable also prevents unnecessary toggle activity on the select signals of the external analog mux. When this enable is low EXT_MUX_SEL value will be ignored and the previous value will be maintained. Note that an external analog mux can only be used in combination with a pin input, i.e. PIN_ADDR&lt;32 or Vmotor. If an internal signal is selected this enable should be 0."]
pub type EXT_MUX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAMPLE_CTL_SPEC, bool, O>;
#[doc = "Field `PRECOND_MODE` reader - Select preconditioning mode. Preconditioning (dis)charges the SAR sample capacitor to the selected reference voltage for PRECOND_TIME (global) cycles, a break before make cycle will be inserted before sampling starts (SAMPLE_TIME)."]
pub type PRECOND_MODE_R = crate::FieldReader<u8, PRECOND_MODE_A>;
#[doc = "Select preconditioning mode. Preconditioning (dis)charges the SAR sample capacitor to the selected reference voltage for PRECOND_TIME (global) cycles, a break before make cycle will be inserted before sampling starts (SAMPLE_TIME).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRECOND_MODE_A {
    #[doc = "0: No preconditioning"]
    OFF = 0,
    #[doc = "1: Discharge to VREFL"]
    VREFL = 1,
    #[doc = "2: Charge to VREFH"]
    VREFH = 2,
    #[doc = "3: Connect the Diagnostic reference output during preconditioning. The Diagnostic reference should be configured to output a reference voltage. Note: this selection is mutual exclusive with using the Diagnostic reference to supply an ibias current for OVERLAP."]
    DIAG = 3,
}
impl From<PRECOND_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRECOND_MODE_A) -> Self {
        variant as _
    }
}
impl PRECOND_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRECOND_MODE_A {
        match self.bits {
            0 => PRECOND_MODE_A::OFF,
            1 => PRECOND_MODE_A::VREFL,
            2 => PRECOND_MODE_A::VREFH,
            3 => PRECOND_MODE_A::DIAG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PRECOND_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `VREFL`"]
    #[inline(always)]
    pub fn is_vrefl(&self) -> bool {
        *self == PRECOND_MODE_A::VREFL
    }
    #[doc = "Checks if the value of the field is `VREFH`"]
    #[inline(always)]
    pub fn is_vrefh(&self) -> bool {
        *self == PRECOND_MODE_A::VREFH
    }
    #[doc = "Checks if the value of the field is `DIAG`"]
    #[inline(always)]
    pub fn is_diag(&self) -> bool {
        *self == PRECOND_MODE_A::DIAG
    }
}
#[doc = "Field `PRECOND_MODE` writer - Select preconditioning mode. Preconditioning (dis)charges the SAR sample capacitor to the selected reference voltage for PRECOND_TIME (global) cycles, a break before make cycle will be inserted before sampling starts (SAMPLE_TIME)."]
pub type PRECOND_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SAMPLE_CTL_SPEC, u8, PRECOND_MODE_A, 2, O>;
impl<'a, const O: u8> PRECOND_MODE_W<'a, O> {
    #[doc = "No preconditioning"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PRECOND_MODE_A::OFF)
    }
    #[doc = "Discharge to VREFL"]
    #[inline(always)]
    pub fn vrefl(self) -> &'a mut W {
        self.variant(PRECOND_MODE_A::VREFL)
    }
    #[doc = "Charge to VREFH"]
    #[inline(always)]
    pub fn vrefh(self) -> &'a mut W {
        self.variant(PRECOND_MODE_A::VREFH)
    }
    #[doc = "Connect the Diagnostic reference output during preconditioning. The Diagnostic reference should be configured to output a reference voltage. Note: this selection is mutual exclusive with using the Diagnostic reference to supply an ibias current for OVERLAP."]
    #[inline(always)]
    pub fn diag(self) -> &'a mut W {
        self.variant(PRECOND_MODE_A::DIAG)
    }
}
#[doc = "Field `OVERLAP_DIAG` reader - Select Overlap mode or SARMUX Diagnostics, in both cases the Diagnostic reference is used. With Overlap the Diagnostic reference typically sources or sinks a small current which is connected at the same time as the analog signal being sampled. For SARMUX Diagnostics the Diagnostic reference should provide a reference voltage which is selected at the SARMUX input instead of the normal analog signal being sampled."]
pub type OVERLAP_DIAG_R = crate::FieldReader<u8, OVERLAP_DIAG_A>;
#[doc = "Select Overlap mode or SARMUX Diagnostics, in both cases the Diagnostic reference is used. With Overlap the Diagnostic reference typically sources or sinks a small current which is connected at the same time as the analog signal being sampled. For SARMUX Diagnostics the Diagnostic reference should provide a reference voltage which is selected at the SARMUX input instead of the normal analog signal being sampled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVERLAP_DIAG_A {
    #[doc = "0: No overlap or SARMUX Diagnostics"]
    OFF = 0,
    #[doc = "1: Sample the selected analog input for 2 SAMPLE_TIME periods. During the first period use overlap sampling, i.e. connect both the analog input and Diagnostic reference. During second period only connect the analog input."]
    HALF = 1,
    #[doc = "2: Like normal sample the selected analog input for a single SAMPLE_TIME period but use overlap sampling, i.e. connect both the analog input and Diagnostic reference."]
    FULL = 2,
    #[doc = "3: Select Diagnostic reference instead of analog signal at the input of the SARMUX. This enables a functional safety check of the SARMUX analog connections."]
    MUX_DIAG = 3,
}
impl From<OVERLAP_DIAG_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERLAP_DIAG_A) -> Self {
        variant as _
    }
}
impl OVERLAP_DIAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERLAP_DIAG_A {
        match self.bits {
            0 => OVERLAP_DIAG_A::OFF,
            1 => OVERLAP_DIAG_A::HALF,
            2 => OVERLAP_DIAG_A::FULL,
            3 => OVERLAP_DIAG_A::MUX_DIAG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == OVERLAP_DIAG_A::OFF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == OVERLAP_DIAG_A::HALF
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == OVERLAP_DIAG_A::FULL
    }
    #[doc = "Checks if the value of the field is `MUX_DIAG`"]
    #[inline(always)]
    pub fn is_mux_diag(&self) -> bool {
        *self == OVERLAP_DIAG_A::MUX_DIAG
    }
}
#[doc = "Field `OVERLAP_DIAG` writer - Select Overlap mode or SARMUX Diagnostics, in both cases the Diagnostic reference is used. With Overlap the Diagnostic reference typically sources or sinks a small current which is connected at the same time as the analog signal being sampled. For SARMUX Diagnostics the Diagnostic reference should provide a reference voltage which is selected at the SARMUX input instead of the normal analog signal being sampled."]
pub type OVERLAP_DIAG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SAMPLE_CTL_SPEC, u8, OVERLAP_DIAG_A, 2, O>;
impl<'a, const O: u8> OVERLAP_DIAG_W<'a, O> {
    #[doc = "No overlap or SARMUX Diagnostics"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(OVERLAP_DIAG_A::OFF)
    }
    #[doc = "Sample the selected analog input for 2 SAMPLE_TIME periods. During the first period use overlap sampling, i.e. connect both the analog input and Diagnostic reference. During second period only connect the analog input."]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(OVERLAP_DIAG_A::HALF)
    }
    #[doc = "Like normal sample the selected analog input for a single SAMPLE_TIME period but use overlap sampling, i.e. connect both the analog input and Diagnostic reference."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(OVERLAP_DIAG_A::FULL)
    }
    #[doc = "Select Diagnostic reference instead of analog signal at the input of the SARMUX. This enables a functional safety check of the SARMUX analog connections."]
    #[inline(always)]
    pub fn mux_diag(self) -> &'a mut W {
        self.variant(OVERLAP_DIAG_A::MUX_DIAG)
    }
}
#[doc = "Field `SAMPLE_TIME` reader - Sample time (aperture) in ADC clock cycles. Minimum is 1 (0 gives the same result as 1), minimum time needed for proper settling is at least 412ns, i.e.11 clock cycles at the max frequency of 26.7MHz."]
pub type SAMPLE_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAMPLE_TIME` writer - Sample time (aperture) in ADC clock cycles. Minimum is 1 (0 gives the same result as 1), minimum time needed for proper settling is at least 412ns, i.e.11 clock cycles at the max frequency of 26.7MHz."]
pub type SAMPLE_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLE_CTL_SPEC, u16, u16, 12, O>;
#[doc = "Field `ALT_CAL` reader - Use alternate calibration values instead of the current calibration values. This allows the firmware to allocate one or more channels to quietly re-calibrate the ADC in the background of regular processing. 0 = use regular calibration values (ANA/DIG_CAL) 1 = use alternate calibration values (ANA/DIG_CAL_ALT) Note: typically calibration measurements select VrefL (PIN_ADDR=62) or VrefH (PIN_ADDR=63)"]
pub type ALT_CAL_R = crate::BitReader<bool>;
#[doc = "Field `ALT_CAL` writer - Use alternate calibration values instead of the current calibration values. This allows the firmware to allocate one or more channels to quietly re-calibrate the ADC in the background of regular processing. 0 = use regular calibration values (ANA/DIG_CAL) 1 = use alternate calibration values (ANA/DIG_CAL_ALT) Note: typically calibration measurements select VrefL (PIN_ADDR=62) or VrefH (PIN_ADDR=63)"]
pub type ALT_CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAMPLE_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - N/A"]
    #[inline(always)]
    pub fn pin_addr(&self) -> PIN_ADDR_R {
        PIN_ADDR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Select the physical port. This field is only valid for ADC0. ADC0 can control and connect to the SARMUX of the neighboring ADC1-3. This requires the corresponding ADC to be off while the SARMUX is left on. When ADC0 controls another SARMUX it uses the PIN_ADDR, EXT_MUX_EN/SEL of this channel to control the other SARMUX."]
    #[inline(always)]
    pub fn port_addr(&self) -> PORT_ADDR_R {
        PORT_ADDR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - External analog mux select. This bit setting is related to EXT_MUX\\[x\\]_y on pin assignment. 0x0: Select EXT_MUX\\[x\\]_0 pin 0x1: Select EXT_MUX\\[x\\]_1 pin"]
    #[inline(always)]
    pub fn ext_mux_sel(&self) -> EXT_MUX_SEL_R {
        EXT_MUX_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - External analog mux enable. This enable can be used as enable (chip select) for the external analog mux (this enable is not used as enable for the GPIO output driver). This enable also prevents unnecessary toggle activity on the select signals of the external analog mux. When this enable is low EXT_MUX_SEL value will be ignored and the previous value will be maintained. Note that an external analog mux can only be used in combination with a pin input, i.e. PIN_ADDR&lt;32 or Vmotor. If an internal signal is selected this enable should be 0."]
    #[inline(always)]
    pub fn ext_mux_en(&self) -> EXT_MUX_EN_R {
        EXT_MUX_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Select preconditioning mode. Preconditioning (dis)charges the SAR sample capacitor to the selected reference voltage for PRECOND_TIME (global) cycles, a break before make cycle will be inserted before sampling starts (SAMPLE_TIME)."]
    #[inline(always)]
    pub fn precond_mode(&self) -> PRECOND_MODE_R {
        PRECOND_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Select Overlap mode or SARMUX Diagnostics, in both cases the Diagnostic reference is used. With Overlap the Diagnostic reference typically sources or sinks a small current which is connected at the same time as the analog signal being sampled. For SARMUX Diagnostics the Diagnostic reference should provide a reference voltage which is selected at the SARMUX input instead of the normal analog signal being sampled."]
    #[inline(always)]
    pub fn overlap_diag(&self) -> OVERLAP_DIAG_R {
        OVERLAP_DIAG_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:27 - Sample time (aperture) in ADC clock cycles. Minimum is 1 (0 gives the same result as 1), minimum time needed for proper settling is at least 412ns, i.e.11 clock cycles at the max frequency of 26.7MHz."]
    #[inline(always)]
    pub fn sample_time(&self) -> SAMPLE_TIME_R {
        SAMPLE_TIME_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Use alternate calibration values instead of the current calibration values. This allows the firmware to allocate one or more channels to quietly re-calibrate the ADC in the background of regular processing. 0 = use regular calibration values (ANA/DIG_CAL) 1 = use alternate calibration values (ANA/DIG_CAL_ALT) Note: typically calibration measurements select VrefL (PIN_ADDR=62) or VrefH (PIN_ADDR=63)"]
    #[inline(always)]
    pub fn alt_cal(&self) -> ALT_CAL_R {
        ALT_CAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn pin_addr(&mut self) -> PIN_ADDR_W<0> {
        PIN_ADDR_W::new(self)
    }
    #[doc = "Bits 6:7 - Select the physical port. This field is only valid for ADC0. ADC0 can control and connect to the SARMUX of the neighboring ADC1-3. This requires the corresponding ADC to be off while the SARMUX is left on. When ADC0 controls another SARMUX it uses the PIN_ADDR, EXT_MUX_EN/SEL of this channel to control the other SARMUX."]
    #[inline(always)]
    #[must_use]
    pub fn port_addr(&mut self) -> PORT_ADDR_W<6> {
        PORT_ADDR_W::new(self)
    }
    #[doc = "Bits 8:10 - External analog mux select. This bit setting is related to EXT_MUX\\[x\\]_y on pin assignment. 0x0: Select EXT_MUX\\[x\\]_0 pin 0x1: Select EXT_MUX\\[x\\]_1 pin"]
    #[inline(always)]
    #[must_use]
    pub fn ext_mux_sel(&mut self) -> EXT_MUX_SEL_W<8> {
        EXT_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 11 - External analog mux enable. This enable can be used as enable (chip select) for the external analog mux (this enable is not used as enable for the GPIO output driver). This enable also prevents unnecessary toggle activity on the select signals of the external analog mux. When this enable is low EXT_MUX_SEL value will be ignored and the previous value will be maintained. Note that an external analog mux can only be used in combination with a pin input, i.e. PIN_ADDR&lt;32 or Vmotor. If an internal signal is selected this enable should be 0."]
    #[inline(always)]
    #[must_use]
    pub fn ext_mux_en(&mut self) -> EXT_MUX_EN_W<11> {
        EXT_MUX_EN_W::new(self)
    }
    #[doc = "Bits 12:13 - Select preconditioning mode. Preconditioning (dis)charges the SAR sample capacitor to the selected reference voltage for PRECOND_TIME (global) cycles, a break before make cycle will be inserted before sampling starts (SAMPLE_TIME)."]
    #[inline(always)]
    #[must_use]
    pub fn precond_mode(&mut self) -> PRECOND_MODE_W<12> {
        PRECOND_MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - Select Overlap mode or SARMUX Diagnostics, in both cases the Diagnostic reference is used. With Overlap the Diagnostic reference typically sources or sinks a small current which is connected at the same time as the analog signal being sampled. For SARMUX Diagnostics the Diagnostic reference should provide a reference voltage which is selected at the SARMUX input instead of the normal analog signal being sampled."]
    #[inline(always)]
    #[must_use]
    pub fn overlap_diag(&mut self) -> OVERLAP_DIAG_W<14> {
        OVERLAP_DIAG_W::new(self)
    }
    #[doc = "Bits 16:27 - Sample time (aperture) in ADC clock cycles. Minimum is 1 (0 gives the same result as 1), minimum time needed for proper settling is at least 412ns, i.e.11 clock cycles at the max frequency of 26.7MHz."]
    #[inline(always)]
    #[must_use]
    pub fn sample_time(&mut self) -> SAMPLE_TIME_W<16> {
        SAMPLE_TIME_W::new(self)
    }
    #[doc = "Bit 31 - Use alternate calibration values instead of the current calibration values. This allows the firmware to allocate one or more channels to quietly re-calibrate the ADC in the background of regular processing. 0 = use regular calibration values (ANA/DIG_CAL) 1 = use alternate calibration values (ANA/DIG_CAL_ALT) Note: typically calibration measurements select VrefL (PIN_ADDR=62) or VrefH (PIN_ADDR=63)"]
    #[inline(always)]
    #[must_use]
    pub fn alt_cal(&mut self) -> ALT_CAL_W<31> {
        ALT_CAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_ctl](index.html) module"]
pub struct SAMPLE_CTL_SPEC;
impl crate::RegisterSpec for SAMPLE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample_ctl::R](R) reader structure"]
impl crate::Readable for SAMPLE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample_ctl::W](W) writer structure"]
impl crate::Writable for SAMPLE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPLE_CTL to value 0"]
impl crate::Resettable for SAMPLE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
