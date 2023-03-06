#[doc = "Register `EVALUPPERLEFT5` reader"]
pub struct R(crate::R<EVALUPPERLEFT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALUPPERLEFT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALUPPERLEFT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALUPPERLEFT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALUPPERLEFT5` writer"]
pub struct W(crate::W<EVALUPPERLEFT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALUPPERLEFT5_SPEC>;
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
impl From<crate::W<EVALUPPERLEFT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALUPPERLEFT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALUPPERLEFT5` reader - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALUPPERLEFT5` writer - See XEvalUpperLeft0."]
pub type XEVALUPPERLEFT5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT5_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALUPPERLEFT5` reader - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALUPPERLEFT5` writer - See YEvalUpperLeft0."]
pub type YEVALUPPERLEFT5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALUPPERLEFT5_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    pub fn xevalupperleft5(&self) -> XEVALUPPERLEFT5_R {
        XEVALUPPERLEFT5_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    pub fn yevalupperleft5(&self) -> YEVALUPPERLEFT5_R {
        YEVALUPPERLEFT5_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn xevalupperleft5(&mut self) -> XEVALUPPERLEFT5_W<0> {
        XEVALUPPERLEFT5_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalUpperLeft0."]
    #[inline(always)]
    #[must_use]
    pub fn yevalupperleft5(&mut self) -> YEVALUPPERLEFT5_W<16> {
        YEVALUPPERLEFT5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper left corner of evaluation window 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalupperleft5](index.html) module"]
pub struct EVALUPPERLEFT5_SPEC;
impl crate::RegisterSpec for EVALUPPERLEFT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalupperleft5::R](R) reader structure"]
impl crate::Readable for EVALUPPERLEFT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalupperleft5::W](W) writer structure"]
impl crate::Writable for EVALUPPERLEFT5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALUPPERLEFT5 to value 0"]
impl crate::Resettable for EVALUPPERLEFT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
