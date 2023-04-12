#[doc = "Register `RX_FIFO_RD` reader"]
pub struct R(crate::R<RX_FIFO_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Received Data field. Software uses this data field. HW shadows the first content of the RX FIFO to this field. Software reading this field will remove the content from the RX FIFO and the next content of the RX FIFO will be shadowed over to this field. This field is 8bits and reflects the width of the RX FIFO. Software needs to rely on the RXPID_FI.FI/RXPID_FI.DLCEXT fields to determine number of bytes. Note that, during debug, a read from test controller would not remove/destory the content. Software needs to ensure it does not read from this field if there is no available content (from RX_FIFO_STATUS.USED). Otherwise, the content is undefined and it would result in RX FIFO underflow error. (INTR.RX_UNDERFLOW_ERROR)."]
pub type DATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Received Data field. Software uses this data field. HW shadows the first content of the RX FIFO to this field. Software reading this field will remove the content from the RX FIFO and the next content of the RX FIFO will be shadowed over to this field. This field is 8bits and reflects the width of the RX FIFO. Software needs to rely on the RXPID_FI.FI/RXPID_FI.DLCEXT fields to determine number of bytes. Note that, during debug, a read from test controller would not remove/destory the content. Software needs to ensure it does not read from this field if there is no available content (from RX_FIFO_STATUS.USED). Otherwise, the content is undefined and it would result in RX FIFO underflow error. (INTR.RX_UNDERFLOW_ERROR)."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd](index.html) module"]
pub struct RX_FIFO_RD_SPEC;
impl crate::RegisterSpec for RX_FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_rd::R](R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_RD to value 0"]
impl crate::Resettable for RX_FIFO_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
