#[doc = "Register `INECC0` reader"]
pub struct R(crate::R<INECC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INECC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INECC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INECC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INECC0` writer"]
pub struct W(crate::W<INECC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INECC0_SPEC>;
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
impl From<crate::W<INECC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INECC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC_BASE` reader - ECC Region Base Address"]
pub type ECC_BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ECC_BASE` writer - ECC Region Base Address"]
pub type ECC_BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INECC0_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - ECC Region Base Address"]
    #[inline(always)]
    pub fn ecc_base(&self) -> ECC_BASE_R {
        ECC_BASE_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - ECC Region Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_base(&mut self) -> ECC_BASE_W<0> {
        ECC_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inline ECC Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inecc0](index.html) module"]
pub struct INECC0_SPEC;
impl crate::RegisterSpec for INECC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inecc0::R](R) reader structure"]
impl crate::Readable for INECC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inecc0::W](W) writer structure"]
impl crate::Writable for INECC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INECC0 to value 0"]
impl crate::Resettable for INECC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
