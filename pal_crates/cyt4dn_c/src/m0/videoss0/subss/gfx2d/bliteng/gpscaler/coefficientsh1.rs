#[doc = "Register `COEFFICIENTSH1` reader"]
pub struct R(crate::R<COEFFICIENTSH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEFFICIENTSH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEFFICIENTSH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEFFICIENTSH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEFFICIENTSH1` writer"]
pub struct W(crate::W<COEFFICIENTSH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEFFICIENTSH1_SPEC>;
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
impl From<crate::W<COEFFICIENTSH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEFFICIENTSH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEFF_H7` reader - Horizontal coefficient 7. (format is signed integer)"]
pub type COEFF_H7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H7` writer - Horizontal coefficient 7. (format is signed integer)"]
pub type COEFF_H7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H6` reader - Horizontal coefficient 6. (format is signed integer)"]
pub type COEFF_H6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H6` writer - Horizontal coefficient 6. (format is signed integer)"]
pub type COEFF_H6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H5` reader - Horizontal coefficient 5. (format is signed integer)"]
pub type COEFF_H5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H5` writer - Horizontal coefficient 5. (format is signed integer)"]
pub type COEFF_H5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H4` reader - Horizontal coefficient 4. (format is signed integer)"]
pub type COEFF_H4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H4` writer - Horizontal coefficient 4. (format is signed integer)"]
pub type COEFF_H4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Horizontal coefficient 7. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h7(&self) -> COEFF_H7_R {
        COEFF_H7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal coefficient 6. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h6(&self) -> COEFF_H6_R {
        COEFF_H6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal coefficient 5. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h5(&self) -> COEFF_H5_R {
        COEFF_H5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal coefficient 4. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_h4(&self) -> COEFF_H4_R {
        COEFF_H4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Horizontal coefficient 7. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h7(&mut self) -> COEFF_H7_W<0> {
        COEFF_H7_W::new(self)
    }
    #[doc = "Bits 8:15 - Horizontal coefficient 6. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h6(&mut self) -> COEFF_H6_W<8> {
        COEFF_H6_W::new(self)
    }
    #[doc = "Bits 16:23 - Horizontal coefficient 5. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h5(&mut self) -> COEFF_H5_W<16> {
        COEFF_H5_W::new(self)
    }
    #[doc = "Bits 24:31 - Horizontal coefficient 4. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h4(&mut self) -> COEFF_H4_W<24> {
        COEFF_H4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal coefficients for filter_mode FIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coefficientsh1](index.html) module"]
pub struct COEFFICIENTSH1_SPEC;
impl crate::RegisterSpec for COEFFICIENTSH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coefficientsh1::R](R) reader structure"]
impl crate::Readable for COEFFICIENTSH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coefficientsh1::W](W) writer structure"]
impl crate::Writable for COEFFICIENTSH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEFFICIENTSH1 to value 0"]
impl crate::Resettable for COEFFICIENTSH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
