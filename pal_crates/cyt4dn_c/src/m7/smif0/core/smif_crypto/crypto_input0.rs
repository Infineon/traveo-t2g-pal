#[doc = "Register `CRYPTO_INPUT0` reader"]
pub struct R(crate::R<CRYPTO_INPUT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_INPUT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_INPUT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_INPUT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_INPUT0` writer"]
pub struct W(crate::W<CRYPTO_INPUT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_INPUT0_SPEC>;
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
impl From<crate::W<CRYPTO_INPUT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_INPUT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT_LSB` reader - Plaintext PT\\[3:0\\]
= CRYPTO_INPUT0.INPUT_LSB."]
pub type INPUT_LSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUT_LSB` writer - Plaintext PT\\[3:0\\]
= CRYPTO_INPUT0.INPUT_LSB."]
pub type INPUT_LSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRYPTO_INPUT0_SPEC, u8, u8, 4, O>;
#[doc = "Field `INPUT_MSB` reader - Plaintext PT\\[31:4\\]
= CRYPTO_INPUT0.INPUT_MSB."]
pub type INPUT_MSB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INPUT_MSB` writer - Plaintext PT\\[31:4\\]
= CRYPTO_INPUT0.INPUT_MSB."]
pub type INPUT_MSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRYPTO_INPUT0_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - Plaintext PT\\[3:0\\]
= CRYPTO_INPUT0.INPUT_LSB."]
    #[inline(always)]
    pub fn input_lsb(&self) -> INPUT_LSB_R {
        INPUT_LSB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - Plaintext PT\\[31:4\\]
= CRYPTO_INPUT0.INPUT_MSB."]
    #[inline(always)]
    pub fn input_msb(&self) -> INPUT_MSB_R {
        INPUT_MSB_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Plaintext PT\\[3:0\\]
= CRYPTO_INPUT0.INPUT_LSB."]
    #[inline(always)]
    #[must_use]
    pub fn input_lsb(&mut self) -> INPUT_LSB_W<0> {
        INPUT_LSB_W::new(self)
    }
    #[doc = "Bits 4:31 - Plaintext PT\\[31:4\\]
= CRYPTO_INPUT0.INPUT_MSB."]
    #[inline(always)]
    #[must_use]
    pub fn input_msb(&mut self) -> INPUT_MSB_W<4> {
        INPUT_MSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography input 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_input0](index.html) module"]
pub struct CRYPTO_INPUT0_SPEC;
impl crate::RegisterSpec for CRYPTO_INPUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_input0::R](R) reader structure"]
impl crate::Readable for CRYPTO_INPUT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_input0::W](W) writer structure"]
impl crate::Writable for CRYPTO_INPUT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_INPUT0 to value 0"]
impl crate::Resettable for CRYPTO_INPUT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
