#[doc = "Register `STORE4_DYNAMIC` reader"]
pub struct R(crate::R<STORE4_DYNAMIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE4_DYNAMIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE4_DYNAMIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE4_DYNAMIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE4_DYNAMIC` writer"]
pub struct W(crate::W<STORE4_DYNAMIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE4_DYNAMIC_SPEC>;
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
impl From<crate::W<STORE4_DYNAMIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE4_DYNAMIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STORE4_SRC_SEL` reader - Selection of the source for the src input of the store4 module"]
pub type STORE4_SRC_SEL_R = crate::FieldReader<u8, STORE4_SRC_SEL_A>;
#[doc = "Selection of the source for the src input of the store4 module\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STORE4_SRC_SEL_A {
    #[doc = "0: Unit store4 input port src is disabled"]
    DISABLE = 0,
    #[doc = "27: Unit store4 input port src is connected to output of unit extsrc8"]
    EXTSRC8 = 27,
    #[doc = "20: Unit store4 input port src is connected to output of unit gpscaler4"]
    GPSCALER4 = 20,
    #[doc = "9: Unit store4 input port src is connected to output of unit extsrc4"]
    EXTSRC4 = 9,
}
impl From<STORE4_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STORE4_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl STORE4_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STORE4_SRC_SEL_A> {
        match self.bits {
            0 => Some(STORE4_SRC_SEL_A::DISABLE),
            27 => Some(STORE4_SRC_SEL_A::EXTSRC8),
            20 => Some(STORE4_SRC_SEL_A::GPSCALER4),
            9 => Some(STORE4_SRC_SEL_A::EXTSRC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STORE4_SRC_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `EXTSRC8`"]
    #[inline(always)]
    pub fn is_extsrc8(&self) -> bool {
        *self == STORE4_SRC_SEL_A::EXTSRC8
    }
    #[doc = "Checks if the value of the field is `GPSCALER4`"]
    #[inline(always)]
    pub fn is_gpscaler4(&self) -> bool {
        *self == STORE4_SRC_SEL_A::GPSCALER4
    }
    #[doc = "Checks if the value of the field is `EXTSRC4`"]
    #[inline(always)]
    pub fn is_extsrc4(&self) -> bool {
        *self == STORE4_SRC_SEL_A::EXTSRC4
    }
}
#[doc = "Field `STORE4_SRC_SEL` writer - Selection of the source for the src input of the store4 module"]
pub type STORE4_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORE4_DYNAMIC_SPEC, u8, STORE4_SRC_SEL_A, 5, O>;
impl<'a, const O: u8> STORE4_SRC_SEL_W<'a, O> {
    #[doc = "Unit store4 input port src is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STORE4_SRC_SEL_A::DISABLE)
    }
    #[doc = "Unit store4 input port src is connected to output of unit extsrc8"]
    #[inline(always)]
    pub fn extsrc8(self) -> &'a mut W {
        self.variant(STORE4_SRC_SEL_A::EXTSRC8)
    }
    #[doc = "Unit store4 input port src is connected to output of unit gpscaler4"]
    #[inline(always)]
    pub fn gpscaler4(self) -> &'a mut W {
        self.variant(STORE4_SRC_SEL_A::GPSCALER4)
    }
    #[doc = "Unit store4 input port src is connected to output of unit extsrc4"]
    #[inline(always)]
    pub fn extsrc4(self) -> &'a mut W {
        self.variant(STORE4_SRC_SEL_A::EXTSRC4)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the store4 module"]
    #[inline(always)]
    pub fn store4_src_sel(&self) -> STORE4_SRC_SEL_R {
        STORE4_SRC_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the store4 module"]
    #[inline(always)]
    #[must_use]
    pub fn store4_src_sel(&mut self) -> STORE4_SRC_SEL_W<0> {
        STORE4_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic pixel engine configuration for store4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store4_dynamic](index.html) module"]
pub struct STORE4_DYNAMIC_SPEC;
impl crate::RegisterSpec for STORE4_DYNAMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store4_dynamic::R](R) reader structure"]
impl crate::Readable for STORE4_DYNAMIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store4_dynamic::W](W) writer structure"]
impl crate::Writable for STORE4_DYNAMIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE4_DYNAMIC to value 0x09"]
impl crate::Resettable for STORE4_DYNAMIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
