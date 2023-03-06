#[doc = "Register `PRECOND_CTL` reader"]
pub struct R(crate::R<PRECOND_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRECOND_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRECOND_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRECOND_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRECOND_CTL` writer"]
pub struct W(crate::W<PRECOND_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRECOND_CTL_SPEC>;
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
impl From<crate::W<PRECOND_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRECOND_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRECOND_TIME` reader - Number ADC clock cycles that Preconditioning is done before the sample window starts. If OVERLAP_EN=0 there will be 1 additional break before make cycle between preconditioning and sampling. Note that the minimum value is 1 (0 gives the same result as 1)."]
pub type PRECOND_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRECOND_TIME` writer - Number ADC clock cycles that Preconditioning is done before the sample window starts. If OVERLAP_EN=0 there will be 1 additional break before make cycle between preconditioning and sampling. Note that the minimum value is 1 (0 gives the same result as 1)."]
pub type PRECOND_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRECOND_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Number ADC clock cycles that Preconditioning is done before the sample window starts. If OVERLAP_EN=0 there will be 1 additional break before make cycle between preconditioning and sampling. Note that the minimum value is 1 (0 gives the same result as 1)."]
    #[inline(always)]
    pub fn precond_time(&self) -> PRECOND_TIME_R {
        PRECOND_TIME_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number ADC clock cycles that Preconditioning is done before the sample window starts. If OVERLAP_EN=0 there will be 1 additional break before make cycle between preconditioning and sampling. Note that the minimum value is 1 (0 gives the same result as 1)."]
    #[inline(always)]
    #[must_use]
    pub fn precond_time(&mut self) -> PRECOND_TIME_W<0> {
        PRECOND_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Preconditioning control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [precond_ctl](index.html) module"]
pub struct PRECOND_CTL_SPEC;
impl crate::RegisterSpec for PRECOND_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [precond_ctl::R](R) reader structure"]
impl crate::Readable for PRECOND_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [precond_ctl::W](W) writer structure"]
impl crate::Writable for PRECOND_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRECOND_CTL to value 0"]
impl crate::Resettable for PRECOND_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
