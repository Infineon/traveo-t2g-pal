#[doc = "Register `TR_IN_EDGE_SEL` reader"]
pub struct R(crate::R<TR_IN_EDGE_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_IN_EDGE_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_IN_EDGE_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_IN_EDGE_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_IN_EDGE_SEL` writer"]
pub struct W(crate::W<TR_IN_EDGE_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_IN_EDGE_SEL_SPEC>;
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
impl From<crate::W<TR_IN_EDGE_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_IN_EDGE_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTURE0_EDGE` reader - A capture 0 event will copy the counter value into the CC0 register."]
pub type CAPTURE0_EDGE_R = crate::FieldReader<u8, CAPTURE0_EDGE_A>;
#[doc = "A capture 0 event will copy the counter value into the CC0 register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTURE0_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    ANY_EDGE = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<CAPTURE0_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTURE0_EDGE_A) -> Self {
        variant as _
    }
}
impl CAPTURE0_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE0_EDGE_A {
        match self.bits {
            0 => CAPTURE0_EDGE_A::RISING_EDGE,
            1 => CAPTURE0_EDGE_A::FALLING_EDGE,
            2 => CAPTURE0_EDGE_A::ANY_EDGE,
            3 => CAPTURE0_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CAPTURE0_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CAPTURE0_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == CAPTURE0_EDGE_A::ANY_EDGE
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == CAPTURE0_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `CAPTURE0_EDGE` writer - A capture 0 event will copy the counter value into the CC0 register."]
pub type CAPTURE0_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_IN_EDGE_SEL_SPEC, u8, CAPTURE0_EDGE_A, 2, O>;
impl<'a, const O: u8> CAPTURE0_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CAPTURE0_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CAPTURE0_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(CAPTURE0_EDGE_A::ANY_EDGE)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(CAPTURE0_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "Field `COUNT_EDGE` reader - A counter event will increase or decrease the counter by '1'."]
pub type COUNT_EDGE_R = crate::FieldReader<u8, COUNT_EDGE_A>;
#[doc = "A counter event will increase or decrease the counter by '1'.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COUNT_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    ANY_EDGE = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<COUNT_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: COUNT_EDGE_A) -> Self {
        variant as _
    }
}
impl COUNT_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COUNT_EDGE_A {
        match self.bits {
            0 => COUNT_EDGE_A::RISING_EDGE,
            1 => COUNT_EDGE_A::FALLING_EDGE,
            2 => COUNT_EDGE_A::ANY_EDGE,
            3 => COUNT_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == COUNT_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == COUNT_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == COUNT_EDGE_A::ANY_EDGE
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == COUNT_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `COUNT_EDGE` writer - A counter event will increase or decrease the counter by '1'."]
pub type COUNT_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_IN_EDGE_SEL_SPEC, u8, COUNT_EDGE_A, 2, O>;
impl<'a, const O: u8> COUNT_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(COUNT_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(COUNT_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(COUNT_EDGE_A::ANY_EDGE)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(COUNT_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "Field `RELOAD_EDGE` reader - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
pub type RELOAD_EDGE_R = crate::FieldReader<u8, RELOAD_EDGE_A>;
#[doc = "A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RELOAD_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    ANY_EDGE = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<RELOAD_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RELOAD_EDGE_A) -> Self {
        variant as _
    }
}
impl RELOAD_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_EDGE_A {
        match self.bits {
            0 => RELOAD_EDGE_A::RISING_EDGE,
            1 => RELOAD_EDGE_A::FALLING_EDGE,
            2 => RELOAD_EDGE_A::ANY_EDGE,
            3 => RELOAD_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == RELOAD_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == RELOAD_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == RELOAD_EDGE_A::ANY_EDGE
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == RELOAD_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `RELOAD_EDGE` writer - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
pub type RELOAD_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_IN_EDGE_SEL_SPEC, u8, RELOAD_EDGE_A, 2, O>;
impl<'a, const O: u8> RELOAD_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(RELOAD_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(RELOAD_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(RELOAD_EDGE_A::ANY_EDGE)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(RELOAD_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "Field `STOP_EDGE` reader - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
pub type STOP_EDGE_R = crate::FieldReader<u8, STOP_EDGE_A>;
#[doc = "A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    ANY_EDGE = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<STOP_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_EDGE_A) -> Self {
        variant as _
    }
}
impl STOP_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_EDGE_A {
        match self.bits {
            0 => STOP_EDGE_A::RISING_EDGE,
            1 => STOP_EDGE_A::FALLING_EDGE,
            2 => STOP_EDGE_A::ANY_EDGE,
            3 => STOP_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == STOP_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == STOP_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == STOP_EDGE_A::ANY_EDGE
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == STOP_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `STOP_EDGE` writer - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
pub type STOP_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_IN_EDGE_SEL_SPEC, u8, STOP_EDGE_A, 2, O>;
impl<'a, const O: u8> STOP_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(STOP_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(STOP_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(STOP_EDGE_A::ANY_EDGE)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(STOP_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "Field `START_EDGE` reader - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
pub type START_EDGE_R = crate::FieldReader<u8, START_EDGE_A>;
#[doc = "A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum START_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    ANY_EDGE = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<START_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: START_EDGE_A) -> Self {
        variant as _
    }
}
impl START_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_EDGE_A {
        match self.bits {
            0 => START_EDGE_A::RISING_EDGE,
            1 => START_EDGE_A::FALLING_EDGE,
            2 => START_EDGE_A::ANY_EDGE,
            3 => START_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == START_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == START_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == START_EDGE_A::ANY_EDGE
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == START_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `START_EDGE` writer - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
pub type START_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_IN_EDGE_SEL_SPEC, u8, START_EDGE_A, 2, O>;
impl<'a, const O: u8> START_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(START_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(START_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(START_EDGE_A::ANY_EDGE)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(START_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "Field `CAPTURE1_EDGE` reader - A capture 1 event will copy the counter value into the CC1 register."]
pub type CAPTURE1_EDGE_R = crate::FieldReader<u8, CAPTURE1_EDGE_A>;
#[doc = "A capture 1 event will copy the counter value into the CC1 register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTURE1_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    ANY_EDGE = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<CAPTURE1_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTURE1_EDGE_A) -> Self {
        variant as _
    }
}
impl CAPTURE1_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE1_EDGE_A {
        match self.bits {
            0 => CAPTURE1_EDGE_A::RISING_EDGE,
            1 => CAPTURE1_EDGE_A::FALLING_EDGE,
            2 => CAPTURE1_EDGE_A::ANY_EDGE,
            3 => CAPTURE1_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CAPTURE1_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CAPTURE1_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `ANY_EDGE`"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == CAPTURE1_EDGE_A::ANY_EDGE
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == CAPTURE1_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `CAPTURE1_EDGE` writer - A capture 1 event will copy the counter value into the CC1 register."]
pub type CAPTURE1_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_IN_EDGE_SEL_SPEC, u8, CAPTURE1_EDGE_A, 2, O>;
impl<'a, const O: u8> CAPTURE1_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CAPTURE1_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CAPTURE1_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut W {
        self.variant(CAPTURE1_EDGE_A::ANY_EDGE)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(CAPTURE1_EDGE_A::NO_EDGE_DET)
    }
}
impl R {
    #[doc = "Bits 0:1 - A capture 0 event will copy the counter value into the CC0 register."]
    #[inline(always)]
    pub fn capture0_edge(&self) -> CAPTURE0_EDGE_R {
        CAPTURE0_EDGE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub fn count_edge(&self) -> COUNT_EDGE_R {
        COUNT_EDGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub fn reload_edge(&self) -> RELOAD_EDGE_R {
        RELOAD_EDGE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub fn stop_edge(&self) -> STOP_EDGE_R {
        STOP_EDGE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub fn start_edge(&self) -> START_EDGE_R {
        START_EDGE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - A capture 1 event will copy the counter value into the CC1 register."]
    #[inline(always)]
    pub fn capture1_edge(&self) -> CAPTURE1_EDGE_R {
        CAPTURE1_EDGE_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - A capture 0 event will copy the counter value into the CC0 register."]
    #[inline(always)]
    #[must_use]
    pub fn capture0_edge(&mut self) -> CAPTURE0_EDGE_W<0> {
        CAPTURE0_EDGE_W::new(self)
    }
    #[doc = "Bits 2:3 - A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    #[must_use]
    pub fn count_edge(&mut self) -> COUNT_EDGE_W<2> {
        COUNT_EDGE_W::new(self)
    }
    #[doc = "Bits 4:5 - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    #[must_use]
    pub fn reload_edge(&mut self) -> RELOAD_EDGE_W<4> {
        RELOAD_EDGE_W::new(self)
    }
    #[doc = "Bits 6:7 - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    #[must_use]
    pub fn stop_edge(&mut self) -> STOP_EDGE_W<6> {
        STOP_EDGE_W::new(self)
    }
    #[doc = "Bits 8:9 - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    #[must_use]
    pub fn start_edge(&mut self) -> START_EDGE_W<8> {
        START_EDGE_W::new(self)
    }
    #[doc = "Bits 10:11 - A capture 1 event will copy the counter value into the CC1 register."]
    #[inline(always)]
    #[must_use]
    pub fn capture1_edge(&mut self) -> CAPTURE1_EDGE_W<10> {
        CAPTURE1_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter input trigger edge selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_in_edge_sel](index.html) module"]
pub struct TR_IN_EDGE_SEL_SPEC;
impl crate::RegisterSpec for TR_IN_EDGE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_in_edge_sel::R](R) reader structure"]
impl crate::Readable for TR_IN_EDGE_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_in_edge_sel::W](W) writer structure"]
impl crate::Writable for TR_IN_EDGE_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_IN_EDGE_SEL to value 0x0fff"]
impl crate::Resettable for TR_IN_EDGE_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
