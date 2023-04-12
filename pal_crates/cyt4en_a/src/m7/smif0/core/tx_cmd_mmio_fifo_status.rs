#[doc = "Register `TX_CMD_MMIO_FIFO_STATUS` reader"]
pub struct R(crate::R<TX_CMD_MMIO_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CMD_MMIO_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CMD_MMIO_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CMD_MMIO_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED4` reader - Number of entries that are used in the TX command MMIO FIFO. Legal range: \\[0, 8\\]."]
pub type USED4_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of entries that are used in the TX command MMIO FIFO. Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub fn used4(&self) -> USED4_R {
        USED4_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Transmitter command MMIO FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_cmd_mmio_fifo_status](index.html) module"]
pub struct TX_CMD_MMIO_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for TX_CMD_MMIO_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_cmd_mmio_fifo_status::R](R) reader structure"]
impl crate::Readable for TX_CMD_MMIO_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_CMD_MMIO_FIFO_STATUS to value 0"]
impl crate::Resettable for TX_CMD_MMIO_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
