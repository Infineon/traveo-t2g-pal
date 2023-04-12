#[doc = "Register `TX_CTL` reader"]
pub struct R(crate::R<TX_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CTL` writer"]
pub struct W(crate::W<TX_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CTL_SPEC>;
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
impl From<crate::W<TX_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WORD_SIZE` reader - PCM word size: '0'-'3': Undefined. '4': 16 bit. The IP always uses 16-bit PCM words. '9'-'15': Undefined."]
pub type WORD_SIZE_R = crate::FieldReader<u8, WORD_SIZE_A>;
#[doc = "PCM word size: '0'-'3': Undefined. '4': 16 bit. The IP always uses 16-bit PCM words. '9'-'15': Undefined.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WORD_SIZE_A {
    #[doc = "4: N/A"]
    SIZE_16 = 4,
}
impl From<WORD_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_SIZE_A) -> Self {
        variant as _
    }
}
impl WORD_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WORD_SIZE_A> {
        match self.bits {
            4 => Some(WORD_SIZE_A::SIZE_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_16`"]
    #[inline(always)]
    pub fn is_size_16(&self) -> bool {
        *self == WORD_SIZE_A::SIZE_16
    }
}
#[doc = "Field `FORMAT` reader - Format: '0': Left-aligned delayed (I2S). '1': Left-aligned. '2': Right-aligned delayed. '3': Right-aligned."]
pub type FORMAT_R = crate::FieldReader<u8, FORMAT_A>;
#[doc = "Format: '0': Left-aligned delayed (I2S). '1': Left-aligned. '2': Right-aligned delayed. '3': Right-aligned.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: N/A"]
    LEFT_DELAYED = 0,
    #[doc = "1: N/A"]
    LEFT = 1,
    #[doc = "2: N/A"]
    RIGHT_DELAYED = 2,
    #[doc = "3: N/A"]
    RIGHT = 3,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            0 => FORMAT_A::LEFT_DELAYED,
            1 => FORMAT_A::LEFT,
            2 => FORMAT_A::RIGHT_DELAYED,
            3 => FORMAT_A::RIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_DELAYED`"]
    #[inline(always)]
    pub fn is_left_delayed(&self) -> bool {
        *self == FORMAT_A::LEFT_DELAYED
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == FORMAT_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT_DELAYED`"]
    #[inline(always)]
    pub fn is_right_delayed(&self) -> bool {
        *self == FORMAT_A::RIGHT_DELAYED
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == FORMAT_A::RIGHT
    }
}
#[doc = "Field `FORMAT` writer - Format: '0': Left-aligned delayed (I2S). '1': Left-aligned. '2': Right-aligned delayed. '3': Right-aligned."]
pub type FORMAT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TX_CTL_SPEC, u8, FORMAT_A, 2, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn left_delayed(self) -> &'a mut W {
        self.variant(FORMAT_A::LEFT_DELAYED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(FORMAT_A::LEFT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn right_delayed(self) -> &'a mut W {
        self.variant(FORMAT_A::RIGHT_DELAYED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(FORMAT_A::RIGHT)
    }
}
#[doc = "Field `MS` reader - Master/slave setting: '0': Slave. - External transmitter 'mixer_tx_sck_in' and transmitter 'mixer_tx_ws_in'. '1': Master. - Interface clock 'clk_if' is used to generate transmitter 'mixer_tx_sck_out' and transmitter 'mixer_tx_ws_out'."]
pub type MS_R = crate::BitReader<MS_A>;
#[doc = "Master/slave setting: '0': Slave. - External transmitter 'mixer_tx_sck_in' and transmitter 'mixer_tx_ws_in'. '1': Master. - Interface clock 'clk_if' is used to generate transmitter 'mixer_tx_sck_out' and transmitter 'mixer_tx_ws_out'.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MS_A {
    #[doc = "0: N/A"]
    SLAVE = 0,
    #[doc = "1: N/A"]
    MASTER = 1,
}
impl From<MS_A> for bool {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as u8 != 0
    }
}
impl MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MS_A {
        match self.bits {
            false => MS_A::SLAVE,
            true => MS_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MS_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MS_A::MASTER
    }
}
#[doc = "Field `MS` writer - Master/slave setting: '0': Slave. - External transmitter 'mixer_tx_sck_in' and transmitter 'mixer_tx_ws_in'. '1': Master. - Interface clock 'clk_if' is used to generate transmitter 'mixer_tx_sck_out' and transmitter 'mixer_tx_ws_out'."]
pub type MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CTL_SPEC, MS_A, O>;
impl<'a, const O: u8> MS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MS_A::SLAVE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MS_A::MASTER)
    }
}
#[doc = "Field `ENABLED` reader - Transmitter (TX) enable: '0': Disabled. All non-retained MMIO registers (e.g. the TX_FIFO_STATUS and INTR_TX registers) have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Transmitter (TX) enable: '0': Disabled. All non-retained MMIO registers (e.g. the TX_FIFO_STATUS and INTR_TX registers) have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - PCM word size: '0'-'3': Undefined. '4': 16 bit. The IP always uses 16-bit PCM words. '9'-'15': Undefined."]
    #[inline(always)]
    pub fn word_size(&self) -> WORD_SIZE_R {
        WORD_SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Format: '0': Left-aligned delayed (I2S). '1': Left-aligned. '2': Right-aligned delayed. '3': Right-aligned."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Master/slave setting: '0': Slave. - External transmitter 'mixer_tx_sck_in' and transmitter 'mixer_tx_ws_in'. '1': Master. - Interface clock 'clk_if' is used to generate transmitter 'mixer_tx_sck_out' and transmitter 'mixer_tx_ws_out'."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmitter (TX) enable: '0': Disabled. All non-retained MMIO registers (e.g. the TX_FIFO_STATUS and INTR_TX registers) have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:13 - Format: '0': Left-aligned delayed (I2S). '1': Left-aligned. '2': Right-aligned delayed. '3': Right-aligned."]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<12> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 16 - Master/slave setting: '0': Slave. - External transmitter 'mixer_tx_sck_in' and transmitter 'mixer_tx_ws_in'. '1': Master. - Interface clock 'clk_if' is used to generate transmitter 'mixer_tx_sck_out' and transmitter 'mixer_tx_ws_out'."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<16> {
        MS_W::new(self)
    }
    #[doc = "Bit 31 - Transmitter (TX) enable: '0': Disabled. All non-retained MMIO registers (e.g. the TX_FIFO_STATUS and INTR_TX registers) have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctl](index.html) module"]
pub struct TX_CTL_SPEC;
impl crate::RegisterSpec for TX_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ctl::R](R) reader structure"]
impl crate::Readable for TX_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ctl::W](W) writer structure"]
impl crate::Writable for TX_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CTL to value 0x0001_0004"]
impl crate::Resettable for TX_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0004;
}
