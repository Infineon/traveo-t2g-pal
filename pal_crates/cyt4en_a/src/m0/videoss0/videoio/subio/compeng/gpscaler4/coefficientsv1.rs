#[doc = "Register `COEFFICIENTSV1` reader"]
pub struct R(crate::R<COEFFICIENTSV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEFFICIENTSV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEFFICIENTSV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEFFICIENTSV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEFFICIENTSV1` writer"]
pub struct W(crate::W<COEFFICIENTSV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEFFICIENTSV1_SPEC>;
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
impl From<crate::W<COEFFICIENTSV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEFFICIENTSV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEFF_V7` reader - Vertical coefficient 7. (format is signed integer)"]
pub type COEFF_V7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V7` writer - Vertical coefficient 7. (format is signed integer)"]
pub type COEFF_V7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V6` reader - Vertical coefficient 6. (format is signed integer)"]
pub type COEFF_V6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V6` writer - Vertical coefficient 6. (format is signed integer)"]
pub type COEFF_V6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V5` reader - Vertical coefficient 5. (format is signed integer)"]
pub type COEFF_V5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V5` writer - Vertical coefficient 5. (format is signed integer)"]
pub type COEFF_V5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V4` reader - Vertical coefficient 4. (format is signed integer)"]
pub type COEFF_V4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V4` writer - Vertical coefficient 4. (format is signed integer)"]
pub type COEFF_V4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Vertical coefficient 7. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v7(&self) -> COEFF_V7_R {
        COEFF_V7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vertical coefficient 6. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v6(&self) -> COEFF_V6_R {
        COEFF_V6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Vertical coefficient 5. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v5(&self) -> COEFF_V5_R {
        COEFF_V5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical coefficient 4. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v4(&self) -> COEFF_V4_R {
        COEFF_V4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Vertical coefficient 7. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v7(&mut self) -> COEFF_V7_W<0> {
        COEFF_V7_W::new(self)
    }
    #[doc = "Bits 8:15 - Vertical coefficient 6. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v6(&mut self) -> COEFF_V6_W<8> {
        COEFF_V6_W::new(self)
    }
    #[doc = "Bits 16:23 - Vertical coefficient 5. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v5(&mut self) -> COEFF_V5_W<16> {
        COEFF_V5_W::new(self)
    }
    #[doc = "Bits 24:31 - Vertical coefficient 4. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v4(&mut self) -> COEFF_V4_W<24> {
        COEFF_V4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical coefficients for filter_mode FIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coefficientsv1](index.html) module"]
pub struct COEFFICIENTSV1_SPEC;
impl crate::RegisterSpec for COEFFICIENTSV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coefficientsv1::R](R) reader structure"]
impl crate::Readable for COEFFICIENTSV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coefficientsv1::W](W) writer structure"]
impl crate::Writable for COEFFICIENTSV1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEFFICIENTSV1 to value 0"]
impl crate::Resettable for COEFFICIENTSV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
