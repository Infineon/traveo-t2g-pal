#[doc = "Register `STORE_FF_STATUS` reader"]
pub struct R(crate::R<STORE_FF_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE_FF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE_FF_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE_FF_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED5` reader - Number of Bytes in the FIFO. The value of this field is in the range \\[0, 16\\]."]
pub type USED5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSY` reader - Reflects the state of the FIFO: '0': FIFO store engine is idle and a new FIFO instruction can be accepted (USED is '0'). '1': FIFO store engine is busy and NO new FIFO instruction can be accepted."]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4 - Number of Bytes in the FIFO. The value of this field is in the range \\[0, 16\\]."]
    #[inline(always)]
    pub fn used5(&self) -> USED5_R {
        USED5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Reflects the state of the FIFO: '0': FIFO store engine is idle and a new FIFO instruction can be accepted (USED is '0'). '1': FIFO store engine is busy and NO new FIFO instruction can be accepted."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Store FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store_ff_status](index.html) module"]
pub struct STORE_FF_STATUS_SPEC;
impl crate::RegisterSpec for STORE_FF_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store_ff_status::R](R) reader structure"]
impl crate::Readable for STORE_FF_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STORE_FF_STATUS to value 0"]
impl crate::Resettable for STORE_FF_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
