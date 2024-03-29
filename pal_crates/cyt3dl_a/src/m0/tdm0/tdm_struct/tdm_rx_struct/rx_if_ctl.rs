#[doc = "Register `RX_IF_CTL` reader"]
pub struct R(crate::R<RX_IF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_IF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_IF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_IF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_IF_CTL` writer"]
pub struct W(crate::W<RX_IF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_IF_CTL_SPEC>;
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
impl From<crate::W<RX_IF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_IF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_DIV` reader - Interface clock divider (legal range \\[1, 255\\]). The TDM interface 'tdm_rx_sck_out' output signals is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value ({1, 3, 5, ..., 255}), to ensure a 50/50 percent duty cycle clock. Note: Used in master configuration only. Note: Due to delays in the IO SubSystem (specifically the IO cells), the maximum TDM interface clock 'tdm_rx_sck_out' may be restricted. As a result, the maximum bitrate is less than the theoretical maximum (32 channels, 32 bits per channel and a high Fs of e.g. 48 kHz). This restriction is most applicable to the master, receiver configuration."]
pub type CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_DIV` writer - Interface clock divider (legal range \\[1, 255\\]). The TDM interface 'tdm_rx_sck_out' output signals is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value ({1, 3, 5, ..., 255}), to ensure a 50/50 percent duty cycle clock. Note: Used in master configuration only. Note: Due to delays in the IO SubSystem (specifically the IO cells), the maximum TDM interface clock 'tdm_rx_sck_out' may be restricted. As a result, the maximum bitrate is less than the theoretical maximum (32 channels, 32 bits per channel and a high Fs of e.g. 48 kHz). This restriction is most applicable to the master, receiver configuration."]
pub type CLOCK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_IF_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLOCK_SEL` reader - Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'tdm_rx_mck_in'. '5'-'7': undefined. Note: the application is always required to program this field to a value different from the default."]
pub type CLOCK_SEL_R = crate::FieldReader<u8, CLOCK_SEL_A>;
#[doc = "Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'tdm_rx_mck_in'. '5'-'7': undefined. Note: the application is always required to program this field to a value different from the default.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLOCK_SEL_A {
    #[doc = "0: N/A"]
    SEL_SRSS_CLOCK0 = 0,
    #[doc = "1: N/A"]
    SEL_SRSS_CLOCK1 = 1,
    #[doc = "2: N/A"]
    SEL_SRSS_CLOCK2 = 2,
    #[doc = "3: N/A"]
    SEL_SRSS_CLOCK3 = 3,
    #[doc = "4: N/A"]
    SEL_TDM_RX_MCK_IN = 4,
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
            0 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK0),
            1 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK1),
            2 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK2),
            3 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK3),
            4 => Some(CLOCK_SEL_A::SEL_TDM_RX_MCK_IN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK0`"]
    #[inline(always)]
    pub fn is_sel_srss_clock0(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK0
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK1`"]
    #[inline(always)]
    pub fn is_sel_srss_clock1(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK1
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK2`"]
    #[inline(always)]
    pub fn is_sel_srss_clock2(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK2
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK3`"]
    #[inline(always)]
    pub fn is_sel_srss_clock3(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK3
    }
    #[doc = "Checks if the value of the field is `SEL_TDM_RX_MCK_IN`"]
    #[inline(always)]
    pub fn is_sel_tdm_rx_mck_in(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_TDM_RX_MCK_IN
    }
}
#[doc = "Field `CLOCK_SEL` writer - Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'tdm_rx_mck_in'. '5'-'7': undefined. Note: the application is always required to program this field to a value different from the default."]
pub type CLOCK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_IF_CTL_SPEC, u8, CLOCK_SEL_A, 3, O>;
impl<'a, const O: u8> CLOCK_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock0(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock1(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock2(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock3(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK3)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_tdm_rx_mck_in(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_TDM_RX_MCK_IN)
    }
}
#[doc = "Field `SCK_POLARITY` reader - Clock polarity: '0': Clock signal is used 'as is'. '1': Clock signal is inverted. Note: Used in BOTH master and slave configurations."]
pub type SCK_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `SCK_POLARITY` writer - Clock polarity: '0': Clock signal is used 'as is'. '1': Clock signal is inverted. Note: Used in BOTH master and slave configurations."]
pub type SCK_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_IF_CTL_SPEC, bool, O>;
#[doc = "Field `FSYNC_POLARITY` reader - Channel synchronization polarity: '0': Channel synchronization signal is used 'as is'. '1': Channel synchronization signal is inverted. Note: Used in BOTH master and slave configurations."]
pub type FSYNC_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `FSYNC_POLARITY` writer - Channel synchronization polarity: '0': Channel synchronization signal is used 'as is'. '1': Channel synchronization signal is inverted. Note: Used in BOTH master and slave configurations."]
pub type FSYNC_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_IF_CTL_SPEC, bool, O>;
#[doc = "Field `LATE_SAMPLE` reader - Interface late sample sample delay: Slave configuration (RX_CTL.MS is '0'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_in'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_in' (half a cycle delay). Master configuration (RX_CTL.MS is '1'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_out'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_out' (half a cycle delay). Note: This field can be set to '1' when the roundtrip delay is large (typically) in a master receiver configuration."]
pub type LATE_SAMPLE_R = crate::BitReader<LATE_SAMPLE_A>;
#[doc = "Interface late sample sample delay: Slave configuration (RX_CTL.MS is '0'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_in'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_in' (half a cycle delay). Master configuration (RX_CTL.MS is '1'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_out'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_out' (half a cycle delay). Note: This field can be set to '1' when the roundtrip delay is large (typically) in a master receiver configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LATE_SAMPLE_A {
    #[doc = "0: N/A"]
    RISING = 0,
    #[doc = "1: N/A"]
    FALLING = 1,
}
impl From<LATE_SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: LATE_SAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LATE_SAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATE_SAMPLE_A {
        match self.bits {
            false => LATE_SAMPLE_A::RISING,
            true => LATE_SAMPLE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LATE_SAMPLE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LATE_SAMPLE_A::FALLING
    }
}
#[doc = "Field `LATE_SAMPLE` writer - Interface late sample sample delay: Slave configuration (RX_CTL.MS is '0'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_in'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_in' (half a cycle delay). Master configuration (RX_CTL.MS is '1'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_out'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_out' (half a cycle delay). Note: This field can be set to '1' when the roundtrip delay is large (typically) in a master receiver configuration."]
pub type LATE_SAMPLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_IF_CTL_SPEC, LATE_SAMPLE_A, O>;
impl<'a, const O: u8> LATE_SAMPLE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LATE_SAMPLE_A::RISING)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LATE_SAMPLE_A::FALLING)
    }
}
#[doc = "Field `FSYNC_FORMAT` reader - Channel synchronization pulse format: '0': Duration of a single bit period. '1': Duration of the first channel."]
pub type FSYNC_FORMAT_R = crate::BitReader<FSYNC_FORMAT_A>;
#[doc = "Channel synchronization pulse format: '0': Duration of a single bit period. '1': Duration of the first channel.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSYNC_FORMAT_A {
    #[doc = "0: N/A"]
    BIT_PERIOD = 0,
    #[doc = "1: N/A"]
    CH_PERIOD = 1,
}
impl From<FSYNC_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: FSYNC_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl FSYNC_FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSYNC_FORMAT_A {
        match self.bits {
            false => FSYNC_FORMAT_A::BIT_PERIOD,
            true => FSYNC_FORMAT_A::CH_PERIOD,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_PERIOD`"]
    #[inline(always)]
    pub fn is_bit_period(&self) -> bool {
        *self == FSYNC_FORMAT_A::BIT_PERIOD
    }
    #[doc = "Checks if the value of the field is `CH_PERIOD`"]
    #[inline(always)]
    pub fn is_ch_period(&self) -> bool {
        *self == FSYNC_FORMAT_A::CH_PERIOD
    }
}
#[doc = "Field `FSYNC_FORMAT` writer - Channel synchronization pulse format: '0': Duration of a single bit period. '1': Duration of the first channel."]
pub type FSYNC_FORMAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_IF_CTL_SPEC, FSYNC_FORMAT_A, O>;
impl<'a, const O: u8> FSYNC_FORMAT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn bit_period(self) -> &'a mut W {
        self.variant(FSYNC_FORMAT_A::BIT_PERIOD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_period(self) -> &'a mut W {
        self.variant(FSYNC_FORMAT_A::CH_PERIOD)
    }
}
#[doc = "Field `CH_NR` reader - Number of channels in the frame: '0': Undefined/illegal. '1': 2 channels. '2': 3 channels. ... '31': 32 channels. Note: the field value chould be less than CH_NR (the number of support channels). Note: the RX_CH_CTL.CH_EN fields can be used to enable/disable indvidual channels."]
pub type CH_NR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_NR` writer - Number of channels in the frame: '0': Undefined/illegal. '1': 2 channels. '2': 3 channels. ... '31': 32 channels. Note: the field value chould be less than CH_NR (the number of support channels). Note: the RX_CH_CTL.CH_EN fields can be used to enable/disable indvidual channels."]
pub type CH_NR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_IF_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH_SIZE` reader - Channel size: '0'-'2': Undefined/illegal. '3': 4 bits. ... '31': 32 bits. Note: if RX_CTL.WORD_SIZE is greater than CH_SIZE, the lesser significant bits of the word are filled with '0's."]
pub type CH_SIZE_R = crate::FieldReader<u8, CH_SIZE_A>;
#[doc = "Channel size: '0'-'2': Undefined/illegal. '3': 4 bits. ... '31': 32 bits. Note: if RX_CTL.WORD_SIZE is greater than CH_SIZE, the lesser significant bits of the word are filled with '0's.\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH_SIZE_A {
    #[doc = "0: N/A"]
    SIZE_1 = 0,
    #[doc = "1: N/A"]
    SIZE_2 = 1,
    #[doc = "31: N/A"]
    SIZE_32 = 31,
}
impl From<CH_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_SIZE_A) -> Self {
        variant as _
    }
}
impl CH_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH_SIZE_A> {
        match self.bits {
            0 => Some(CH_SIZE_A::SIZE_1),
            1 => Some(CH_SIZE_A::SIZE_2),
            31 => Some(CH_SIZE_A::SIZE_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_1`"]
    #[inline(always)]
    pub fn is_size_1(&self) -> bool {
        *self == CH_SIZE_A::SIZE_1
    }
    #[doc = "Checks if the value of the field is `SIZE_2`"]
    #[inline(always)]
    pub fn is_size_2(&self) -> bool {
        *self == CH_SIZE_A::SIZE_2
    }
    #[doc = "Checks if the value of the field is `SIZE_32`"]
    #[inline(always)]
    pub fn is_size_32(&self) -> bool {
        *self == CH_SIZE_A::SIZE_32
    }
}
#[doc = "Field `CH_SIZE` writer - Channel size: '0'-'2': Undefined/illegal. '3': 4 bits. ... '31': 32 bits. Note: if RX_CTL.WORD_SIZE is greater than CH_SIZE, the lesser significant bits of the word are filled with '0's."]
pub type CH_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_IF_CTL_SPEC, u8, CH_SIZE_A, 5, O>;
impl<'a, const O: u8> CH_SIZE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn size_1(self) -> &'a mut W {
        self.variant(CH_SIZE_A::SIZE_1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn size_2(self) -> &'a mut W {
        self.variant(CH_SIZE_A::SIZE_2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn size_32(self) -> &'a mut W {
        self.variant(CH_SIZE_A::SIZE_32)
    }
}
#[doc = "Field `LATE_CAPTURE` reader - Extra delay (in 'rx_sck_out' cycles) for capturing 'tdm_rx_sd_in': '0': no extra delay '1': 1 cycle extra delay '2': 2 cycles extra delay '3': 3 cycles extra delay Note: the value of this field pushes further out the capturing edges used by the receiver to sample 'tdm_rx_sd_in'. This function is intended to support very large round-trip delays in a master receiver configuration, where the delay at the receiver between 'tdm_rx_fsync_out' and the arrival of the first bit on 'tdm_rx_sd_in' is multiple clock cycles."]
pub type LATE_CAPTURE_R = crate::FieldReader<u8, LATE_CAPTURE_A>;
#[doc = "Extra delay (in 'rx_sck_out' cycles) for capturing 'tdm_rx_sd_in': '0': no extra delay '1': 1 cycle extra delay '2': 2 cycles extra delay '3': 3 cycles extra delay Note: the value of this field pushes further out the capturing edges used by the receiver to sample 'tdm_rx_sd_in'. This function is intended to support very large round-trip delays in a master receiver configuration, where the delay at the receiver between 'tdm_rx_fsync_out' and the arrival of the first bit on 'tdm_rx_sd_in' is multiple clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATE_CAPTURE_A {
    #[doc = "0: N/A"]
    EXTRA_DELAY_0 = 0,
    #[doc = "1: N/A"]
    EXTRA_DELAY_1 = 1,
    #[doc = "2: N/A"]
    EXTRA_DELAY_2 = 2,
    #[doc = "3: N/A"]
    EXTRA_DELAY_3 = 3,
}
impl From<LATE_CAPTURE_A> for u8 {
    #[inline(always)]
    fn from(variant: LATE_CAPTURE_A) -> Self {
        variant as _
    }
}
impl LATE_CAPTURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATE_CAPTURE_A {
        match self.bits {
            0 => LATE_CAPTURE_A::EXTRA_DELAY_0,
            1 => LATE_CAPTURE_A::EXTRA_DELAY_1,
            2 => LATE_CAPTURE_A::EXTRA_DELAY_2,
            3 => LATE_CAPTURE_A::EXTRA_DELAY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTRA_DELAY_0`"]
    #[inline(always)]
    pub fn is_extra_delay_0(&self) -> bool {
        *self == LATE_CAPTURE_A::EXTRA_DELAY_0
    }
    #[doc = "Checks if the value of the field is `EXTRA_DELAY_1`"]
    #[inline(always)]
    pub fn is_extra_delay_1(&self) -> bool {
        *self == LATE_CAPTURE_A::EXTRA_DELAY_1
    }
    #[doc = "Checks if the value of the field is `EXTRA_DELAY_2`"]
    #[inline(always)]
    pub fn is_extra_delay_2(&self) -> bool {
        *self == LATE_CAPTURE_A::EXTRA_DELAY_2
    }
    #[doc = "Checks if the value of the field is `EXTRA_DELAY_3`"]
    #[inline(always)]
    pub fn is_extra_delay_3(&self) -> bool {
        *self == LATE_CAPTURE_A::EXTRA_DELAY_3
    }
}
#[doc = "Field `LATE_CAPTURE` writer - Extra delay (in 'rx_sck_out' cycles) for capturing 'tdm_rx_sd_in': '0': no extra delay '1': 1 cycle extra delay '2': 2 cycles extra delay '3': 3 cycles extra delay Note: the value of this field pushes further out the capturing edges used by the receiver to sample 'tdm_rx_sd_in'. This function is intended to support very large round-trip delays in a master receiver configuration, where the delay at the receiver between 'tdm_rx_fsync_out' and the arrival of the first bit on 'tdm_rx_sd_in' is multiple clock cycles."]
pub type LATE_CAPTURE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RX_IF_CTL_SPEC, u8, LATE_CAPTURE_A, 2, O>;
impl<'a, const O: u8> LATE_CAPTURE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn extra_delay_0(self) -> &'a mut W {
        self.variant(LATE_CAPTURE_A::EXTRA_DELAY_0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn extra_delay_1(self) -> &'a mut W {
        self.variant(LATE_CAPTURE_A::EXTRA_DELAY_1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn extra_delay_2(self) -> &'a mut W {
        self.variant(LATE_CAPTURE_A::EXTRA_DELAY_2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn extra_delay_3(self) -> &'a mut W {
        self.variant(LATE_CAPTURE_A::EXTRA_DELAY_3)
    }
}
#[doc = "Field `I2S_MODE` reader - I2S mode setting: '0': TDM mode. '1': I2S mode."]
pub type I2S_MODE_R = crate::BitReader<I2S_MODE_A>;
#[doc = "I2S mode setting: '0': TDM mode. '1': I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_MODE_A {
    #[doc = "0: N/A"]
    TDM = 0,
    #[doc = "1: N/A"]
    I2S = 1,
}
impl From<I2S_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MODE_A {
        match self.bits {
            false => I2S_MODE_A::TDM,
            true => I2S_MODE_A::I2S,
        }
    }
    #[doc = "Checks if the value of the field is `TDM`"]
    #[inline(always)]
    pub fn is_tdm(&self) -> bool {
        *self == I2S_MODE_A::TDM
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == I2S_MODE_A::I2S
    }
}
#[doc = "Field `I2S_MODE` writer - I2S mode setting: '0': TDM mode. '1': I2S mode."]
pub type I2S_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_IF_CTL_SPEC, I2S_MODE_A, O>;
impl<'a, const O: u8> I2S_MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tdm(self) -> &'a mut W {
        self.variant(I2S_MODE_A::TDM)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(I2S_MODE_A::I2S)
    }
}
impl R {
    #[doc = "Bits 0:7 - Interface clock divider (legal range \\[1, 255\\]). The TDM interface 'tdm_rx_sck_out' output signals is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value ({1, 3, 5, ..., 255}), to ensure a 50/50 percent duty cycle clock. Note: Used in master configuration only. Note: Due to delays in the IO SubSystem (specifically the IO cells), the maximum TDM interface clock 'tdm_rx_sck_out' may be restricted. As a result, the maximum bitrate is less than the theoretical maximum (32 channels, 32 bits per channel and a high Fs of e.g. 48 kHz). This restriction is most applicable to the master, receiver configuration."]
    #[inline(always)]
    pub fn clock_div(&self) -> CLOCK_DIV_R {
        CLOCK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'tdm_rx_mck_in'. '5'-'7': undefined. Note: the application is always required to program this field to a value different from the default."]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Clock polarity: '0': Clock signal is used 'as is'. '1': Clock signal is inverted. Note: Used in BOTH master and slave configurations."]
    #[inline(always)]
    pub fn sck_polarity(&self) -> SCK_POLARITY_R {
        SCK_POLARITY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel synchronization polarity: '0': Channel synchronization signal is used 'as is'. '1': Channel synchronization signal is inverted. Note: Used in BOTH master and slave configurations."]
    #[inline(always)]
    pub fn fsync_polarity(&self) -> FSYNC_POLARITY_R {
        FSYNC_POLARITY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interface late sample sample delay: Slave configuration (RX_CTL.MS is '0'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_in'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_in' (half a cycle delay). Master configuration (RX_CTL.MS is '1'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_out'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_out' (half a cycle delay). Note: This field can be set to '1' when the roundtrip delay is large (typically) in a master receiver configuration."]
    #[inline(always)]
    pub fn late_sample(&self) -> LATE_SAMPLE_R {
        LATE_SAMPLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel synchronization pulse format: '0': Duration of a single bit period. '1': Duration of the first channel."]
    #[inline(always)]
    pub fn fsync_format(&self) -> FSYNC_FORMAT_R {
        FSYNC_FORMAT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Number of channels in the frame: '0': Undefined/illegal. '1': 2 channels. '2': 3 channels. ... '31': 32 channels. Note: the field value chould be less than CH_NR (the number of support channels). Note: the RX_CH_CTL.CH_EN fields can be used to enable/disable indvidual channels."]
    #[inline(always)]
    pub fn ch_nr(&self) -> CH_NR_R {
        CH_NR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Channel size: '0'-'2': Undefined/illegal. '3': 4 bits. ... '31': 32 bits. Note: if RX_CTL.WORD_SIZE is greater than CH_SIZE, the lesser significant bits of the word are filled with '0's."]
    #[inline(always)]
    pub fn ch_size(&self) -> CH_SIZE_R {
        CH_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - Extra delay (in 'rx_sck_out' cycles) for capturing 'tdm_rx_sd_in': '0': no extra delay '1': 1 cycle extra delay '2': 2 cycles extra delay '3': 3 cycles extra delay Note: the value of this field pushes further out the capturing edges used by the receiver to sample 'tdm_rx_sd_in'. This function is intended to support very large round-trip delays in a master receiver configuration, where the delay at the receiver between 'tdm_rx_fsync_out' and the arrival of the first bit on 'tdm_rx_sd_in' is multiple clock cycles."]
    #[inline(always)]
    pub fn late_capture(&self) -> LATE_CAPTURE_R {
        LATE_CAPTURE_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - I2S mode setting: '0': TDM mode. '1': I2S mode."]
    #[inline(always)]
    pub fn i2s_mode(&self) -> I2S_MODE_R {
        I2S_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interface clock divider (legal range \\[1, 255\\]). The TDM interface 'tdm_rx_sck_out' output signals is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value ({1, 3, 5, ..., 255}), to ensure a 50/50 percent duty cycle clock. Note: Used in master configuration only. Note: Due to delays in the IO SubSystem (specifically the IO cells), the maximum TDM interface clock 'tdm_rx_sck_out' may be restricted. As a result, the maximum bitrate is less than the theoretical maximum (32 channels, 32 bits per channel and a high Fs of e.g. 48 kHz). This restriction is most applicable to the master, receiver configuration."]
    #[inline(always)]
    #[must_use]
    pub fn clock_div(&mut self) -> CLOCK_DIV_W<0> {
        CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 8:10 - Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'tdm_rx_mck_in'. '5'-'7': undefined. Note: the application is always required to program this field to a value different from the default."]
    #[inline(always)]
    #[must_use]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W<8> {
        CLOCK_SEL_W::new(self)
    }
    #[doc = "Bit 12 - Clock polarity: '0': Clock signal is used 'as is'. '1': Clock signal is inverted. Note: Used in BOTH master and slave configurations."]
    #[inline(always)]
    #[must_use]
    pub fn sck_polarity(&mut self) -> SCK_POLARITY_W<12> {
        SCK_POLARITY_W::new(self)
    }
    #[doc = "Bit 13 - Channel synchronization polarity: '0': Channel synchronization signal is used 'as is'. '1': Channel synchronization signal is inverted. Note: Used in BOTH master and slave configurations."]
    #[inline(always)]
    #[must_use]
    pub fn fsync_polarity(&mut self) -> FSYNC_POLARITY_W<13> {
        FSYNC_POLARITY_W::new(self)
    }
    #[doc = "Bit 14 - Interface late sample sample delay: Slave configuration (RX_CTL.MS is '0'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_in'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_in' (half a cycle delay). Master configuration (RX_CTL.MS is '1'). '0': Sample PCM bit value on rising edge (SCK_POLARITY is '0') or falling edge (SCK_POLARITY is '1') of receiver 'rx_sck_out'. '1': Sample PCM bit value on falling edge (SCK_POLARITY is '0') or rising edge (SCK_POLARITY is '1') of receiver 'rx_sck_out' (half a cycle delay). Note: This field can be set to '1' when the roundtrip delay is large (typically) in a master receiver configuration."]
    #[inline(always)]
    #[must_use]
    pub fn late_sample(&mut self) -> LATE_SAMPLE_W<14> {
        LATE_SAMPLE_W::new(self)
    }
    #[doc = "Bit 15 - Channel synchronization pulse format: '0': Duration of a single bit period. '1': Duration of the first channel."]
    #[inline(always)]
    #[must_use]
    pub fn fsync_format(&mut self) -> FSYNC_FORMAT_W<15> {
        FSYNC_FORMAT_W::new(self)
    }
    #[doc = "Bits 16:20 - Number of channels in the frame: '0': Undefined/illegal. '1': 2 channels. '2': 3 channels. ... '31': 32 channels. Note: the field value chould be less than CH_NR (the number of support channels). Note: the RX_CH_CTL.CH_EN fields can be used to enable/disable indvidual channels."]
    #[inline(always)]
    #[must_use]
    pub fn ch_nr(&mut self) -> CH_NR_W<16> {
        CH_NR_W::new(self)
    }
    #[doc = "Bits 24:28 - Channel size: '0'-'2': Undefined/illegal. '3': 4 bits. ... '31': 32 bits. Note: if RX_CTL.WORD_SIZE is greater than CH_SIZE, the lesser significant bits of the word are filled with '0's."]
    #[inline(always)]
    #[must_use]
    pub fn ch_size(&mut self) -> CH_SIZE_W<24> {
        CH_SIZE_W::new(self)
    }
    #[doc = "Bits 29:30 - Extra delay (in 'rx_sck_out' cycles) for capturing 'tdm_rx_sd_in': '0': no extra delay '1': 1 cycle extra delay '2': 2 cycles extra delay '3': 3 cycles extra delay Note: the value of this field pushes further out the capturing edges used by the receiver to sample 'tdm_rx_sd_in'. This function is intended to support very large round-trip delays in a master receiver configuration, where the delay at the receiver between 'tdm_rx_fsync_out' and the arrival of the first bit on 'tdm_rx_sd_in' is multiple clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn late_capture(&mut self) -> LATE_CAPTURE_W<29> {
        LATE_CAPTURE_W::new(self)
    }
    #[doc = "Bit 31 - I2S mode setting: '0': TDM mode. '1': I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_mode(&mut self) -> I2S_MODE_W<31> {
        I2S_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX interface control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_if_ctl](index.html) module"]
pub struct RX_IF_CTL_SPEC;
impl crate::RegisterSpec for RX_IF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_if_ctl::R](R) reader structure"]
impl crate::Readable for RX_IF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_if_ctl::W](W) writer structure"]
impl crate::Writable for RX_IF_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_IF_CTL to value 0x1f01_8707"]
impl crate::Resettable for RX_IF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f01_8707;
}
