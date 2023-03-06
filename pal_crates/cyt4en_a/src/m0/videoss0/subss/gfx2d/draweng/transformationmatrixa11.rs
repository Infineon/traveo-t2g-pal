#[doc = "Register `TRANSFORMATIONMATRIXA11` reader"]
pub struct R(crate::R<TRANSFORMATIONMATRIXA11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSFORMATIONMATRIXA11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSFORMATIONMATRIXA11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSFORMATIONMATRIXA11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSFORMATIONMATRIXA11` writer"]
pub struct W(crate::W<TRANSFORMATIONMATRIXA11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSFORMATIONMATRIXA11_SPEC>;
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
impl From<crate::W<TRANSFORMATIONMATRIXA11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSFORMATIONMATRIXA11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A11` reader - Matrix coefficient A11. (format is signed fix-point 16.16)"]
pub type A11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `A11` writer - Matrix coefficient A11. (format is signed fix-point 16.16)"]
pub type A11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSFORMATIONMATRIXA11_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Matrix coefficient A11. (format is signed fix-point 16.16)"]
    #[inline(always)]
    pub fn a11(&self) -> A11_R {
        A11_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Matrix coefficient A11. (format is signed fix-point 16.16)"]
    #[inline(always)]
    #[must_use]
    pub fn a11(&mut self) -> A11_W<0> {
        A11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transformation Matrix.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transformationmatrixa11](index.html) module"]
pub struct TRANSFORMATIONMATRIXA11_SPEC;
impl crate::RegisterSpec for TRANSFORMATIONMATRIXA11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transformationmatrixa11::R](R) reader structure"]
impl crate::Readable for TRANSFORMATIONMATRIXA11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transformationmatrixa11::W](W) writer structure"]
impl crate::Writable for TRANSFORMATIONMATRIXA11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSFORMATIONMATRIXA11 to value 0x0001_0000"]
impl crate::Resettable for TRANSFORMATIONMATRIXA11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
