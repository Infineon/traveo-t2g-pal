#[doc = "Register `IF_CTL` reader"]
pub struct R(crate::R<IF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF_CTL` writer"]
pub struct W(crate::W<IF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_CTL_SPEC>;
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
impl From<crate::W<IF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_DIV` reader - Interface clock divider (legal range \\[0, 255\\]). The DAC system clock clk_dac is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value (\\[1,3,5,...255\\]) or 0 (divide by 1), to ensure a 50/50 percent duty cycle clock output. '0': clk_dac frequency is clk_if frequency (1 clk_dac cycle consists of 1 clk_if cycles). '1': clk_dac frequency is 1/2 clk_if frequency (1 clk_dac cycle consists of 2 clk_if cycles). '2': clk_dac frequency is 1/3 clk_if frequency. Note: results in a non 50/50 percent duty cycle. ... '255': clk_dac frequency is 1/256 clk_if frequency. Note: This field can only be changed when CLOCK_SEL= clk_if off."]
pub type CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_DIV` writer - Interface clock divider (legal range \\[0, 255\\]). The DAC system clock clk_dac is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value (\\[1,3,5,...255\\]) or 0 (divide by 1), to ensure a 50/50 percent duty cycle clock output. '0': clk_dac frequency is clk_if frequency (1 clk_dac cycle consists of 1 clk_if cycles). '1': clk_dac frequency is 1/2 clk_if frequency (1 clk_dac cycle consists of 2 clk_if cycles). '2': clk_dac frequency is 1/3 clk_if frequency. Note: results in a non 50/50 percent duty cycle. ... '255': clk_dac frequency is 1/256 clk_if frequency. Note: This field can only be changed when CLOCK_SEL= clk_if off."]
pub type CLOCK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IF_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLOCK_SEL` reader - Interface clock clk_if selection: '0': SRSS clock clk_if_srss. '1'-'3': Clock 'clk_if' is off. '4': Master interface clock 'dac_mck'. '5'-'7': Clock 'clk_if' is off."]
pub type CLOCK_SEL_R = crate::FieldReader<u8, CLOCK_SEL_A>;
#[doc = "Interface clock clk_if selection: '0': SRSS clock clk_if_srss. '1'-'3': Clock 'clk_if' is off. '4': Master interface clock 'dac_mck'. '5'-'7': Clock 'clk_if' is off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLOCK_SEL_A {
    #[doc = "0: N/A"]
    SEL_SRSS_CLOCK = 0,
    #[doc = "1: N/A"]
    SEL_CLK_IF_OFF = 1,
    #[doc = "4: N/A"]
    SEL_DAC_MCK = 4,
}
impl From<CLOCK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLOCK_SEL_A) -> Self {
        variant as _
    }
}
impl CLOCK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCK_SEL_A> {
        match self.bits {
            0 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK),
            1 => Some(CLOCK_SEL_A::SEL_CLK_IF_OFF),
            4 => Some(CLOCK_SEL_A::SEL_DAC_MCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK`"]
    #[inline(always)]
    pub fn is_sel_srss_clock(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK
    }
    #[doc = "Checks if the value of the field is `SEL_CLK_IF_OFF`"]
    #[inline(always)]
    pub fn is_sel_clk_if_off(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_CLK_IF_OFF
    }
    #[doc = "Checks if the value of the field is `SEL_DAC_MCK`"]
    #[inline(always)]
    pub fn is_sel_dac_mck(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_DAC_MCK
    }
}
#[doc = "Field `CLOCK_SEL` writer - Interface clock clk_if selection: '0': SRSS clock clk_if_srss. '1'-'3': Clock 'clk_if' is off. '4': Master interface clock 'dac_mck'. '5'-'7': Clock 'clk_if' is off."]
pub type CLOCK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_CTL_SPEC, u8, CLOCK_SEL_A, 3, O>;
impl<'a, const O: u8> CLOCK_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_clk_if_off(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_CLK_IF_OFF)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_dac_mck(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_DAC_MCK)
    }
}
#[doc = "Field `FS_SEL` reader - N/A"]
pub type FS_SEL_R = crate::FieldReader<u8, FS_SEL_A>;
#[doc = "N/A\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FS_SEL_A {
    #[doc = "0: N/A"]
    RSVD = 0,
    #[doc = "1: N/A"]
    _256FS_128OSR = 1,
    #[doc = "2: N/A"]
    _256FS_256OSR = 2,
    #[doc = "3: N/A"]
    _512FS_512OSR = 3,
}
impl From<FS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_SEL_A) -> Self {
        variant as _
    }
}
impl FS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_SEL_A {
        match self.bits {
            0 => FS_SEL_A::RSVD,
            1 => FS_SEL_A::_256FS_128OSR,
            2 => FS_SEL_A::_256FS_256OSR,
            3 => FS_SEL_A::_512FS_512OSR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == FS_SEL_A::RSVD
    }
    #[doc = "Checks if the value of the field is `_256FS_128OSR`"]
    #[inline(always)]
    pub fn is_256fs_128osr(&self) -> bool {
        *self == FS_SEL_A::_256FS_128OSR
    }
    #[doc = "Checks if the value of the field is `_256FS_256OSR`"]
    #[inline(always)]
    pub fn is_256fs_256osr(&self) -> bool {
        *self == FS_SEL_A::_256FS_256OSR
    }
    #[doc = "Checks if the value of the field is `_512FS_512OSR`"]
    #[inline(always)]
    pub fn is_512fs_512osr(&self) -> bool {
        *self == FS_SEL_A::_512FS_512OSR
    }
}
#[doc = "Field `FS_SEL` writer - N/A"]
pub type FS_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IF_CTL_SPEC, u8, FS_SEL_A, 2, O>;
impl<'a, const O: u8> FS_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(FS_SEL_A::RSVD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn _256fs_128osr(self) -> &'a mut W {
        self.variant(FS_SEL_A::_256FS_128OSR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn _256fs_256osr(self) -> &'a mut W {
        self.variant(FS_SEL_A::_256FS_256OSR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn _512fs_512osr(self) -> &'a mut W {
        self.variant(FS_SEL_A::_512FS_512OSR)
    }
}
#[doc = "Field `LDATA_POLARITY` reader - Left Data polarity. '0' - No change in Left data. '1' - Invert (2's complement) Left data."]
pub type LDATA_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `LDATA_POLARITY` writer - Left Data polarity. '0' - No change in Left data. '1' - Invert (2's complement) Left data."]
pub type LDATA_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_CTL_SPEC, bool, O>;
#[doc = "Field `RDATA_POLARITY` reader - Right Data polarity. '0' - No change in Right data. '1' - Invert (2's complement) Right data."]
pub type RDATA_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `RDATA_POLARITY` writer - Right Data polarity. '0' - No change in Right data. '1' - Invert (2's complement) Right data."]
pub type RDATA_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_CTL_SPEC, bool, O>;
#[doc = "Field `LDATA_SEL` reader - Left Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). Note: LDATA_SEL and RDATA_SEL are used to specify mono and stereo audio formats."]
pub type LDATA_SEL_R = crate::BitReader<LDATA_SEL_A>;
#[doc = "Left Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). Note: LDATA_SEL and RDATA_SEL are used to specify mono and stereo audio formats.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDATA_SEL_A {
    #[doc = "0: N/A"]
    DATA_LOW = 0,
    #[doc = "1: N/A"]
    DATA_HIGH = 1,
}
impl From<LDATA_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LDATA_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LDATA_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDATA_SEL_A {
        match self.bits {
            false => LDATA_SEL_A::DATA_LOW,
            true => LDATA_SEL_A::DATA_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_LOW`"]
    #[inline(always)]
    pub fn is_data_low(&self) -> bool {
        *self == LDATA_SEL_A::DATA_LOW
    }
    #[doc = "Checks if the value of the field is `DATA_HIGH`"]
    #[inline(always)]
    pub fn is_data_high(&self) -> bool {
        *self == LDATA_SEL_A::DATA_HIGH
    }
}
#[doc = "Field `LDATA_SEL` writer - Left Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). Note: LDATA_SEL and RDATA_SEL are used to specify mono and stereo audio formats."]
pub type LDATA_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_CTL_SPEC, LDATA_SEL_A, O>;
impl<'a, const O: u8> LDATA_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_low(self) -> &'a mut W {
        self.variant(LDATA_SEL_A::DATA_LOW)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_high(self) -> &'a mut W {
        self.variant(LDATA_SEL_A::DATA_HIGH)
    }
}
#[doc = "Field `RDATA_SEL` reader - Right Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]."]
pub type RDATA_SEL_R = crate::BitReader<RDATA_SEL_A>;
#[doc = "Right Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDATA_SEL_A {
    #[doc = "0: N/A"]
    DATA_LOW = 0,
    #[doc = "1: N/A"]
    DATA_HIGH = 1,
}
impl From<RDATA_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: RDATA_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RDATA_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDATA_SEL_A {
        match self.bits {
            false => RDATA_SEL_A::DATA_LOW,
            true => RDATA_SEL_A::DATA_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_LOW`"]
    #[inline(always)]
    pub fn is_data_low(&self) -> bool {
        *self == RDATA_SEL_A::DATA_LOW
    }
    #[doc = "Checks if the value of the field is `DATA_HIGH`"]
    #[inline(always)]
    pub fn is_data_high(&self) -> bool {
        *self == RDATA_SEL_A::DATA_HIGH
    }
}
#[doc = "Field `RDATA_SEL` writer - Right Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]."]
pub type RDATA_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_CTL_SPEC, RDATA_SEL_A, O>;
impl<'a, const O: u8> RDATA_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_low(self) -> &'a mut W {
        self.variant(RDATA_SEL_A::DATA_LOW)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn data_high(self) -> &'a mut W {
        self.variant(RDATA_SEL_A::DATA_HIGH)
    }
}
#[doc = "Field `SW_OVERRIDE_FAST_RAMP_EN` reader - SW Override Fast Ramp enable. '0' - No SW override for fast ramp enable. HW controls the fast ramp enable. '1' - SW override for fast ramp enable. HW control is disabled. SW set the Fast Ramp enable before DAC_EN is set to 1. When SW override is used, 1. SW needs to disable the fast ramp enable by clearing this bit after the fast ramp up circuit is complete i.e. 10ms after setting DAC_EN. 2. After that, SW needs to ensure the DAC's stabilization time is met (i.e. 60ms after disabling the fast ramp circuit) before starting transactions to DAC."]
pub type SW_OVERRIDE_FAST_RAMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SW_OVERRIDE_FAST_RAMP_EN` writer - SW Override Fast Ramp enable. '0' - No SW override for fast ramp enable. HW controls the fast ramp enable. '1' - SW override for fast ramp enable. HW control is disabled. SW set the Fast Ramp enable before DAC_EN is set to 1. When SW override is used, 1. SW needs to disable the fast ramp enable by clearing this bit after the fast ramp up circuit is complete i.e. 10ms after setting DAC_EN. 2. After that, SW needs to ensure the DAC's stabilization time is met (i.e. 60ms after disabling the fast ramp circuit) before starting transactions to DAC."]
pub type SW_OVERRIDE_FAST_RAMP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IF_CTL_SPEC, bool, O>;
#[doc = "Field `DAC_EN` reader - DAC Enable. '0' - DAC Analog is disabled. '1' - DAC Analog is enabled. The analog DAC output pins will be enabled. Note that after enabling the DAC Analog circuit, it requires 70ms to be stable. The same ramp up time applies when waking up from DeepSleep. DACEN is only valid when ENABLED=1."]
pub type DAC_EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC_EN` writer - DAC Enable. '0' - DAC Analog is disabled. '1' - DAC Analog is enabled. The analog DAC output pins will be enabled. Note that after enabling the DAC Analog circuit, it requires 70ms to be stable. The same ramp up time applies when waking up from DeepSleep. DACEN is only valid when ENABLED=1."]
pub type DAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Interface clock divider (legal range \\[0, 255\\]). The DAC system clock clk_dac is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value (\\[1,3,5,...255\\]) or 0 (divide by 1), to ensure a 50/50 percent duty cycle clock output. '0': clk_dac frequency is clk_if frequency (1 clk_dac cycle consists of 1 clk_if cycles). '1': clk_dac frequency is 1/2 clk_if frequency (1 clk_dac cycle consists of 2 clk_if cycles). '2': clk_dac frequency is 1/3 clk_if frequency. Note: results in a non 50/50 percent duty cycle. ... '255': clk_dac frequency is 1/256 clk_if frequency. Note: This field can only be changed when CLOCK_SEL= clk_if off."]
    #[inline(always)]
    pub fn clock_div(&self) -> CLOCK_DIV_R {
        CLOCK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Interface clock clk_if selection: '0': SRSS clock clk_if_srss. '1'-'3': Clock 'clk_if' is off. '4': Master interface clock 'dac_mck'. '5'-'7': Clock 'clk_if' is off."]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn fs_sel(&self) -> FS_SEL_R {
        FS_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Left Data polarity. '0' - No change in Left data. '1' - Invert (2's complement) Left data."]
    #[inline(always)]
    pub fn ldata_polarity(&self) -> LDATA_POLARITY_R {
        LDATA_POLARITY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Right Data polarity. '0' - No change in Right data. '1' - Invert (2's complement) Right data."]
    #[inline(always)]
    pub fn rdata_polarity(&self) -> RDATA_POLARITY_R {
        RDATA_POLARITY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Left Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). Note: LDATA_SEL and RDATA_SEL are used to specify mono and stereo audio formats."]
    #[inline(always)]
    pub fn ldata_sel(&self) -> LDATA_SEL_R {
        LDATA_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Right Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]."]
    #[inline(always)]
    pub fn rdata_sel(&self) -> RDATA_SEL_R {
        RDATA_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - SW Override Fast Ramp enable. '0' - No SW override for fast ramp enable. HW controls the fast ramp enable. '1' - SW override for fast ramp enable. HW control is disabled. SW set the Fast Ramp enable before DAC_EN is set to 1. When SW override is used, 1. SW needs to disable the fast ramp enable by clearing this bit after the fast ramp up circuit is complete i.e. 10ms after setting DAC_EN. 2. After that, SW needs to ensure the DAC's stabilization time is met (i.e. 60ms after disabling the fast ramp circuit) before starting transactions to DAC."]
    #[inline(always)]
    pub fn sw_override_fast_ramp_en(&self) -> SW_OVERRIDE_FAST_RAMP_EN_R {
        SW_OVERRIDE_FAST_RAMP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC Enable. '0' - DAC Analog is disabled. '1' - DAC Analog is enabled. The analog DAC output pins will be enabled. Note that after enabling the DAC Analog circuit, it requires 70ms to be stable. The same ramp up time applies when waking up from DeepSleep. DACEN is only valid when ENABLED=1."]
    #[inline(always)]
    pub fn dac_en(&self) -> DAC_EN_R {
        DAC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interface clock divider (legal range \\[0, 255\\]). The DAC system clock clk_dac is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value (\\[1,3,5,...255\\]) or 0 (divide by 1), to ensure a 50/50 percent duty cycle clock output. '0': clk_dac frequency is clk_if frequency (1 clk_dac cycle consists of 1 clk_if cycles). '1': clk_dac frequency is 1/2 clk_if frequency (1 clk_dac cycle consists of 2 clk_if cycles). '2': clk_dac frequency is 1/3 clk_if frequency. Note: results in a non 50/50 percent duty cycle. ... '255': clk_dac frequency is 1/256 clk_if frequency. Note: This field can only be changed when CLOCK_SEL= clk_if off."]
    #[inline(always)]
    #[must_use]
    pub fn clock_div(&mut self) -> CLOCK_DIV_W<0> {
        CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 8:10 - Interface clock clk_if selection: '0': SRSS clock clk_if_srss. '1'-'3': Clock 'clk_if' is off. '4': Master interface clock 'dac_mck'. '5'-'7': Clock 'clk_if' is off."]
    #[inline(always)]
    #[must_use]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W<8> {
        CLOCK_SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs_sel(&mut self) -> FS_SEL_W<16> {
        FS_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Left Data polarity. '0' - No change in Left data. '1' - Invert (2's complement) Left data."]
    #[inline(always)]
    #[must_use]
    pub fn ldata_polarity(&mut self) -> LDATA_POLARITY_W<24> {
        LDATA_POLARITY_W::new(self)
    }
    #[doc = "Bit 25 - Right Data polarity. '0' - No change in Right data. '1' - Invert (2's complement) Right data."]
    #[inline(always)]
    #[must_use]
    pub fn rdata_polarity(&mut self) -> RDATA_POLARITY_W<25> {
        RDATA_POLARITY_W::new(self)
    }
    #[doc = "Bit 26 - Left Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]. '1': data\\[31:16\\]
(default setting). Note: LDATA_SEL and RDATA_SEL are used to specify mono and stereo audio formats."]
    #[inline(always)]
    #[must_use]
    pub fn ldata_sel(&mut self) -> LDATA_SEL_W<26> {
        LDATA_SEL_W::new(self)
    }
    #[doc = "Bit 27 - Right Data Selection. Specifies selection of the channel's 16-bit PCM data from the source FIFO data\\[31:0\\]: '0': data\\[15:0\\]
(default setting). '1': data\\[31:16\\]."]
    #[inline(always)]
    #[must_use]
    pub fn rdata_sel(&mut self) -> RDATA_SEL_W<27> {
        RDATA_SEL_W::new(self)
    }
    #[doc = "Bit 30 - SW Override Fast Ramp enable. '0' - No SW override for fast ramp enable. HW controls the fast ramp enable. '1' - SW override for fast ramp enable. HW control is disabled. SW set the Fast Ramp enable before DAC_EN is set to 1. When SW override is used, 1. SW needs to disable the fast ramp enable by clearing this bit after the fast ramp up circuit is complete i.e. 10ms after setting DAC_EN. 2. After that, SW needs to ensure the DAC's stabilization time is met (i.e. 60ms after disabling the fast ramp circuit) before starting transactions to DAC."]
    #[inline(always)]
    #[must_use]
    pub fn sw_override_fast_ramp_en(&mut self) -> SW_OVERRIDE_FAST_RAMP_EN_W<30> {
        SW_OVERRIDE_FAST_RAMP_EN_W::new(self)
    }
    #[doc = "Bit 31 - DAC Enable. '0' - DAC Analog is disabled. '1' - DAC Analog is enabled. The analog DAC output pins will be enabled. Note that after enabling the DAC Analog circuit, it requires 70ms to be stable. The same ramp up time applies when waking up from DeepSleep. DACEN is only valid when ENABLED=1."]
    #[inline(always)]
    #[must_use]
    pub fn dac_en(&mut self) -> DAC_EN_W<31> {
        DAC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_ctl](index.html) module"]
pub struct IF_CTL_SPEC;
impl crate::RegisterSpec for IF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_ctl::R](R) reader structure"]
impl crate::Readable for IF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_ctl::W](W) writer structure"]
impl crate::Writable for IF_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF_CTL to value 0x0402_0007"]
impl crate::Resettable for IF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0402_0007;
}
