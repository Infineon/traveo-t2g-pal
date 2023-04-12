#[doc = "Register `PERSPSTARTW` reader"]
pub struct R(crate::R<PERSPSTARTW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPSTARTW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPSTARTW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPSTARTW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPSTARTW` writer"]
pub struct W(crate::W<PERSPSTARTW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPSTARTW_SPEC>;
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
impl From<crate::W<PERSPSTARTW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPSTARTW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPSTARTW` reader - Start value for homogenous W coordinate relative to origin."]
pub type PERSPSTARTW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPSTARTW` writer - Start value for homogenous W coordinate relative to origin."]
pub type PERSPSTARTW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPSTARTW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start value for homogenous W coordinate relative to origin."]
    #[inline(always)]
    pub fn perspstartw(&self) -> PERSPSTARTW_R {
        PERSPSTARTW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value for homogenous W coordinate relative to origin."]
    #[inline(always)]
    #[must_use]
    pub fn perspstartw(&mut self) -> PERSPSTARTW_W<0> {
        PERSPSTARTW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start value W for perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspstartw](index.html) module"]
pub struct PERSPSTARTW_SPEC;
impl crate::RegisterSpec for PERSPSTARTW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspstartw::R](R) reader structure"]
impl crate::Readable for PERSPSTARTW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspstartw::W](W) writer structure"]
impl crate::Writable for PERSPSTARTW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPSTARTW to value 0x3f80_0000"]
impl crate::Resettable for PERSPSTARTW_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f80_0000;
}
