#[doc = "Register `ARBSTARTY` reader"]
pub struct R(crate::R<ARBSTARTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARBSTARTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARBSTARTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARBSTARTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARBSTARTY` writer"]
pub struct W(crate::W<ARBSTARTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARBSTARTY_SPEC>;
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
impl From<crate::W<ARBSTARTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARBSTARTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBSTARTY` reader - Start point for sample-point interpolation (Y coordinate)."]
pub type ARBSTARTY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ARBSTARTY` writer - Start point for sample-point interpolation (Y coordinate)."]
pub type ARBSTARTY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ARBSTARTY_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - Start point for sample-point interpolation (Y coordinate)."]
    #[inline(always)]
    pub fn arbstarty(&self) -> ARBSTARTY_R {
        ARBSTARTY_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Start point for sample-point interpolation (Y coordinate)."]
    #[inline(always)]
    #[must_use]
    pub fn arbstarty(&mut self) -> ARBSTARTY_W<0> {
        ARBSTARTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start value Y for arbitrary warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arbstarty](index.html) module"]
pub struct ARBSTARTY_SPEC;
impl crate::RegisterSpec for ARBSTARTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arbstarty::R](R) reader structure"]
impl crate::Readable for ARBSTARTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arbstarty::W](W) writer structure"]
impl crate::Writable for ARBSTARTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARBSTARTY to value 0"]
impl crate::Resettable for ARBSTARTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
