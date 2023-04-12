#[doc = "Register `BLENDCONTROL` reader"]
pub struct R(crate::R<BLENDCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLENDCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLENDCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLENDCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLENDCONTROL` writer"]
pub struct W(crate::W<BLENDCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLENDCONTROL_SPEC>;
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
impl From<crate::W<BLENDCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLENDCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIM_C_BLD_FUNC` reader - Primary (background) input color blending function"]
pub type PRIM_C_BLD_FUNC_R = crate::FieldReader<u8, PRIM_C_BLD_FUNC_A>;
#[doc = "Primary (background) input color blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIM_C_BLD_FUNC_A {
    #[doc = "0: Cout = Cin * 0"]
    ZERO = 0,
    #[doc = "1: Cout = Cin * 1"]
    ONE = 1,
    #[doc = "2: Cout = Cin * ALPHA_prim"]
    PRIM_ALPHA = 2,
    #[doc = "3: Cout = Cin * (1 - ALPHA_prim)"]
    ONE_MINUS_PRIM_ALPHA = 3,
    #[doc = "4: Cout = Cin * ALPHA_sec"]
    SEC_ALPHA = 4,
    #[doc = "5: Cout = Cin * (1 - ALPHA_sec)"]
    ONE_MINUS_SEC_ALPHA = 5,
    #[doc = "6: Cout = Cin * ALPHA_const"]
    CONST_ALPHA = 6,
    #[doc = "7: Cout = Cin * (1 - ALPHA_const)"]
    ONE_MINUS_CONST_ALPHA = 7,
}
impl From<PRIM_C_BLD_FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIM_C_BLD_FUNC_A) -> Self {
        variant as _
    }
}
impl PRIM_C_BLD_FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIM_C_BLD_FUNC_A {
        match self.bits {
            0 => PRIM_C_BLD_FUNC_A::ZERO,
            1 => PRIM_C_BLD_FUNC_A::ONE,
            2 => PRIM_C_BLD_FUNC_A::PRIM_ALPHA,
            3 => PRIM_C_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA,
            4 => PRIM_C_BLD_FUNC_A::SEC_ALPHA,
            5 => PRIM_C_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA,
            6 => PRIM_C_BLD_FUNC_A::CONST_ALPHA,
            7 => PRIM_C_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == PRIM_C_BLD_FUNC_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PRIM_C_BLD_FUNC_A::ONE
    }
    #[doc = "Checks if the value of the field is `PRIM_ALPHA`"]
    #[inline(always)]
    pub fn is_prim_alpha(&self) -> bool {
        *self == PRIM_C_BLD_FUNC_A::PRIM_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_PRIM_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_prim_alpha(&self) -> bool {
        *self == PRIM_C_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA
    }
    #[doc = "Checks if the value of the field is `SEC_ALPHA`"]
    #[inline(always)]
    pub fn is_sec_alpha(&self) -> bool {
        *self == PRIM_C_BLD_FUNC_A::SEC_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_SEC_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_sec_alpha(&self) -> bool {
        *self == PRIM_C_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA
    }
    #[doc = "Checks if the value of the field is `CONST_ALPHA`"]
    #[inline(always)]
    pub fn is_const_alpha(&self) -> bool {
        *self == PRIM_C_BLD_FUNC_A::CONST_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_CONST_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_const_alpha(&self) -> bool {
        *self == PRIM_C_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA
    }
}
#[doc = "Field `PRIM_C_BLD_FUNC` writer - Primary (background) input color blending function"]
pub type PRIM_C_BLD_FUNC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BLENDCONTROL_SPEC, u8, PRIM_C_BLD_FUNC_A, 3, O>;
impl<'a, const O: u8> PRIM_C_BLD_FUNC_W<'a, O> {
    #[doc = "Cout = Cin * 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(PRIM_C_BLD_FUNC_A::ZERO)
    }
    #[doc = "Cout = Cin * 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PRIM_C_BLD_FUNC_A::ONE)
    }
    #[doc = "Cout = Cin * ALPHA_prim"]
    #[inline(always)]
    pub fn prim_alpha(self) -> &'a mut W {
        self.variant(PRIM_C_BLD_FUNC_A::PRIM_ALPHA)
    }
    #[doc = "Cout = Cin * (1 - ALPHA_prim)"]
    #[inline(always)]
    pub fn one_minus_prim_alpha(self) -> &'a mut W {
        self.variant(PRIM_C_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA)
    }
    #[doc = "Cout = Cin * ALPHA_sec"]
    #[inline(always)]
    pub fn sec_alpha(self) -> &'a mut W {
        self.variant(PRIM_C_BLD_FUNC_A::SEC_ALPHA)
    }
    #[doc = "Cout = Cin * (1 - ALPHA_sec)"]
    #[inline(always)]
    pub fn one_minus_sec_alpha(self) -> &'a mut W {
        self.variant(PRIM_C_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA)
    }
    #[doc = "Cout = Cin * ALPHA_const"]
    #[inline(always)]
    pub fn const_alpha(self) -> &'a mut W {
        self.variant(PRIM_C_BLD_FUNC_A::CONST_ALPHA)
    }
    #[doc = "Cout = Cin * (1 - ALPHA_const)"]
    #[inline(always)]
    pub fn one_minus_const_alpha(self) -> &'a mut W {
        self.variant(PRIM_C_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA)
    }
}
#[doc = "Field `SEC_C_BLD_FUNC` reader - Secondary (overlay) input color blending function"]
pub type SEC_C_BLD_FUNC_R = crate::FieldReader<u8, SEC_C_BLD_FUNC_A>;
#[doc = "Secondary (overlay) input color blending function\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_C_BLD_FUNC_A {
    #[doc = "0: Cout = Cin * 0"]
    ZERO = 0,
    #[doc = "1: Cout = Cin * 1"]
    ONE = 1,
    #[doc = "2: Cout = Cin * ALPHA_prim"]
    PRIM_ALPHA = 2,
    #[doc = "3: Cout = Cin * (1 - ALPHA_prim)"]
    ONE_MINUS_PRIM_ALPHA = 3,
    #[doc = "4: Cout = Cin * ALPHA_sec"]
    SEC_ALPHA = 4,
    #[doc = "5: Cout = Cin * (1 - ALPHA_sec)"]
    ONE_MINUS_SEC_ALPHA = 5,
    #[doc = "6: Cout = Cin * ALPHA_const"]
    CONST_ALPHA = 6,
    #[doc = "7: Cout = Cin * (1 - ALPHA_const)"]
    ONE_MINUS_CONST_ALPHA = 7,
}
impl From<SEC_C_BLD_FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_C_BLD_FUNC_A) -> Self {
        variant as _
    }
}
impl SEC_C_BLD_FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_C_BLD_FUNC_A {
        match self.bits {
            0 => SEC_C_BLD_FUNC_A::ZERO,
            1 => SEC_C_BLD_FUNC_A::ONE,
            2 => SEC_C_BLD_FUNC_A::PRIM_ALPHA,
            3 => SEC_C_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA,
            4 => SEC_C_BLD_FUNC_A::SEC_ALPHA,
            5 => SEC_C_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA,
            6 => SEC_C_BLD_FUNC_A::CONST_ALPHA,
            7 => SEC_C_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == SEC_C_BLD_FUNC_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SEC_C_BLD_FUNC_A::ONE
    }
    #[doc = "Checks if the value of the field is `PRIM_ALPHA`"]
    #[inline(always)]
    pub fn is_prim_alpha(&self) -> bool {
        *self == SEC_C_BLD_FUNC_A::PRIM_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_PRIM_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_prim_alpha(&self) -> bool {
        *self == SEC_C_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA
    }
    #[doc = "Checks if the value of the field is `SEC_ALPHA`"]
    #[inline(always)]
    pub fn is_sec_alpha(&self) -> bool {
        *self == SEC_C_BLD_FUNC_A::SEC_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_SEC_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_sec_alpha(&self) -> bool {
        *self == SEC_C_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA
    }
    #[doc = "Checks if the value of the field is `CONST_ALPHA`"]
    #[inline(always)]
    pub fn is_const_alpha(&self) -> bool {
        *self == SEC_C_BLD_FUNC_A::CONST_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_CONST_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_const_alpha(&self) -> bool {
        *self == SEC_C_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA
    }
}
#[doc = "Field `SEC_C_BLD_FUNC` writer - Secondary (overlay) input color blending function"]
pub type SEC_C_BLD_FUNC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BLENDCONTROL_SPEC, u8, SEC_C_BLD_FUNC_A, 3, O>;
impl<'a, const O: u8> SEC_C_BLD_FUNC_W<'a, O> {
    #[doc = "Cout = Cin * 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(SEC_C_BLD_FUNC_A::ZERO)
    }
    #[doc = "Cout = Cin * 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SEC_C_BLD_FUNC_A::ONE)
    }
    #[doc = "Cout = Cin * ALPHA_prim"]
    #[inline(always)]
    pub fn prim_alpha(self) -> &'a mut W {
        self.variant(SEC_C_BLD_FUNC_A::PRIM_ALPHA)
    }
    #[doc = "Cout = Cin * (1 - ALPHA_prim)"]
    #[inline(always)]
    pub fn one_minus_prim_alpha(self) -> &'a mut W {
        self.variant(SEC_C_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA)
    }
    #[doc = "Cout = Cin * ALPHA_sec"]
    #[inline(always)]
    pub fn sec_alpha(self) -> &'a mut W {
        self.variant(SEC_C_BLD_FUNC_A::SEC_ALPHA)
    }
    #[doc = "Cout = Cin * (1 - ALPHA_sec)"]
    #[inline(always)]
    pub fn one_minus_sec_alpha(self) -> &'a mut W {
        self.variant(SEC_C_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA)
    }
    #[doc = "Cout = Cin * ALPHA_const"]
    #[inline(always)]
    pub fn const_alpha(self) -> &'a mut W {
        self.variant(SEC_C_BLD_FUNC_A::CONST_ALPHA)
    }
    #[doc = "Cout = Cin * (1 - ALPHA_const)"]
    #[inline(always)]
    pub fn one_minus_const_alpha(self) -> &'a mut W {
        self.variant(SEC_C_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA)
    }
}
#[doc = "Field `PRIM_A_BLD_FUNC` reader - Primary (background) input alpha blending function"]
pub type PRIM_A_BLD_FUNC_R = crate::FieldReader<u8, PRIM_A_BLD_FUNC_A>;
#[doc = "Primary (background) input alpha blending function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIM_A_BLD_FUNC_A {
    #[doc = "0: Aout = Ain * 0"]
    ZERO = 0,
    #[doc = "1: Aout = Ain * 1"]
    ONE = 1,
    #[doc = "2: Aout = Ain * ALPHA_prim"]
    PRIM_ALPHA = 2,
    #[doc = "3: Aout = Ain * (1 - ALPHA_prim)"]
    ONE_MINUS_PRIM_ALPHA = 3,
    #[doc = "4: Aout = Ain * ALPHA_sec"]
    SEC_ALPHA = 4,
    #[doc = "5: Aout = Ain * (1 - ALPHA_sec)"]
    ONE_MINUS_SEC_ALPHA = 5,
    #[doc = "6: Aout = Ain * ALPHA_const"]
    CONST_ALPHA = 6,
    #[doc = "7: Aout = Ain * (1 - ALPHA_const)"]
    ONE_MINUS_CONST_ALPHA = 7,
}
impl From<PRIM_A_BLD_FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIM_A_BLD_FUNC_A) -> Self {
        variant as _
    }
}
impl PRIM_A_BLD_FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIM_A_BLD_FUNC_A {
        match self.bits {
            0 => PRIM_A_BLD_FUNC_A::ZERO,
            1 => PRIM_A_BLD_FUNC_A::ONE,
            2 => PRIM_A_BLD_FUNC_A::PRIM_ALPHA,
            3 => PRIM_A_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA,
            4 => PRIM_A_BLD_FUNC_A::SEC_ALPHA,
            5 => PRIM_A_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA,
            6 => PRIM_A_BLD_FUNC_A::CONST_ALPHA,
            7 => PRIM_A_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == PRIM_A_BLD_FUNC_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PRIM_A_BLD_FUNC_A::ONE
    }
    #[doc = "Checks if the value of the field is `PRIM_ALPHA`"]
    #[inline(always)]
    pub fn is_prim_alpha(&self) -> bool {
        *self == PRIM_A_BLD_FUNC_A::PRIM_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_PRIM_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_prim_alpha(&self) -> bool {
        *self == PRIM_A_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA
    }
    #[doc = "Checks if the value of the field is `SEC_ALPHA`"]
    #[inline(always)]
    pub fn is_sec_alpha(&self) -> bool {
        *self == PRIM_A_BLD_FUNC_A::SEC_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_SEC_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_sec_alpha(&self) -> bool {
        *self == PRIM_A_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA
    }
    #[doc = "Checks if the value of the field is `CONST_ALPHA`"]
    #[inline(always)]
    pub fn is_const_alpha(&self) -> bool {
        *self == PRIM_A_BLD_FUNC_A::CONST_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_CONST_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_const_alpha(&self) -> bool {
        *self == PRIM_A_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA
    }
}
#[doc = "Field `PRIM_A_BLD_FUNC` writer - Primary (background) input alpha blending function"]
pub type PRIM_A_BLD_FUNC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BLENDCONTROL_SPEC, u8, PRIM_A_BLD_FUNC_A, 3, O>;
impl<'a, const O: u8> PRIM_A_BLD_FUNC_W<'a, O> {
    #[doc = "Aout = Ain * 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(PRIM_A_BLD_FUNC_A::ZERO)
    }
    #[doc = "Aout = Ain * 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PRIM_A_BLD_FUNC_A::ONE)
    }
    #[doc = "Aout = Ain * ALPHA_prim"]
    #[inline(always)]
    pub fn prim_alpha(self) -> &'a mut W {
        self.variant(PRIM_A_BLD_FUNC_A::PRIM_ALPHA)
    }
    #[doc = "Aout = Ain * (1 - ALPHA_prim)"]
    #[inline(always)]
    pub fn one_minus_prim_alpha(self) -> &'a mut W {
        self.variant(PRIM_A_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA)
    }
    #[doc = "Aout = Ain * ALPHA_sec"]
    #[inline(always)]
    pub fn sec_alpha(self) -> &'a mut W {
        self.variant(PRIM_A_BLD_FUNC_A::SEC_ALPHA)
    }
    #[doc = "Aout = Ain * (1 - ALPHA_sec)"]
    #[inline(always)]
    pub fn one_minus_sec_alpha(self) -> &'a mut W {
        self.variant(PRIM_A_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA)
    }
    #[doc = "Aout = Ain * ALPHA_const"]
    #[inline(always)]
    pub fn const_alpha(self) -> &'a mut W {
        self.variant(PRIM_A_BLD_FUNC_A::CONST_ALPHA)
    }
    #[doc = "Aout = Ain * (1 - ALPHA_const)"]
    #[inline(always)]
    pub fn one_minus_const_alpha(self) -> &'a mut W {
        self.variant(PRIM_A_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA)
    }
}
#[doc = "Field `SEC_A_BLD_FUNC` reader - Secondary (overlay) input alpha blending function"]
pub type SEC_A_BLD_FUNC_R = crate::FieldReader<u8, SEC_A_BLD_FUNC_A>;
#[doc = "Secondary (overlay) input alpha blending function\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_A_BLD_FUNC_A {
    #[doc = "0: Aout = Ain * 0"]
    ZERO = 0,
    #[doc = "1: Aout = Ain * 1"]
    ONE = 1,
    #[doc = "2: Aout = Ain * ALPHA_prim"]
    PRIM_ALPHA = 2,
    #[doc = "3: Aout = Ain * (1 - ALPHA_prim)"]
    ONE_MINUS_PRIM_ALPHA = 3,
    #[doc = "4: Aout = Ain * ALPHA_sec"]
    SEC_ALPHA = 4,
    #[doc = "5: Aout = Ain * (1 - ALPHA_sec)"]
    ONE_MINUS_SEC_ALPHA = 5,
    #[doc = "6: Aout = Ain * ALPHA_const"]
    CONST_ALPHA = 6,
    #[doc = "7: Aout = Ain * (1 - ALPHA_const)"]
    ONE_MINUS_CONST_ALPHA = 7,
}
impl From<SEC_A_BLD_FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_A_BLD_FUNC_A) -> Self {
        variant as _
    }
}
impl SEC_A_BLD_FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_A_BLD_FUNC_A {
        match self.bits {
            0 => SEC_A_BLD_FUNC_A::ZERO,
            1 => SEC_A_BLD_FUNC_A::ONE,
            2 => SEC_A_BLD_FUNC_A::PRIM_ALPHA,
            3 => SEC_A_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA,
            4 => SEC_A_BLD_FUNC_A::SEC_ALPHA,
            5 => SEC_A_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA,
            6 => SEC_A_BLD_FUNC_A::CONST_ALPHA,
            7 => SEC_A_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == SEC_A_BLD_FUNC_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SEC_A_BLD_FUNC_A::ONE
    }
    #[doc = "Checks if the value of the field is `PRIM_ALPHA`"]
    #[inline(always)]
    pub fn is_prim_alpha(&self) -> bool {
        *self == SEC_A_BLD_FUNC_A::PRIM_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_PRIM_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_prim_alpha(&self) -> bool {
        *self == SEC_A_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA
    }
    #[doc = "Checks if the value of the field is `SEC_ALPHA`"]
    #[inline(always)]
    pub fn is_sec_alpha(&self) -> bool {
        *self == SEC_A_BLD_FUNC_A::SEC_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_SEC_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_sec_alpha(&self) -> bool {
        *self == SEC_A_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA
    }
    #[doc = "Checks if the value of the field is `CONST_ALPHA`"]
    #[inline(always)]
    pub fn is_const_alpha(&self) -> bool {
        *self == SEC_A_BLD_FUNC_A::CONST_ALPHA
    }
    #[doc = "Checks if the value of the field is `ONE_MINUS_CONST_ALPHA`"]
    #[inline(always)]
    pub fn is_one_minus_const_alpha(&self) -> bool {
        *self == SEC_A_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA
    }
}
#[doc = "Field `SEC_A_BLD_FUNC` writer - Secondary (overlay) input alpha blending function"]
pub type SEC_A_BLD_FUNC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BLENDCONTROL_SPEC, u8, SEC_A_BLD_FUNC_A, 3, O>;
impl<'a, const O: u8> SEC_A_BLD_FUNC_W<'a, O> {
    #[doc = "Aout = Ain * 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(SEC_A_BLD_FUNC_A::ZERO)
    }
    #[doc = "Aout = Ain * 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SEC_A_BLD_FUNC_A::ONE)
    }
    #[doc = "Aout = Ain * ALPHA_prim"]
    #[inline(always)]
    pub fn prim_alpha(self) -> &'a mut W {
        self.variant(SEC_A_BLD_FUNC_A::PRIM_ALPHA)
    }
    #[doc = "Aout = Ain * (1 - ALPHA_prim)"]
    #[inline(always)]
    pub fn one_minus_prim_alpha(self) -> &'a mut W {
        self.variant(SEC_A_BLD_FUNC_A::ONE_MINUS_PRIM_ALPHA)
    }
    #[doc = "Aout = Ain * ALPHA_sec"]
    #[inline(always)]
    pub fn sec_alpha(self) -> &'a mut W {
        self.variant(SEC_A_BLD_FUNC_A::SEC_ALPHA)
    }
    #[doc = "Aout = Ain * (1 - ALPHA_sec)"]
    #[inline(always)]
    pub fn one_minus_sec_alpha(self) -> &'a mut W {
        self.variant(SEC_A_BLD_FUNC_A::ONE_MINUS_SEC_ALPHA)
    }
    #[doc = "Aout = Ain * ALPHA_const"]
    #[inline(always)]
    pub fn const_alpha(self) -> &'a mut W {
        self.variant(SEC_A_BLD_FUNC_A::CONST_ALPHA)
    }
    #[doc = "Aout = Ain * (1 - ALPHA_const)"]
    #[inline(always)]
    pub fn one_minus_const_alpha(self) -> &'a mut W {
        self.variant(SEC_A_BLD_FUNC_A::ONE_MINUS_CONST_ALPHA)
    }
}
#[doc = "Field `BLENDALPHA` reader - Constant alpha value, used for constant alpha blending"]
pub type BLENDALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLENDALPHA` writer - Constant alpha value, used for constant alpha blending"]
pub type BLENDALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLENDCONTROL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Primary (background) input color blending function"]
    #[inline(always)]
    pub fn prim_c_bld_func(&self) -> PRIM_C_BLD_FUNC_R {
        PRIM_C_BLD_FUNC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Secondary (overlay) input color blending function"]
    #[inline(always)]
    pub fn sec_c_bld_func(&self) -> SEC_C_BLD_FUNC_R {
        SEC_C_BLD_FUNC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Primary (background) input alpha blending function"]
    #[inline(always)]
    pub fn prim_a_bld_func(&self) -> PRIM_A_BLD_FUNC_R {
        PRIM_A_BLD_FUNC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Secondary (overlay) input alpha blending function"]
    #[inline(always)]
    pub fn sec_a_bld_func(&self) -> SEC_A_BLD_FUNC_R {
        SEC_A_BLD_FUNC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Constant alpha value, used for constant alpha blending"]
    #[inline(always)]
    pub fn blendalpha(&self) -> BLENDALPHA_R {
        BLENDALPHA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Primary (background) input color blending function"]
    #[inline(always)]
    #[must_use]
    pub fn prim_c_bld_func(&mut self) -> PRIM_C_BLD_FUNC_W<0> {
        PRIM_C_BLD_FUNC_W::new(self)
    }
    #[doc = "Bits 4:6 - Secondary (overlay) input color blending function"]
    #[inline(always)]
    #[must_use]
    pub fn sec_c_bld_func(&mut self) -> SEC_C_BLD_FUNC_W<4> {
        SEC_C_BLD_FUNC_W::new(self)
    }
    #[doc = "Bits 8:10 - Primary (background) input alpha blending function"]
    #[inline(always)]
    #[must_use]
    pub fn prim_a_bld_func(&mut self) -> PRIM_A_BLD_FUNC_W<8> {
        PRIM_A_BLD_FUNC_W::new(self)
    }
    #[doc = "Bits 12:14 - Secondary (overlay) input alpha blending function"]
    #[inline(always)]
    #[must_use]
    pub fn sec_a_bld_func(&mut self) -> SEC_A_BLD_FUNC_W<12> {
        SEC_A_BLD_FUNC_W::new(self)
    }
    #[doc = "Bits 16:23 - Constant alpha value, used for constant alpha blending"]
    #[inline(always)]
    #[must_use]
    pub fn blendalpha(&mut self) -> BLENDALPHA_W<16> {
        BLENDALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Options for blend operations. (shadowed)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blendcontrol](index.html) module"]
pub struct BLENDCONTROL_SPEC;
impl crate::RegisterSpec for BLENDCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blendcontrol::R](R) reader structure"]
impl crate::Readable for BLENDCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blendcontrol::W](W) writer structure"]
impl crate::Writable for BLENDCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLENDCONTROL to value 0x1010"]
impl crate::Resettable for BLENDCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1010;
}
