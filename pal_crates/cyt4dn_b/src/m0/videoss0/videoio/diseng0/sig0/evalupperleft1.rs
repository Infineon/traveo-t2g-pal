#[doc = "Register `EVALUPPERLEFT1` reader"]
pub struct R(crate::R<EVALUPPERLEFT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALUPPERLEFT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALUPPERLEFT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALUPPERLEFT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALUPPERLEFT1` writer"]
pub struct W(crate::W<EVALUPPERLEFT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALUPPERLEFT1_SPEC>;
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
impl From<crate::W<EVALUPPERLEFT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALUPPERLEFT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALUPPERLEFT1` reader - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALUPPERLEFT1` writer - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT1_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALUPPERLEFT1` reader - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALUPPERLEFT1` writer - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    pub fn xevalupperleft1(&self) -> XEVALUPPERLEFT1_R {
        XEVALUPPERLEFT1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    pub fn yevalupperleft1(&self) -> YEVALUPPERLEFT1_R {
        YEVALUPPERLEFT1_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn xevalupperleft1(&mut self) -> XEVALUPPERLEFT1_W<0> {
        XEVALUPPERLEFT1_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn yevalupperleft1(&mut self) -> YEVALUPPERLEFT1_W<16> {
        YEVALUPPERLEFT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper left corner of evaluation window 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalupperleft1](index.html) module"]
pub struct EVALUPPERLEFT1_SPEC;
impl crate::RegisterSpec for EVALUPPERLEFT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalupperleft1::R](R) reader structure"]
impl crate::Readable for EVALUPPERLEFT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalupperleft1::W](W) writer structure"]
impl crate::Writable for EVALUPPERLEFT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALUPPERLEFT1 to value 0"]
impl crate::Resettable for EVALUPPERLEFT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
