#[doc = "Register `OCTETS_RXED_TOP` reader"]
pub struct R(crate::R<OCTETS_RXED_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTETS_RXED_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTETS_RXED_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTETS_RXED_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_TOP` reader - Received octets in frame without errors \\[47:32\\]. The number of octets received in valid frames of any type. This counter is 48-bits and is read through two registers. This count does not include octets from pause frames, and is only incremented if the frame is successfully filtered."]
pub type COUNT_TOP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received octets in frame without errors \\[47:32\\]. The number of octets received in valid frames of any type. This counter is 48-bits and is read through two registers. This count does not include octets from pause frames, and is only incremented if the frame is successfully filtered."]
    #[inline(always)]
    pub fn count_top(&self) -> COUNT_TOP_R {
        COUNT_TOP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Octets Received (47 to 32 bits)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octets_rxed_top](index.html) module"]
pub struct OCTETS_RXED_TOP_SPEC;
impl crate::RegisterSpec for OCTETS_RXED_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octets_rxed_top::R](R) reader structure"]
impl crate::Readable for OCTETS_RXED_TOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OCTETS_RXED_TOP to value 0"]
impl crate::Resettable for OCTETS_RXED_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
