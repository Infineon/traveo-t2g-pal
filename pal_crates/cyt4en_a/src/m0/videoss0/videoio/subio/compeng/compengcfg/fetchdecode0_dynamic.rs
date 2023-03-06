#[doc = "Register `FETCHDECODE0_DYNAMIC` reader"]
pub struct R(crate::R<FETCHDECODE0_DYNAMIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCHDECODE0_DYNAMIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCHDECODE0_DYNAMIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCHDECODE0_DYNAMIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FETCHDECODE0_DYNAMIC` writer"]
pub struct W(crate::W<FETCHDECODE0_DYNAMIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FETCHDECODE0_DYNAMIC_SPEC>;
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
impl From<crate::W<FETCHDECODE0_DYNAMIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FETCHDECODE0_DYNAMIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETCHDECODE0_SRC_SEL` reader - Selection of the source for the src input of the fetchdecode0 module"]
pub type FETCHDECODE0_SRC_SEL_R = crate::FieldReader<u8, FETCHDECODE0_SRC_SEL_A>;
#[doc = "Selection of the source for the src input of the fetchdecode0 module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FETCHDECODE0_SRC_SEL_A {
    #[doc = "0: Unit fetchdecode0 input port src is disabled"]
    DISABLE = 0,
    #[doc = "15: Unit fetchdecode0 input port src is connected to output of unit fetcheco1"]
    FETCHECO1 = 15,
    #[doc = "13: Unit fetchdecode0 input port src is connected to output of unit fetcheco4"]
    FETCHECO4 = 13,
}
impl From<FETCHDECODE0_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FETCHDECODE0_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl FETCHDECODE0_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FETCHDECODE0_SRC_SEL_A> {
        match self.bits {
            0 => Some(FETCHDECODE0_SRC_SEL_A::DISABLE),
            15 => Some(FETCHDECODE0_SRC_SEL_A::FETCHECO1),
            13 => Some(FETCHDECODE0_SRC_SEL_A::FETCHECO4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FETCHDECODE0_SRC_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `FETCHECO1`"]
    #[inline(always)]
    pub fn is_fetcheco1(&self) -> bool {
        *self == FETCHDECODE0_SRC_SEL_A::FETCHECO1
    }
    #[doc = "Checks if the value of the field is `FETCHECO4`"]
    #[inline(always)]
    pub fn is_fetcheco4(&self) -> bool {
        *self == FETCHDECODE0_SRC_SEL_A::FETCHECO4
    }
}
#[doc = "Field `FETCHDECODE0_SRC_SEL` writer - Selection of the source for the src input of the fetchdecode0 module"]
pub type FETCHDECODE0_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FETCHDECODE0_DYNAMIC_SPEC, u8, FETCHDECODE0_SRC_SEL_A, 5, O>;
impl<'a, const O: u8> FETCHDECODE0_SRC_SEL_W<'a, O> {
    #[doc = "Unit fetchdecode0 input port src is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FETCHDECODE0_SRC_SEL_A::DISABLE)
    }
    #[doc = "Unit fetchdecode0 input port src is connected to output of unit fetcheco1"]
    #[inline(always)]
    pub fn fetcheco1(self) -> &'a mut W {
        self.variant(FETCHDECODE0_SRC_SEL_A::FETCHECO1)
    }
    #[doc = "Unit fetchdecode0 input port src is connected to output of unit fetcheco4"]
    #[inline(always)]
    pub fn fetcheco4(self) -> &'a mut W {
        self.variant(FETCHDECODE0_SRC_SEL_A::FETCHECO4)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the fetchdecode0 module"]
    #[inline(always)]
    pub fn fetchdecode0_src_sel(&self) -> FETCHDECODE0_SRC_SEL_R {
        FETCHDECODE0_SRC_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the fetchdecode0 module"]
    #[inline(always)]
    #[must_use]
    pub fn fetchdecode0_src_sel(&mut self) -> FETCHDECODE0_SRC_SEL_W<0> {
        FETCHDECODE0_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic pixel engine configuration for fetchdecode0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchdecode0_dynamic](index.html) module"]
pub struct FETCHDECODE0_DYNAMIC_SPEC;
impl crate::RegisterSpec for FETCHDECODE0_DYNAMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetchdecode0_dynamic::R](R) reader structure"]
impl crate::Readable for FETCHDECODE0_DYNAMIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fetchdecode0_dynamic::W](W) writer structure"]
impl crate::Writable for FETCHDECODE0_DYNAMIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FETCHDECODE0_DYNAMIC to value 0"]
impl crate::Resettable for FETCHDECODE0_DYNAMIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
