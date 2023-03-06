#[doc = "Register `EVALUPPERLEFT0` reader"]
pub struct R(crate::R<EVALUPPERLEFT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALUPPERLEFT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALUPPERLEFT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALUPPERLEFT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALUPPERLEFT0` writer"]
pub struct W(crate::W<EVALUPPERLEFT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALUPPERLEFT0_SPEC>;
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
impl From<crate::W<EVALUPPERLEFT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALUPPERLEFT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALUPPERLEFT0` reader - X coordinate."]
pub type XEVALUPPERLEFT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALUPPERLEFT0` writer - X coordinate."]
pub type XEVALUPPERLEFT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT0_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALUPPERLEFT0` reader - Y coordinate."]
pub type YEVALUPPERLEFT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALUPPERLEFT0` writer - Y coordinate."]
pub type YEVALUPPERLEFT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT0_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - X coordinate."]
    #[inline(always)]
    pub fn xevalupperleft0(&self) -> XEVALUPPERLEFT0_R {
        XEVALUPPERLEFT0_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Y coordinate."]
    #[inline(always)]
    pub fn yevalupperleft0(&self) -> YEVALUPPERLEFT0_R {
        YEVALUPPERLEFT0_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - X coordinate."]
    #[inline(always)]
    #[must_use]
    pub fn xevalupperleft0(&mut self) -> XEVALUPPERLEFT0_W<0> {
        XEVALUPPERLEFT0_W::new(self)
    }
    #[doc = "Bits 16:29 - Y coordinate."]
    #[inline(always)]
    #[must_use]
    pub fn yevalupperleft0(&mut self) -> YEVALUPPERLEFT0_W<16> {
        YEVALUPPERLEFT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper left corner of evaluation window 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalupperleft0](index.html) module"]
pub struct EVALUPPERLEFT0_SPEC;
impl crate::RegisterSpec for EVALUPPERLEFT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalupperleft0::R](R) reader structure"]
impl crate::Readable for EVALUPPERLEFT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalupperleft0::W](W) writer structure"]
impl crate::Writable for EVALUPPERLEFT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALUPPERLEFT0 to value 0"]
impl crate::Resettable for EVALUPPERLEFT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
