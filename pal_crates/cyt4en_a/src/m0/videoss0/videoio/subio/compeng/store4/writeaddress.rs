#[doc = "Register `WRITEADDRESS` reader"]
pub struct R(crate::R<WRITEADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITEADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITEADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITEADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRITEADDRESS` reader - Last burst address that was written to the destination buffer."]
pub type WRITEADDRESS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last burst address that was written to the destination buffer."]
    #[inline(always)]
    pub fn writeaddress(&self) -> WRITEADDRESS_R {
        WRITEADDRESS_R::new(self.bits)
    }
}
#[doc = "Ring buffer synchronization.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writeaddress](index.html) module"]
pub struct WRITEADDRESS_SPEC;
impl crate::RegisterSpec for WRITEADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writeaddress::R](R) reader structure"]
impl crate::Readable for WRITEADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRITEADDRESS to value 0"]
impl crate::Resettable for WRITEADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
