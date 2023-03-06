#[doc = "Register `EVALUPPERLEFT6` reader"]
pub struct R(crate::R<EVALUPPERLEFT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALUPPERLEFT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALUPPERLEFT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALUPPERLEFT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALUPPERLEFT6` writer"]
pub struct W(crate::W<EVALUPPERLEFT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALUPPERLEFT6_SPEC>;
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
impl From<crate::W<EVALUPPERLEFT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALUPPERLEFT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALUPPERLEFT6` reader - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALUPPERLEFT6` writer - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT6_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALUPPERLEFT6` reader - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALUPPERLEFT6` writer - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT6_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    pub fn xevalupperleft6(&self) -> XEVALUPPERLEFT6_R {
        XEVALUPPERLEFT6_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    pub fn yevalupperleft6(&self) -> YEVALUPPERLEFT6_R {
        YEVALUPPERLEFT6_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn xevalupperleft6(&mut self) -> XEVALUPPERLEFT6_W<0> {
        XEVALUPPERLEFT6_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn yevalupperleft6(&mut self) -> YEVALUPPERLEFT6_W<16> {
        YEVALUPPERLEFT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper left corner of evaluation window 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalupperleft6](index.html) module"]
pub struct EVALUPPERLEFT6_SPEC;
impl crate::RegisterSpec for EVALUPPERLEFT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalupperleft6::R](R) reader structure"]
impl crate::Readable for EVALUPPERLEFT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalupperleft6::W](W) writer structure"]
impl crate::Writable for EVALUPPERLEFT6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALUPPERLEFT6 to value 0"]
impl crate::Resettable for EVALUPPERLEFT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
