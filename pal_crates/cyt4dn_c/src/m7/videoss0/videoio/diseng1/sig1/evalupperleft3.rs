#[doc = "Register `EVALUPPERLEFT3` reader"]
pub struct R(crate::R<EVALUPPERLEFT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALUPPERLEFT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALUPPERLEFT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALUPPERLEFT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALUPPERLEFT3` writer"]
pub struct W(crate::W<EVALUPPERLEFT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALUPPERLEFT3_SPEC>;
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
impl From<crate::W<EVALUPPERLEFT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALUPPERLEFT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALUPPERLEFT3` reader - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALUPPERLEFT3` writer - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT3_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALUPPERLEFT3` reader - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALUPPERLEFT3` writer - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT3_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    pub fn xevalupperleft3(&self) -> XEVALUPPERLEFT3_R {
        XEVALUPPERLEFT3_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    pub fn yevalupperleft3(&self) -> YEVALUPPERLEFT3_R {
        YEVALUPPERLEFT3_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn xevalupperleft3(&mut self) -> XEVALUPPERLEFT3_W<0> {
        XEVALUPPERLEFT3_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn yevalupperleft3(&mut self) -> YEVALUPPERLEFT3_W<16> {
        YEVALUPPERLEFT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper left corner of evaluation window 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalupperleft3](index.html) module"]
pub struct EVALUPPERLEFT3_SPEC;
impl crate::RegisterSpec for EVALUPPERLEFT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalupperleft3::R](R) reader structure"]
impl crate::Readable for EVALUPPERLEFT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalupperleft3::W](W) writer structure"]
impl crate::Writable for EVALUPPERLEFT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALUPPERLEFT3 to value 0"]
impl crate::Resettable for EVALUPPERLEFT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
