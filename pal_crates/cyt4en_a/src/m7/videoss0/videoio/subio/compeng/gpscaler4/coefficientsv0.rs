#[doc = "Register `COEFFICIENTSV0` reader"]
pub struct R(crate::R<COEFFICIENTSV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEFFICIENTSV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEFFICIENTSV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEFFICIENTSV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEFFICIENTSV0` writer"]
pub struct W(crate::W<COEFFICIENTSV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEFFICIENTSV0_SPEC>;
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
impl From<crate::W<COEFFICIENTSV0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEFFICIENTSV0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEFF_V3` reader - Vertical coefficient 3. (format is signed integer)"]
pub type COEFF_V3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V3` writer - Vertical coefficient 3. (format is signed integer)"]
pub type COEFF_V3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV0_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V2` reader - Vertical coefficient 2. (format is signed integer)"]
pub type COEFF_V2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V2` writer - Vertical coefficient 2. (format is signed integer)"]
pub type COEFF_V2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV0_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V1` reader - Vertical coefficient 1. (format is signed integer)"]
pub type COEFF_V1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V1` writer - Vertical coefficient 1. (format is signed integer)"]
pub type COEFF_V1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV0_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V0` reader - Vertical coefficient 0. (format is signed integer)"]
pub type COEFF_V0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V0` writer - Vertical coefficient 0. (format is signed integer)"]
pub type COEFF_V0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Vertical coefficient 3. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v3(&self) -> COEFF_V3_R {
        COEFF_V3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vertical coefficient 2. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v2(&self) -> COEFF_V2_R {
        COEFF_V2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Vertical coefficient 1. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v1(&self) -> COEFF_V1_R {
        COEFF_V1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical coefficient 0. (format is signed integer)"]
    #[inline(always)]
    pub fn coeff_v0(&self) -> COEFF_V0_R {
        COEFF_V0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Vertical coefficient 3. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v3(&mut self) -> COEFF_V3_W<0> {
        COEFF_V3_W::new(self)
    }
    #[doc = "Bits 8:15 - Vertical coefficient 2. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v2(&mut self) -> COEFF_V2_W<8> {
        COEFF_V2_W::new(self)
    }
    #[doc = "Bits 16:23 - Vertical coefficient 1. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v1(&mut self) -> COEFF_V1_W<16> {
        COEFF_V1_W::new(self)
    }
    #[doc = "Bits 24:31 - Vertical coefficient 0. (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v0(&mut self) -> COEFF_V0_W<24> {
        COEFF_V0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical coefficients for filter_mode FIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coefficientsv0](index.html) module"]
pub struct COEFFICIENTSV0_SPEC;
impl crate::RegisterSpec for COEFFICIENTSV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coefficientsv0::R](R) reader structure"]
impl crate::Readable for COEFFICIENTSV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coefficientsv0::W](W) writer structure"]
impl crate::Writable for COEFFICIENTSV0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEFFICIENTSV0 to value 0"]
impl crate::Resettable for COEFFICIENTSV0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
