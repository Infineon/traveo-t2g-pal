#[doc = "Register `DFT_CTL` reader"]
pub struct R(crate::R<DFT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFT_CTL` writer"]
pub struct W(crate::W<DFT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFT_CTL_SPEC>;
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
impl From<crate::W<DFT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_ATB` reader - Analog Test bus select bits used to choose an analog parameter to test"]
pub type TEST_ATB_R = crate::FieldReader<u8, TEST_ATB_A>;
#[doc = "Analog Test bus select bits used to choose an analog parameter to test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TEST_ATB_A {
    #[doc = "0: Not Testing"]
    NO_TESTING = 0,
    #[doc = "1: Testing Bandgap Current"]
    BANDGAP_CURRENT = 1,
    #[doc = "2: Testing Common Mode Voltage"]
    COMMON_MODE_VOLTAGE = 2,
    #[doc = "3: Testing Charge Pump Up Current"]
    CHARGE_PUMP_UP_CURRENT = 3,
    #[doc = "4: Testing Charge Pump Down Current"]
    CHARGE_PUMP_DOWN_CURRENT = 4,
    #[doc = "5: Testing Charge Pump Mismatch Current"]
    CHARGE_PUMP_MISMATCH_CURRENT = 5,
    #[doc = "6: Testing Loop Filter Leakage"]
    LOOP_FILTER_LEAKAGE = 6,
    #[doc = "7: Testing VCO Voltage"]
    VCO_VOLTAGE = 7,
}
impl From<TEST_ATB_A> for u8 {
    #[inline(always)]
    fn from(variant: TEST_ATB_A) -> Self {
        variant as _
    }
}
impl TEST_ATB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_ATB_A {
        match self.bits {
            0 => TEST_ATB_A::NO_TESTING,
            1 => TEST_ATB_A::BANDGAP_CURRENT,
            2 => TEST_ATB_A::COMMON_MODE_VOLTAGE,
            3 => TEST_ATB_A::CHARGE_PUMP_UP_CURRENT,
            4 => TEST_ATB_A::CHARGE_PUMP_DOWN_CURRENT,
            5 => TEST_ATB_A::CHARGE_PUMP_MISMATCH_CURRENT,
            6 => TEST_ATB_A::LOOP_FILTER_LEAKAGE,
            7 => TEST_ATB_A::VCO_VOLTAGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_TESTING`"]
    #[inline(always)]
    pub fn is_no_testing(&self) -> bool {
        *self == TEST_ATB_A::NO_TESTING
    }
    #[doc = "Checks if the value of the field is `BANDGAP_CURRENT`"]
    #[inline(always)]
    pub fn is_bandgap_current(&self) -> bool {
        *self == TEST_ATB_A::BANDGAP_CURRENT
    }
    #[doc = "Checks if the value of the field is `COMMON_MODE_VOLTAGE`"]
    #[inline(always)]
    pub fn is_common_mode_voltage(&self) -> bool {
        *self == TEST_ATB_A::COMMON_MODE_VOLTAGE
    }
    #[doc = "Checks if the value of the field is `CHARGE_PUMP_UP_CURRENT`"]
    #[inline(always)]
    pub fn is_charge_pump_up_current(&self) -> bool {
        *self == TEST_ATB_A::CHARGE_PUMP_UP_CURRENT
    }
    #[doc = "Checks if the value of the field is `CHARGE_PUMP_DOWN_CURRENT`"]
    #[inline(always)]
    pub fn is_charge_pump_down_current(&self) -> bool {
        *self == TEST_ATB_A::CHARGE_PUMP_DOWN_CURRENT
    }
    #[doc = "Checks if the value of the field is `CHARGE_PUMP_MISMATCH_CURRENT`"]
    #[inline(always)]
    pub fn is_charge_pump_mismatch_current(&self) -> bool {
        *self == TEST_ATB_A::CHARGE_PUMP_MISMATCH_CURRENT
    }
    #[doc = "Checks if the value of the field is `LOOP_FILTER_LEAKAGE`"]
    #[inline(always)]
    pub fn is_loop_filter_leakage(&self) -> bool {
        *self == TEST_ATB_A::LOOP_FILTER_LEAKAGE
    }
    #[doc = "Checks if the value of the field is `VCO_VOLTAGE`"]
    #[inline(always)]
    pub fn is_vco_voltage(&self) -> bool {
        *self == TEST_ATB_A::VCO_VOLTAGE
    }
}
#[doc = "Field `TEST_ATB` writer - Analog Test bus select bits used to choose an analog parameter to test"]
pub type TEST_ATB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DFT_CTL_SPEC, u8, TEST_ATB_A, 3, O>;
impl<'a, const O: u8> TEST_ATB_W<'a, O> {
    #[doc = "Not Testing"]
    #[inline(always)]
    pub fn no_testing(self) -> &'a mut W {
        self.variant(TEST_ATB_A::NO_TESTING)
    }
    #[doc = "Testing Bandgap Current"]
    #[inline(always)]
    pub fn bandgap_current(self) -> &'a mut W {
        self.variant(TEST_ATB_A::BANDGAP_CURRENT)
    }
    #[doc = "Testing Common Mode Voltage"]
    #[inline(always)]
    pub fn common_mode_voltage(self) -> &'a mut W {
        self.variant(TEST_ATB_A::COMMON_MODE_VOLTAGE)
    }
    #[doc = "Testing Charge Pump Up Current"]
    #[inline(always)]
    pub fn charge_pump_up_current(self) -> &'a mut W {
        self.variant(TEST_ATB_A::CHARGE_PUMP_UP_CURRENT)
    }
    #[doc = "Testing Charge Pump Down Current"]
    #[inline(always)]
    pub fn charge_pump_down_current(self) -> &'a mut W {
        self.variant(TEST_ATB_A::CHARGE_PUMP_DOWN_CURRENT)
    }
    #[doc = "Testing Charge Pump Mismatch Current"]
    #[inline(always)]
    pub fn charge_pump_mismatch_current(self) -> &'a mut W {
        self.variant(TEST_ATB_A::CHARGE_PUMP_MISMATCH_CURRENT)
    }
    #[doc = "Testing Loop Filter Leakage"]
    #[inline(always)]
    pub fn loop_filter_leakage(self) -> &'a mut W {
        self.variant(TEST_ATB_A::LOOP_FILTER_LEAKAGE)
    }
    #[doc = "Testing VCO Voltage"]
    #[inline(always)]
    pub fn vco_voltage(self) -> &'a mut W {
        self.variant(TEST_ATB_A::VCO_VOLTAGE)
    }
}
#[doc = "Field `PLLTEST` reader - PLL Charge Pump current control, connected to port TST of LVDS wrapper"]
pub type PLLTEST_R = crate::FieldReader<u8, PLLTEST_A>;
#[doc = "PLL Charge Pump current control, connected to port TST of LVDS wrapper\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLTEST_A {
    #[doc = "9: Default value to be programmed"]
    DEFAULT = 9,
}
impl From<PLLTEST_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLTEST_A) -> Self {
        variant as _
    }
}
impl PLLTEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLTEST_A> {
        match self.bits {
            9 => Some(PLLTEST_A::DEFAULT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == PLLTEST_A::DEFAULT
    }
}
#[doc = "Field `PLLTEST` writer - PLL Charge Pump current control, connected to port TST of LVDS wrapper"]
pub type PLLTEST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFT_CTL_SPEC, u8, PLLTEST_A, 4, O>;
impl<'a, const O: u8> PLLTEST_W<'a, O> {
    #[doc = "Default value to be programmed"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(PLLTEST_A::DEFAULT)
    }
}
#[doc = "Field `CLKREF_SEL` reader - Select line of the mux used for selecting reference clock for the internal PLL"]
pub type CLKREF_SEL_R = crate::FieldReader<u8, CLKREF_SEL_A>;
#[doc = "Select line of the mux used for selecting reference clock for the internal PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKREF_SEL_A {
    #[doc = "0: Output of SRSSPLL, approx 200Mhz, passed through the DIV/2 divider and then used as a reference clock for the PLL"]
    SRSSPLL_OUTPUT_W_DIV2 = 0,
    #[doc = "1: Use the input clock of the SRSSPLL ,approx 100Mhz, and directly use it as reference clock for the PLL without passing it through the DIV/2 divider"]
    SRSSPLL_INPUT_WO_DIV2 = 1,
    #[doc = "2: Use external clock from the GPIO ,approx 100Mhz, directly as a reference clock for the PLL"]
    EXT_GPIO = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<CLKREF_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKREF_SEL_A) -> Self {
        variant as _
    }
}
impl CLKREF_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKREF_SEL_A {
        match self.bits {
            0 => CLKREF_SEL_A::SRSSPLL_OUTPUT_W_DIV2,
            1 => CLKREF_SEL_A::SRSSPLL_INPUT_WO_DIV2,
            2 => CLKREF_SEL_A::EXT_GPIO,
            3 => CLKREF_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SRSSPLL_OUTPUT_W_DIV2`"]
    #[inline(always)]
    pub fn is_srsspll_output_w_div2(&self) -> bool {
        *self == CLKREF_SEL_A::SRSSPLL_OUTPUT_W_DIV2
    }
    #[doc = "Checks if the value of the field is `SRSSPLL_INPUT_WO_DIV2`"]
    #[inline(always)]
    pub fn is_srsspll_input_wo_div2(&self) -> bool {
        *self == CLKREF_SEL_A::SRSSPLL_INPUT_WO_DIV2
    }
    #[doc = "Checks if the value of the field is `EXT_GPIO`"]
    #[inline(always)]
    pub fn is_ext_gpio(&self) -> bool {
        *self == CLKREF_SEL_A::EXT_GPIO
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == CLKREF_SEL_A::RSVD
    }
}
#[doc = "Field `CLKREF_SEL` writer - Select line of the mux used for selecting reference clock for the internal PLL"]
pub type CLKREF_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DFT_CTL_SPEC, u8, CLKREF_SEL_A, 2, O>;
impl<'a, const O: u8> CLKREF_SEL_W<'a, O> {
    #[doc = "Output of SRSSPLL, approx 200Mhz, passed through the DIV/2 divider and then used as a reference clock for the PLL"]
    #[inline(always)]
    pub fn srsspll_output_w_div2(self) -> &'a mut W {
        self.variant(CLKREF_SEL_A::SRSSPLL_OUTPUT_W_DIV2)
    }
    #[doc = "Use the input clock of the SRSSPLL ,approx 100Mhz, and directly use it as reference clock for the PLL without passing it through the DIV/2 divider"]
    #[inline(always)]
    pub fn srsspll_input_wo_div2(self) -> &'a mut W {
        self.variant(CLKREF_SEL_A::SRSSPLL_INPUT_WO_DIV2)
    }
    #[doc = "Use external clock from the GPIO ,approx 100Mhz, directly as a reference clock for the PLL"]
    #[inline(always)]
    pub fn ext_gpio(self) -> &'a mut W {
        self.variant(CLKREF_SEL_A::EXT_GPIO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(CLKREF_SEL_A::RSVD)
    }
}
impl R {
    #[doc = "Bits 0:2 - Analog Test bus select bits used to choose an analog parameter to test"]
    #[inline(always)]
    pub fn test_atb(&self) -> TEST_ATB_R {
        TEST_ATB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - PLL Charge Pump current control, connected to port TST of LVDS wrapper"]
    #[inline(always)]
    pub fn plltest(&self) -> PLLTEST_R {
        PLLTEST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Select line of the mux used for selecting reference clock for the internal PLL"]
    #[inline(always)]
    pub fn clkref_sel(&self) -> CLKREF_SEL_R {
        CLKREF_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Analog Test bus select bits used to choose an analog parameter to test"]
    #[inline(always)]
    #[must_use]
    pub fn test_atb(&mut self) -> TEST_ATB_W<0> {
        TEST_ATB_W::new(self)
    }
    #[doc = "Bits 4:7 - PLL Charge Pump current control, connected to port TST of LVDS wrapper"]
    #[inline(always)]
    #[must_use]
    pub fn plltest(&mut self) -> PLLTEST_W<4> {
        PLLTEST_W::new(self)
    }
    #[doc = "Bits 8:9 - Select line of the mux used for selecting reference clock for the internal PLL"]
    #[inline(always)]
    #[must_use]
    pub fn clkref_sel(&mut self) -> CLKREF_SEL_W<8> {
        CLKREF_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT configuration register for fpdlink\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dft_ctl](index.html) module"]
pub struct DFT_CTL_SPEC;
impl crate::RegisterSpec for DFT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dft_ctl::R](R) reader structure"]
impl crate::Readable for DFT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dft_ctl::W](W) writer structure"]
impl crate::Writable for DFT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFT_CTL to value 0"]
impl crate::Resettable for DFT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
