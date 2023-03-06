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
#[doc = "Field `MANAGEDBURSTBUFFERS` reader - Maximum number of burst buffers that can be administrated in the AXI interface."]
pub type MANAGEDBURSTBUFFERS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BURSTLENGTHFORMAXBUFFERS` reader - Maximum Burst Length that can be used when ManagedBurstBuffers burst buffers are used."]
pub type BURSTLENGTHFORMAXBUFFERS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Maximum number of burst buffers that can be administrated in the AXI interface."]
    #[inline(always)]
    pub fn managedburstbuffers(&self) -> MANAGEDBURSTBUFFERS_R {
        MANAGEDBURSTBUFFERS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Maximum Burst Length that can be used when ManagedBurstBuffers burst buffers are used."]
    #[inline(always)]
    pub fn burstlengthformaxbuffers(&self) -> BURSTLENGTHFORMAXBUFFERS_R {
        BURSTLENGTHFORMAXBUFFERS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "Burst buffer properties.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burstbufferproperties](index.html) module"]
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
