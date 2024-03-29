#[doc = "Register `HASH_TOP` reader"]
pub struct R(crate::R<HASH_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_TOP` writer"]
pub struct W(crate::W<HASH_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_TOP_SPEC>;
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
impl From<crate::W<HASH_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS_HASH_T` reader - The remaining 32 bits of the hash address register."]
pub type ADDRESS_HASH_T_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS_HASH_T` writer - The remaining 32 bits of the hash address register."]
pub type ADDRESS_HASH_T_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASH_TOP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The remaining 32 bits of the hash address register."]
    #[inline(always)]
    pub fn address_hash_t(&self) -> ADDRESS_HASH_T_R {
        ADDRESS_HASH_T_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The remaining 32 bits of the hash address register."]
    #[inline(always)]
    #[must_use]
    pub fn address_hash_t(&mut self) -> ADDRESS_HASH_T_W<0> {
        ADDRESS_HASH_T_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Register Top (63 to 32 bits)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_top](index.html) module"]
pub struct HASH_TOP_SPEC;
impl crate::RegisterSpec for HASH_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_top::R](R) reader structure"]
impl crate::Readable for HASH_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_top::W](W) writer structure"]
impl crate::Writable for HASH_TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_TOP to value 0"]
impl crate::Resettable for HASH_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
