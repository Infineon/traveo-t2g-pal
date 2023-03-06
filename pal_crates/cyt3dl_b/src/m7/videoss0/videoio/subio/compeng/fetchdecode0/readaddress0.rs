#[doc = "Register `READADDRESS0` reader"]
pub struct R(crate::R<READADDRESS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READADDRESS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READADDRESS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READADDRESS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READADDRESS0` reader - Last burst address that was read from the layer's source buffer."]
pub type READADDRESS0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last burst address that was read from the layer's source buffer."]
    #[inline(always)]
    pub fn readaddress0(&self) -> READADDRESS0_R {
        READADDRESS0_R::new(self.bits)
    }
}
#[doc = "Ring buffer synchronization for layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readaddress0](index.html) module"]
pub struct READADDRESS0_SPEC;
impl crate::RegisterSpec for READADDRESS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readaddress0::R](R) reader structure"]
impl crate::Readable for READADDRESS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets READADDRESS0 to value 0"]
impl crate::Resettable for READADDRESS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
