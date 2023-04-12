#[doc = "Register `PERSPSTARTY` reader"]
pub struct R(crate::R<PERSPSTARTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPSTARTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPSTARTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPSTARTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPSTARTY` writer"]
pub struct W(crate::W<PERSPSTARTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPSTARTY_SPEC>;
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
impl From<crate::W<PERSPSTARTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPSTARTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPSTARTY` reader - Start value for homogenous Y coordinate relative to origin."]
pub type PERSPSTARTY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPSTARTY` writer - Start value for homogenous Y coordinate relative to origin."]
pub type PERSPSTARTY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPSTARTY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start value for homogenous Y coordinate relative to origin."]
    #[inline(always)]
    pub fn perspstarty(&self) -> PERSPSTARTY_R {
        PERSPSTARTY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value for homogenous Y coordinate relative to origin."]
    #[inline(always)]
    #[must_use]
    pub fn perspstarty(&mut self) -> PERSPSTARTY_W<0> {
        PERSPSTARTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start value Y for affine/perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspstarty](index.html) module"]
pub struct PERSPSTARTY_SPEC;
impl crate::RegisterSpec for PERSPSTARTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspstarty::R](R) reader structure"]
impl crate::Readable for PERSPSTARTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspstarty::W](W) writer structure"]
impl crate::Writable for PERSPSTARTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPSTARTY to value 0"]
impl crate::Resettable for PERSPSTARTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
