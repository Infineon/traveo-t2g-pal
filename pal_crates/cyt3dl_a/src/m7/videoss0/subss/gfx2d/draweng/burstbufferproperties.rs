#[doc = "Register `BURSTBUFFERPROPERTIES` reader"]
pub struct R(crate::R<BURSTBUFFERPROPERTIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BURSTBUFFERPROPERTIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BURSTBUFFERPROPERTIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BURSTBUFFERPROPERTIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAXFETCHBURSTLENGTH` reader - Maximum Burst Length that can be configured for command fetch."]
pub type MAXFETCHBURSTLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAXSTOREBURSTLENGTH` reader - Maximum Burst Length that can be configured for alpha store."]
pub type MAXSTOREBURSTLENGTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Maximum Burst Length that can be configured for command fetch."]
    #[inline(always)]
    pub fn maxfetchburstlength(&self) -> MAXFETCHBURSTLENGTH_R {
        MAXFETCHBURSTLENGTH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Maximum Burst Length that can be configured for alpha store."]
    #[inline(always)]
    pub fn maxstoreburstlength(&self) -> MAXSTOREBURSTLENGTH_R {
        MAXSTOREBURSTLENGTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Burst Buffer Property register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burstbufferproperties](index.html) module"]
pub struct BURSTBUFFERPROPERTIES_SPEC;
impl crate::RegisterSpec for BURSTBUFFERPROPERTIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [burstbufferproperties::R](R) reader structure"]
impl crate::Readable for BURSTBUFFERPROPERTIES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BURSTBUFFERPROPERTIES to value 0"]
impl crate::Resettable for BURSTBUFFERPROPERTIES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
