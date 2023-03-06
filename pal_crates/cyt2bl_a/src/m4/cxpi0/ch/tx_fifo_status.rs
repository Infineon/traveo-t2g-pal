#[doc = "Register `TX_FIFO_STATUS` reader"]
pub struct R(crate::R<TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Number of used/occupied entries in the TX FIFO. The field value is in the range \\[0, 16\\]. When '0', the TX FIFO is empty. When '16', the TX FIFO is full."]
pub type USED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVAIL` reader - TX FIFO Avail 0-No available slot in TX FIFO 1-1 available slot in TX FIFO. 2-2 available slot in TX FIFO. .. 16-16 available slot in TX FIFO. Note that the Fifo Width is 1Byte and each slot in this context is 1 depth of the Fifo. The number of bytes are determine through the number of data bytes in a message frame. (TXPID_FI.FI/TXPID_FI.DLCEXT)"]
pub type AVAIL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Number of used/occupied entries in the TX FIFO. The field value is in the range \\[0, 16\\]. When '0', the TX FIFO is empty. When '16', the TX FIFO is full."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - TX FIFO Avail 0-No available slot in TX FIFO 1-1 available slot in TX FIFO. 2-2 available slot in TX FIFO. .. 16-16 available slot in TX FIFO. Note that the Fifo Width is 1Byte and each slot in this context is 1 depth of the Fifo. The number of bytes are determine through the number of data bytes in a message frame. (TXPID_FI.FI/TXPID_FI.DLCEXT)"]
    #[inline(always)]
    pub fn avail(&self) -> AVAIL_R {
        AVAIL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "TX FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_status](index.html) module"]
pub struct TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_fifo_status::R](R) reader structure"]
impl crate::Readable for TX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_FIFO_STATUS to value 0"]
impl crate::Resettable for TX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
