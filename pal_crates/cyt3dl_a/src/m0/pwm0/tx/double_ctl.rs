#[doc = "Register `DOUBLE_CTL` reader"]
pub struct R(crate::R<DOUBLE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUBLE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUBLE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUBLE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUBLE_CTL` writer"]
pub struct W(crate::W<DOUBLE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUBLE_CTL_SPEC>;
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
impl From<crate::W<DOUBLE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUBLE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Doubler mode (only applicable when ENABLED is '1'): '0': Repetition. An additional sample is created by repeating each TX FIFO data sample. '1': Averaging. An additional sample is calculated by averaging two consecutive TX FIFO data samples."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Doubler mode (only applicable when ENABLED is '1'): '0': Repetition. An additional sample is created by repeating each TX FIFO data sample. '1': Averaging. An additional sample is calculated by averaging two consecutive TX FIFO data samples.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: N/A"]
    REPETITION = 0,
    #[doc = "1: N/A"]
    AVERAGING = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::REPETITION,
            true => MODE_A::AVERAGING,
        }
    }
    #[doc = "Checks if the value of the field is `REPETITION`"]
    #[inline(always)]
    pub fn is_repetition(&self) -> bool {
        *self == MODE_A::REPETITION
    }
    #[doc = "Checks if the value of the field is `AVERAGING`"]
    #[inline(always)]
    pub fn is_averaging(&self) -> bool {
        *self == MODE_A::AVERAGING
    }
}
#[doc = "Field `MODE` writer - Doubler mode (only applicable when ENABLED is '1'): '0': Repetition. An additional sample is created by repeating each TX FIFO data sample. '1': Averaging. An additional sample is calculated by averaging two consecutive TX FIFO data samples."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUBLE_CTL_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn repetition(self) -> &'a mut W {
        self.variant(MODE_A::REPETITION)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn averaging(self) -> &'a mut W {
        self.variant(MODE_A::AVERAGING)
    }
}
#[doc = "Field `ENABLED` reader - Doubler enable: '0': Disabled. doubler_pcm\\[23:0\\]
= pcm_data\\[23:0\\]. '1': Enabled. An 'intermediate' sample is generated through either repetition or averaging (as specified by MODE). This effectively doubles the sample frequency Fs and effectively halves the dynamic PWM sample range (at the same PWM interface clock frequency)."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Doubler enable: '0': Disabled. doubler_pcm\\[23:0\\]
= pcm_data\\[23:0\\]. '1': Enabled. An 'intermediate' sample is generated through either repetition or averaging (as specified by MODE). This effectively doubles the sample frequency Fs and effectively halves the dynamic PWM sample range (at the same PWM interface clock frequency)."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUBLE_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Doubler mode (only applicable when ENABLED is '1'): '0': Repetition. An additional sample is created by repeating each TX FIFO data sample. '1': Averaging. An additional sample is calculated by averaging two consecutive TX FIFO data samples."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Doubler enable: '0': Disabled. doubler_pcm\\[23:0\\]
= pcm_data\\[23:0\\]. '1': Enabled. An 'intermediate' sample is generated through either repetition or averaging (as specified by MODE). This effectively doubles the sample frequency Fs and effectively halves the dynamic PWM sample range (at the same PWM interface clock frequency)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Doubler mode (only applicable when ENABLED is '1'): '0': Repetition. An additional sample is created by repeating each TX FIFO data sample. '1': Averaging. An additional sample is calculated by averaging two consecutive TX FIFO data samples."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 31 - Doubler enable: '0': Disabled. doubler_pcm\\[23:0\\]
= pcm_data\\[23:0\\]. '1': Enabled. An 'intermediate' sample is generated through either repetition or averaging (as specified by MODE). This effectively doubles the sample frequency Fs and effectively halves the dynamic PWM sample range (at the same PWM interface clock frequency)."]
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
#[doc = "Double control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [double_ctl](index.html) module"]
pub struct DOUBLE_CTL_SPEC;
impl crate::RegisterSpec for DOUBLE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [double_ctl::R](R) reader structure"]
impl crate::Readable for DOUBLE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [double_ctl::W](W) writer structure"]
impl crate::Writable for DOUBLE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUBLE_CTL to value 0"]
impl crate::Resettable for DOUBLE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
