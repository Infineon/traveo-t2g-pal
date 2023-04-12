#[doc = "Register `INECC1` reader"]
pub struct R(crate::R<INECC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INECC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INECC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INECC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INECC1` writer"]
pub struct W(crate::W<INECC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INECC1_SPEC>;
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
impl From<crate::W<INECC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INECC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT_MEM_BASE` reader - Protected Region Base Address"]
pub type PROT_MEM_BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PROT_MEM_BASE` writer - Protected Region Base Address"]
pub type PROT_MEM_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INECC1_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Protected Region Base Address"]
    #[inline(always)]
    pub fn prot_mem_base(&self) -> PROT_MEM_BASE_R {
        PROT_MEM_BASE_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Protected Region Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn prot_mem_base(&mut self) -> PROT_MEM_BASE_W<0> {
        PROT_MEM_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inline ECC Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inecc1](index.html) module"]
pub struct INECC1_SPEC;
impl crate::RegisterSpec for INECC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inecc1::R](R) reader structure"]
impl crate::Readable for INECC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inecc1::W](W) writer structure"]
impl crate::Writable for INECC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INECC1 to value 0"]
impl crate::Resettable for INECC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
