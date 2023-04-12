#[doc = "Register `INECCSTT1` reader"]
pub struct R(crate::R<INECCSTT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INECCSTT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INECCSTT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INECCSTT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC_ERROR_SYNDROME` reader - ECC Error FIFO: Four syndromes of a DRAM Request. Each 8-bit syndrome corresponds to 64-bit data"]
pub type ECC_ERROR_SYNDROME_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Error FIFO: Four syndromes of a DRAM Request. Each 8-bit syndrome corresponds to 64-bit data"]
    #[inline(always)]
    pub fn ecc_error_syndrome(&self) -> ECC_ERROR_SYNDROME_R {
        ECC_ERROR_SYNDROME_R::new(self.bits)
    }
}
#[doc = "Inline ECC Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ineccstt1](index.html) module"]
pub struct INECCSTT1_SPEC;
impl crate::RegisterSpec for INECCSTT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ineccstt1::R](R) reader structure"]
impl crate::Readable for INECCSTT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INECCSTT1 to value 0"]
impl crate::Resettable for INECCSTT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
