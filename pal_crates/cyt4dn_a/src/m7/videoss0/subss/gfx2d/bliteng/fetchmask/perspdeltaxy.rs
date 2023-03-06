#[doc = "Register `PERSPDELTAXY` reader"]
pub struct R(crate::R<PERSPDELTAXY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPDELTAXY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPDELTAXY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPDELTAXY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPDELTAXY` writer"]
pub struct W(crate::W<PERSPDELTAXY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPDELTAXY_SPEC>;
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
impl From<crate::W<PERSPDELTAXY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPDELTAXY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPDELTAXY` reader - Increment of homogenous Y coordinate for horizontal step (X) in destination frame."]
pub type PERSPDELTAXY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPDELTAXY` writer - Increment of homogenous Y coordinate for horizontal step (X) in destination frame."]
pub type PERSPDELTAXY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPDELTAXY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Increment of homogenous Y coordinate for horizontal step (X) in destination frame."]
    #[inline(always)]
    pub fn perspdeltaxy(&self) -> PERSPDELTAXY_R {
        PERSPDELTAXY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Increment of homogenous Y coordinate for horizontal step (X) in destination frame."]
    #[inline(always)]
    #[must_use]
    pub fn perspdeltaxy(&mut self) -> PERSPDELTAXY_W<0> {
        PERSPDELTAXY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DeltaXY increment for affine/perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspdeltaxy](index.html) module"]
pub struct PERSPDELTAXY_SPEC;
impl crate::RegisterSpec for PERSPDELTAXY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspdeltaxy::R](R) reader structure"]
impl crate::Readable for PERSPDELTAXY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspdeltaxy::W](W) writer structure"]
impl crate::Writable for PERSPDELTAXY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPDELTAXY to value 0"]
impl crate::Resettable for PERSPDELTAXY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
