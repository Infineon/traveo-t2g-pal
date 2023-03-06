#[doc = "Register `DECODINGSTATUS` reader"]
pub struct R(crate::R<DECODINGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECODINGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECODINGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECODINGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DECODINGSTATUS` reader - Decoding status. 0: Not decoding 1: Decoding"]
pub type DECODINGSTATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Decoding status. 0: Not decoding 1: Decoding"]
    #[inline(always)]
    pub fn decodingstatus(&self) -> DECODINGSTATUS_R {
        DECODINGSTATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Decoding status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decodingstatus](index.html) module"]
pub struct DECODINGSTATUS_SPEC;
impl crate::RegisterSpec for DECODINGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decodingstatus::R](R) reader structure"]
impl crate::Readable for DECODINGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DECODINGSTATUS to value 0"]
impl crate::Resettable for DECODINGSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
