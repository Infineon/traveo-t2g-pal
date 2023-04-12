#[doc = "Register `COEFFICIENTSV2` reader"]
pub struct R(crate::R<COEFFICIENTSV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEFFICIENTSV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEFFICIENTSV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEFFICIENTSV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEFFICIENTSV2` writer"]
pub struct W(crate::W<COEFFICIENTSV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEFFICIENTSV2_SPEC>;
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
impl From<crate::W<COEFFICIENTSV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEFFICIENTSV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEFF_V11` reader - Vertical coefficient 11. (format is signed integer)"]
pub type COEFF_V11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V11` writer - Vertical coefficient 11. (format is signed integer)"]
pub type COEFF_V11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V10` reader - Vertical coefficient 10. (format is signed integer)"]
pub type COEFF_V10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V10` writer - Vertical coefficient 10. (format is signed integer)"]
pub type COEFF_V10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V9` reader - Vertical coefficient 9. (format is signed integer)"]
pub type COEFF_V9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V9` writer - Vertical coefficient 9. (format is signed integer)"]
pub type COEFF_V9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV2_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V8` reader - Vertical coefficient 8. (format is signed integer)"]
pub type COEFF_V8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V8` writer - Vertical coefficient 8. (format is signed integer)"]
pub type COEFF_V8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Vertical coefficient 11. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v11(&self) -> COEFF_V11_R {
        COEFF_V11_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vertical coefficient 10. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v10(&self) -> COEFF_V10_R {
        COEFF_V10_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Vertical coefficient 9. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v9(&self) -> COEFF_V9_R {
        COEFF_V9_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical coefficient 8. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v8(&self) -> COEFF_V8_R {
        COEFF_V8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Vertical coefficient 11. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v11(&mut self) -> COEFF_V11_W<0> {
        COEFF_V11_W::new(self)
    }
    #[doc = "Bits 8:15 - Vertical coefficient 10. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v10(&mut self) -> COEFF_V10_W<8> {
        COEFF_V10_W::new(self)
    }
    #[doc = "Bits 16:23 - Vertical coefficient 9. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v9(&mut self) -> COEFF_V9_W<16> {
        COEFF_V9_W::new(self)
    }
    #[doc = "Bits 24:31 - Vertical coefficient 8. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v8(&mut self) -> COEFF_V8_W<24> {
        COEFF_V8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical coefficients for filter_mode FIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coefficientsv2](index.html) module"]
pub struct COEFFICIENTSV2_SPEC;
impl crate::RegisterSpec for COEFFICIENTSV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coefficientsv2::R](R) reader structure"]
impl crate::Readable for COEFFICIENTSV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coefficientsv2::W](W) writer structure"]
impl crate::Writable for COEFFICIENTSV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEFFICIENTSV2 to value 0"]
impl crate::Resettable for COEFFICIENTSV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
