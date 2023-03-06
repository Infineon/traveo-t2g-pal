#[doc = "Register `T1C` reader"]
pub struct R(crate::R<T1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1C` writer"]
pub struct W(crate::W<T1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1C_SPEC>;
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
impl From<crate::W<T1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T1RC` reader - Timer 1 Run Control 1 = Timer 1 running 0 = Timer 1 halted"]
pub type T1RC_R = crate::BitReader<T1RC_A>;
#[doc = "Timer 1 Run Control 1 = Timer 1 running 0 = Timer 1 halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1RC_A {
    #[doc = "0: N/A"]
    TIMER_1_HALTED = 0,
    #[doc = "1: N/A"]
    TIMER_1_RUNNING = 1,
}
impl From<T1RC_A> for bool {
    #[inline(always)]
    fn from(variant: T1RC_A) -> Self {
        variant as u8 != 0
    }
}
impl T1RC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T1RC_A {
        match self.bits {
            false => T1RC_A::TIMER_1_HALTED,
            true => T1RC_A::TIMER_1_RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_1_HALTED`"]
    #[inline(always)]
    pub fn is_timer_1_halted(&self) -> bool {
        *self == T1RC_A::TIMER_1_HALTED
    }
    #[doc = "Checks if the value of the field is `TIMER_1_RUNNING`"]
    #[inline(always)]
    pub fn is_timer_1_running(&self) -> bool {
        *self == T1RC_A::TIMER_1_RUNNING
    }
}
#[doc = "Field `T1RC` writer - Timer 1 Run Control 1 = Timer 1 running 0 = Timer 1 halted"]
pub type T1RC_W<'a, const O: u8> = crate::BitWriter<'a, u32, T1C_SPEC, T1RC_A, O>;
impl<'a, const O: u8> T1RC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_1_halted(self) -> &'a mut W {
        self.variant(T1RC_A::TIMER_1_HALTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_1_running(self) -> &'a mut W {
        self.variant(T1RC_A::TIMER_1_RUNNING)
    }
}
#[doc = "Field `T1MS` reader - Timer 1 Mode Select 1 = Continuous mode 0 = Single-shot mode"]
pub type T1MS_R = crate::BitReader<T1MS_A>;
#[doc = "Timer 1 Mode Select 1 = Continuous mode 0 = Single-shot mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1MS_A {
    #[doc = "0: N/A"]
    TIMER_1_SINGLE_SHOT_MODE = 0,
    #[doc = "1: N/A"]
    TIMER_1_CONTINUOUS_MODE = 1,
}
impl From<T1MS_A> for bool {
    #[inline(always)]
    fn from(variant: T1MS_A) -> Self {
        variant as u8 != 0
    }
}
impl T1MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T1MS_A {
        match self.bits {
            false => T1MS_A::TIMER_1_SINGLE_SHOT_MODE,
            true => T1MS_A::TIMER_1_CONTINUOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_1_SINGLE_SHOT_MODE`"]
    #[inline(always)]
    pub fn is_timer_1_single_shot_mode(&self) -> bool {
        *self == T1MS_A::TIMER_1_SINGLE_SHOT_MODE
    }
    #[doc = "Checks if the value of the field is `TIMER_1_CONTINUOUS_MODE`"]
    #[inline(always)]
    pub fn is_timer_1_continuous_mode(&self) -> bool {
        *self == T1MS_A::TIMER_1_CONTINUOUS_MODE
    }
}
#[doc = "Field `T1MS` writer - Timer 1 Mode Select 1 = Continuous mode 0 = Single-shot mode"]
pub type T1MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, T1C_SPEC, T1MS_A, O>;
impl<'a, const O: u8> T1MS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_1_single_shot_mode(self) -> &'a mut W {
        self.variant(T1MS_A::TIMER_1_SINGLE_SHOT_MODE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_1_continuous_mode(self) -> &'a mut W {
        self.variant(T1MS_A::TIMER_1_CONTINUOUS_MODE)
    }
}
#[doc = "Field `T1MC` reader - Timer 1 Macrotick Count When the configured macrotick count is reached the timer 1 interrupt is generated. Valid values are: 2 to 16383 MT in continuous mode 1 to 16383 MT in single-shot mode"]
pub type T1MC_R = crate::FieldReader<u16, T1MC_A>;
#[doc = "Timer 1 Macrotick Count When the configured macrotick count is reached the timer 1 interrupt is generated. Valid values are: 2 to 16383 MT in continuous mode 1 to 16383 MT in single-shot mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum T1MC_A {
    #[doc = "1: N/A"]
    MIN = 1,
    #[doc = "16383: N/A"]
    MAX = 16383,
}
impl From<T1MC_A> for u16 {
    #[inline(always)]
    fn from(variant: T1MC_A) -> Self {
        variant as _
    }
}
impl T1MC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<T1MC_A> {
        match self.bits {
            1 => Some(T1MC_A::MIN),
            16383 => Some(T1MC_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == T1MC_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == T1MC_A::MAX
    }
}
#[doc = "Field `T1MC` writer - Timer 1 Macrotick Count When the configured macrotick count is reached the timer 1 interrupt is generated. Valid values are: 2 to 16383 MT in continuous mode 1 to 16383 MT in single-shot mode"]
pub type T1MC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T1C_SPEC, u16, T1MC_A, 14, O>;
impl<'a, const O: u8> T1MC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(T1MC_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(T1MC_A::MAX)
    }
}
impl R {
    #[doc = "Bit 0 - Timer 1 Run Control 1 = Timer 1 running 0 = Timer 1 halted"]
    #[inline(always)]
    pub fn t1rc(&self) -> T1RC_R {
        T1RC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Mode Select 1 = Continuous mode 0 = Single-shot mode"]
    #[inline(always)]
    pub fn t1ms(&self) -> T1MS_R {
        T1MS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Timer 1 Macrotick Count When the configured macrotick count is reached the timer 1 interrupt is generated. Valid values are: 2 to 16383 MT in continuous mode 1 to 16383 MT in single-shot mode"]
    #[inline(always)]
    pub fn t1mc(&self) -> T1MC_R {
        T1MC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 1 Run Control 1 = Timer 1 running 0 = Timer 1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn t1rc(&mut self) -> T1RC_W<0> {
        T1RC_W::new(self)
    }
    #[doc = "Bit 1 - Timer 1 Mode Select 1 = Continuous mode 0 = Single-shot mode"]
    #[inline(always)]
    #[must_use]
    pub fn t1ms(&mut self) -> T1MS_W<1> {
        T1MS_W::new(self)
    }
    #[doc = "Bits 16:29 - Timer 1 Macrotick Count When the configured macrotick count is reached the timer 1 interrupt is generated. Valid values are: 2 to 16383 MT in continuous mode 1 to 16383 MT in single-shot mode"]
    #[inline(always)]
    #[must_use]
    pub fn t1mc(&mut self) -> T1MC_W<16> {
        T1MC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 1 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1c](index.html) module"]
pub struct T1C_SPEC;
impl crate::RegisterSpec for T1C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1c::R](R) reader structure"]
impl crate::Readable for T1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1c::W](W) writer structure"]
impl crate::Writable for T1C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T1C to value 0x0002_0000"]
impl crate::Resettable for T1C_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0000;
}
