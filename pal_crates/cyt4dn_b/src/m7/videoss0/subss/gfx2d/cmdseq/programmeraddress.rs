#[doc = "Register `PROGRAMMERADDRESS` reader"]
pub struct R(crate::R<PROGRAMMERADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROGRAMMERADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROGRAMMERADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROGRAMMERADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROGRAMMERADDRESS` reader - Last fetched address of programmer"]
pub type PROGRAMMERADDRESS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last fetched address of programmer"]
    #[inline(always)]
    pub fn programmeraddress(&self) -> PROGRAMMERADDRESS_R {
        PROGRAMMERADDRESS_R::new(self.bits)
    }
}
#[doc = "Last fetched address of programmer.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [programmeraddress](index.html) module"]
pub struct PROGRAMMERADDRESS_SPEC;
impl crate::RegisterSpec for PROGRAMMERADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [programmeraddress::R](R) reader structure"]
impl crate::Readable for PROGRAMMERADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROGRAMMERADDRESS to value 0"]
impl crate::Resettable for PROGRAMMERADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
