#[doc = "Register `DT` reader"]
pub struct R(crate::R<DT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT` writer"]
pub struct W(crate::W<DT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT_SPEC>;
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
impl From<crate::W<DT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT_LINE_OUT_L` reader - In PWM_DT mode, this field is used to determine the low byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock. Note: This field determines the low byte of the 16-bit dead time before activating 'line_out' when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by this DT_LINE_OUT_L field is used before activating 'line_out' and 'line_compl_out'."]
pub type DT_LINE_OUT_L_R = crate::FieldReader<u8, DT_LINE_OUT_L_A>;
#[doc = "In PWM_DT mode, this field is used to determine the low byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock. Note: This field determines the low byte of the 16-bit dead time before activating 'line_out' when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by this DT_LINE_OUT_L field is used before activating 'line_out' and 'line_compl_out'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DT_LINE_OUT_L_A {
    #[doc = "0: Divide by 1 (other-than-PWM_DT mode)"]
    DIVBY1 = 0,
    #[doc = "1: Divide by 2 (other-than-PWM_DT mode)"]
    DIVBY2 = 1,
    #[doc = "2: Divide by 4 (other-than-PWM_DT mode)"]
    DIVBY4 = 2,
    #[doc = "3: Divide by 8 (other-than-PWM_DT mode)"]
    DIVBY8 = 3,
    #[doc = "4: Divide by 16 (other-than-PWM_DT mode)"]
    DIVBY16 = 4,
    #[doc = "5: Divide by 32 (other-than-PWM_DT mode)"]
    DIVBY32 = 5,
    #[doc = "6: Divide by 64 (other-than-PWM_DT mode)"]
    DIVBY64 = 6,
    #[doc = "7: Divide by 128 (other-than-PWM_DT mode)"]
    DIVBY128 = 7,
}
impl From<DT_LINE_OUT_L_A> for u8 {
    #[inline(always)]
    fn from(variant: DT_LINE_OUT_L_A) -> Self {
        variant as _
    }
}
impl DT_LINE_OUT_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DT_LINE_OUT_L_A> {
        match self.bits {
            0 => Some(DT_LINE_OUT_L_A::DIVBY1),
            1 => Some(DT_LINE_OUT_L_A::DIVBY2),
            2 => Some(DT_LINE_OUT_L_A::DIVBY4),
            3 => Some(DT_LINE_OUT_L_A::DIVBY8),
            4 => Some(DT_LINE_OUT_L_A::DIVBY16),
            5 => Some(DT_LINE_OUT_L_A::DIVBY32),
            6 => Some(DT_LINE_OUT_L_A::DIVBY64),
            7 => Some(DT_LINE_OUT_L_A::DIVBY128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIVBY1`"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == DT_LINE_OUT_L_A::DIVBY1
    }
    #[doc = "Checks if the value of the field is `DIVBY2`"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == DT_LINE_OUT_L_A::DIVBY2
    }
    #[doc = "Checks if the value of the field is `DIVBY4`"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == DT_LINE_OUT_L_A::DIVBY4
    }
    #[doc = "Checks if the value of the field is `DIVBY8`"]
    #[inline(always)]
    pub fn is_divby8(&self) -> bool {
        *self == DT_LINE_OUT_L_A::DIVBY8
    }
    #[doc = "Checks if the value of the field is `DIVBY16`"]
    #[inline(always)]
    pub fn is_divby16(&self) -> bool {
        *self == DT_LINE_OUT_L_A::DIVBY16
    }
    #[doc = "Checks if the value of the field is `DIVBY32`"]
    #[inline(always)]
    pub fn is_divby32(&self) -> bool {
        *self == DT_LINE_OUT_L_A::DIVBY32
    }
    #[doc = "Checks if the value of the field is `DIVBY64`"]
    #[inline(always)]
    pub fn is_divby64(&self) -> bool {
        *self == DT_LINE_OUT_L_A::DIVBY64
    }
    #[doc = "Checks if the value of the field is `DIVBY128`"]
    #[inline(always)]
    pub fn is_divby128(&self) -> bool {
        *self == DT_LINE_OUT_L_A::DIVBY128
    }
}
#[doc = "Field `DT_LINE_OUT_L` writer - In PWM_DT mode, this field is used to determine the low byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock. Note: This field determines the low byte of the 16-bit dead time before activating 'line_out' when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by this DT_LINE_OUT_L field is used before activating 'line_out' and 'line_compl_out'."]
pub type DT_LINE_OUT_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DT_SPEC, u8, DT_LINE_OUT_L_A, 8, O>;
impl<'a, const O: u8> DT_LINE_OUT_L_W<'a, O> {
    #[doc = "Divide by 1 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut W {
        self.variant(DT_LINE_OUT_L_A::DIVBY1)
    }
    #[doc = "Divide by 2 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut W {
        self.variant(DT_LINE_OUT_L_A::DIVBY2)
    }
    #[doc = "Divide by 4 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut W {
        self.variant(DT_LINE_OUT_L_A::DIVBY4)
    }
    #[doc = "Divide by 8 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby8(self) -> &'a mut W {
        self.variant(DT_LINE_OUT_L_A::DIVBY8)
    }
    #[doc = "Divide by 16 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby16(self) -> &'a mut W {
        self.variant(DT_LINE_OUT_L_A::DIVBY16)
    }
    #[doc = "Divide by 32 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby32(self) -> &'a mut W {
        self.variant(DT_LINE_OUT_L_A::DIVBY32)
    }
    #[doc = "Divide by 64 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby64(self) -> &'a mut W {
        self.variant(DT_LINE_OUT_L_A::DIVBY64)
    }
    #[doc = "Divide by 128 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby128(self) -> &'a mut W {
        self.variant(DT_LINE_OUT_L_A::DIVBY128)
    }
}
#[doc = "Field `DT_LINE_OUT_H` reader - In PWM_DT mode, this field is used to determine the high byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
pub type DT_LINE_OUT_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT_LINE_OUT_H` writer - In PWM_DT mode, this field is used to determine the high byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
pub type DT_LINE_OUT_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DT_SPEC, u8, u8, 8, O>;
#[doc = "Field `DT_LINE_COMPL_OUT` reader - In PWM_DT mode, this field is used to determine the dead time before activating the complementary PWM line output signal 'line_compl_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
pub type DT_LINE_COMPL_OUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DT_LINE_COMPL_OUT` writer - In PWM_DT mode, this field is used to determine the dead time before activating the complementary PWM line output signal 'line_compl_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
pub type DT_LINE_COMPL_OUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - In PWM_DT mode, this field is used to determine the low byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock. Note: This field determines the low byte of the 16-bit dead time before activating 'line_out' when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by this DT_LINE_OUT_L field is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_out_l(&self) -> DT_LINE_OUT_L_R {
        DT_LINE_OUT_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In PWM_DT mode, this field is used to determine the high byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_out_h(&self) -> DT_LINE_OUT_H_R {
        DT_LINE_OUT_H_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - In PWM_DT mode, this field is used to determine the dead time before activating the complementary PWM line output signal 'line_compl_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    pub fn dt_line_compl_out(&self) -> DT_LINE_COMPL_OUT_R {
        DT_LINE_COMPL_OUT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - In PWM_DT mode, this field is used to determine the low byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock. Note: This field determines the low byte of the 16-bit dead time before activating 'line_out' when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by this DT_LINE_OUT_L field is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    #[must_use]
    pub fn dt_line_out_l(&mut self) -> DT_LINE_OUT_L_W<0> {
        DT_LINE_OUT_L_W::new(self)
    }
    #[doc = "Bits 8:15 - In PWM_DT mode, this field is used to determine the high byte of the dead time before activating the PWM line output signal 'line_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    #[must_use]
    pub fn dt_line_out_h(&mut self) -> DT_LINE_OUT_H_W<8> {
        DT_LINE_OUT_H_W::new(self)
    }
    #[doc = "Bits 16:31 - In PWM_DT mode, this field is used to determine the dead time before activating the complementary PWM line output signal 'line_compl_out': amount of dead time cycles in the counter clock domain. In all other modes, this field has no effect. Note: This field only exists when parameter GRP_AMC_PRESENT for advanced motor control is set to 1. Otherwise the dead time is only 8 bit wide and the same dead time specified by field DT_LINE_OUT_L is used before activating 'line_out' and 'line_compl_out'."]
    #[inline(always)]
    #[must_use]
    pub fn dt_line_compl_out(&mut self) -> DT_LINE_COMPL_OUT_W<16> {
        DT_LINE_COMPL_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter PWM dead time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt](index.html) module"]
pub struct DT_SPEC;
impl crate::RegisterSpec for DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt::R](R) reader structure"]
impl crate::Readable for DT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt::W](W) writer structure"]
impl crate::Writable for DT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT to value 0"]
impl crate::Resettable for DT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
