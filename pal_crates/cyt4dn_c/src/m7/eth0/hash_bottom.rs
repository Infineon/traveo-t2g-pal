#[doc = "Register `HASH_BOTTOM` reader"]
pub struct R(crate::R<HASH_BOTTOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_BOTTOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_BOTTOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_BOTTOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_BOTTOM` writer"]
pub struct W(crate::W<HASH_BOTTOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_BOTTOM_SPEC>;
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
impl From<crate::W<HASH_BOTTOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_BOTTOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS_HASH_B` reader - The first 32 bits of the hash address register."]
pub type ADDRESS_HASH_B_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS_HASH_B` writer - The first 32 bits of the hash address register."]
pub type ADDRESS_HASH_B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASH_BOTTOM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The first 32 bits of the hash address register."]
    #[inline(always)]
    pub fn address_hash_b(&self) -> ADDRESS_HASH_B_R {
        ADDRESS_HASH_B_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The first 32 bits of the hash address register."]
    #[inline(always)]
    #[must_use]
    pub fn address_hash_b(&mut self) -> ADDRESS_HASH_B_W<0> {
        ADDRESS_HASH_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The unicast hash enable and the multicast hash enable bits in the network configuration register enable the reception of hash matched frames. Hash Register Bottom (31 to 0 bits)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_bottom](index.html) module"]
pub struct HASH_BOTTOM_SPEC;
impl crate::RegisterSpec for HASH_BOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_bottom::R](R) reader structure"]
impl crate::Readable for HASH_BOTTOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_bottom::W](W) writer structure"]
impl crate::Writable for HASH_BOTTOM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_BOTTOM to value 0"]
impl crate::Resettable for HASH_BOTTOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
