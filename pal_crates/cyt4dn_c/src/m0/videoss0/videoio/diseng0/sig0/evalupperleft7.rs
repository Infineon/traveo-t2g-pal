#[doc = "Register `EVALUPPERLEFT7` reader"]
pub struct R(crate::R<EVALUPPERLEFT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALUPPERLEFT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALUPPERLEFT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALUPPERLEFT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALUPPERLEFT7` writer"]
pub struct W(crate::W<EVALUPPERLEFT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALUPPERLEFT7_SPEC>;
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
impl From<crate::W<EVALUPPERLEFT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALUPPERLEFT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALUPPERLEFT7` reader - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALUPPERLEFT7` writer - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT7_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALUPPERLEFT7` reader - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALUPPERLEFT7` writer - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT7_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    pub fn xevalupperleft7(&self) -> XEVALUPPERLEFT7_R {
        XEVALUPPERLEFT7_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    pub fn yevalupperleft7(&self) -> YEVALUPPERLEFT7_R {
        YEVALUPPERLEFT7_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn xevalupperleft7(&mut self) -> XEVALUPPERLEFT7_W<0> {
        XEVALUPPERLEFT7_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn yevalupperleft7(&mut self) -> YEVALUPPERLEFT7_W<16> {
        YEVALUPPERLEFT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper left corner of evaluation window 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalupperleft7](index.html) module"]
pub struct EVALUPPERLEFT7_SPEC;
impl crate::RegisterSpec for EVALUPPERLEFT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalupperleft7::R](R) reader structure"]
impl crate::Readable for EVALUPPERLEFT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalupperleft7::W](W) writer structure"]
impl crate::Writable for EVALUPPERLEFT7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALUPPERLEFT7 to value 0"]
impl crate::Resettable for EVALUPPERLEFT7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
