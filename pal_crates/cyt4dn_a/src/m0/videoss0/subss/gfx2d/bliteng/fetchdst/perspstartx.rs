#[doc = "Register `PERSPSTARTX` reader"]
pub struct R(crate::R<PERSPSTARTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPSTARTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPSTARTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPSTARTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPSTARTX` writer"]
pub struct W(crate::W<PERSPSTARTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPSTARTX_SPEC>;
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
impl From<crate::W<PERSPSTARTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPSTARTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPSTARTX` reader - Start value for homogenous X coordinate relative to origin."]
pub type PERSPSTARTX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPSTARTX` writer - Start value for homogenous X coordinate relative to origin."]
pub type PERSPSTARTX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPSTARTX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start value for homogenous X coordinate relative to origin."]
    #[inline(always)]
    pub fn perspstartx(&self) -> PERSPSTARTX_R {
        PERSPSTARTX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value for homogenous X coordinate relative to origin."]
    #[inline(always)]
    #[must_use]
    pub fn perspstartx(&mut self) -> PERSPSTARTX_W<0> {
        PERSPSTARTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start value X for affine/perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspstartx](index.html) module"]
pub struct PERSPSTARTX_SPEC;
impl crate::RegisterSpec for PERSPSTARTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspstartx::R](R) reader structure"]
impl crate::Readable for PERSPSTARTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspstartx::W](W) writer structure"]
impl crate::Writable for PERSPSTARTX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPSTARTX to value 0"]
impl crate::Resettable for PERSPSTARTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
