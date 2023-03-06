#[doc = "Register `COEFFICIENTSH3` reader"]
pub struct R(crate::R<COEFFICIENTSH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEFFICIENTSH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEFFICIENTSH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEFFICIENTSH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COEFFICIENTSH3` writer"]
pub struct W(crate::W<COEFFICIENTSH3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COEFFICIENTSH3_SPEC>;
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
impl From<crate::W<COEFFICIENTSH3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COEFFICIENTSH3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEFF_H14` reader - Horizontal coefficient 14. The rightmost coefficient regardless of the filter width. In contrast to the coeff_h0 this coefficient has to be always programmed."]
pub type COEFF_H14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H14` writer - Horizontal coefficient 14. The rightmost coefficient regardless of the filter width. In contrast to the coeff_h0 this coefficient has to be always programmed."]
pub type COEFF_H14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH3_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H13` reader - Horizontal coefficient 13."]
pub type COEFF_H13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H13` writer - Horizontal coefficient 13."]
pub type COEFF_H13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH3_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEFF_H12` reader - Horizontal coefficient 12."]
pub type COEFF_H12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEFF_H12` writer - Horizontal coefficient 12."]
pub type COEFF_H12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COEFFICIENTSH3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - Horizontal coefficient 14. The rightmost coefficient regardless of the filter width. In contrast to the coeff_h0 this coefficient has to be always programmed."]
    #[inline(always)]
    pub fn coeff_h14(&self) -> COEFF_H14_R {
        COEFF_H14_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal coefficient 13."]
    #[inline(always)]
    pub fn coeff_h13(&self) -> COEFF_H13_R {
        COEFF_H13_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal coefficient 12."]
    #[inline(always)]
    pub fn coeff_h12(&self) -> COEFF_H12_R {
        COEFF_H12_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Horizontal coefficient 14. The rightmost coefficient regardless of the filter width. In contrast to the coeff_h0 this coefficient has to be always programmed."]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h14(&mut self) -> COEFF_H14_W<8> {
        COEFF_H14_W::new(self)
    }
    #[doc = "Bits 16:23 - Horizontal coefficient 13."]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h13(&mut self) -> COEFF_H13_W<16> {
        COEFF_H13_W::new(self)
    }
    #[doc = "Bits 24:31 - Horizontal coefficient 12."]
    #[inline(always)]
    #[must_use]
    pub fn coeff_h12(&mut self) -> COEFF_H12_W<24> {
        COEFF_H12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal coefficients for filter_mode FIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coefficientsh3](index.html) module"]
pub struct COEFFICIENTSH3_SPEC;
impl crate::RegisterSpec for COEFFICIENTSH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coefficientsh3::R](R) reader structure"]
impl crate::Readable for COEFFICIENTSH3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coefficientsh3::W](W) writer structure"]
impl crate::Writable for COEFFICIENTSH3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COEFFICIENTSH3 to value 0"]
impl crate::Resettable for COEFFICIENTSH3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
