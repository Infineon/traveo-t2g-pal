#[doc = "Register `EXTDST1_DYNAMIC` reader"]
pub struct R(crate::R<EXTDST1_DYNAMIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTDST1_DYNAMIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTDST1_DYNAMIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTDST1_DYNAMIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTDST1_DYNAMIC` writer"]
pub struct W(crate::W<EXTDST1_DYNAMIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTDST1_DYNAMIC_SPEC>;
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
impl From<crate::W<EXTDST1_DYNAMIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTDST1_DYNAMIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTDST1_SRC_SEL` reader - Selection of the source for the src input of the extdst1 module"]
pub type EXTDST1_SRC_SEL_R = crate::FieldReader<u8, EXTDST1_SRC_SEL_A>;
#[doc = "Selection of the source for the src input of the extdst1 module\n\nValue on reset: 24"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTDST1_SRC_SEL_A {
    #[doc = "0: Unit extdst1 input port src is disabled"]
    DISABLE = 0,
    #[doc = "5: Unit extdst1 input port src is connected to output of unit constframe1"]
    CONSTFRAME1 = 5,
    #[doc = "20: Unit extdst1 input port src is connected to output of unit gpscaler4"]
    GPSCALER4 = 20,
    #[doc = "9: Unit extdst1 input port src is connected to output of unit extsrc4"]
    EXTSRC4 = 9,
    #[doc = "26: Unit extdst1 input port src is connected to output of unit layerblend5"]
    LAYERBLEND5 = 26,
    #[doc = "25: Unit extdst1 input port src is connected to output of unit layerblend4"]
    LAYERBLEND4 = 25,
    #[doc = "24: Unit extdst1 input port src is connected to output of unit layerblend3"]
    LAYERBLEND3 = 24,
    #[doc = "23: Unit extdst1 input port src is connected to output of unit layerblend2"]
    LAYERBLEND2 = 23,
    #[doc = "22: Unit extdst1 input port src is connected to output of unit layerblend1"]
    LAYERBLEND1 = 22,
}
impl From<EXTDST1_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTDST1_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl EXTDST1_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTDST1_SRC_SEL_A> {
        match self.bits {
            0 => Some(EXTDST1_SRC_SEL_A::DISABLE),
            5 => Some(EXTDST1_SRC_SEL_A::CONSTFRAME1),
            20 => Some(EXTDST1_SRC_SEL_A::GPSCALER4),
            9 => Some(EXTDST1_SRC_SEL_A::EXTSRC4),
            26 => Some(EXTDST1_SRC_SEL_A::LAYERBLEND5),
            25 => Some(EXTDST1_SRC_SEL_A::LAYERBLEND4),
            24 => Some(EXTDST1_SRC_SEL_A::LAYERBLEND3),
            23 => Some(EXTDST1_SRC_SEL_A::LAYERBLEND2),
            22 => Some(EXTDST1_SRC_SEL_A::LAYERBLEND1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CONSTFRAME1`"]
    #[inline(always)]
    pub fn is_constframe1(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::CONSTFRAME1
    }
    #[doc = "Checks if the value of the field is `GPSCALER4`"]
    #[inline(always)]
    pub fn is_gpscaler4(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::GPSCALER4
    }
    #[doc = "Checks if the value of the field is `EXTSRC4`"]
    #[inline(always)]
    pub fn is_extsrc4(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::EXTSRC4
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND5`"]
    #[inline(always)]
    pub fn is_layerblend5(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::LAYERBLEND5
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND4`"]
    #[inline(always)]
    pub fn is_layerblend4(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::LAYERBLEND4
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND3`"]
    #[inline(always)]
    pub fn is_layerblend3(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::LAYERBLEND3
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND2`"]
    #[inline(always)]
    pub fn is_layerblend2(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::LAYERBLEND2
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND1`"]
    #[inline(always)]
    pub fn is_layerblend1(&self) -> bool {
        *self == EXTDST1_SRC_SEL_A::LAYERBLEND1
    }
}
#[doc = "Field `EXTDST1_SRC_SEL` writer - Selection of the source for the src input of the extdst1 module"]
pub type EXTDST1_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTDST1_DYNAMIC_SPEC, u8, EXTDST1_SRC_SEL_A, 5, O>;
impl<'a, const O: u8> EXTDST1_SRC_SEL_W<'a, O> {
    #[doc = "Unit extdst1 input port src is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::DISABLE)
    }
    #[doc = "Unit extdst1 input port src is connected to output of unit constframe1"]
    #[inline(always)]
    pub fn constframe1(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::CONSTFRAME1)
    }
    #[doc = "Unit extdst1 input port src is connected to output of unit gpscaler4"]
    #[inline(always)]
    pub fn gpscaler4(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::GPSCALER4)
    }
    #[doc = "Unit extdst1 input port src is connected to output of unit extsrc4"]
    #[inline(always)]
    pub fn extsrc4(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::EXTSRC4)
    }
    #[doc = "Unit extdst1 input port src is connected to output of unit layerblend5"]
    #[inline(always)]
    pub fn layerblend5(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::LAYERBLEND5)
    }
    #[doc = "Unit extdst1 input port src is connected to output of unit layerblend4"]
    #[inline(always)]
    pub fn layerblend4(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::LAYERBLEND4)
    }
    #[doc = "Unit extdst1 input port src is connected to output of unit layerblend3"]
    #[inline(always)]
    pub fn layerblend3(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::LAYERBLEND3)
    }
    #[doc = "Unit extdst1 input port src is connected to output of unit layerblend2"]
    #[inline(always)]
    pub fn layerblend2(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::LAYERBLEND2)
    }
    #[doc = "Unit extdst1 input port src is connected to output of unit layerblend1"]
    #[inline(always)]
    pub fn layerblend1(self) -> &'a mut W {
        self.variant(EXTDST1_SRC_SEL_A::LAYERBLEND1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the extdst1 module"]
    #[inline(always)]
    pub fn extdst1_src_sel(&self) -> EXTDST1_SRC_SEL_R {
        EXTDST1_SRC_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the extdst1 module"]
    #[inline(always)]
    #[must_use]
    pub fn extdst1_src_sel(&mut self) -> EXTDST1_SRC_SEL_W<0> {
        EXTDST1_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic pixel engine configuration for extdst1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extdst1_dynamic](index.html) module"]
pub struct EXTDST1_DYNAMIC_SPEC;
impl crate::RegisterSpec for EXTDST1_DYNAMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extdst1_dynamic::R](R) reader structure"]
impl crate::Readable for EXTDST1_DYNAMIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extdst1_dynamic::W](W) writer structure"]
impl crate::Writable for EXTDST1_DYNAMIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTDST1_DYNAMIC to value 0x18"]
impl crate::Resettable for EXTDST1_DYNAMIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
