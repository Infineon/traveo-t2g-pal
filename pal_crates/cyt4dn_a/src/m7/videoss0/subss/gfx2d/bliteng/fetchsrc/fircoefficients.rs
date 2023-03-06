#[doc = "Register `FIRCOEFFICIENTS` reader"]
pub struct R(crate::R<FIRCOEFFICIENTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIRCOEFFICIENTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIRCOEFFICIENTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIRCOEFFICIENTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIRCOEFFICIENTS` writer"]
pub struct W(crate::W<FIRCOEFFICIENTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIRCOEFFICIENTS_SPEC>;
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
impl From<crate::W<FIRCOEFFICIENTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIRCOEFFICIENTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIR0COEFFICIENT` reader - First coefficient."]
pub type FIR0COEFFICIENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIR0COEFFICIENT` writer - First coefficient."]
pub type FIR0COEFFICIENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRCOEFFICIENTS_SPEC, u8, u8, 8, O>;
#[doc = "Field `FIR1COEFFICIENT` reader - Second coefficient."]
pub type FIR1COEFFICIENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIR1COEFFICIENT` writer - Second coefficient."]
pub type FIR1COEFFICIENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRCOEFFICIENTS_SPEC, u8, u8, 8, O>;
#[doc = "Field `FIR2COEFFICIENT` reader - Third coefficient."]
pub type FIR2COEFFICIENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIR2COEFFICIENT` writer - Third coefficient."]
pub type FIR2COEFFICIENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRCOEFFICIENTS_SPEC, u8, u8, 8, O>;
#[doc = "Field `FIR3COEFFICIENT` reader - Fourth coefficient."]
pub type FIR3COEFFICIENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIR3COEFFICIENT` writer - Fourth coefficient."]
pub type FIR3COEFFICIENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIRCOEFFICIENTS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - First coefficient."]
    #[inline(always)]
    pub fn fir0coefficient(&self) -> FIR0COEFFICIENT_R {
        FIR0COEFFICIENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Second coefficient."]
    #[inline(always)]
    pub fn fir1coefficient(&self) -> FIR1COEFFICIENT_R {
        FIR1COEFFICIENT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Third coefficient."]
    #[inline(always)]
    pub fn fir2coefficient(&self) -> FIR2COEFFICIENT_R {
        FIR2COEFFICIENT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fourth coefficient."]
    #[inline(always)]
    pub fn fir3coefficient(&self) -> FIR3COEFFICIENT_R {
        FIR3COEFFICIENT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First coefficient."]
    #[inline(always)]
    #[must_use]
    pub fn fir0coefficient(&mut self) -> FIR0COEFFICIENT_W<0> {
        FIR0COEFFICIENT_W::new(self)
    }
    #[doc = "Bits 8:15 - Second coefficient."]
    #[inline(always)]
    #[must_use]
    pub fn fir1coefficient(&mut self) -> FIR1COEFFICIENT_W<8> {
        FIR1COEFFICIENT_W::new(self)
    }
    #[doc = "Bits 16:23 - Third coefficient."]
    #[inline(always)]
    #[must_use]
    pub fn fir2coefficient(&mut self) -> FIR2COEFFICIENT_W<16> {
        FIR2COEFFICIENT_W::new(self)
    }
    #[doc = "Bits 24:31 - Fourth coefficient."]
    #[inline(always)]
    #[must_use]
    pub fn fir3coefficient(&mut self) -> FIR3COEFFICIENT_W<24> {
        FIR3COEFFICIENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIR coefficients register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fircoefficients](index.html) module"]
pub struct FIRCOEFFICIENTS_SPEC;
impl crate::RegisterSpec for FIRCOEFFICIENTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fircoefficients::R](R) reader structure"]
impl crate::Readable for FIRCOEFFICIENTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fircoefficients::W](W) writer structure"]
impl crate::Writable for FIRCOEFFICIENTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIRCOEFFICIENTS to value 0x20"]
impl crate::Resettable for FIRCOEFFICIENTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
