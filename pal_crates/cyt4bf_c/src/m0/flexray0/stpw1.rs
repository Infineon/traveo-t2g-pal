#[doc = "Register `STPW1` reader"]
pub struct R(crate::R<STPW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STPW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STPW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STPW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STPW1` writer"]
pub struct W(crate::W<STPW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STPW1_SPEC>;
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
impl From<crate::W<STPW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STPW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESWT` reader - Enable Stop Watch Trigger If enabled an edge on input eray_stpwt or an interrupt 0,1 event (rising edge on pin eray_int0 or eray_int1) activates the stop watch. In single-shot mode this bit is reset to '0' after the actual cycle counter and macrotick value are stored in the Stop Watch register. 1 = Stop watch trigger enabled 0 = Stop watch trigger disabled"]
pub type ESWT_R = crate::BitReader<ESWT_A>;
#[doc = "Enable Stop Watch Trigger If enabled an edge on input eray_stpwt or an interrupt 0,1 event (rising edge on pin eray_int0 or eray_int1) activates the stop watch. In single-shot mode this bit is reset to '0' after the actual cycle counter and macrotick value are stored in the Stop Watch register. 1 = Stop watch trigger enabled 0 = Stop watch trigger disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESWT_A {
    #[doc = "0: N/A"]
    STOP_WATCH_TRIGGER_DISABLED = 0,
    #[doc = "1: N/A"]
    STOP_WATCH_TRIGGER_ENABLED = 1,
}
impl From<ESWT_A> for bool {
    #[inline(always)]
    fn from(variant: ESWT_A) -> Self {
        variant as u8 != 0
    }
}
impl ESWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESWT_A {
        match self.bits {
            false => ESWT_A::STOP_WATCH_TRIGGER_DISABLED,
            true => ESWT_A::STOP_WATCH_TRIGGER_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_TRIGGER_DISABLED`"]
    #[inline(always)]
    pub fn is_stop_watch_trigger_disabled(&self) -> bool {
        *self == ESWT_A::STOP_WATCH_TRIGGER_DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_TRIGGER_ENABLED`"]
    #[inline(always)]
    pub fn is_stop_watch_trigger_enabled(&self) -> bool {
        *self == ESWT_A::STOP_WATCH_TRIGGER_ENABLED
    }
}
#[doc = "Field `ESWT` writer - Enable Stop Watch Trigger If enabled an edge on input eray_stpwt or an interrupt 0,1 event (rising edge on pin eray_int0 or eray_int1) activates the stop watch. In single-shot mode this bit is reset to '0' after the actual cycle counter and macrotick value are stored in the Stop Watch register. 1 = Stop watch trigger enabled 0 = Stop watch trigger disabled"]
pub type ESWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STPW1_SPEC, ESWT_A, O>;
impl<'a, const O: u8> ESWT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_trigger_disabled(self) -> &'a mut W {
        self.variant(ESWT_A::STOP_WATCH_TRIGGER_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_trigger_enabled(self) -> &'a mut W {
        self.variant(ESWT_A::STOP_WATCH_TRIGGER_ENABLED)
    }
}
#[doc = "Field `SWMS` reader - Stop Watch Mode Select 1 = Continuous mode 0 = Single-shot mode"]
pub type SWMS_R = crate::BitReader<SWMS_A>;
#[doc = "Stop Watch Mode Select 1 = Continuous mode 0 = Single-shot mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWMS_A {
    #[doc = "0: N/A"]
    STOP_WATCH_SINGLE_SHOT_MODE = 0,
    #[doc = "1: N/A"]
    STOP_WATCH_CONTINUOUS_MODE = 1,
}
impl From<SWMS_A> for bool {
    #[inline(always)]
    fn from(variant: SWMS_A) -> Self {
        variant as u8 != 0
    }
}
impl SWMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWMS_A {
        match self.bits {
            false => SWMS_A::STOP_WATCH_SINGLE_SHOT_MODE,
            true => SWMS_A::STOP_WATCH_CONTINUOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_SINGLE_SHOT_MODE`"]
    #[inline(always)]
    pub fn is_stop_watch_single_shot_mode(&self) -> bool {
        *self == SWMS_A::STOP_WATCH_SINGLE_SHOT_MODE
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_CONTINUOUS_MODE`"]
    #[inline(always)]
    pub fn is_stop_watch_continuous_mode(&self) -> bool {
        *self == SWMS_A::STOP_WATCH_CONTINUOUS_MODE
    }
}
#[doc = "Field `SWMS` writer - Stop Watch Mode Select 1 = Continuous mode 0 = Single-shot mode"]
pub type SWMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STPW1_SPEC, SWMS_A, O>;
impl<'a, const O: u8> SWMS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_single_shot_mode(self) -> &'a mut W {
        self.variant(SWMS_A::STOP_WATCH_SINGLE_SHOT_MODE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_continuous_mode(self) -> &'a mut W {
        self.variant(SWMS_A::STOP_WATCH_CONTINUOUS_MODE)
    }
}
#[doc = "Field `EDGE` reader - Stop Watch Trigger Edge Select 1 = Rising edge 0 = Falling edge"]
pub type EDGE_R = crate::BitReader<EDGE_A>;
#[doc = "Stop Watch Trigger Edge Select 1 = Rising edge 0 = Falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE_A {
    #[doc = "0: N/A"]
    STOP_WATCH_FALLING_EDGE = 0,
    #[doc = "1: N/A"]
    STOP_WATCH_RISING_EDGE = 1,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::STOP_WATCH_FALLING_EDGE,
            true => EDGE_A::STOP_WATCH_RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_stop_watch_falling_edge(&self) -> bool {
        *self == EDGE_A::STOP_WATCH_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_stop_watch_rising_edge(&self) -> bool {
        *self == EDGE_A::STOP_WATCH_RISING_EDGE
    }
}
#[doc = "Field `EDGE` writer - Stop Watch Trigger Edge Select 1 = Rising edge 0 = Falling edge"]
pub type EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STPW1_SPEC, EDGE_A, O>;
impl<'a, const O: u8> EDGE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_falling_edge(self) -> &'a mut W {
        self.variant(EDGE_A::STOP_WATCH_FALLING_EDGE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_rising_edge(self) -> &'a mut W {
        self.variant(EDGE_A::STOP_WATCH_RISING_EDGE)
    }
}
#[doc = "Field `SSWT` reader - Software Stop Watch Trigger When the Host writes this bit to '1' the stop watch is activated. After the actual cycle counter and macrotick value are stored in the Stop Watch register this bit is reset to '0'. The bit is only writeable while ESWT = '0'. 1 = Stop watch activated by software trigger 0 = Software trigger reset"]
pub type SSWT_R = crate::BitReader<SSWT_A>;
#[doc = "Software Stop Watch Trigger When the Host writes this bit to '1' the stop watch is activated. After the actual cycle counter and macrotick value are stored in the Stop Watch register this bit is reset to '0'. The bit is only writeable while ESWT = '0'. 1 = Stop watch activated by software trigger 0 = Software trigger reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSWT_A {
    #[doc = "0: N/A"]
    STOP_WATCH_SOFTWARE_TRIGGER_RESET = 0,
    #[doc = "1: N/A"]
    STOP_WATCH_SOFTWARE_TRIGGER_ACTIVATED = 1,
}
impl From<SSWT_A> for bool {
    #[inline(always)]
    fn from(variant: SSWT_A) -> Self {
        variant as u8 != 0
    }
}
impl SSWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSWT_A {
        match self.bits {
            false => SSWT_A::STOP_WATCH_SOFTWARE_TRIGGER_RESET,
            true => SSWT_A::STOP_WATCH_SOFTWARE_TRIGGER_ACTIVATED,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_SOFTWARE_TRIGGER_RESET`"]
    #[inline(always)]
    pub fn is_stop_watch_software_trigger_reset(&self) -> bool {
        *self == SSWT_A::STOP_WATCH_SOFTWARE_TRIGGER_RESET
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_SOFTWARE_TRIGGER_ACTIVATED`"]
    #[inline(always)]
    pub fn is_stop_watch_software_trigger_activated(&self) -> bool {
        *self == SSWT_A::STOP_WATCH_SOFTWARE_TRIGGER_ACTIVATED
    }
}
#[doc = "Field `SSWT` writer - Software Stop Watch Trigger When the Host writes this bit to '1' the stop watch is activated. After the actual cycle counter and macrotick value are stored in the Stop Watch register this bit is reset to '0'. The bit is only writeable while ESWT = '0'. 1 = Stop watch activated by software trigger 0 = Software trigger reset"]
pub type SSWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STPW1_SPEC, SSWT_A, O>;
impl<'a, const O: u8> SSWT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_software_trigger_reset(self) -> &'a mut W {
        self.variant(SSWT_A::STOP_WATCH_SOFTWARE_TRIGGER_RESET)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_software_trigger_activated(self) -> &'a mut W {
        self.variant(SSWT_A::STOP_WATCH_SOFTWARE_TRIGGER_ACTIVATED)
    }
}
#[doc = "Field `EETP` reader - Enable External Trigger Pin Enables stop watch trigger event via pin eray_stpwt if ESWT = '1'. 1 = Edge on pin eray_stpwt triggers stop watch 0 = Stop watch trigger via pin eray_stpwt disabled"]
pub type EETP_R = crate::BitReader<EETP_A>;
#[doc = "Enable External Trigger Pin Enables stop watch trigger event via pin eray_stpwt if ESWT = '1'. 1 = Edge on pin eray_stpwt triggers stop watch 0 = Stop watch trigger via pin eray_stpwt disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EETP_A {
    #[doc = "0: N/A"]
    ESTPWT_DISABLED = 0,
    #[doc = "1: N/A"]
    ESTPWT_ENABLED = 1,
}
impl From<EETP_A> for bool {
    #[inline(always)]
    fn from(variant: EETP_A) -> Self {
        variant as u8 != 0
    }
}
impl EETP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EETP_A {
        match self.bits {
            false => EETP_A::ESTPWT_DISABLED,
            true => EETP_A::ESTPWT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ESTPWT_DISABLED`"]
    #[inline(always)]
    pub fn is_estpwt_disabled(&self) -> bool {
        *self == EETP_A::ESTPWT_DISABLED
    }
    #[doc = "Checks if the value of the field is `ESTPWT_ENABLED`"]
    #[inline(always)]
    pub fn is_estpwt_enabled(&self) -> bool {
        *self == EETP_A::ESTPWT_ENABLED
    }
}
#[doc = "Field `EETP` writer - Enable External Trigger Pin Enables stop watch trigger event via pin eray_stpwt if ESWT = '1'. 1 = Edge on pin eray_stpwt triggers stop watch 0 = Stop watch trigger via pin eray_stpwt disabled"]
pub type EETP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STPW1_SPEC, EETP_A, O>;
impl<'a, const O: u8> EETP_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn estpwt_disabled(self) -> &'a mut W {
        self.variant(EETP_A::ESTPWT_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn estpwt_enabled(self) -> &'a mut W {
        self.variant(EETP_A::ESTPWT_ENABLED)
    }
}
#[doc = "Field `EINT0` reader - Enable Interrupt 0 Trigger Enables stop watch trigger by interrupt 0 event if ESWT = '1'. 1 = Interrupt 0 event triggers stop watch 0 = Stop watch trigger by interrupt 0 disabled"]
pub type EINT0_R = crate::BitReader<EINT0_A>;
#[doc = "Enable Interrupt 0 Trigger Enables stop watch trigger by interrupt 0 event if ESWT = '1'. 1 = Interrupt 0 event triggers stop watch 0 = Stop watch trigger by interrupt 0 disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT0_A {
    #[doc = "0: N/A"]
    STPWT_TRIGGER_BY_INT0_DISABLED = 0,
    #[doc = "1: N/A"]
    STPWT_TRIGGER_BY_INT0_ENABLED = 1,
}
impl From<EINT0_A> for bool {
    #[inline(always)]
    fn from(variant: EINT0_A) -> Self {
        variant as u8 != 0
    }
}
impl EINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT0_A {
        match self.bits {
            false => EINT0_A::STPWT_TRIGGER_BY_INT0_DISABLED,
            true => EINT0_A::STPWT_TRIGGER_BY_INT0_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `STPWT_TRIGGER_BY_INT0_DISABLED`"]
    #[inline(always)]
    pub fn is_stpwt_trigger_by_int0_disabled(&self) -> bool {
        *self == EINT0_A::STPWT_TRIGGER_BY_INT0_DISABLED
    }
    #[doc = "Checks if the value of the field is `STPWT_TRIGGER_BY_INT0_ENABLED`"]
    #[inline(always)]
    pub fn is_stpwt_trigger_by_int0_enabled(&self) -> bool {
        *self == EINT0_A::STPWT_TRIGGER_BY_INT0_ENABLED
    }
}
#[doc = "Field `EINT0` writer - Enable Interrupt 0 Trigger Enables stop watch trigger by interrupt 0 event if ESWT = '1'. 1 = Interrupt 0 event triggers stop watch 0 = Stop watch trigger by interrupt 0 disabled"]
pub type EINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STPW1_SPEC, EINT0_A, O>;
impl<'a, const O: u8> EINT0_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stpwt_trigger_by_int0_disabled(self) -> &'a mut W {
        self.variant(EINT0_A::STPWT_TRIGGER_BY_INT0_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stpwt_trigger_by_int0_enabled(self) -> &'a mut W {
        self.variant(EINT0_A::STPWT_TRIGGER_BY_INT0_ENABLED)
    }
}
#[doc = "Field `EINT1` reader - Enable Interrupt 1 Trigger Enables stop watch trigger by interrupt 1event if ESWT = '1'. 1 = Interrupt 1 event triggers stop watch 0 = Stop watch trigger by interrupt 1 disabled"]
pub type EINT1_R = crate::BitReader<EINT1_A>;
#[doc = "Enable Interrupt 1 Trigger Enables stop watch trigger by interrupt 1event if ESWT = '1'. 1 = Interrupt 1 event triggers stop watch 0 = Stop watch trigger by interrupt 1 disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT1_A {
    #[doc = "0: N/A"]
    STPWT_TRIGGER_BY_INT1_DISABLED = 0,
    #[doc = "1: N/A"]
    STPWT_TRIGGER_BY_INT1_ENABLED = 1,
}
impl From<EINT1_A> for bool {
    #[inline(always)]
    fn from(variant: EINT1_A) -> Self {
        variant as u8 != 0
    }
}
impl EINT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT1_A {
        match self.bits {
            false => EINT1_A::STPWT_TRIGGER_BY_INT1_DISABLED,
            true => EINT1_A::STPWT_TRIGGER_BY_INT1_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `STPWT_TRIGGER_BY_INT1_DISABLED`"]
    #[inline(always)]
    pub fn is_stpwt_trigger_by_int1_disabled(&self) -> bool {
        *self == EINT1_A::STPWT_TRIGGER_BY_INT1_DISABLED
    }
    #[doc = "Checks if the value of the field is `STPWT_TRIGGER_BY_INT1_ENABLED`"]
    #[inline(always)]
    pub fn is_stpwt_trigger_by_int1_enabled(&self) -> bool {
        *self == EINT1_A::STPWT_TRIGGER_BY_INT1_ENABLED
    }
}
#[doc = "Field `EINT1` writer - Enable Interrupt 1 Trigger Enables stop watch trigger by interrupt 1event if ESWT = '1'. 1 = Interrupt 1 event triggers stop watch 0 = Stop watch trigger by interrupt 1 disabled"]
pub type EINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STPW1_SPEC, EINT1_A, O>;
impl<'a, const O: u8> EINT1_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stpwt_trigger_by_int1_disabled(self) -> &'a mut W {
        self.variant(EINT1_A::STPWT_TRIGGER_BY_INT1_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stpwt_trigger_by_int1_enabled(self) -> &'a mut W {
        self.variant(EINT1_A::STPWT_TRIGGER_BY_INT1_ENABLED)
    }
}
#[doc = "Field `SCCV` reader - Stop Watch Captured Cycle Counter Value State of the cycle counter when the stop watch event occurred. Valid values are 0 to 63."]
pub type SCCV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMTV` reader - Stop Watch Captured Macrotick Value State of the macrotick counter when the stop watch event occurred. Valid values are 0 to 15999."]
pub type SMTV_R = crate::FieldReader<u16, SMTV_A>;
#[doc = "Stop Watch Captured Macrotick Value State of the macrotick counter when the stop watch event occurred. Valid values are 0 to 15999.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SMTV_A {
    #[doc = "15999: N/A"]
    MAX = 15999,
}
impl From<SMTV_A> for u16 {
    #[inline(always)]
    fn from(variant: SMTV_A) -> Self {
        variant as _
    }
}
impl SMTV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMTV_A> {
        match self.bits {
            15999 => Some(SMTV_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == SMTV_A::MAX
    }
}
impl R {
    #[doc = "Bit 0 - Enable Stop Watch Trigger If enabled an edge on input eray_stpwt or an interrupt 0,1 event (rising edge on pin eray_int0 or eray_int1) activates the stop watch. In single-shot mode this bit is reset to '0' after the actual cycle counter and macrotick value are stored in the Stop Watch register. 1 = Stop watch trigger enabled 0 = Stop watch trigger disabled"]
    #[inline(always)]
    pub fn eswt(&self) -> ESWT_R {
        ESWT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop Watch Mode Select 1 = Continuous mode 0 = Single-shot mode"]
    #[inline(always)]
    pub fn swms(&self) -> SWMS_R {
        SWMS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Watch Trigger Edge Select 1 = Rising edge 0 = Falling edge"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Stop Watch Trigger When the Host writes this bit to '1' the stop watch is activated. After the actual cycle counter and macrotick value are stored in the Stop Watch register this bit is reset to '0'. The bit is only writeable while ESWT = '0'. 1 = Stop watch activated by software trigger 0 = Software trigger reset"]
    #[inline(always)]
    pub fn sswt(&self) -> SSWT_R {
        SSWT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External Trigger Pin Enables stop watch trigger event via pin eray_stpwt if ESWT = '1'. 1 = Edge on pin eray_stpwt triggers stop watch 0 = Stop watch trigger via pin eray_stpwt disabled"]
    #[inline(always)]
    pub fn eetp(&self) -> EETP_R {
        EETP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Interrupt 0 Trigger Enables stop watch trigger by interrupt 0 event if ESWT = '1'. 1 = Interrupt 0 event triggers stop watch 0 = Stop watch trigger by interrupt 0 disabled"]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Interrupt 1 Trigger Enables stop watch trigger by interrupt 1event if ESWT = '1'. 1 = Interrupt 1 event triggers stop watch 0 = Stop watch trigger by interrupt 1 disabled"]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Stop Watch Captured Cycle Counter Value State of the cycle counter when the stop watch event occurred. Valid values are 0 to 63."]
    #[inline(always)]
    pub fn sccv(&self) -> SCCV_R {
        SCCV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:29 - Stop Watch Captured Macrotick Value State of the macrotick counter when the stop watch event occurred. Valid values are 0 to 15999."]
    #[inline(always)]
    pub fn smtv(&self) -> SMTV_R {
        SMTV_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Stop Watch Trigger If enabled an edge on input eray_stpwt or an interrupt 0,1 event (rising edge on pin eray_int0 or eray_int1) activates the stop watch. In single-shot mode this bit is reset to '0' after the actual cycle counter and macrotick value are stored in the Stop Watch register. 1 = Stop watch trigger enabled 0 = Stop watch trigger disabled"]
    #[inline(always)]
    #[must_use]
    pub fn eswt(&mut self) -> ESWT_W<0> {
        ESWT_W::new(self)
    }
    #[doc = "Bit 1 - Stop Watch Mode Select 1 = Continuous mode 0 = Single-shot mode"]
    #[inline(always)]
    #[must_use]
    pub fn swms(&mut self) -> SWMS_W<1> {
        SWMS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Watch Trigger Edge Select 1 = Rising edge 0 = Falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<2> {
        EDGE_W::new(self)
    }
    #[doc = "Bit 3 - Software Stop Watch Trigger When the Host writes this bit to '1' the stop watch is activated. After the actual cycle counter and macrotick value are stored in the Stop Watch register this bit is reset to '0'. The bit is only writeable while ESWT = '0'. 1 = Stop watch activated by software trigger 0 = Software trigger reset"]
    #[inline(always)]
    #[must_use]
    pub fn sswt(&mut self) -> SSWT_W<3> {
        SSWT_W::new(self)
    }
    #[doc = "Bit 4 - Enable External Trigger Pin Enables stop watch trigger event via pin eray_stpwt if ESWT = '1'. 1 = Edge on pin eray_stpwt triggers stop watch 0 = Stop watch trigger via pin eray_stpwt disabled"]
    #[inline(always)]
    #[must_use]
    pub fn eetp(&mut self) -> EETP_W<4> {
        EETP_W::new(self)
    }
    #[doc = "Bit 5 - Enable Interrupt 0 Trigger Enables stop watch trigger by interrupt 0 event if ESWT = '1'. 1 = Interrupt 0 event triggers stop watch 0 = Stop watch trigger by interrupt 0 disabled"]
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> EINT0_W<5> {
        EINT0_W::new(self)
    }
    #[doc = "Bit 6 - Enable Interrupt 1 Trigger Enables stop watch trigger by interrupt 1event if ESWT = '1'. 1 = Interrupt 1 event triggers stop watch 0 = Stop watch trigger by interrupt 1 disabled"]
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> EINT1_W<6> {
        EINT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop Watch Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stpw1](index.html) module"]
pub struct STPW1_SPEC;
impl crate::RegisterSpec for STPW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stpw1::R](R) reader structure"]
impl crate::Readable for STPW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stpw1::W](W) writer structure"]
impl crate::Writable for STPW1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STPW1 to value 0"]
impl crate::Resettable for STPW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
