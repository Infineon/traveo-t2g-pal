#[doc = "Register `CRYPTO_ADDR` reader"]
pub struct R(crate::R<CRYPTO_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_ADDR` writer"]
pub struct W(crate::W<CRYPTO_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_ADDR_SPEC>;
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
impl From<crate::W<CRYPTO_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Only applies to XIP accesses. Specifies the base address of the memory region that will have cryptography applied on XIP accesses with a unique key. If this region overlaps with another key's region, the lower numbered key is used (i.e., crypto key 0 has the highest priority). If the region is 2^m Bytes, ADDR MUST be a multiple of 2^m. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]. Note that in order for cryptogrpahy to be applied the DEVICE.CTL.CRYPTO_EN must be set to 1 as well for any devices that share this region."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Only applies to XIP accesses. Specifies the base address of the memory region that will have cryptography applied on XIP accesses with a unique key. If this region overlaps with another key's region, the lower numbered key is used (i.e., crypto key 0 has the highest priority). If the region is 2^m Bytes, ADDR MUST be a multiple of 2^m. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]. Note that in order for cryptogrpahy to be applied the DEVICE.CTL.CRYPTO_EN must be set to 1 as well for any devices that share this region."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYPTO_ADDR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 8:31 - Only applies to XIP accesses. Specifies the base address of the memory region that will have cryptography applied on XIP accesses with a unique key. If this region overlaps with another key's region, the lower numbered key is used (i.e., crypto key 0 has the highest priority). If the region is 2^m Bytes, ADDR MUST be a multiple of 2^m. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]. Note that in order for cryptogrpahy to be applied the DEVICE.CTL.CRYPTO_EN must be set to 1 as well for any devices that share this region."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Only applies to XIP accesses. Specifies the base address of the memory region that will have cryptography applied on XIP accesses with a unique key. If this region overlaps with another key's region, the lower numbered key is used (i.e., crypto key 0 has the highest priority). If the region is 2^m Bytes, ADDR MUST be a multiple of 2^m. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]. Note that in order for cryptogrpahy to be applied the DEVICE.CTL.CRYPTO_EN must be set to 1 as well for any devices that share this region."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<8> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_addr](index.html) module"]
pub struct CRYPTO_ADDR_SPEC;
impl crate::RegisterSpec for CRYPTO_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_addr::R](R) reader structure"]
impl crate::Readable for CRYPTO_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_addr::W](W) writer structure"]
impl crate::Writable for CRYPTO_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_ADDR to value 0"]
impl crate::Resettable for CRYPTO_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
