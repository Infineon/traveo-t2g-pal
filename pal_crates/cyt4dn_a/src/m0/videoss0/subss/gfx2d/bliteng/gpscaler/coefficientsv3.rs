#[doc = "Register `COEFFICIENTSV3` reader"]
pub struct R(crate::R<COEFFICIENTSV3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEFFICIENTSV3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEFFICIENTSV3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEFFICIENTSV3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEFFICIENTSV3` writer"]
pub struct W(crate::W<COEFFICIENTSV3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEFFICIENTSV3_SPEC>;
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
impl From<crate::W<COEFFICIENTSV3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEFFICIENTSV3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEFF_V14` reader - Vertical coefficient 14. The bottommost coefficient regardless of the filter height. In contrast to the coeff_v0 this coefficient has to be always programmed."]
pub type COEFF_V14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V14` writer - Vertical coefficient 14. The bottommost coefficient regardless of the filter height. In contrast to the coeff_v0 this coefficient has to be always programmed."]
pub type COEFF_V14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV3_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V13` reader - Vertical coefficient 13."]
pub type COEFF_V13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V13` writer - Vertical coefficient 13."]
pub type COEFF_V13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV3_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_V12` reader - Vertical coefficient 12."]
pub type COEFF_V12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_V12` writer - Vertical coefficient 12."]
pub type COEFF_V12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSV3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - Vertical coefficient 14. The bottommost coefficient regardless of the filter height. In contrast to the coeff_v0 this coefficient has to be always programmed."]
    #[inline(always)]
    pub fn coeff_v14(&self) -> COEFF_V14_R {
        COEFF_V14_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Vertical coefficient 13."]
    #[inline(always)]
    pub fn coeff_v13(&self) -> COEFF_V13_R {
        COEFF_V13_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical coefficient 12."]
    #[inline(always)]
    pub fn coeff_v12(&self) -> COEFF_V12_R {
        COEFF_V12_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Vertical coefficient 14. The bottommost coefficient regardless of the filter height. In contrast to the coeff_v0 this coefficient has to be always programmed."]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v14(&mut self) -> COEFF_V14_W<8> {
        COEFF_V14_W::new(self)
    }
    #[doc = "Bits 16:23 - Vertical coefficient 13."]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v13(&mut self) -> COEFF_V13_W<16> {
        COEFF_V13_W::new(self)
    }
    #[doc = "Bits 24:31 - Vertical coefficient 12."]
    #[inline(always)]
    #[must_use]
    pub fn coeff_v12(&mut self) -> COEFF_V12_W<24> {
        COEFF_V12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical coefficients for filter_mode FIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coefficientsv3](index.html) module"]
pub struct COEFFICIENTSV3_SPEC;
impl crate::RegisterSpec for COEFFICIENTSV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coefficientsv3::R](R) reader structure"]
impl crate::Readable for COEFFICIENTSV3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coefficientsv3::W](W) writer structure"]
impl crate::Writable for COEFFICIENTSV3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEFFICIENTSV3 to value 0"]
impl crate::Resettable for COEFFICIENTSV3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
