#[doc = "Register `CMDBUFFERREADPTR` reader"]
pub struct R(crate::R<CMDBUFFERREADPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDBUFFERREADPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDBUFFERREADPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDBUFFERREADPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDBUFFERREADPTR` reader - Command buffer read pointer in words. This pointer is modified by the Drawing Engine hardware and indicates which words are ultimately consumed."]
pub type CMDBUFFERREADPTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - Command buffer read pointer in words. This pointer is modified by the Drawing Engine hardware and indicates which words are ultimately consumed."]
    #[inline(always)]
    pub fn cmdbufferreadptr(&self) -> CMDBUFFERREADPTR_R {
        CMDBUFFERREADPTR_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Buffer end address of command list\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdbufferreadptr](index.html) module"]
pub struct CMDBUFFERREADPTR_SPEC;
impl crate::RegisterSpec for CMDBUFFERREADPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdbufferreadptr::R](R) reader structure"]
impl crate::Readable for CMDBUFFERREADPTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMDBUFFERREADPTR to value 0"]
impl crate::Resettable for CMDBUFFERREADPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
