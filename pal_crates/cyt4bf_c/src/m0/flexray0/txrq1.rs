#[doc = "Register `TXRQ1` reader"]
pub struct R(crate::R<TXRQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXRQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXRQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXRQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXR` reader - Transmission Request TRX\\[31:0\\]
If the flag is set, the respective message buffer is ready for transmission respectively transmission of this message buffer is in progress. In single-shot mode the flags are reset after transmission has completed."]
pub type TXR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Request TRX\\[31:0\\]
If the flag is set, the respective message buffer is ready for transmission respectively transmission of this message buffer is in progress. In single-shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr(&self) -> TXR_R {
        TXR_R::new(self.bits)
    }
}
#[doc = "Transmission Request 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txrq1](index.html) module"]
pub struct TXRQ1_SPEC;
impl crate::RegisterSpec for TXRQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txrq1::R](R) reader structure"]
impl crate::Readable for TXRQ1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXRQ1 to value 0"]
impl crate::Resettable for TXRQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
