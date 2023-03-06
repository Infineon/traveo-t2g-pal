#[doc = "Register `TRANSFORMATIONMATRIXA12` reader"]
pub struct R(crate::R<TRANSFORMATIONMATRIXA12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSFORMATIONMATRIXA12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSFORMATIONMATRIXA12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSFORMATIONMATRIXA12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSFORMATIONMATRIXA12` writer"]
pub struct W(crate::W<TRANSFORMATIONMATRIXA12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSFORMATIONMATRIXA12_SPEC>;
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
impl From<crate::W<TRANSFORMATIONMATRIXA12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSFORMATIONMATRIXA12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A12` reader - A12, given in signed fixed-point 26.6 two's complement notation."]
pub type A12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `A12` writer - A12, given in signed fixed-point 26.6 two's complement notation."]
pub type A12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSFORMATIONMATRIXA12_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A12, given in signed fixed-point 26.6 two's complement notation."]
    #[inline(always)]
    pub fn a12(&self) -> A12_R {
        A12_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A12, given in signed fixed-point 26.6 two's complement notation."]
    #[inline(always)]
    #[must_use]
    pub fn a12(&mut self) -> A12_W<0> {
        A12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Component A12 of transformation Matrix A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transformationmatrixa12](index.html) module"]
pub struct TRANSFORMATIONMATRIXA12_SPEC;
impl crate::RegisterSpec for TRANSFORMATIONMATRIXA12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transformationmatrixa12::R](R) reader structure"]
impl crate::Readable for TRANSFORMATIONMATRIXA12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transformationmatrixa12::W](W) writer structure"]
impl crate::Writable for TRANSFORMATIONMATRIXA12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSFORMATIONMATRIXA12 to value 0"]
impl crate::Resettable for TRANSFORMATIONMATRIXA12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
