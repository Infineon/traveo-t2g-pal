#[doc = "Register `PERSPDELTAYX` reader"]
pub struct R(crate::R<PERSPDELTAYX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPDELTAYX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPDELTAYX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPDELTAYX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPDELTAYX` writer"]
pub struct W(crate::W<PERSPDELTAYX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPDELTAYX_SPEC>;
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
impl From<crate::W<PERSPDELTAYX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPDELTAYX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPDELTAYX` reader - Increment of homogenous X coordinate for vertical step (Y) in destination frame. (format is floating-point 1e8m23)"]
pub type PERSPDELTAYX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPDELTAYX` writer - Increment of homogenous X coordinate for vertical step (Y) in destination frame. (format is floating-point 1e8m23)"]
pub type PERSPDELTAYX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPDELTAYX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Increment of homogenous X coordinate for vertical step (Y) in destination frame. (format is floating-point 1e8m23)"]
    #[inline(always)]
    pub fn perspdeltayx(&self) -> PERSPDELTAYX_R {
        PERSPDELTAYX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Increment of homogenous X coordinate for vertical step (Y) in destination frame. (format is floating-point 1e8m23)"]
    #[inline(always)]
    #[must_use]
    pub fn perspdeltayx(&mut self) -> PERSPDELTAYX_W<0> {
        PERSPDELTAYX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DeltaYX increment for affine/perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspdeltayx](index.html) module"]
pub struct PERSPDELTAYX_SPEC;
impl crate::RegisterSpec for PERSPDELTAYX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspdeltayx::R](R) reader structure"]
impl crate::Readable for PERSPDELTAYX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspdeltayx::W](W) writer structure"]
impl crate::Writable for PERSPDELTAYX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPDELTAYX to value 0"]
impl crate::Resettable for PERSPDELTAYX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
