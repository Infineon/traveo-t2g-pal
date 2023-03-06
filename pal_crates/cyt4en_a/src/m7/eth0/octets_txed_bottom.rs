#[doc = "Register `OCTETS_TXED_BOTTOM` reader"]
pub struct R(crate::R<OCTETS_TXED_BOTTOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTETS_TXED_BOTTOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTETS_TXED_BOTTOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTETS_TXED_BOTTOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_BOTTOM` reader - Transmitted octets in frame without errors \\[31:0\\]. The number of octets transmitted in valid frames of any type. This counter is 48-bits, and is read through two registers. This count does not include octets from automatically generated pause frames."]
pub type COUNT_BOTTOM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted octets in frame without errors \\[31:0\\]. The number of octets transmitted in valid frames of any type. This counter is 48-bits, and is read through two registers. This count does not include octets from automatically generated pause frames."]
    #[inline(always)]
    pub fn count_bottom(&self) -> COUNT_BOTTOM_R {
        COUNT_BOTTOM_R::new(self.bits)
    }
}
#[doc = "Octets Transmitted lower bits (31 to 0 bits)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octets_txed_bottom](index.html) module"]
pub struct OCTETS_TXED_BOTTOM_SPEC;
impl crate::RegisterSpec for OCTETS_TXED_BOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octets_txed_bottom::R](R) reader structure"]
impl crate::Readable for OCTETS_TXED_BOTTOM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OCTETS_TXED_BOTTOM to value 0"]
impl crate::Resettable for OCTETS_TXED_BOTTOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
