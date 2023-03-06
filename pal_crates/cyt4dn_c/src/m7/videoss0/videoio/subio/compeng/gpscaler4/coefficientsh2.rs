#[doc = "Register `COEFFICIENTSH2` reader"]
pub struct R(crate::R<COEFFICIENTSH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEFFICIENTSH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEFFICIENTSH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEFFICIENTSH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEFFICIENTSH2` writer"]
pub struct W(crate::W<COEFFICIENTSH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEFFICIENTSH2_SPEC>;
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
impl From<crate::W<COEFFICIENTSH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEFFICIENTSH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEFF_H11` reader - Horizontal coefficient 11. (format is signed integer)"]
pub type COEFF_H11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H11` writer - Horizontal coefficient 11. (format is signed integer)"]
pub type COEFF_H11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H10` reader - Horizontal coefficient 10. (format is signed integer)"]
pub type COEFF_H10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H10` writer - Horizontal coefficient 10. (format is signed integer)"]
pub type COEFF_H10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H9` reader - Horizontal coefficient 9. (format is signed integer)"]
pub type COEFF_H9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H9` writer - Horizontal coefficient 9. (format is signed integer)"]
pub type COEFF_H9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H8` reader - Horizontal coefficient 8. (format is signed integer)"]
pub type COEFF_H8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H8` writer - Horizontal coefficient 8. (format is signed integer)"]
pub type COEFF_H8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Horizontal coefficient 11. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h11(&self) -> COEFF_H11_R {
        COEFF_H11_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal coefficient 10. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h10(&self) -> COEFF_H10_R {
        COEFF_H10_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal coefficient 9. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h9(&self) -> COEFF_H9_R {
        COEFF_H9_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal coefficient 8. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h8(&self) -> COEFF_H8_R {
        COEFF_H8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Horizontal coefficient 11. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h11(&mut self) -> COEFF_H11_W<0> {
        COEFF_H11_W::new(self)
    }
    #[doc = "Bits 8:15 - Horizontal coefficient 10. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h10(&mut self) -> COEFF_H10_W<8> {
        COEFF_H10_W::new(self)
    }
    #[doc = "Bits 16:23 - Horizontal coefficient 9. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h9(&mut self) -> COEFF_H9_W<16> {
        COEFF_H9_W::new(self)
    }
    #[doc = "Bits 24:31 - Horizontal coefficient 8. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h8(&mut self) -> COEFF_H8_W<24> {
        COEFF_H8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal coefficients for filter_mode FIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coefficientsh2](index.html) module"]
pub struct COEFFICIENTSH2_SPEC;
impl crate::RegisterSpec for COEFFICIENTSH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coefficientsh2::R](R) reader structure"]
impl crate::Readable for COEFFICIENTSH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coefficientsh2::W](W) writer structure"]
impl crate::Writable for COEFFICIENTSH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEFFICIENTSH2 to value 0"]
impl crate::Resettable for COEFFICIENTSH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
