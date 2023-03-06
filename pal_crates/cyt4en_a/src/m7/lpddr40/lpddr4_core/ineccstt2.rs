#[doc = "Register `INECCSTT2` reader"]
pub struct R(crate::R<INECCSTT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INECCSTT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INECCSTT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INECCSTT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC_ERROR_SEC` reader - ECC Error FIFO: Four SEC bits of a DRAM Request. Each bit corresponds to 64-bit data."]
pub type ECC_ERROR_SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECC_ERROR_DED` reader - ECC Error FIFO: Four DED bits of a DRAM Request. Each bit corresponds to 64-bit data."]
pub type ECC_ERROR_DED_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - ECC Error FIFO: Four SEC bits of a DRAM Request. Each bit corresponds to 64-bit data."]
    #[inline(always)]
    pub fn ecc_error_sec(&self) -> ECC_ERROR_SEC_R {
        ECC_ERROR_SEC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ECC Error FIFO: Four DED bits of a DRAM Request. Each bit corresponds to 64-bit data."]
    #[inline(always)]
    pub fn ecc_error_ded(&self) -> ECC_ERROR_DED_R {
        ECC_ERROR_DED_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Inline ECC Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ineccstt2](index.html) module"]
pub struct INECCSTT2_SPEC;
impl crate::RegisterSpec for INECCSTT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ineccstt2::R](R) reader structure"]
impl crate::Readable for INECCSTT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INECCSTT2 to value 0"]
impl crate::Resettable for INECCSTT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
