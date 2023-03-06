#[doc = "Register `GAMMACOR4_DYNAMIC` reader"]
pub struct R(crate::R<GAMMACOR4_DYNAMIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAMMACOR4_DYNAMIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAMMACOR4_DYNAMIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAMMACOR4_DYNAMIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAMMACOR4_DYNAMIC` writer"]
pub struct W(crate::W<GAMMACOR4_DYNAMIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAMMACOR4_DYNAMIC_SPEC>;
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
impl From<crate::W<GAMMACOR4_DYNAMIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAMMACOR4_DYNAMIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAMMACOR4_SRC_SEL` reader - Selection of the source for the src input of the gammacor4 module"]
pub type GAMMACOR4_SRC_SEL_R = crate::FieldReader<u8, GAMMACOR4_SRC_SEL_A>;
#[doc = "Selection of the source for the src input of the gammacor4 module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GAMMACOR4_SRC_SEL_A {
    #[doc = "0: Unit gammacor4 input port src is disabled"]
    DISABLE = 0,
    #[doc = "9: Unit gammacor4 input port src is connected to output of unit extsrc4"]
    EXTSRC4 = 9,
    #[doc = "12: Unit gammacor4 input port src is connected to output of unit fetchdecode4"]
    FETCHDECODE4 = 12,
}
impl From<GAMMACOR4_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAMMACOR4_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl GAMMACOR4_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAMMACOR4_SRC_SEL_A> {
        match self.bits {
            0 => Some(GAMMACOR4_SRC_SEL_A::DISABLE),
            9 => Some(GAMMACOR4_SRC_SEL_A::EXTSRC4),
            12 => Some(GAMMACOR4_SRC_SEL_A::FETCHDECODE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GAMMACOR4_SRC_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `EXTSRC4`"]
    #[inline(always)]
    pub fn is_extsrc4(&self) -> bool {
        *self == GAMMACOR4_SRC_SEL_A::EXTSRC4
    }
    #[doc = "Checks if the value of the field is `FETCHDECODE4`"]
    #[inline(always)]
    pub fn is_fetchdecode4(&self) -> bool {
        *self == GAMMACOR4_SRC_SEL_A::FETCHDECODE4
    }
}
#[doc = "Field `GAMMACOR4_SRC_SEL` writer - Selection of the source for the src input of the gammacor4 module"]
pub type GAMMACOR4_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GAMMACOR4_DYNAMIC_SPEC, u8, GAMMACOR4_SRC_SEL_A, 5, O>;
impl<'a, const O: u8> GAMMACOR4_SRC_SEL_W<'a, O> {
    #[doc = "Unit gammacor4 input port src is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GAMMACOR4_SRC_SEL_A::DISABLE)
    }
    #[doc = "Unit gammacor4 input port src is connected to output of unit extsrc4"]
    #[inline(always)]
    pub fn extsrc4(self) -> &'a mut W {
        self.variant(GAMMACOR4_SRC_SEL_A::EXTSRC4)
    }
    #[doc = "Unit gammacor4 input port src is connected to output of unit fetchdecode4"]
    #[inline(always)]
    pub fn fetchdecode4(self) -> &'a mut W {
        self.variant(GAMMACOR4_SRC_SEL_A::FETCHDECODE4)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the gammacor4 module"]
    #[inline(always)]
    pub fn gammacor4_src_sel(&self) -> GAMMACOR4_SRC_SEL_R {
        GAMMACOR4_SRC_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selection of the source for the src input of the gammacor4 module"]
    #[inline(always)]
    #[must_use]
    pub fn gammacor4_src_sel(&mut self) -> GAMMACOR4_SRC_SEL_W<0> {
        GAMMACOR4_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic pixel engine configuration for gammacor4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gammacor4_dynamic](index.html) module"]
pub struct GAMMACOR4_DYNAMIC_SPEC;
impl crate::RegisterSpec for GAMMACOR4_DYNAMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gammacor4_dynamic::R](R) reader structure"]
impl crate::Readable for GAMMACOR4_DYNAMIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gammacor4_dynamic::W](W) writer structure"]
impl crate::Writable for GAMMACOR4_DYNAMIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAMMACOR4_DYNAMIC to value 0"]
impl crate::Resettable for GAMMACOR4_DYNAMIC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
