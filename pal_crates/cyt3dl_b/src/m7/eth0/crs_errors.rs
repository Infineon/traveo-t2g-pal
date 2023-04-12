#[doc = "Register `CRS_ERRORS` reader"]
pub struct R(crate::R<CRS_ERRORS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRS_ERRORS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRS_ERRORS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRS_ERRORS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT19` reader - Carrier sense errors - a 10 bit register counting the number of frames transmitted where carrier sense was not seen during transmission or where carrier sense was deasserted after being asserted in a transmit frame without collision (no under run). Only incremented in half duplex mode. The only effect of a carrier sense error is to increment this register. The behaviour of the other statistics registers is unaffected by the detection of a carrier sense error."]
pub type COUNT19_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Carrier sense errors - a 10 bit register counting the number of frames transmitted where carrier sense was not seen during transmission or where carrier sense was deasserted after being asserted in a transmit frame without collision (no under run). Only incremented in half duplex mode. The only effect of a carrier sense error is to increment this register. The behaviour of the other statistics registers is unaffected by the detection of a carrier sense error."]
    #[inline(always)]
    pub fn count19(&self) -> COUNT19_R {
        COUNT19_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Carrier Sense Errors. Presents in design but not support.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crs_errors](index.html) module"]
pub struct CRS_ERRORS_SPEC;
impl crate::RegisterSpec for CRS_ERRORS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crs_errors::R](R) reader structure"]
impl crate::Readable for CRS_ERRORS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRS_ERRORS to value 0"]
impl crate::Resettable for CRS_ERRORS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
