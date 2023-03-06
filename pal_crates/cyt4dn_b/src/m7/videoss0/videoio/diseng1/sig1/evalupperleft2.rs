#[doc = "Register `EVALUPPERLEFT2` reader"]
pub struct R(crate::R<EVALUPPERLEFT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALUPPERLEFT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALUPPERLEFT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALUPPERLEFT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALUPPERLEFT2` writer"]
pub struct W(crate::W<EVALUPPERLEFT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALUPPERLEFT2_SPEC>;
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
impl From<crate::W<EVALUPPERLEFT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALUPPERLEFT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALUPPERLEFT2` reader - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALUPPERLEFT2` writer - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT2_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALUPPERLEFT2` reader - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALUPPERLEFT2` writer - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT2_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    pub fn xevalupperleft2(&self) -> XEVALUPPERLEFT2_R {
        XEVALUPPERLEFT2_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    pub fn yevalupperleft2(&self) -> YEVALUPPERLEFT2_R {
        YEVALUPPERLEFT2_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn xevalupperleft2(&mut self) -> XEVALUPPERLEFT2_W<0> {
        XEVALUPPERLEFT2_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn yevalupperleft2(&mut self) -> YEVALUPPERLEFT2_W<16> {
        YEVALUPPERLEFT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper left corner of evaluation window 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalupperleft2](index.html) module"]
pub struct EVALUPPERLEFT2_SPEC;
impl crate::RegisterSpec for EVALUPPERLEFT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalupperleft2::R](R) reader structure"]
impl crate::Readable for EVALUPPERLEFT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalupperleft2::W](W) writer structure"]
impl crate::Writable for EVALUPPERLEFT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALUPPERLEFT2 to value 0"]
impl crate::Resettable for EVALUPPERLEFT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
