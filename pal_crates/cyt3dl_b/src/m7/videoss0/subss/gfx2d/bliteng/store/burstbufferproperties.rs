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
#[doc = "Field `MAXBURSTLENGTH` reader - Maximum Burst Length that can be configured."]
pub type MAXBURSTLENGTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 8:12 - Maximum Burst Length that can be configured."]
    #[inline(always)]
    pub fn maxburstlength(&self) -> MAXBURSTLENGTH_R {
        MAXBURSTLENGTH_R::new(((self.bits >> 8) & 0x1f) as u8)
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
