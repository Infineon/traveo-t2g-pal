#[doc = "Register `GPSCALER4_DYNAMIC` reader"]
pub struct R(crate::R<GPSCALER4_DYNAMIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPSCALER4_DYNAMIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPSCALER4_DYNAMIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPSCALER4_DYNAMIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPSCALER4_DYNAMIC` writer"]
pub struct W(crate::W<GPSCALER4_DYNAMIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPSCALER4_DYNAMIC_SPEC>;
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
impl From<crate::W<GPSCALER4_DYNAMIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPSCALER4_DYNAMIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPSCALER4_SRC_SEL` reader - Selection of the source for the src input of the gpscaler4 module Value fetcheco4 is only supported for IPVersion 1 IPPhase 2 or higher."]
pub type GPSCALER4_SRC_SEL_R = crate::FieldReader<u8, GPSCALER4_SRC_SEL_A>;
#[doc = "Selection of the source for the src input of the gpscaler4 module Value fetcheco4 is only supported for IPVersion 1 IPPhase 2 or higher.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPSCALER4_SRC_SEL_A {
    #[doc = "0: Unit gpscaler4 input port src is disabled"]
    DISABLE = 0,
    #[doc = "19: Unit gpscaler4 input port src is connected to output of unit matrix4"]
    MATRIX4 = 19,
    #[doc = "13: Unit gpscaler4 input port src is connected to output of unit fetcheco4"]
    FETCHECO4 = 13,
}
impl From<GPSCALER4_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPSCALER4_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl GPSCALER4_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPSCALER4_SRC_SEL_A> {
        match self.bits {
            0 => Some(GPSCALER4_SRC_SEL_A::DISABLE),
            19 => Some(GPSCALER4_SRC_SEL_A::MATRIX4),
            13 => Some(GPSCALER4_SRC_SEL_A::FETCHECO4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPSCALER4_SRC_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MATRIX4`"]
    #[inline(always)]
    pub fn is_matrix4(&self) -> bool {
        *self == GPSCALER4_SRC_SEL_A::MATRIX4
    }
    #[doc = "Checks if the value of the field is `FETCHECO4`"]
    #[inline(always)]
    pub fn is_fetcheco4(&self) -> bool {
        *self == GPSCALER4_SRC_SEL_A::FETCHECO4
    }
}
#[doc = "Field `GPSCALER4_SRC_SEL` writer - Selection of the source for the src input of the gpscaler4 module Value fetcheco4 is only supported for IPVersion 1 IPPhase 2 or higher."]
pub type GPSCALER4_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPSCALER4_DYNAMIC_SPEC, u8, GPSCALER4_SRC_SEL_A, 5, O>;
impl<'a, const O: u8> GPSCALER4_SRC_SEL_W<'a, O> {
    #[doc = "Unit gpscaler4 input port src is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPSCALER4_SRC_SEL_A::DISABLE)
    }
    #[doc = "Unit gpscaler4 input port src is connected to output of unit matrix4"]
    #[inline(always)]
    pub fn matrix4(self) -> &'a mut W {
        self.variant(GPSCALER4_SRC_SEL_A::MATRIX4)
    }
    #[doc = "Unit gpscaler4 input port src is connected to output of unit fetcheco4"]
    #[inline(always)]
    pub fn fetcheco4(self) -> &'a mut W {
        self.variant(GPSCALER4_SRC_SEL_A::FETCHECO4)
    }
}
#[doc = "Field `GPSCALER4_CLKEN` reader - Enable of gpscaler4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80."]
pub type GPSCALER4_CLKEN_R = crate::FieldReader<u8, GPSCALER4_CLKEN_A>;
#[doc = "Enable of gpscaler4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPSCALER4_CLKEN_A {
    #[doc = "0: Clock for gpscaler4 is disabled"]
    DISABLE = 0,
    #[doc = "1: Clock is enabled if unit is used, frequency is defined by the register setting for this pipeline (see \\[endpoint_name\\]_Static register)"]
    AUTOMATIC = 1,
    #[doc = "3: Clock for gpscaler4 is without gating"]
    FULL = 3,
}
impl From<GPSCALER4_CLKEN_A> for u8 {
    #[inline(always)]
    fn from(variant: GPSCALER4_CLKEN_A) -> Self {
        variant as _
    }
}
impl GPSCALER4_CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPSCALER4_CLKEN_A> {
        match self.bits {
            0 => Some(GPSCALER4_CLKEN_A::DISABLE),
            1 => Some(GPSCALER4_CLKEN_A::AUTOMATIC),
            3 => Some(GPSCALER4_CLKEN_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPSCALER4_CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == GPSCALER4_CLKEN_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == GPSCALER4_CLKEN_A::FULL
    }
}
#[doc = "Field `GPSCALER4_CLKEN` writer - Enable of gpscaler4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80."]
pub type GPSCALER4_CLKEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPSCALER4_DYNAMIC_SPEC, u8, GPSCALER4_CLKEN_A, 2, O>;
impl<'a, const O: u8> GPSCALER4_CLKEN_W<'a, O> {
    #[doc = "Clock for gpscaler4 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPSCALER4_CLKEN_A::DISABLE)
    }
    #[doc = "Clock is enabled if unit is used, frequency is defined by the register setting for this pipeline (see \\[endpoint_name\\]_Static register)"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(GPSCALER4_CLKEN_A::AUTOMATIC)
    }
    #[doc = "Clock for gpscaler4 is without gating"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(GPSCALER4_CLKEN_A::FULL)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the gpscaler4 module Value fetcheco4 is only supported for IPVersion 1 IPPhase 2 or higher."]
    #[inline(always)]
    pub fn gpscaler4_src_sel(&self) -> GPSCALER4_SRC_SEL_R {
        GPSCALER4_SRC_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - Enable of gpscaler4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80."]
    #[inline(always)]
    pub fn gpscaler4_clken(&self) -> GPSCALER4_CLKEN_R {
        GPSCALER4_CLKEN_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the gpscaler4 module Value fetcheco4 is only supported for IPVersion 1 IPPhase 2 or higher."]
    #[inline(always)]
    #[must_use]
    pub fn gpscaler4_src_sel(&mut self) -> GPSCALER4_SRC_SEL_W<0> {
        GPSCALER4_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 24:25 - Enable of gpscaler4 clock (this setting has to be the same for all modules of one processing pipeline). If a submodule is enabled and FULL is used, then the register \\[endpoint_name\\]_clk must be set to 0x80."]
    #[inline(always)]
    #[must_use]
    pub fn gpscaler4_clken(&mut self) -> GPSCALER4_CLKEN_W<24> {
        GPSCALER4_CLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic pixel engine configuration for gpscaler4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpscaler4_dynamic](index.html) module"]
pub struct GPSCALER4_DYNAMIC_SPEC;
impl crate::RegisterSpec for GPSCALER4_DYNAMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpscaler4_dynamic::R](R) reader structure"]
impl crate::Readable for GPSCALER4_DYNAMIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpscaler4_dynamic::W](W) writer structure"]
impl crate::Writable for GPSCALER4_DYNAMIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPSCALER4_DYNAMIC to value 0x0100_0000"]
impl crate::Resettable for GPSCALER4_DYNAMIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
