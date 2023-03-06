#[doc = "Register `TRANSFORMATIONMATRIXA22` reader"]
pub struct R(crate::R<TRANSFORMATIONMATRIXA22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSFORMATIONMATRIXA22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSFORMATIONMATRIXA22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSFORMATIONMATRIXA22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSFORMATIONMATRIXA22` writer"]
pub struct W(crate::W<TRANSFORMATIONMATRIXA22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSFORMATIONMATRIXA22_SPEC>;
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
impl From<crate::W<TRANSFORMATIONMATRIXA22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSFORMATIONMATRIXA22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A22` reader - Matrix coefficient A22. (format is signed fix-point 16.16)"]
pub type A22_R = crate::FieldReader<u32, u32>;
#[doc = "Field `A22` writer - Matrix coefficient A22. (format is signed fix-point 16.16)"]
pub type A22_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSFORMATIONMATRIXA22_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Matrix coefficient A22. (format is signed fix-point 16.16)"]
    #[inline(always)]
    pub fn a22(&self) -> A22_R {
        A22_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Matrix coefficient A22. (format is signed fix-point 16.16)"]
    #[inline(always)]
    #[must_use]
    pub fn a22(&mut self) -> A22_W<0> {
        A22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transformation Matrix.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transformationmatrixa22](index.html) module"]
pub struct TRANSFORMATIONMATRIXA22_SPEC;
impl crate::RegisterSpec for TRANSFORMATIONMATRIXA22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transformationmatrixa22::R](R) reader structure"]
impl crate::Readable for TRANSFORMATIONMATRIXA22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transformationmatrixa22::W](W) writer structure"]
impl crate::Writable for TRANSFORMATIONMATRIXA22_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSFORMATIONMATRIXA22 to value 0x0001_0000"]
impl crate::Resettable for TRANSFORMATIONMATRIXA22_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
