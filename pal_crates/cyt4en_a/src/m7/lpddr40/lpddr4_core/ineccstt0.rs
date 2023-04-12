#[doc = "Register `INECCSTT0` reader"]
pub struct R(crate::R<INECCSTT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INECCSTT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INECCSTT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INECCSTT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC_ERROR_ADDR` reader - ECC Error FIFO: Address of the DRAM access causing the ECC error. One AXI4 Request can be split into 2 DRAM requests if the AXI4 address is unaligned to 32-byte. This register stores the address of DRAM request."]
pub type ECC_ERROR_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - ECC Error FIFO: Address of the DRAM access causing the ECC error. One AXI4 Request can be split into 2 DRAM requests if the AXI4 address is unaligned to 32-byte. This register stores the address of DRAM request."]
    #[inline(always)]
    pub fn ecc_error_addr(&self) -> ECC_ERROR_ADDR_R {
        ECC_ERROR_ADDR_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Inline ECC Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ineccstt0](index.html) module"]
pub struct INECCSTT0_SPEC;
impl crate::RegisterSpec for INECCSTT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ineccstt0::R](R) reader structure"]
impl crate::Readable for INECCSTT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INECCSTT0 to value 0"]
impl crate::Resettable for INECCSTT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
