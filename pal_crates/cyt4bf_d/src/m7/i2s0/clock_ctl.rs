#[doc = "Register `CLOCK_CTL` reader"]
pub struct R(crate::R<CLOCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CTL` writer"]
pub struct W(crate::W<CLOCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CTL_SPEC>;
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
impl From<crate::W<CLOCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_DIV` reader - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
pub type CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_DIV` writer - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
pub type CLOCK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `CLOCK_SEL` reader - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
pub type CLOCK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLOCK_SEL` writer - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
pub type CLOCK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_CTL_SPEC, bool, O>;
#[doc = "Field `MCLK_DIV` reader - Selects clock divider for MCLK_OUT. CTL.TX_ENABLED=1 or CTL.RX_ENABLED=1 and and CLOCK_CTL.CLOCK_SEL=0 to enable the clock."]
pub type MCLK_DIV_R = crate::FieldReader<u8, MCLK_DIV_A>;
#[doc = "Selects clock divider for MCLK_OUT. CTL.TX_ENABLED=1 or CTL.RX_ENABLED=1 and and CLOCK_CTL.CLOCK_SEL=0 to enable the clock.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCLK_DIV_A {
    #[doc = "0: Divide clk_audio_i2s by 1 (Bypass)"]
    DIV1 = 0,
    #[doc = "1: Divide clk_audio_i2s by 2"]
    DIV2 = 1,
    #[doc = "2: Divide clk_audio_i2s by 4"]
    DIV4 = 2,
    #[doc = "3: Divide clk-audio_i2s by 8"]
    DIV8 = 3,
}
impl From<MCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCLK_DIV_A) -> Self {
        variant as _
    }
}
impl MCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLK_DIV_A {
        match self.bits {
            0 => MCLK_DIV_A::DIV1,
            1 => MCLK_DIV_A::DIV2,
            2 => MCLK_DIV_A::DIV4,
            3 => MCLK_DIV_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCLK_DIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCLK_DIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCLK_DIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCLK_DIV_A::DIV8
    }
}
#[doc = "Field `MCLK_DIV` writer - Selects clock divider for MCLK_OUT. CTL.TX_ENABLED=1 or CTL.RX_ENABLED=1 and and CLOCK_CTL.CLOCK_SEL=0 to enable the clock."]
pub type MCLK_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLOCK_CTL_SPEC, u8, MCLK_DIV_A, 2, O>;
impl<'a, const O: u8> MCLK_DIV_W<'a, O> {
    #[doc = "Divide clk_audio_i2s by 1 (Bypass)"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCLK_DIV_A::DIV1)
    }
    #[doc = "Divide clk_audio_i2s by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCLK_DIV_A::DIV2)
    }
    #[doc = "Divide clk_audio_i2s by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCLK_DIV_A::DIV4)
    }
    #[doc = "Divide clk-audio_i2s by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCLK_DIV_A::DIV8)
    }
}
#[doc = "Field `MCLK_EN` reader - Enable MCLK - enables MCLK divider operation Upon assertion allows MCLK Divider to begin operation. Upon de-assertion, i.e. 1-0 transition, allows MCLK divider to reach the all 0s state (reset state) then freezes the divider. This permits a controlled power-on/power-off sequence that may be used prior to going to DeepSleep/IP Disable and avoids glitching the MCLK."]
pub type MCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `MCLK_EN` writer - Enable MCLK - enables MCLK divider operation Upon assertion allows MCLK Divider to begin operation. Upon de-assertion, i.e. 1-0 transition, allows MCLK divider to reach the all 0s state (reset state) then freezes the divider. This permits a controlled power-on/power-off sequence that may be used prior to going to DeepSleep/IP Disable and avoids glitching the MCLK."]
pub type MCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    pub fn clock_div(&self) -> CLOCK_DIV_R {
        CLOCK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Selects clock divider for MCLK_OUT. CTL.TX_ENABLED=1 or CTL.RX_ENABLED=1 and and CLOCK_CTL.CLOCK_SEL=0 to enable the clock."]
    #[inline(always)]
    pub fn mclk_div(&self) -> MCLK_DIV_R {
        MCLK_DIV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable MCLK - enables MCLK divider operation Upon assertion allows MCLK Divider to begin operation. Upon de-assertion, i.e. 1-0 transition, allows MCLK divider to reach the all 0s state (reset state) then freezes the divider. This permits a controlled power-on/power-off sequence that may be used prior to going to DeepSleep/IP Disable and avoids glitching the MCLK."]
    #[inline(always)]
    pub fn mclk_en(&self) -> MCLK_EN_R {
        MCLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency divisor for generating I2S clock frequency. The selected clock with CLOCK_SEL is divided by this. '0': Bypass '1': 2 x '2': 3 x '3': 4 x ... '62': 63 x '63': 64 x"]
    #[inline(always)]
    #[must_use]
    pub fn clock_div(&mut self) -> CLOCK_DIV_W<0> {
        CLOCK_DIV_W::new(self)
    }
    #[doc = "Bit 8 - Selects clock to be used by I2S: '0': Internal clock ('clk_audio_i2s') '1': External clock ('clk_i2s_if')"]
    #[inline(always)]
    #[must_use]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W<8> {
        CLOCK_SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Selects clock divider for MCLK_OUT. CTL.TX_ENABLED=1 or CTL.RX_ENABLED=1 and and CLOCK_CTL.CLOCK_SEL=0 to enable the clock."]
    #[inline(always)]
    #[must_use]
    pub fn mclk_div(&mut self) -> MCLK_DIV_W<12> {
        MCLK_DIV_W::new(self)
    }
    #[doc = "Bit 16 - Enable MCLK - enables MCLK divider operation Upon assertion allows MCLK Divider to begin operation. Upon de-assertion, i.e. 1-0 transition, allows MCLK divider to reach the all 0s state (reset state) then freezes the divider. This permits a controlled power-on/power-off sequence that may be used prior to going to DeepSleep/IP Disable and avoids glitching the MCLK."]
    #[inline(always)]
    #[must_use]
    pub fn mclk_en(&mut self) -> MCLK_EN_W<16> {
        MCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](index.html) module"]
pub struct CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_ctl::R](R) reader structure"]
impl crate::Readable for CLOCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](W) writer structure"]
impl crate::Writable for CLOCK_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0x3000"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3000;
}
