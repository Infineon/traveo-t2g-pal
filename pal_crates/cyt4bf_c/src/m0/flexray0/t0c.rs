#[doc = "Register `T0C` reader"]
pub struct R(crate::R<T0C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0C` writer"]
pub struct W(crate::W<T0C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0C_SPEC>;
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
impl From<crate::W<T0C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0RC` reader - Timer 0 Run Control 1 = Timer 0 running 0 = Timer 0 halted"]
pub type T0RC_R = crate::BitReader<T0RC_A>;
#[doc = "Timer 0 Run Control 1 = Timer 0 running 0 = Timer 0 halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T0RC_A {
    #[doc = "0: N/A"]
    TIMER_0_HALTED = 0,
    #[doc = "1: N/A"]
    TIMER_0_RUNNING = 1,
}
impl From<T0RC_A> for bool {
    #[inline(always)]
    fn from(variant: T0RC_A) -> Self {
        variant as u8 != 0
    }
}
impl T0RC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T0RC_A {
        match self.bits {
            false => T0RC_A::TIMER_0_HALTED,
            true => T0RC_A::TIMER_0_RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_0_HALTED`"]
    #[inline(always)]
    pub fn is_timer_0_halted(&self) -> bool {
        *self == T0RC_A::TIMER_0_HALTED
    }
    #[doc = "Checks if the value of the field is `TIMER_0_RUNNING`"]
    #[inline(always)]
    pub fn is_timer_0_running(&self) -> bool {
        *self == T0RC_A::TIMER_0_RUNNING
    }
}
#[doc = "Field `T0RC` writer - Timer 0 Run Control 1 = Timer 0 running 0 = Timer 0 halted"]
pub type T0RC_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0C_SPEC, T0RC_A, O>;
impl<'a, const O: u8> T0RC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_0_halted(self) -> &'a mut W {
        self.variant(T0RC_A::TIMER_0_HALTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_0_running(self) -> &'a mut W {
        self.variant(T0RC_A::TIMER_0_RUNNING)
    }
}
#[doc = "Field `T0MS` reader - Timer 0 Mode Select 1 = Continuous mode 0 = Single-shot mode"]
pub type T0MS_R = crate::BitReader<T0MS_A>;
#[doc = "Timer 0 Mode Select 1 = Continuous mode 0 = Single-shot mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T0MS_A {
    #[doc = "0: N/A"]
    TIMER_0_SINGLE_SHOT_MODE = 0,
    #[doc = "1: N/A"]
    TIMER_0_CONTINUOUS_MODE = 1,
}
impl From<T0MS_A> for bool {
    #[inline(always)]
    fn from(variant: T0MS_A) -> Self {
        variant as u8 != 0
    }
}
impl T0MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T0MS_A {
        match self.bits {
            false => T0MS_A::TIMER_0_SINGLE_SHOT_MODE,
            true => T0MS_A::TIMER_0_CONTINUOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_0_SINGLE_SHOT_MODE`"]
    #[inline(always)]
    pub fn is_timer_0_single_shot_mode(&self) -> bool {
        *self == T0MS_A::TIMER_0_SINGLE_SHOT_MODE
    }
    #[doc = "Checks if the value of the field is `TIMER_0_CONTINUOUS_MODE`"]
    #[inline(always)]
    pub fn is_timer_0_continuous_mode(&self) -> bool {
        *self == T0MS_A::TIMER_0_CONTINUOUS_MODE
    }
}
#[doc = "Field `T0MS` writer - Timer 0 Mode Select 1 = Continuous mode 0 = Single-shot mode"]
pub type T0MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0C_SPEC, T0MS_A, O>;
impl<'a, const O: u8> T0MS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_0_single_shot_mode(self) -> &'a mut W {
        self.variant(T0MS_A::TIMER_0_SINGLE_SHOT_MODE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_0_continuous_mode(self) -> &'a mut W {
        self.variant(T0MS_A::TIMER_0_CONTINUOUS_MODE)
    }
}
#[doc = "Field `T0CC` reader - Timer 0 Cycle Code The 7-bit timer 0 cycle code determines the cycle set used for generation of the timer 0 interrupt. For details about the configuration of the cycle code see \\[01\\]Section 5.7.2 Cycle Counter Filtering."]
pub type T0CC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T0CC` writer - Timer 0 Cycle Code The 7-bit timer 0 cycle code determines the cycle set used for generation of the timer 0 interrupt. For details about the configuration of the cycle code see \\[01\\]Section 5.7.2 Cycle Counter Filtering."]
pub type T0CC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0C_SPEC, u8, u8, 7, O>;
#[doc = "Field `T0MO` reader - Timer 0 Macrotick Offset Configures the macrotick offset from the beginning of the cycle where the interrupt is to occur. The Timer 0 Interrupt occurs at this offset for each cycle of the cycle set."]
pub type T0MO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T0MO` writer - Timer 0 Macrotick Offset Configures the macrotick offset from the beginning of the cycle where the interrupt is to occur. The Timer 0 Interrupt occurs at this offset for each cycle of the cycle set."]
pub type T0MO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0C_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bit 0 - Timer 0 Run Control 1 = Timer 0 running 0 = Timer 0 halted"]
    #[inline(always)]
    pub fn t0rc(&self) -> T0RC_R {
        T0RC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 0 Mode Select 1 = Continuous mode 0 = Single-shot mode"]
    #[inline(always)]
    pub fn t0ms(&self) -> T0MS_R {
        T0MS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Timer 0 Cycle Code The 7-bit timer 0 cycle code determines the cycle set used for generation of the timer 0 interrupt. For details about the configuration of the cycle code see \\[01\\]Section 5.7.2 Cycle Counter Filtering."]
    #[inline(always)]
    pub fn t0cc(&self) -> T0CC_R {
        T0CC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:29 - Timer 0 Macrotick Offset Configures the macrotick offset from the beginning of the cycle where the interrupt is to occur. The Timer 0 Interrupt occurs at this offset for each cycle of the cycle set."]
    #[inline(always)]
    pub fn t0mo(&self) -> T0MO_R {
        T0MO_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Run Control 1 = Timer 0 running 0 = Timer 0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn t0rc(&mut self) -> T0RC_W<0> {
        T0RC_W::new(self)
    }
    #[doc = "Bit 1 - Timer 0 Mode Select 1 = Continuous mode 0 = Single-shot mode"]
    #[inline(always)]
    #[must_use]
    pub fn t0ms(&mut self) -> T0MS_W<1> {
        T0MS_W::new(self)
    }
    #[doc = "Bits 8:14 - Timer 0 Cycle Code The 7-bit timer 0 cycle code determines the cycle set used for generation of the timer 0 interrupt. For details about the configuration of the cycle code see \\[01\\]Section 5.7.2 Cycle Counter Filtering."]
    #[inline(always)]
    #[must_use]
    pub fn t0cc(&mut self) -> T0CC_W<8> {
        T0CC_W::new(self)
    }
    #[doc = "Bits 16:29 - Timer 0 Macrotick Offset Configures the macrotick offset from the beginning of the cycle where the interrupt is to occur. The Timer 0 Interrupt occurs at this offset for each cycle of the cycle set."]
    #[inline(always)]
    #[must_use]
    pub fn t0mo(&mut self) -> T0MO_W<16> {
        T0MO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 0 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0c](index.html) module"]
pub struct T0C_SPEC;
impl crate::RegisterSpec for T0C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0c::R](R) reader structure"]
impl crate::Readable for T0C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0c::W](W) writer structure"]
impl crate::Writable for T0C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0C to value 0"]
impl crate::Resettable for T0C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
