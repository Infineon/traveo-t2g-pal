#[doc = "Register `ACTIVE_SEC` reader"]
pub struct R(crate::R<ACTIVE_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVE_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVE_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVE_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVE` reader - Specifies active secure channels; i.e. enabled secure channels whose trigger got activated. The bits corresponding to non-secure channels are '0'."]
pub type ACTIVE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Specifies active secure channels; i.e. enabled secure channels whose trigger got activated. The bits corresponding to non-secure channels are '0'."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Active secure channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active_sec](index.html) module"]
pub struct ACTIVE_SEC_SPEC;
impl crate::RegisterSpec for ACTIVE_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [active_sec::R](R) reader structure"]
impl crate::Readable for ACTIVE_SEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVE_SEC to value 0"]
impl crate::Resettable for ACTIVE_SEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
