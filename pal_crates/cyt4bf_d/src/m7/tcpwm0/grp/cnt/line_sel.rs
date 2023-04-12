#[doc = "Register `LINE_SEL` reader"]
pub struct R(crate::R<LINE_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINE_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINE_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINE_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINE_SEL` writer"]
pub struct W(crate::W<LINE_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINE_SEL_SPEC>;
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
impl From<crate::W<LINE_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINE_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_SEL` reader - Selects the source for the output signal 'line_out'. Default setting is the PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[0\\])."]
pub type OUT_SEL_R = crate::FieldReader<u8, OUT_SEL_A>;
#[doc = "Selects the source for the output signal 'line_out'. Default setting is the PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[0\\]).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT_SEL_A {
    #[doc = "0: fixed '0'"]
    L = 0,
    #[doc = "1: fixed '1'"]
    H = 1,
    #[doc = "2: PWM signal 'line'"]
    PWM = 2,
    #[doc = "3: inverted PWM signal 'line'"]
    PWM_INV = 3,
    #[doc = "4: The output 'line_out' is not driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance). Note: This is realized by driving the output 'line_out_en' to 0."]
    Z = 4,
    #[doc = "5: N/A"]
    RSVD5 = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: N/A"]
    RSVD7 = 7,
}
impl From<OUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_SEL_A) -> Self {
        variant as _
    }
}
impl OUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT_SEL_A {
        match self.bits {
            0 => OUT_SEL_A::L,
            1 => OUT_SEL_A::H,
            2 => OUT_SEL_A::PWM,
            3 => OUT_SEL_A::PWM_INV,
            4 => OUT_SEL_A::Z,
            5 => OUT_SEL_A::RSVD5,
            6 => OUT_SEL_A::RSVD6,
            7 => OUT_SEL_A::RSVD7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == OUT_SEL_A::L
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == OUT_SEL_A::H
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == OUT_SEL_A::PWM
    }
    #[doc = "Checks if the value of the field is `PWM_INV`"]
    #[inline(always)]
    pub fn is_pwm_inv(&self) -> bool {
        *self == OUT_SEL_A::PWM_INV
    }
    #[doc = "Checks if the value of the field is `Z`"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == OUT_SEL_A::Z
    }
    #[doc = "Checks if the value of the field is `RSVD5`"]
    #[inline(always)]
    pub fn is_rsvd5(&self) -> bool {
        *self == OUT_SEL_A::RSVD5
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == OUT_SEL_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `RSVD7`"]
    #[inline(always)]
    pub fn is_rsvd7(&self) -> bool {
        *self == OUT_SEL_A::RSVD7
    }
}
#[doc = "Field `OUT_SEL` writer - Selects the source for the output signal 'line_out'. Default setting is the PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[0\\])."]
pub type OUT_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LINE_SEL_SPEC, u8, OUT_SEL_A, 3, O>;
impl<'a, const O: u8> OUT_SEL_W<'a, O> {
    #[doc = "fixed '0'"]
    #[inline(always)]
    pub fn l(self) -> &'a mut W {
        self.variant(OUT_SEL_A::L)
    }
    #[doc = "fixed '1'"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(OUT_SEL_A::H)
    }
    #[doc = "PWM signal 'line'"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(OUT_SEL_A::PWM)
    }
    #[doc = "inverted PWM signal 'line'"]
    #[inline(always)]
    pub fn pwm_inv(self) -> &'a mut W {
        self.variant(OUT_SEL_A::PWM_INV)
    }
    #[doc = "The output 'line_out' is not driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance). Note: This is realized by driving the output 'line_out_en' to 0."]
    #[inline(always)]
    pub fn z(self) -> &'a mut W {
        self.variant(OUT_SEL_A::Z)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd5(self) -> &'a mut W {
        self.variant(OUT_SEL_A::RSVD5)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(OUT_SEL_A::RSVD6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd7(self) -> &'a mut W {
        self.variant(OUT_SEL_A::RSVD7)
    }
}
#[doc = "Field `COMPL_OUT_SEL` reader - Selects the source for the output signal 'line_compl_out'. Default setting is the inverted PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_compl_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[1\\])."]
pub type COMPL_OUT_SEL_R = crate::FieldReader<u8, COMPL_OUT_SEL_A>;
#[doc = "Selects the source for the output signal 'line_compl_out'. Default setting is the inverted PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_compl_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[1\\]).\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMPL_OUT_SEL_A {
    #[doc = "0: fixed '0'"]
    L = 0,
    #[doc = "1: fixed '1'"]
    H = 1,
    #[doc = "2: PWM signal 'line'"]
    PWM = 2,
    #[doc = "3: inverted PWM signal 'line'"]
    PWM_INV = 3,
    #[doc = "4: The output 'line_compl_out' is not driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance). Note: This is realized by driving the output 'line_compl_out_en' to 0."]
    Z = 4,
    #[doc = "5: N/A"]
    RSVD5 = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: N/A"]
    RSVD7 = 7,
}
impl From<COMPL_OUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMPL_OUT_SEL_A) -> Self {
        variant as _
    }
}
impl COMPL_OUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPL_OUT_SEL_A {
        match self.bits {
            0 => COMPL_OUT_SEL_A::L,
            1 => COMPL_OUT_SEL_A::H,
            2 => COMPL_OUT_SEL_A::PWM,
            3 => COMPL_OUT_SEL_A::PWM_INV,
            4 => COMPL_OUT_SEL_A::Z,
            5 => COMPL_OUT_SEL_A::RSVD5,
            6 => COMPL_OUT_SEL_A::RSVD6,
            7 => COMPL_OUT_SEL_A::RSVD7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L`"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == COMPL_OUT_SEL_A::L
    }
    #[doc = "Checks if the value of the field is `H`"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == COMPL_OUT_SEL_A::H
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == COMPL_OUT_SEL_A::PWM
    }
    #[doc = "Checks if the value of the field is `PWM_INV`"]
    #[inline(always)]
    pub fn is_pwm_inv(&self) -> bool {
        *self == COMPL_OUT_SEL_A::PWM_INV
    }
    #[doc = "Checks if the value of the field is `Z`"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        *self == COMPL_OUT_SEL_A::Z
    }
    #[doc = "Checks if the value of the field is `RSVD5`"]
    #[inline(always)]
    pub fn is_rsvd5(&self) -> bool {
        *self == COMPL_OUT_SEL_A::RSVD5
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == COMPL_OUT_SEL_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `RSVD7`"]
    #[inline(always)]
    pub fn is_rsvd7(&self) -> bool {
        *self == COMPL_OUT_SEL_A::RSVD7
    }
}
#[doc = "Field `COMPL_OUT_SEL` writer - Selects the source for the output signal 'line_compl_out'. Default setting is the inverted PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_compl_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[1\\])."]
pub type COMPL_OUT_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LINE_SEL_SPEC, u8, COMPL_OUT_SEL_A, 3, O>;
impl<'a, const O: u8> COMPL_OUT_SEL_W<'a, O> {
    #[doc = "fixed '0'"]
    #[inline(always)]
    pub fn l(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::L)
    }
    #[doc = "fixed '1'"]
    #[inline(always)]
    pub fn h(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::H)
    }
    #[doc = "PWM signal 'line'"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::PWM)
    }
    #[doc = "inverted PWM signal 'line'"]
    #[inline(always)]
    pub fn pwm_inv(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::PWM_INV)
    }
    #[doc = "The output 'line_compl_out' is not driven by the TCPWM. Instead the port default level configuration applies, e.g. 'Z' (high impedance). Note: This is realized by driving the output 'line_compl_out_en' to 0."]
    #[inline(always)]
    pub fn z(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::Z)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd5(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::RSVD5)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::RSVD6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd7(self) -> &'a mut W {
        self.variant(COMPL_OUT_SEL_A::RSVD7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the source for the output signal 'line_out'. Default setting is the PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[0\\])."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selects the source for the output signal 'line_compl_out'. Default setting is the inverted PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_compl_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[1\\])."]
    #[inline(always)]
    pub fn compl_out_sel(&self) -> COMPL_OUT_SEL_R {
        COMPL_OUT_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the source for the output signal 'line_out'. Default setting is the PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<0> {
        OUT_SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Selects the source for the output signal 'line_compl_out'. Default setting is the inverted PWM signal 'line'. Other settings are useful for Stepper Motor Control. This field has a function in PWM and PWM_PR modes only. Note: The output signal of this selection can be further modified by the stop / kill logic and line_compl_out polarity setting (CTRL.QUAD_ENCODING_MODE\\[1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn compl_out_sel(&mut self) -> COMPL_OUT_SEL_W<4> {
        COMPL_OUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter line selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [line_sel](index.html) module"]
pub struct LINE_SEL_SPEC;
impl crate::RegisterSpec for LINE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [line_sel::R](R) reader structure"]
impl crate::Readable for LINE_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [line_sel::W](W) writer structure"]
impl crate::Writable for LINE_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINE_SEL to value 0x32"]
impl crate::Resettable for LINE_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x32;
}
