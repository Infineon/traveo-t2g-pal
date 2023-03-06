#[doc = "Register `RX_FIFO_STATUS` reader"]
pub struct R(crate::R<RX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Number of used/occupied entries in the RX FIFO. The field value is in the range \\[0, 16\\]. When '0', the RX FIFO is empty. When '16', the RX FIFO is full."]
pub type USED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVAIL` reader - RX FIFO avail 0-No content in RX FIFO 1-1 available content in RX FIFO. 2-2 available content in RX FIFO. .. 16-16 available content in RX FIFO. Note that the Fifo Width is 1Byte and each content in this context means 1 fifo slot. The number of bytes in each slot are determine through the number of data bytes in a message frame. (RXPID_FI.FI/RXPID_FI.DLCEXT)"]
pub type AVAIL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Number of used/occupied entries in the RX FIFO. The field value is in the range \\[0, 16\\]. When '0', the RX FIFO is empty. When '16', the RX FIFO is full."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - RX FIFO avail 0-No content in RX FIFO 1-1 available content in RX FIFO. 2-2 available content in RX FIFO. .. 16-16 available content in RX FIFO. Note that the Fifo Width is 1Byte and each content in this context means 1 fifo slot. The number of bytes in each slot are determine through the number of data bytes in a message frame. (RXPID_FI.FI/RXPID_FI.DLCEXT)"]
    #[inline(always)]
    pub fn avail(&self) -> AVAIL_R {
        AVAIL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "RX FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_status](index.html) module"]
pub struct RX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for RX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_status::R](R) reader structure"]
impl crate::Readable for RX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_STATUS to value 0"]
impl crate::Resettable for RX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
