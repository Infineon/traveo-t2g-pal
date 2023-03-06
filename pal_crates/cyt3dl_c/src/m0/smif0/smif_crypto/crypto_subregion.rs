#[doc = "Register `CRYPTO_SUBREGION` reader"]
pub struct R(crate::R<CRYPTO_SUBREGION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_SUBREGION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_SUBREGION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_SUBREGION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_SUBREGION` writer"]
pub struct W(crate::W<CRYPTO_SUBREGION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_SUBREGION_SPEC>;
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
impl From<crate::W<CRYPTO_SUBREGION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_SUBREGION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBREGION_DISABLE` reader - Only applies to XIP accesses. The cryptography region for this key as determined by CRYPTO_ADDR and CRYPTO_MASK is further divided into 8 equal subregions. e.g., if a 1 MB region is specified then the 8 subregions will be 128 KB. If SUBREGION_DISABLE=0 then cryptography will be performed on that subregion (assuming any associated DEVICE.CTL.CRYPTO_EN=1). Otherwise if SUBREGION_DISABLE=1 then cryptography is not applied. Note that setting all SUBREGION_DISABLE bits to 1 will effectively disable this key. The encryption decision and the associated key are locked in at the beginning of a memory access and will apply until the end of such atomic accesses. Therefore, it is the user's responsibility to ensure memory accesses do not span across (sub)regions that have different keys and/or cypto enabled/disabled differently across such (sub)regions."]
pub type SUBREGION_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBREGION_DISABLE` writer - Only applies to XIP accesses. The cryptography region for this key as determined by CRYPTO_ADDR and CRYPTO_MASK is further divided into 8 equal subregions. e.g., if a 1 MB region is specified then the 8 subregions will be 128 KB. If SUBREGION_DISABLE=0 then cryptography will be performed on that subregion (assuming any associated DEVICE.CTL.CRYPTO_EN=1). Otherwise if SUBREGION_DISABLE=1 then cryptography is not applied. Note that setting all SUBREGION_DISABLE bits to 1 will effectively disable this key. The encryption decision and the associated key are locked in at the beginning of a memory access and will apply until the end of such atomic accesses. Therefore, it is the user's responsibility to ensure memory accesses do not span across (sub)regions that have different keys and/or cypto enabled/disabled differently across such (sub)regions."]
pub type SUBREGION_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRYPTO_SUBREGION_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Only applies to XIP accesses. The cryptography region for this key as determined by CRYPTO_ADDR and CRYPTO_MASK is further divided into 8 equal subregions. e.g., if a 1 MB region is specified then the 8 subregions will be 128 KB. If SUBREGION_DISABLE=0 then cryptography will be performed on that subregion (assuming any associated DEVICE.CTL.CRYPTO_EN=1). Otherwise if SUBREGION_DISABLE=1 then cryptography is not applied. Note that setting all SUBREGION_DISABLE bits to 1 will effectively disable this key. The encryption decision and the associated key are locked in at the beginning of a memory access and will apply until the end of such atomic accesses. Therefore, it is the user's responsibility to ensure memory accesses do not span across (sub)regions that have different keys and/or cypto enabled/disabled differently across such (sub)regions."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SUBREGION_DISABLE_R {
        SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Only applies to XIP accesses. The cryptography region for this key as determined by CRYPTO_ADDR and CRYPTO_MASK is further divided into 8 equal subregions. e.g., if a 1 MB region is specified then the 8 subregions will be 128 KB. If SUBREGION_DISABLE=0 then cryptography will be performed on that subregion (assuming any associated DEVICE.CTL.CRYPTO_EN=1). Otherwise if SUBREGION_DISABLE=1 then cryptography is not applied. Note that setting all SUBREGION_DISABLE bits to 1 will effectively disable this key. The encryption decision and the associated key are locked in at the beginning of a memory access and will apply until the end of such atomic accesses. Therefore, it is the user's responsibility to ensure memory accesses do not span across (sub)regions that have different keys and/or cypto enabled/disabled differently across such (sub)regions."]
    #[inline(always)]
    #[must_use]
    pub fn subregion_disable(&mut self) -> SUBREGION_DISABLE_W<0> {
        SUBREGION_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography subregion disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_subregion](index.html) module"]
pub struct CRYPTO_SUBREGION_SPEC;
impl crate::RegisterSpec for CRYPTO_SUBREGION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_subregion::R](R) reader structure"]
impl crate::Readable for CRYPTO_SUBREGION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_subregion::W](W) writer structure"]
impl crate::Writable for CRYPTO_SUBREGION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_SUBREGION to value 0"]
impl crate::Resettable for CRYPTO_SUBREGION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
