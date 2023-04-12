#[doc = "Register `STORESTATUS` reader"]
pub struct R(crate::R<STORESTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORESTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORESTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORESTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STOREBYTES` reader - Number of total stored bytes (sum of both buffers in case of semi-planar formats)."]
pub type STOREBYTES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - Number of total stored bytes (sum of both buffers in case of semi-planar formats)."]
    #[inline(always)]
    pub fn storebytes(&self) -> STOREBYTES_R {
        STOREBYTES_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Store Unit status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [storestatus](index.html) module"]
pub struct STORESTATUS_SPEC;
impl crate::RegisterSpec for STORESTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [storestatus::R](R) reader structure"]
impl crate::Readable for STORESTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STORESTATUS to value 0"]
impl crate::Resettable for STORESTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
