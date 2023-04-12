#[doc = "Register `PERSPDELTAXW` reader"]
pub struct R(crate::R<PERSPDELTAXW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPDELTAXW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPDELTAXW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPDELTAXW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPDELTAXW` writer"]
pub struct W(crate::W<PERSPDELTAXW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPDELTAXW_SPEC>;
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
impl From<crate::W<PERSPDELTAXW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPDELTAXW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPDELTAXW` reader - Increment of homogenous W coordinate for horizontal step (X) in destination frame. (format is floating-point 1e8m23)"]
pub type PERSPDELTAXW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPDELTAXW` writer - Increment of homogenous W coordinate for horizontal step (X) in destination frame. (format is floating-point 1e8m23)"]
pub type PERSPDELTAXW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPDELTAXW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Increment of homogenous W coordinate for horizontal step (X) in destination frame. (format is floating-point 1e8m23)"]
    #[inline(always)]
    pub fn perspdeltaxw(&self) -> PERSPDELTAXW_R {
        PERSPDELTAXW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Increment of homogenous W coordinate for horizontal step (X) in destination frame. (format is floating-point 1e8m23)"]
    #[inline(always)]
    #[must_use]
    pub fn perspdeltaxw(&mut self) -> PERSPDELTAXW_W<0> {
        PERSPDELTAXW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DeltaWX increment for perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspdeltaxw](index.html) module"]
pub struct PERSPDELTAXW_SPEC;
impl crate::RegisterSpec for PERSPDELTAXW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspdeltaxw::R](R) reader structure"]
impl crate::Readable for PERSPDELTAXW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspdeltaxw::W](W) writer structure"]
impl crate::Writable for PERSPDELTAXW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPDELTAXW to value 0"]
impl crate::Resettable for PERSPDELTAXW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
