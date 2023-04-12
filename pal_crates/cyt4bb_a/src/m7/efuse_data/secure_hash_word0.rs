#[doc = "Register `SECURE_HASH_WORD0` reader"]
pub struct R(crate::R<SECURE_HASH_WORD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_HASH_WORD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_HASH_WORD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_HASH_WORD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_HASH_WORD0` writer"]
pub struct W(crate::W<SECURE_HASH_WORD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_HASH_WORD0_SPEC>;
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
impl From<crate::W<SECURE_HASH_WORD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_HASH_WORD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASH_WORD0` reader - Secure 128 bits HASH word 0 that is used for authentication in SECURE protection state."]
pub type HASH_WORD0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HASH_WORD0` writer - Secure 128 bits HASH word 0 that is used for authentication in SECURE protection state."]
pub type HASH_WORD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_HASH_WORD0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Secure 128 bits HASH word 0 that is used for authentication in SECURE protection state."]
    #[inline(always)]
    pub fn hash_word0(&self) -> HASH_WORD0_R {
        HASH_WORD0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure 128 bits HASH word 0 that is used for authentication in SECURE protection state."]
    #[inline(always)]
    #[must_use]
    pub fn hash_word0(&mut self) -> HASH_WORD0_W<0> {
        HASH_WORD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure 128 bits HASH word 0 that is used for authentication in SECURE protection state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_hash_word0](index.html) module"]
pub struct SECURE_HASH_WORD0_SPEC;
impl crate::RegisterSpec for SECURE_HASH_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_hash_word0::R](R) reader structure"]
impl crate::Readable for SECURE_HASH_WORD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_hash_word0::W](W) writer structure"]
impl crate::Writable for SECURE_HASH_WORD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECURE_HASH_WORD0 to value 0"]
impl crate::Resettable for SECURE_HASH_WORD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
