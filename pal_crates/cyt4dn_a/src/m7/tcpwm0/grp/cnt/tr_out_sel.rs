#[doc = "Register `TR_OUT_SEL` reader"]
pub struct R(crate::R<TR_OUT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_OUT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_OUT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_OUT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_OUT_SEL` writer"]
pub struct W(crate::W<TR_OUT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_OUT_SEL_SPEC>;
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
impl From<crate::W<TR_OUT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_OUT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT0` reader - Selects one of the internal events to generate the output trigger 0. Default setting selects the terminal count event."]
pub type OUT0_R = crate::FieldReader<u8, OUT0_A>;
#[doc = "Selects one of the internal events to generate the output trigger 0. Default setting selects the terminal count event.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT0_A {
    #[doc = "0: Overflow event"]
    OVERFLOW = 0,
    #[doc = "1: Underflow event"]
    UNDERFLOW = 1,
    #[doc = "2: Terminal count event (default selection)"]
    TC = 2,
    #[doc = "3: Compare match 0 event"]
    CC0_MATCH = 3,
    #[doc = "4: Compare match 1 event"]
    CC1_MATCH = 4,
    #[doc = "5: PWM output signal 'line_out'"]
    LINE_OUT = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: Output trigger disabled."]
    DISABLED = 7,
}
impl From<OUT0_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT0_A) -> Self {
        variant as _
    }
}
impl OUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT0_A {
        match self.bits {
            0 => OUT0_A::OVERFLOW,
            1 => OUT0_A::UNDERFLOW,
            2 => OUT0_A::TC,
            3 => OUT0_A::CC0_MATCH,
            4 => OUT0_A::CC1_MATCH,
            5 => OUT0_A::LINE_OUT,
            6 => OUT0_A::RSVD6,
            7 => OUT0_A::DISABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OUT0_A::OVERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == OUT0_A::UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == OUT0_A::TC
    }
    #[doc = "Checks if the value of the field is `CC0_MATCH`"]
    #[inline(always)]
    pub fn is_cc0_match(&self) -> bool {
        *self == OUT0_A::CC0_MATCH
    }
    #[doc = "Checks if the value of the field is `CC1_MATCH`"]
    #[inline(always)]
    pub fn is_cc1_match(&self) -> bool {
        *self == OUT0_A::CC1_MATCH
    }
    #[doc = "Checks if the value of the field is `LINE_OUT`"]
    #[inline(always)]
    pub fn is_line_out(&self) -> bool {
        *self == OUT0_A::LINE_OUT
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == OUT0_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUT0_A::DISABLED
    }
}
#[doc = "Field `OUT0` writer - Selects one of the internal events to generate the output trigger 0. Default setting selects the terminal count event."]
pub type OUT0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_OUT_SEL_SPEC, u8, OUT0_A, 3, O>;
impl<'a, const O: u8> OUT0_W<'a, O> {
    #[doc = "Overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut W {
        self.variant(OUT0_A::OVERFLOW)
    }
    #[doc = "Underflow event"]
    #[inline(always)]
    pub fn underflow(self) -> &'a mut W {
        self.variant(OUT0_A::UNDERFLOW)
    }
    #[doc = "Terminal count event (default selection)"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(OUT0_A::TC)
    }
    #[doc = "Compare match 0 event"]
    #[inline(always)]
    pub fn cc0_match(self) -> &'a mut W {
        self.variant(OUT0_A::CC0_MATCH)
    }
    #[doc = "Compare match 1 event"]
    #[inline(always)]
    pub fn cc1_match(self) -> &'a mut W {
        self.variant(OUT0_A::CC1_MATCH)
    }
    #[doc = "PWM output signal 'line_out'"]
    #[inline(always)]
    pub fn line_out(self) -> &'a mut W {
        self.variant(OUT0_A::LINE_OUT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(OUT0_A::RSVD6)
    }
    #[doc = "Output trigger disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OUT0_A::DISABLED)
    }
}
#[doc = "Field `OUT1` reader - Selects one of the internal events to generate the output trigger 1. Default setting selects the compare match 0 event."]
pub type OUT1_R = crate::FieldReader<u8, OUT1_A>;
#[doc = "Selects one of the internal events to generate the output trigger 1. Default setting selects the compare match 0 event.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT1_A {
    #[doc = "0: Overflow event"]
    OVERFLOW = 0,
    #[doc = "1: Underflow event"]
    UNDERFLOW = 1,
    #[doc = "2: Terminal count event"]
    TC = 2,
    #[doc = "3: Compare match 0 event (default selection)"]
    CC0_MATCH = 3,
    #[doc = "4: Compare match 1 event"]
    CC1_MATCH = 4,
    #[doc = "5: PWM output signal 'line_out'"]
    LINE_OUT = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: Output trigger disabled."]
    DISABLED = 7,
}
impl From<OUT1_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as _
    }
}
impl OUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT1_A {
        match self.bits {
            0 => OUT1_A::OVERFLOW,
            1 => OUT1_A::UNDERFLOW,
            2 => OUT1_A::TC,
            3 => OUT1_A::CC0_MATCH,
            4 => OUT1_A::CC1_MATCH,
            5 => OUT1_A::LINE_OUT,
            6 => OUT1_A::RSVD6,
            7 => OUT1_A::DISABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OUT1_A::OVERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == OUT1_A::UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `TC`"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == OUT1_A::TC
    }
    #[doc = "Checks if the value of the field is `CC0_MATCH`"]
    #[inline(always)]
    pub fn is_cc0_match(&self) -> bool {
        *self == OUT1_A::CC0_MATCH
    }
    #[doc = "Checks if the value of the field is `CC1_MATCH`"]
    #[inline(always)]
    pub fn is_cc1_match(&self) -> bool {
        *self == OUT1_A::CC1_MATCH
    }
    #[doc = "Checks if the value of the field is `LINE_OUT`"]
    #[inline(always)]
    pub fn is_line_out(&self) -> bool {
        *self == OUT1_A::LINE_OUT
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == OUT1_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUT1_A::DISABLED
    }
}
#[doc = "Field `OUT1` writer - Selects one of the internal events to generate the output trigger 1. Default setting selects the compare match 0 event."]
pub type OUT1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_OUT_SEL_SPEC, u8, OUT1_A, 3, O>;
impl<'a, const O: u8> OUT1_W<'a, O> {
    #[doc = "Overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut W {
        self.variant(OUT1_A::OVERFLOW)
    }
    #[doc = "Underflow event"]
    #[inline(always)]
    pub fn underflow(self) -> &'a mut W {
        self.variant(OUT1_A::UNDERFLOW)
    }
    #[doc = "Terminal count event"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut W {
        self.variant(OUT1_A::TC)
    }
    #[doc = "Compare match 0 event (default selection)"]
    #[inline(always)]
    pub fn cc0_match(self) -> &'a mut W {
        self.variant(OUT1_A::CC0_MATCH)
    }
    #[doc = "Compare match 1 event"]
    #[inline(always)]
    pub fn cc1_match(self) -> &'a mut W {
        self.variant(OUT1_A::CC1_MATCH)
    }
    #[doc = "PWM output signal 'line_out'"]
    #[inline(always)]
    pub fn line_out(self) -> &'a mut W {
        self.variant(OUT1_A::LINE_OUT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(OUT1_A::RSVD6)
    }
    #[doc = "Output trigger disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OUT1_A::DISABLED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects one of the internal events to generate the output trigger 0. Default setting selects the terminal count event."]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selects one of the internal events to generate the output trigger 1. Default setting selects the compare match 0 event."]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects one of the internal events to generate the output trigger 0. Default setting selects the terminal count event."]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> OUT0_W<0> {
        OUT0_W::new(self)
    }
    #[doc = "Bits 4:6 - Selects one of the internal events to generate the output trigger 1. Default setting selects the compare match 0 event."]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> OUT1_W<4> {
        OUT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter output trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_out_sel](index.html) module"]
pub struct TR_OUT_SEL_SPEC;
impl crate::RegisterSpec for TR_OUT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_out_sel::R](R) reader structure"]
impl crate::Readable for TR_OUT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_out_sel::W](W) writer structure"]
impl crate::Writable for TR_OUT_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_OUT_SEL to value 0x32"]
impl crate::Resettable for TR_OUT_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x32;
}
