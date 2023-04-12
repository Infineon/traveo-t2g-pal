#[doc = "Register `COEFFICIENTSH0` reader"]
pub struct R(crate::R<COEFFICIENTSH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEFFICIENTSH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEFFICIENTSH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEFFICIENTSH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEFFICIENTSH0` writer"]
pub struct W(crate::W<COEFFICIENTSH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEFFICIENTSH0_SPEC>;
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
impl From<crate::W<COEFFICIENTSH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEFFICIENTSH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEFF_H3` reader - Horizontal coefficient 3. (format is signed integer)"]
pub type COEFF_H3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H3` writer - Horizontal coefficient 3. (format is signed integer)"]
pub type COEFF_H3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH0_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H2` reader - Horizontal coefficient 2. (format is signed integer)"]
pub type COEFF_H2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H2` writer - Horizontal coefficient 2. (format is signed integer)"]
pub type COEFF_H2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH0_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H1` reader - Horizontal coefficient 1. (format is signed integer)"]
pub type COEFF_H1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H1` writer - Horizontal coefficient 1. (format is signed integer)"]
pub type COEFF_H1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH0_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H0` reader - Horizontal coefficient 0. (format is signed integer)"]
pub type COEFF_H0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H0` writer - Horizontal coefficient 0. (format is signed integer)"]
pub type COEFF_H0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Horizontal coefficient 3. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h3(&self) -> COEFF_H3_R {
        COEFF_H3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal coefficient 2. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h2(&self) -> COEFF_H2_R {
        COEFF_H2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal coefficient 1. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h1(&self) -> COEFF_H1_R {
        COEFF_H1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal coefficient 0. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h0(&self) -> COEFF_H0_R {
        COEFF_H0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Horizontal coefficient 3. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h3(&mut self) -> COEFF_H3_W<0> {
        COEFF_H3_W::new(self)
    }
    #[doc = "Bits 8:15 - Horizontal coefficient 2. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h2(&mut self) -> COEFF_H2_W<8> {
        COEFF_H2_W::new(self)
    }
    #[doc = "Bits 16:23 - Horizontal coefficient 1. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h1(&mut self) -> COEFF_H1_W<16> {
        COEFF_H1_W::new(self)
    }
    #[doc = "Bits 24:31 - Horizontal coefficient 0. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h0(&mut self) -> COEFF_H0_W<24> {
        COEFF_H0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal coefficients for filter_mode FIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coefficientsh0](index.html) module"]
pub struct COEFFICIENTSH0_SPEC;
impl crate::RegisterSpec for COEFFICIENTSH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coefficientsh0::R](R) reader structure"]
impl crate::Readable for COEFFICIENTSH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coefficientsh0::W](W) writer structure"]
impl crate::Writable for COEFFICIENTSH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEFFICIENTSH0 to value 0"]
impl crate::Resettable for COEFFICIENTSH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
