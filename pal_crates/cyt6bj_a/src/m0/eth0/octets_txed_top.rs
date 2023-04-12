#[doc = "Register `OCTETS_TXED_TOP` reader"]
pub struct R(crate::R<OCTETS_TXED_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTETS_TXED_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTETS_TXED_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTETS_TXED_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_TOP` reader - Transmitted octets in frame without errors \\[47:32\\]. The number of octets transmitted in valid frames of any type. This counter is 48-bits, and is read through two registers. This count does not include octets from automatically generated pause frames."]
pub type COUNT_TOP_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitted octets in frame without errors \\[47:32\\]. The number of octets transmitted in valid frames of any type. This counter is 48-bits, and is read through two registers. This count does not include octets from automatically generated pause frames."]
    #[inline(always)]
    pub fn count_top(&self) -> COUNT_TOP_R {
        COUNT_TOP_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Octets Transmitted higher bits (47 to 32 bits)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octets_txed_top](index.html) module"]
pub struct OCTETS_TXED_TOP_SPEC;
impl crate::RegisterSpec for OCTETS_TXED_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octets_txed_top::R](R) reader structure"]
impl crate::Readable for OCTETS_TXED_TOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OCTETS_TXED_TOP to value 0"]
impl crate::Resettable for OCTETS_TXED_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
