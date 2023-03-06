#[doc = "Register `CRYPTO_MASK` reader"]
pub struct R(crate::R<CRYPTO_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_MASK` writer"]
pub struct W(crate::W<CRYPTO_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_MASK_SPEC>;
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
impl From<crate::W<CRYPTO_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Only applies to XIP accesses. Specifies the size of the memory region whose base address is determined by CRYPTO_ADDR. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in CRYPTO_ADDR.ADDR: Address A is in the cryptography region when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == CRYPTO_ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff."]
pub type MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MASK` writer - Only applies to XIP accesses. Specifies the size of the memory region whose base address is determined by CRYPTO_ADDR. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in CRYPTO_ADDR.ADDR: Address A is in the cryptography region when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == CRYPTO_ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff."]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYPTO_MASK_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 8:31 - Only applies to XIP accesses. Specifies the size of the memory region whose base address is determined by CRYPTO_ADDR. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in CRYPTO_ADDR.ADDR: Address A is in the cryptography region when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == CRYPTO_ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Only applies to XIP accesses. Specifies the size of the memory region whose base address is determined by CRYPTO_ADDR. All '1' bits are used to compare the incoming transfer request address A\\[31:0\\]
with the address as specified in CRYPTO_ADDR.ADDR: Address A is in the cryptography region when (A\\[31:8\\]
&amp; MASK\\[31:8\\]) == CRYPTO_ADDR.ADDR\\[31:8\\]. The most significant bit fields are constants and set to'1'. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), MASK\\[31:24\\]
= 0xff."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<8> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_mask](index.html) module"]
pub struct CRYPTO_MASK_SPEC;
impl crate::RegisterSpec for CRYPTO_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_mask::R](R) reader structure"]
impl crate::Readable for CRYPTO_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_mask::W](W) writer structure"]
impl crate::Writable for CRYPTO_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_MASK to value 0"]
impl crate::Resettable for CRYPTO_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
