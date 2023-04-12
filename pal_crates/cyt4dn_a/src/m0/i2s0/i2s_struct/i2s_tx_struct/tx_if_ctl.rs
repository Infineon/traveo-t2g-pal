#[doc = "Register `TX_IF_CTL` reader"]
pub struct R(crate::R<TX_IF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_IF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_IF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_IF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_IF_CTL` writer"]
pub struct W(crate::W<TX_IF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_IF_CTL_SPEC>;
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
impl From<crate::W<TX_IF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_IF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_DIV` reader - Interface clock divider (legal range \\[1, 255\\]). The I2S interface 'i2s_tx_sck_out' output signal is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value ({1, 3, 5, ..., 255}), to ensure a 50/50 percent duty cycle clock. Note: Used in master configuration only."]
pub type CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_DIV` writer - Interface clock divider (legal range \\[1, 255\\]). The I2S interface 'i2s_tx_sck_out' output signal is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value ({1, 3, 5, ..., 255}), to ensure a 50/50 percent duty cycle clock. Note: Used in master configuration only."]
pub type CLOCK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_IF_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLOCK_SEL` reader - Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'i2s_tx_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release."]
pub type CLOCK_SEL_R = crate::FieldReader<u8, CLOCK_SEL_A>;
#[doc = "Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'i2s_tx_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release.\n\nValue on reset: 7"]
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
    SEL_I2S_TX_MCK_IN = 4,
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
            4 => Some(CLOCK_SEL_A::SEL_I2S_TX_MCK_IN),
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
    #[doc = "Checks if the value of the field is `SEL_I2S_TX_MCK_IN`"]
    #[inline(always)]
    pub fn is_sel_i2s_tx_mck_in(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_I2S_TX_MCK_IN
    }
}
#[doc = "Field `CLOCK_SEL` writer - Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'i2s_tx_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release."]
pub type CLOCK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_IF_CTL_SPEC, u8, CLOCK_SEL_A, 3, O>;
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
    pub fn sel_i2s_tx_mck_in(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_I2S_TX_MCK_IN)
    }
}
#[doc = "Field `SCK_POLARITY` reader - Clock polarity: '0': Clock signal is used 'as is'. '1': Clock signal is inverted. Note: Used in BOTH master and slave configurations."]
pub type SCK_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `SCK_POLARITY` writer - Clock polarity: '0': Clock signal is used 'as is'. '1': Clock signal is inverted. Note: Used in BOTH master and slave configurations."]
pub type SCK_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_IF_CTL_SPEC, bool, O>;
#[doc = "Field `WS_POLARITY` reader - Word select polarity: '0': Word select signal is used 'as is'. '1': Word select signal is inverted. Note: Used in BOTH master and slave configurations."]
pub type WS_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `WS_POLARITY` writer - Word select polarity: '0': Word select signal is used 'as is'. '1': Word select signal is inverted. Note: Used in BOTH master and slave configurations."]
pub type WS_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_IF_CTL_SPEC, bool, O>;
#[doc = "Field `CH0_EN` reader - Channel 0 enable: '0': Disabled. The TX FIFO does not produce channel 0 words and 'i2s_tx_sd_out' is not driven. '1': Enabled."]
pub type CH0_EN_R = crate::BitReader<bool>;
#[doc = "Field `CH0_EN` writer - Channel 0 enable: '0': Disabled. The TX FIFO does not produce channel 0 words and 'i2s_tx_sd_out' is not driven. '1': Enabled."]
pub type CH0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_IF_CTL_SPEC, bool, O>;
#[doc = "Field `CH1_EN` reader - Channel 1 enable: '0': Disabled. The TX FIFO does not produce channel 1 words and 'i2s_tx_sd_out' is not driven. '1': Enabled."]
pub type CH1_EN_R = crate::BitReader<bool>;
#[doc = "Field `CH1_EN` writer - Channel 1 enable: '0': Disabled. The TX FIFO does not produce channel 1 words and 'i2s_tx_sd_out' is not driven. '1': Enabled."]
pub type CH1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_IF_CTL_SPEC, bool, O>;
#[doc = "Field `CH_SIZE` reader - Channel size: '0': 1 bit. '1': 2 bits. ... '31': 32 bits. Note: channel size must be greater or equal to the word size."]
pub type CH_SIZE_R = crate::FieldReader<u8, CH_SIZE_A>;
#[doc = "Channel size: '0': 1 bit. '1': 2 bits. ... '31': 32 bits. Note: channel size must be greater or equal to the word size.\n\nValue on reset: 31"]
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
#[doc = "Field `CH_SIZE` writer - Channel size: '0': 1 bit. '1': 2 bits. ... '31': 32 bits. Note: channel size must be greater or equal to the word size."]
pub type CH_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_IF_CTL_SPEC, u8, CH_SIZE_A, 5, O>;
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
impl R {
    #[doc = "Bits 0:7 - Interface clock divider (legal range \\[1, 255\\]). The I2S interface 'i2s_tx_sck_out' output signal is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value ({1, 3, 5, ..., 255}), to ensure a 50/50 percent duty cycle clock. Note: Used in master configuration only."]
    #[inline(always)]
    pub fn clock_div(&self) -> CLOCK_DIV_R {
        CLOCK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'i2s_tx_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release."]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Clock polarity: '0': Clock signal is used 'as is'. '1': Clock signal is inverted. Note: Used in BOTH master and slave configurations."]
    #[inline(always)]
    pub fn sck_polarity(&self) -> SCK_POLARITY_R {
        SCK_POLARITY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Word select polarity: '0': Word select signal is used 'as is'. '1': Word select signal is inverted. Note: Used in BOTH master and slave configurations."]
    #[inline(always)]
    pub fn ws_polarity(&self) -> WS_POLARITY_R {
        WS_POLARITY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 0 enable: '0': Disabled. The TX FIFO does not produce channel 0 words and 'i2s_tx_sd_out' is not driven. '1': Enabled."]
    #[inline(always)]
    pub fn ch0_en(&self) -> CH0_EN_R {
        CH0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 enable: '0': Disabled. The TX FIFO does not produce channel 1 words and 'i2s_tx_sd_out' is not driven. '1': Enabled."]
    #[inline(always)]
    pub fn ch1_en(&self) -> CH1_EN_R {
        CH1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Channel size: '0': 1 bit. '1': 2 bits. ... '31': 32 bits. Note: channel size must be greater or equal to the word size."]
    #[inline(always)]
    pub fn ch_size(&self) -> CH_SIZE_R {
        CH_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interface clock divider (legal range \\[1, 255\\]). The I2S interface 'i2s_tx_sck_out' output signal is defined as clk_if / (CLOCK_DIV + 1). CLOCK_DIV should be set to an odd value ({1, 3, 5, ..., 255}), to ensure a 50/50 percent duty cycle clock. Note: Used in master configuration only."]
    #[inline(always)]
    #[must_use]
    pub fn clock_div(&mut self) -> CLOCK_DIV_W<0> {
        CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 8:10 - Interface clock 'clk_if' selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'i2s_tx_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release."]
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
    #[doc = "Bit 13 - Word select polarity: '0': Word select signal is used 'as is'. '1': Word select signal is inverted. Note: Used in BOTH master and slave configurations."]
    #[inline(always)]
    #[must_use]
    pub fn ws_polarity(&mut self) -> WS_POLARITY_W<13> {
        WS_POLARITY_W::new(self)
    }
    #[doc = "Bit 16 - Channel 0 enable: '0': Disabled. The TX FIFO does not produce channel 0 words and 'i2s_tx_sd_out' is not driven. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_en(&mut self) -> CH0_EN_W<16> {
        CH0_EN_W::new(self)
    }
    #[doc = "Bit 17 - Channel 1 enable: '0': Disabled. The TX FIFO does not produce channel 1 words and 'i2s_tx_sd_out' is not driven. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_en(&mut self) -> CH1_EN_W<17> {
        CH1_EN_W::new(self)
    }
    #[doc = "Bits 24:28 - Channel size: '0': 1 bit. '1': 2 bits. ... '31': 32 bits. Note: channel size must be greater or equal to the word size."]
    #[inline(always)]
    #[must_use]
    pub fn ch_size(&mut self) -> CH_SIZE_W<24> {
        CH_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX interface control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_if_ctl](index.html) module"]
pub struct TX_IF_CTL_SPEC;
impl crate::RegisterSpec for TX_IF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_if_ctl::R](R) reader structure"]
impl crate::Readable for TX_IF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_if_ctl::W](W) writer structure"]
impl crate::Writable for TX_IF_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_IF_CTL to value 0x1f00_0707"]
impl crate::Resettable for TX_IF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f00_0707;
}
