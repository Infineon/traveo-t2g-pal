#[doc = "Register `ARBSTARTX` reader"]
pub struct R(crate::R<ARBSTARTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARBSTARTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARBSTARTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARBSTARTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARBSTARTX` writer"]
pub struct W(crate::W<ARBSTARTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARBSTARTX_SPEC>;
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
impl From<crate::W<ARBSTARTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARBSTARTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBSTARTX` reader - Start point for sample-point interpolation (X coordinate). (format is signed fix-point 16.5)"]
pub type ARBSTARTX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ARBSTARTX` writer - Start point for sample-point interpolation (X coordinate). (format is signed fix-point 16.5)"]
pub type ARBSTARTX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ARBSTARTX_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - Start point for sample-point interpolation (X coordinate). (format is signed fix-point 16.5)"]
    #[inline(always)]
    pub fn arbstartx(&self) -> ARBSTARTX_R {
        ARBSTARTX_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Start point for sample-point interpolation (X coordinate). (format is signed fix-point 16.5)"]
    #[inline(always)]
    #[must_use]
    pub fn arbstartx(&mut self) -> ARBSTARTX_W<0> {
        ARBSTARTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start value X for arbitrary warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arbstartx](index.html) module"]
pub struct ARBSTARTX_SPEC;
impl crate::RegisterSpec for ARBSTARTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arbstartx::R](R) reader structure"]
impl crate::Readable for ARBSTARTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arbstartx::W](W) writer structure"]
impl crate::Writable for ARBSTARTX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARBSTARTX to value 0"]
impl crate::Resettable for ARBSTARTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
