#[doc = "Register `ERROR_STATUS0` reader"]
pub struct R(crate::R<ERROR_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA32` reader - Specifies error description information. - For INSTR_OPC_ERROR/ INSTR_CC_ERROR/ INSTR_DEV_KEY_ERROR: - Violating instruction (from instruction FIFO). - For BUS_ERROR: - Violating transfer, address."]
pub type DATA32_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies error description information. - For INSTR_OPC_ERROR/ INSTR_CC_ERROR/ INSTR_DEV_KEY_ERROR: - Violating instruction (from instruction FIFO). - For BUS_ERROR: - Violating transfer, address."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new(self.bits)
    }
}
#[doc = "Error status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_status0](index.html) module"]
pub struct ERROR_STATUS0_SPEC;
impl crate::RegisterSpec for ERROR_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_status0::R](R) reader structure"]
impl crate::Readable for ERROR_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERROR_STATUS0 to value 0"]
impl crate::Resettable for ERROR_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
