#[doc = "Register `VU_RF_DATA[%s]` reader"]
pub struct R(crate::R<VU_RF_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VU_RF_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VU_RF_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VU_RF_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA32` reader - Vector unit register-file data. A register-file register has the following layout: DATA\\[28:16\\]: data (typically used as a word offset in vector unit operand memory). DATA\\[12:0\\]: bit size minus 1."]
pub type DATA32_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Vector unit register-file data. A register-file register has the following layout: DATA\\[28:16\\]: data (typically used as a word offset in vector unit operand memory). DATA\\[12:0\\]: bit size minus 1."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new(self.bits)
    }
}
#[doc = "Vector unit register-file\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vu_rf_data](index.html) module"]
pub struct VU_RF_DATA_SPEC;
impl crate::RegisterSpec for VU_RF_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vu_rf_data::R](R) reader structure"]
impl crate::Readable for VU_RF_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VU_RF_DATA[%s]
to value 0"]
impl crate::Resettable for VU_RF_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
