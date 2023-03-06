#[doc = "Register `TR_PWM_CTRL` reader"]
pub struct R(crate::R<TR_PWM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_PWM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_PWM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_PWM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_PWM_CTRL` writer"]
pub struct W(crate::W<TR_PWM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_PWM_CTRL_SPEC>;
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
impl From<crate::W<TR_PWM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_PWM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC0_MATCH_MODE` reader - Determines the effect of a compare match 0 event (COUNTER equals CC0 register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC0 register should be set to '0'. For a 100 percent duty cycle, the counter CC0 register should be set to larger than the counter PERIOD register."]
pub type CC0_MATCH_MODE_R = crate::FieldReader<u8, CC0_MATCH_MODE_A>;
#[doc = "Determines the effect of a compare match 0 event (COUNTER equals CC0 register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC0 register should be set to '0'. For a 100 percent duty cycle, the counter CC0 register should be set to larger than the counter PERIOD register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC0_MATCH_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<CC0_MATCH_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CC0_MATCH_MODE_A) -> Self {
        variant as _
    }
}
impl CC0_MATCH_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC0_MATCH_MODE_A {
        match self.bits {
            0 => CC0_MATCH_MODE_A::SET,
            1 => CC0_MATCH_MODE_A::CLEAR,
            2 => CC0_MATCH_MODE_A::INVERT,
            3 => CC0_MATCH_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CC0_MATCH_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CC0_MATCH_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == CC0_MATCH_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CC0_MATCH_MODE_A::NO_CHANGE
    }
}
#[doc = "Field `CC0_MATCH_MODE` writer - Determines the effect of a compare match 0 event (COUNTER equals CC0 register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC0 register should be set to '0'. For a 100 percent duty cycle, the counter CC0 register should be set to larger than the counter PERIOD register."]
pub type CC0_MATCH_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_PWM_CTRL_SPEC, u8, CC0_MATCH_MODE_A, 2, O>;
impl<'a, const O: u8> CC0_MATCH_MODE_W<'a, O> {
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CC0_MATCH_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC0_MATCH_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(CC0_MATCH_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CC0_MATCH_MODE_A::NO_CHANGE)
    }
}
#[doc = "Field `OVERFLOW_MODE` reader - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
pub type OVERFLOW_MODE_R = crate::FieldReader<u8, OVERFLOW_MODE_A>;
#[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVERFLOW_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<OVERFLOW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERFLOW_MODE_A) -> Self {
        variant as _
    }
}
impl OVERFLOW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_MODE_A {
        match self.bits {
            0 => OVERFLOW_MODE_A::SET,
            1 => OVERFLOW_MODE_A::CLEAR,
            2 => OVERFLOW_MODE_A::INVERT,
            3 => OVERFLOW_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OVERFLOW_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == OVERFLOW_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == OVERFLOW_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == OVERFLOW_MODE_A::NO_CHANGE
    }
}
#[doc = "Field `OVERFLOW_MODE` writer - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
pub type OVERFLOW_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_PWM_CTRL_SPEC, u8, OVERFLOW_MODE_A, 2, O>;
impl<'a, const O: u8> OVERFLOW_MODE_W<'a, O> {
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(OVERFLOW_MODE_A::NO_CHANGE)
    }
}
#[doc = "Field `UNDERFLOW_MODE` reader - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
pub type UNDERFLOW_MODE_R = crate::FieldReader<u8, UNDERFLOW_MODE_A>;
#[doc = "Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UNDERFLOW_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<UNDERFLOW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UNDERFLOW_MODE_A) -> Self {
        variant as _
    }
}
impl UNDERFLOW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_MODE_A {
        match self.bits {
            0 => UNDERFLOW_MODE_A::SET,
            1 => UNDERFLOW_MODE_A::CLEAR,
            2 => UNDERFLOW_MODE_A::INVERT,
            3 => UNDERFLOW_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UNDERFLOW_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UNDERFLOW_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == UNDERFLOW_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == UNDERFLOW_MODE_A::NO_CHANGE
    }
}
#[doc = "Field `UNDERFLOW_MODE` writer - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
pub type UNDERFLOW_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_PWM_CTRL_SPEC, u8, UNDERFLOW_MODE_A, 2, O>;
impl<'a, const O: u8> UNDERFLOW_MODE_W<'a, O> {
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(UNDERFLOW_MODE_A::NO_CHANGE)
    }
}
#[doc = "Field `CC1_MATCH_MODE` reader - Determines the effect of a compare match 1 event (COUNTER equals CC1 register) on the 'line_out' output signals."]
pub type CC1_MATCH_MODE_R = crate::FieldReader<u8, CC1_MATCH_MODE_A>;
#[doc = "Determines the effect of a compare match 1 event (COUNTER equals CC1 register) on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1_MATCH_MODE_A {
    #[doc = "0: Set to '1'"]
    SET = 0,
    #[doc = "1: Set to '0'"]
    CLEAR = 1,
    #[doc = "2: Invert"]
    INVERT = 2,
    #[doc = "3: No Change"]
    NO_CHANGE = 3,
}
impl From<CC1_MATCH_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1_MATCH_MODE_A) -> Self {
        variant as _
    }
}
impl CC1_MATCH_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1_MATCH_MODE_A {
        match self.bits {
            0 => CC1_MATCH_MODE_A::SET,
            1 => CC1_MATCH_MODE_A::CLEAR,
            2 => CC1_MATCH_MODE_A::INVERT,
            3 => CC1_MATCH_MODE_A::NO_CHANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CC1_MATCH_MODE_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CC1_MATCH_MODE_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == CC1_MATCH_MODE_A::INVERT
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CC1_MATCH_MODE_A::NO_CHANGE
    }
}
#[doc = "Field `CC1_MATCH_MODE` writer - Determines the effect of a compare match 1 event (COUNTER equals CC1 register) on the 'line_out' output signals."]
pub type CC1_MATCH_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_PWM_CTRL_SPEC, u8, CC1_MATCH_MODE_A, 2, O>;
impl<'a, const O: u8> CC1_MATCH_MODE_W<'a, O> {
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CC1_MATCH_MODE_A::SET)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1_MATCH_MODE_A::CLEAR)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(CC1_MATCH_MODE_A::INVERT)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CC1_MATCH_MODE_A::NO_CHANGE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Determines the effect of a compare match 0 event (COUNTER equals CC0 register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC0 register should be set to '0'. For a 100 percent duty cycle, the counter CC0 register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn cc0_match_mode(&self) -> CC0_MATCH_MODE_R {
        CC0_MATCH_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn overflow_mode(&self) -> OVERFLOW_MODE_R {
        OVERFLOW_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub fn underflow_mode(&self) -> UNDERFLOW_MODE_R {
        UNDERFLOW_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Determines the effect of a compare match 1 event (COUNTER equals CC1 register) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn cc1_match_mode(&self) -> CC1_MATCH_MODE_R {
        CC1_MATCH_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines the effect of a compare match 0 event (COUNTER equals CC0 register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC0 register should be set to '0'. For a 100 percent duty cycle, the counter CC0 register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    #[must_use]
    pub fn cc0_match_mode(&mut self) -> CC0_MATCH_MODE_W<0> {
        CC0_MATCH_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_mode(&mut self) -> OVERFLOW_MODE_W<2> {
        OVERFLOW_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    #[must_use]
    pub fn underflow_mode(&mut self) -> UNDERFLOW_MODE_W<4> {
        UNDERFLOW_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Determines the effect of a compare match 1 event (COUNTER equals CC1 register) on the 'line_out' output signals."]
    #[inline(always)]
    #[must_use]
    pub fn cc1_match_mode(&mut self) -> CC1_MATCH_MODE_W<6> {
        CC1_MATCH_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter trigger PWM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_pwm_ctrl](index.html) module"]
pub struct TR_PWM_CTRL_SPEC;
impl crate::RegisterSpec for TR_PWM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_pwm_ctrl::R](R) reader structure"]
impl crate::Readable for TR_PWM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_pwm_ctrl::W](W) writer structure"]
impl crate::Writable for TR_PWM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_PWM_CTRL to value 0xff"]
impl crate::Resettable for TR_PWM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
