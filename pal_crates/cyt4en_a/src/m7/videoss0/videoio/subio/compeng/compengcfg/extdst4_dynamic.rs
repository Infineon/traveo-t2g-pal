#[doc = "Register `EXTDST4_DYNAMIC` reader"]
pub struct R(crate::R<EXTDST4_DYNAMIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTDST4_DYNAMIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTDST4_DYNAMIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTDST4_DYNAMIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTDST4_DYNAMIC` writer"]
pub struct W(crate::W<EXTDST4_DYNAMIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTDST4_DYNAMIC_SPEC>;
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
impl From<crate::W<EXTDST4_DYNAMIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTDST4_DYNAMIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTDST4_SRC_SEL` reader - Selection of the source for the src input of the extdst4 module"]
pub type EXTDST4_SRC_SEL_R = crate::FieldReader<u8, EXTDST4_SRC_SEL_A>;
#[doc = "Selection of the source for the src input of the extdst4 module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTDST4_SRC_SEL_A {
    #[doc = "0: Unit extdst4 input port src is disabled"]
    DISABLE = 0,
    #[doc = "3: Unit extdst4 input port src is connected to output of unit constframe4"]
    CONSTFRAME4 = 3,
    #[doc = "20: Unit extdst4 input port src is connected to output of unit gpscaler4"]
    GPSCALER4 = 20,
    #[doc = "9: Unit extdst4 input port src is connected to output of unit extsrc4"]
    EXTSRC4 = 9,
    #[doc = "26: Unit extdst4 input port src is connected to output of unit layerblend5"]
    LAYERBLEND5 = 26,
    #[doc = "25: Unit extdst4 input port src is connected to output of unit layerblend4"]
    LAYERBLEND4 = 25,
    #[doc = "24: Unit extdst4 input port src is connected to output of unit layerblend3"]
    LAYERBLEND3 = 24,
    #[doc = "23: Unit extdst4 input port src is connected to output of unit layerblend2"]
    LAYERBLEND2 = 23,
    #[doc = "22: Unit extdst4 input port src is connected to output of unit layerblend1"]
    LAYERBLEND1 = 22,
}
impl From<EXTDST4_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTDST4_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl EXTDST4_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTDST4_SRC_SEL_A> {
        match self.bits {
            0 => Some(EXTDST4_SRC_SEL_A::DISABLE),
            3 => Some(EXTDST4_SRC_SEL_A::CONSTFRAME4),
            20 => Some(EXTDST4_SRC_SEL_A::GPSCALER4),
            9 => Some(EXTDST4_SRC_SEL_A::EXTSRC4),
            26 => Some(EXTDST4_SRC_SEL_A::LAYERBLEND5),
            25 => Some(EXTDST4_SRC_SEL_A::LAYERBLEND4),
            24 => Some(EXTDST4_SRC_SEL_A::LAYERBLEND3),
            23 => Some(EXTDST4_SRC_SEL_A::LAYERBLEND2),
            22 => Some(EXTDST4_SRC_SEL_A::LAYERBLEND1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CONSTFRAME4`"]
    #[inline(always)]
    pub fn is_constframe4(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::CONSTFRAME4
    }
    #[doc = "Checks if the value of the field is `GPSCALER4`"]
    #[inline(always)]
    pub fn is_gpscaler4(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::GPSCALER4
    }
    #[doc = "Checks if the value of the field is `EXTSRC4`"]
    #[inline(always)]
    pub fn is_extsrc4(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::EXTSRC4
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND5`"]
    #[inline(always)]
    pub fn is_layerblend5(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::LAYERBLEND5
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND4`"]
    #[inline(always)]
    pub fn is_layerblend4(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::LAYERBLEND4
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND3`"]
    #[inline(always)]
    pub fn is_layerblend3(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::LAYERBLEND3
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND2`"]
    #[inline(always)]
    pub fn is_layerblend2(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::LAYERBLEND2
    }
    #[doc = "Checks if the value of the field is `LAYERBLEND1`"]
    #[inline(always)]
    pub fn is_layerblend1(&self) -> bool {
        *self == EXTDST4_SRC_SEL_A::LAYERBLEND1
    }
}
#[doc = "Field `EXTDST4_SRC_SEL` writer - Selection of the source for the src input of the extdst4 module"]
pub type EXTDST4_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTDST4_DYNAMIC_SPEC, u8, EXTDST4_SRC_SEL_A, 5, O>;
impl<'a, const O: u8> EXTDST4_SRC_SEL_W<'a, O> {
    #[doc = "Unit extdst4 input port src is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::DISABLE)
    }
    #[doc = "Unit extdst4 input port src is connected to output of unit constframe4"]
    #[inline(always)]
    pub fn constframe4(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::CONSTFRAME4)
    }
    #[doc = "Unit extdst4 input port src is connected to output of unit gpscaler4"]
    #[inline(always)]
    pub fn gpscaler4(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::GPSCALER4)
    }
    #[doc = "Unit extdst4 input port src is connected to output of unit extsrc4"]
    #[inline(always)]
    pub fn extsrc4(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::EXTSRC4)
    }
    #[doc = "Unit extdst4 input port src is connected to output of unit layerblend5"]
    #[inline(always)]
    pub fn layerblend5(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::LAYERBLEND5)
    }
    #[doc = "Unit extdst4 input port src is connected to output of unit layerblend4"]
    #[inline(always)]
    pub fn layerblend4(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::LAYERBLEND4)
    }
    #[doc = "Unit extdst4 input port src is connected to output of unit layerblend3"]
    #[inline(always)]
    pub fn layerblend3(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::LAYERBLEND3)
    }
    #[doc = "Unit extdst4 input port src is connected to output of unit layerblend2"]
    #[inline(always)]
    pub fn layerblend2(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::LAYERBLEND2)
    }
    #[doc = "Unit extdst4 input port src is connected to output of unit layerblend1"]
    #[inline(always)]
    pub fn layerblend1(self) -> &'a mut W {
        self.variant(EXTDST4_SRC_SEL_A::LAYERBLEND1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the extdst4 module"]
    #[inline(always)]
    pub fn extdst4_src_sel(&self) -> EXTDST4_SRC_SEL_R {
        EXTDST4_SRC_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the extdst4 module"]
    #[inline(always)]
    #[must_use]
    pub fn extdst4_src_sel(&mut self) -> EXTDST4_SRC_SEL_W<0> {
        EXTDST4_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic pixel engine configuration for extdst4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extdst4_dynamic](index.html) module"]
pub struct EXTDST4_DYNAMIC_SPEC;
impl crate::RegisterSpec for EXTDST4_DYNAMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extdst4_dynamic::R](R) reader structure"]
impl crate::Readable for EXTDST4_DYNAMIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extdst4_dynamic::W](W) writer structure"]
impl crate::Writable for EXTDST4_DYNAMIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTDST4_DYNAMIC to value 0"]
impl crate::Resettable for EXTDST4_DYNAMIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
