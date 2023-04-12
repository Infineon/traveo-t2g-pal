#[doc = "Register `DST_FIFO_STATUS` reader"]
pub struct R(crate::R<DST_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Number of used/occupied entries in the destination FIFO. The field value is in the range \\[0, 64\\]. When '0', the FIFO is empty. When '64', the FIFO is full."]
pub type USED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_PTR` reader - Destination FIFO read pointer: Destination FIFO location from which a data is read. Note: This functionality is intended for debugging purposes."]
pub type RD_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_PTR` reader - Destination FIFO write pointer: Destination FIFO location at which a new data is written by the hardware. Note: This functionality is intended for debugging purposes."]
pub type WR_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Number of used/occupied entries in the destination FIFO. The field value is in the range \\[0, 64\\]. When '0', the FIFO is empty. When '64', the FIFO is full."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Destination FIFO read pointer: Destination FIFO location from which a data is read. Note: This functionality is intended for debugging purposes."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Destination FIFO write pointer: Destination FIFO location at which a new data is written by the hardware. Note: This functionality is intended for debugging purposes."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Destination FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_fifo_status](index.html) module"]
pub struct DST_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for DST_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_fifo_status::R](R) reader structure"]
impl crate::Readable for DST_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DST_FIFO_STATUS to value 0"]
impl crate::Resettable for DST_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
