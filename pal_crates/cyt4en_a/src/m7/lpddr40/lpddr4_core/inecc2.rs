#[doc = "Register `INECC2` reader"]
pub struct R(crate::R<INECC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INECC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INECC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INECC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INECC2` writer"]
pub struct W(crate::W<INECC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INECC2_SPEC>;
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
impl From<crate::W<INECC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INECC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT_MEM_SIZE` reader - Protected Region Size (Bytes)"]
pub type PROT_MEM_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PROT_MEM_SIZE` writer - Protected Region Size (Bytes)"]
pub type PROT_MEM_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INECC2_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Protected Region Size (Bytes)"]
    #[inline(always)]
    pub fn prot_mem_size(&self) -> PROT_MEM_SIZE_R {
        PROT_MEM_SIZE_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Protected Region Size (Bytes)"]
    #[inline(always)]
    #[must_use]
    pub fn prot_mem_size(&mut self) -> PROT_MEM_SIZE_W<0> {
        PROT_MEM_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inline ECC Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inecc2](index.html) module"]
pub struct INECC2_SPEC;
impl crate::RegisterSpec for INECC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inecc2::R](R) reader structure"]
impl crate::Readable for INECC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inecc2::W](W) writer structure"]
impl crate::Writable for INECC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INECC2 to value 0"]
impl crate::Resettable for INECC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
