#[doc = "Register `LAYERBLEND4_DYNAMIC` reader"]
pub struct R(crate::R<LAYERBLEND4_DYNAMIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERBLEND4_DYNAMIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERBLEND4_DYNAMIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERBLEND4_DYNAMIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERBLEND4_DYNAMIC` writer"]
pub struct W(crate::W<LAYERBLEND4_DYNAMIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERBLEND4_DYNAMIC_SPEC>;
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
impl From<crate::W<LAYERBLEND4_DYNAMIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERBLEND4_DYNAMIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LAYERBLEND4_PRIM_SEL` reader - Selection of the source for the prim input of the layerblend4 module"]
pub type LAYERBLEND4_PRIM_SEL_R = crate::FieldReader<u8, LAYERBLEND4_PRIM_SEL_A>;
#[doc = "Selection of the source for the prim input of the layerblend4 module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LAYERBLEND4_PRIM_SEL_A {
    #[doc = "0: Unit layerblend4 input port prim is disabled"]
    DISABLE = 0,
    #[doc = "1: Unit layerblend4 input port prim is connected to output of unit constframe0"]
    CONSTFRAME0 = 1,
    #[doc = "5: Unit layerblend4 input port prim is connected to output of unit constframe1"]
    CONSTFRAME1 = 5,
    #[doc = "3: Unit layerblend4 input port prim is connected to output of unit constframe4"]
    CONSTFRAME4 = 3,
    #[doc = "7: Unit layerblend4 input port prim is connected to output of unit constframe5"]
    CONSTFRAME5 = 7,
    #[doc = "19: Unit layerblend4 input port prim is connected to output of unit matrix4"]
    MATRIX4 = 19,
    #[doc = "20: Unit layerblend4 input port prim is connected to output of unit gpscaler4"]
    GPSCALER4 = 20,
    #[doc = "9: Unit layerblend4 input port prim is connected to output of unit extsrc4"]
    EXTSRC4 = 9,
    #[doc = "24: Unit layerblend4 input port prim is connected to output of unit layerblend3"]
    LAYERBLEND3 = 24,
    #[doc = "23: Unit layerblend4 input port prim is connected to output of unit layerblend2"]
    LAYERBLEND2 = 23,
    #[doc = "22: Unit layerblend4 input port prim is connected to output of unit layerblend1"]
    LAYERBLEND1 = 22,
}
impl From<LAYERBLEND4_PRIM_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LAYERBLEND4_PRIM_SEL_A) -> Self {
        variant as _
    }
}
impl LAYERBLEND4_PRIM_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LAYERBLEND4_PRIM_SEL_A> {
        match self.bits {
            0 => Some(LAYERBLEND4_PRIM_SEL_A::DISABLE),
            1 => Some(LAYERBLEND4_PRIM_SEL_A::CONSTFRAME0),
            5 => Some(LAYERBLEND4_PRIM_SEL_A::CONSTFRAME1),
            3 => Some(LAYERBLEND4_PRIM_SEL_A::CONSTFRAME4),
            7 => Some(LAYERBLEND4_PRIM_SEL_A::CONSTFRAME5),
            19 => Some(LAYERBLEND4_PRIM_SEL_A::MATRIX4),
            20 => Some(LAYERBLEND4_PRIM_SEL_A::GPSCALER4),
            9 => Some(LAYERBLEND4_PRIM_SEL_A::EXTSRC4),
            24 => Some(LAYERBLEND4_PRIM_SEL_A::LAYERBLEND3),
            23 => Some(LAYERBLEND4_PRIM_SEL_A::LAYERBLEND2),
            22 => Some(LAYERBLEND4_PRIM_SEL_A::LAYERBLEND1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CONSTFRAME0`"]
    #[inline(always)]
    pub fn is_constframe0(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::CONSTFRAME0
    }
    #[doc = "Checks if the value of the field is `CONSTFRAME1`"]
    #[inline(always)]
    pub fn is_constframe1(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::CONSTFRAME1
    }
    #[doc = "Checks if the value of the field is `CONSTFRAME4`"]
    #[inline(always)]
    pub fn is_constframe4(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::CONSTFRAME4
    }
    #[doc = "Checks if the value of the field is `CONSTFRAME5`"]
    #[inline(always)]
    pub fn is_constframe5(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::CONSTFRAME5
    }
    #[doc = "Checks if the value of the field is `MATRIX4`"]
    #[inline(always)]
    pub fn is_matrix4(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::MATRIX4
    }
    #[doc = "Checks if the value of the field is `GPSCALER4`"]
    #[inline(always)]
    pub fn is_gpscaler4(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::GPSCALER4
    }
    #[doc = "Checks if the value of the field is `EXTSRC4`"]
    #[inline(always)]
    pub fn is_extsrc4(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::EXTSRC4
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND3`"]
    #[inline(always)]
    pub fn is_layerblend3(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::LAYERBLEND3
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND2`"]
    #[inline(always)]
    pub fn is_layerblend2(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::LAYERBLEND2
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND1`"]
    #[inline(always)]
    pub fn is_layerblend1(&self) -> bool {
        *self == LAYERBLEND4_PRIM_SEL_A::LAYERBLEND1
    }
}
#[doc = "Field `LAYERBLEND4_PRIM_SEL` writer - Selection of the source for the prim input of the layerblend4 module"]
pub type LAYERBLEND4_PRIM_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERBLEND4_DYNAMIC_SPEC, u8, LAYERBLEND4_PRIM_SEL_A, 5, O>;
impl<'a, const O: u8> LAYERBLEND4_PRIM_SEL_W<'a, O> {
    #[doc = "Unit layerblend4 input port prim is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::DISABLE)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit constframe0"]
    #[inline(always)]
    pub fn constframe0(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::CONSTFRAME0)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit constframe1"]
    #[inline(always)]
    pub fn constframe1(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::CONSTFRAME1)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit constframe4"]
    #[inline(always)]
    pub fn constframe4(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::CONSTFRAME4)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit constframe5"]
    #[inline(always)]
    pub fn constframe5(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::CONSTFRAME5)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit matrix4"]
    #[inline(always)]
    pub fn matrix4(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::MATRIX4)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit gpscaler4"]
    #[inline(always)]
    pub fn gpscaler4(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::GPSCALER4)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit extsrc4"]
    #[inline(always)]
    pub fn extsrc4(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::EXTSRC4)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit layerblend3"]
    #[inline(always)]
    pub fn layerblend3(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::LAYERBLEND3)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit layerblend2"]
    #[inline(always)]
    pub fn layerblend2(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::LAYERBLEND2)
    }
    #[doc = "Unit layerblend4 input port prim is connected to output of unit layerblend1"]
    #[inline(always)]
    pub fn layerblend1(self) -> &'a mut W {
        self.variant(LAYERBLEND4_PRIM_SEL_A::LAYERBLEND1)
    }
}
#[doc = "Field `LAYERBLEND4_SEC_SEL` reader - Selection of the source for the sec input of the layerblend4 module"]
pub type LAYERBLEND4_SEC_SEL_R = crate::FieldReader<u8, LAYERBLEND4_SEC_SEL_A>;
#[doc = "Selection of the source for the sec input of the layerblend4 module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LAYERBLEND4_SEC_SEL_A {
    #[doc = "0: Unit layerblend4 input port sec is disabled"]
    DISABLE = 0,
    #[doc = "12: Unit layerblend4 input port sec is connected to output of unit fetchdecode4"]
    FETCHDECODE4 = 12,
    #[doc = "14: Unit layerblend4 input port sec is connected to output of unit fetchwarp1"]
    FETCHWARP1 = 14,
    #[doc = "17: Unit layerblend4 input port sec is connected to output of unit fetchdecode0"]
    FETCHDECODE0 = 17,
    #[doc = "19: Unit layerblend4 input port sec is connected to output of unit matrix4"]
    MATRIX4 = 19,
    #[doc = "20: Unit layerblend4 input port sec is connected to output of unit gpscaler4"]
    GPSCALER4 = 20,
    #[doc = "11: Unit layerblend4 input port sec is connected to output of unit fetchlayer0"]
    FETCHLAYER0 = 11,
    #[doc = "16: Unit layerblend4 input port sec is connected to output of unit fetchlayer1"]
    FETCHLAYER1 = 16,
}
impl From<LAYERBLEND4_SEC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LAYERBLEND4_SEC_SEL_A) -> Self {
        variant as _
    }
}
impl LAYERBLEND4_SEC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LAYERBLEND4_SEC_SEL_A> {
        match self.bits {
            0 => Some(LAYERBLEND4_SEC_SEL_A::DISABLE),
            12 => Some(LAYERBLEND4_SEC_SEL_A::FETCHDECODE4),
            14 => Some(LAYERBLEND4_SEC_SEL_A::FETCHWARP1),
            17 => Some(LAYERBLEND4_SEC_SEL_A::FETCHDECODE0),
            19 => Some(LAYERBLEND4_SEC_SEL_A::MATRIX4),
            20 => Some(LAYERBLEND4_SEC_SEL_A::GPSCALER4),
            11 => Some(LAYERBLEND4_SEC_SEL_A::FETCHLAYER0),
            16 => Some(LAYERBLEND4_SEC_SEL_A::FETCHLAYER1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LAYERBLEND4_SEC_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `FETCHDECODE4`"]
    #[inline(always)]
    pub fn is_fetchdecode4(&self) -> bool {
        *self == LAYERBLEND4_SEC_SEL_A::FETCHDECODE4
    }
    #[doc = "Checks if the value of the field is `FETCHWARP1`"]
    #[inline(always)]
    pub fn is_fetchwarp1(&self) -> bool {
        *self == LAYERBLEND4_SEC_SEL_A::FETCHWARP1
    }
    #[doc = "Checks if the value of the field is `FETCHDECODE0`"]
    #[inline(always)]
    pub fn is_fetchdecode0(&self) -> bool {
        *self == LAYERBLEND4_SEC_SEL_A::FETCHDECODE0
    }
    #[doc = "Checks if the value of the field is `MATRIX4`"]
    #[inline(always)]
    pub fn is_matrix4(&self) -> bool {
        *self == LAYERBLEND4_SEC_SEL_A::MATRIX4
    }
    #[doc = "Checks if the value of the field is `GPSCALER4`"]
    #[inline(always)]
    pub fn is_gpscaler4(&self) -> bool {
        *self == LAYERBLEND4_SEC_SEL_A::GPSCALER4
    }
    #[doc = "Checks if the value of the field is `FETCHLAYER0`"]
    #[inline(always)]
    pub fn is_fetchlayer0(&self) -> bool {
        *self == LAYERBLEND4_SEC_SEL_A::FETCHLAYER0
    }
    #[doc = "Checks if the value of the field is `FETCHLAYER1`"]
    #[inline(always)]
    pub fn is_fetchlayer1(&self) -> bool {
        *self == LAYERBLEND4_SEC_SEL_A::FETCHLAYER1
    }
}
#[doc = "Field `LAYERBLEND4_SEC_SEL` writer - Selection of the source for the sec input of the layerblend4 module"]
pub type LAYERBLEND4_SEC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERBLEND4_DYNAMIC_SPEC, u8, LAYERBLEND4_SEC_SEL_A, 5, O>;
impl<'a, const O: u8> LAYERBLEND4_SEC_SEL_W<'a, O> {
    #[doc = "Unit layerblend4 input port sec is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LAYERBLEND4_SEC_SEL_A::DISABLE)
    }
    #[doc = "Unit layerblend4 input port sec is connected to output of unit fetchdecode4"]
    #[inline(always)]
    pub fn fetchdecode4(self) -> &'a mut W {
        self.variant(LAYERBLEND4_SEC_SEL_A::FETCHDECODE4)
    }
    #[doc = "Unit layerblend4 input port sec is connected to output of unit fetchwarp1"]
    #[inline(always)]
    pub fn fetchwarp1(self) -> &'a mut W {
        self.variant(LAYERBLEND4_SEC_SEL_A::FETCHWARP1)
    }
    #[doc = "Unit layerblend4 input port sec is connected to output of unit fetchdecode0"]
    #[inline(always)]
    pub fn fetchdecode0(self) -> &'a mut W {
        self.variant(LAYERBLEND4_SEC_SEL_A::FETCHDECODE0)
    }
    #[doc = "Unit layerblend4 input port sec is connected to output of unit matrix4"]
    #[inline(always)]
    pub fn matrix4(self) -> &'a mut W {
        self.variant(LAYERBLEND4_SEC_SEL_A::MATRIX4)
    }
    #[doc = "Unit layerblend4 input port sec is connected to output of unit gpscaler4"]
    #[inline(always)]
    pub fn gpscaler4(self) -> &'a mut W {
        self.variant(LAYERBLEND4_SEC_SEL_A::GPSCALER4)
    }
    #[doc = "Unit layerblend4 input port sec is connected to output of unit fetchlayer0"]
    #[inline(always)]
    pub fn fetchlayer0(self) -> &'a mut W {
        self.variant(LAYERBLEND4_SEC_SEL_A::FETCHLAYER0)
    }
    #[doc = "Unit layerblend4 input port sec is connected to output of unit fetchlayer1"]
    #[inline(always)]
    pub fn fetchlayer1(self) -> &'a mut W {
        self.variant(LAYERBLEND4_SEC_SEL_A::FETCHLAYER1)
    }
}
#[doc = "Field `LAYERBLEND4_CLKEN` reader - Enable of layerblend4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80."]
pub type LAYERBLEND4_CLKEN_R = crate::FieldReader<u8, LAYERBLEND4_CLKEN_A>;
#[doc = "Enable of layerblend4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LAYERBLEND4_CLKEN_A {
    #[doc = "0: Clock for layerblend4 is disabled"]
    DISABLE = 0,
    #[doc = "1: Clock is enabled if unit is used, frequency is defined by the register setting for this pipeline (see \\[endpoint_name\\]_Static register)"]
    AUTOMATIC = 1,
    #[doc = "3: Clock for layerblend4 is without gating"]
    FULL = 3,
}
impl From<LAYERBLEND4_CLKEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LAYERBLEND4_CLKEN_A) -> Self {
        variant as _
    }
}
impl LAYERBLEND4_CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LAYERBLEND4_CLKEN_A> {
        match self.bits {
            0 => Some(LAYERBLEND4_CLKEN_A::DISABLE),
            1 => Some(LAYERBLEND4_CLKEN_A::AUTOMATIC),
            3 => Some(LAYERBLEND4_CLKEN_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LAYERBLEND4_CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == LAYERBLEND4_CLKEN_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == LAYERBLEND4_CLKEN_A::FULL
    }
}
#[doc = "Field `LAYERBLEND4_CLKEN` writer - Enable of layerblend4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80."]
pub type LAYERBLEND4_CLKEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERBLEND4_DYNAMIC_SPEC, u8, LAYERBLEND4_CLKEN_A, 2, O>;
impl<'a, const O: u8> LAYERBLEND4_CLKEN_W<'a, O> {
    #[doc = "Clock for layerblend4 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LAYERBLEND4_CLKEN_A::DISABLE)
    }
    #[doc = "Clock is enabled if unit is used, frequency is defined by the register setting for this pipeline (see \\[endpoint_name\\]_Static register)"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(LAYERBLEND4_CLKEN_A::AUTOMATIC)
    }
    #[doc = "Clock for layerblend4 is without gating"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(LAYERBLEND4_CLKEN_A::FULL)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selection of the source for the prim input of the layerblend4 module"]
    #[inline(always)]
    pub fn layerblend4_prim_sel(&self) -> LAYERBLEND4_PRIM_SEL_R {
        LAYERBLEND4_PRIM_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Selection of the source for the sec input of the layerblend4 module"]
    #[inline(always)]
    pub fn layerblend4_sec_sel(&self) -> LAYERBLEND4_SEC_SEL_R {
        LAYERBLEND4_SEC_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - Enable of layerblend4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80."]
    #[inline(always)]
    pub fn layerblend4_clken(&self) -> LAYERBLEND4_CLKEN_R {
        LAYERBLEND4_CLKEN_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selection of the source for the prim input of the layerblend4 module"]
    #[inline(always)]
    #[must_use]
    pub fn layerblend4_prim_sel(&mut self) -> LAYERBLEND4_PRIM_SEL_W<0> {
        LAYERBLEND4_PRIM_SEL_W::new(self)
    }
    #[doc = "Bits 8:12 - Selection of the source for the sec input of the layerblend4 module"]
    #[inline(always)]
    #[must_use]
    pub fn layerblend4_sec_sel(&mut self) -> LAYERBLEND4_SEC_SEL_W<8> {
        LAYERBLEND4_SEC_SEL_W::new(self)
    }
    #[doc = "Bits 24:25 - Enable of layerblend4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80."]
    #[inline(always)]
    #[must_use]
    pub fn layerblend4_clken(&mut self) -> LAYERBLEND4_CLKEN_W<24> {
        LAYERBLEND4_CLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic pixel engine configuration for layerblend4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerblend4_dynamic](index.html) module"]
pub struct LAYERBLEND4_DYNAMIC_SPEC;
impl crate::RegisterSpec for LAYERBLEND4_DYNAMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerblend4_dynamic::R](R) reader structure"]
impl crate::Readable for LAYERBLEND4_DYNAMIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerblend4_dynamic::W](W) writer structure"]
impl crate::Writable for LAYERBLEND4_DYNAMIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERBLEND4_DYNAMIC to value 0x0100_0000"]
impl crate::Resettable for LAYERBLEND4_DYNAMIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
