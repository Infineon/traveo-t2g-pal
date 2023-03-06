#[doc = "Register `TXRQ2` reader"]
pub struct R(crate::R<TXRQ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXRQ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXRQ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXRQ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXR` reader - Transmission Request TRX\\[63:32\\]
If the flag is set, the respective message buffer is ready for transmission respectively transmission of this message buffer is in progress. In single-shot mode the flags are reset after transmission has completed."]
pub type TXR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Request TRX\\[63:32\\]
If the flag is set, the respective message buffer is ready for transmission respectively transmission of this message buffer is in progress. In single-shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr(&self) -> TXR_R {
        TXR_R::new(self.bits)
    }
}
#[doc = "Transmission Request 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txrq2](index.html) module"]
pub struct TXRQ2_SPEC;
impl crate::RegisterSpec for TXRQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txrq2::R](R) reader structure"]
impl crate::Readable for TXRQ2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXRQ2 to value 0"]
impl crate::Resettable for TXRQ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
