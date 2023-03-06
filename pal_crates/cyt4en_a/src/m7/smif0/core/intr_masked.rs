#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TR_TX_REQ` reader - Logical and of corresponding request and mask bits."]
pub type TR_TX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_RX_REQ` reader - Logical and of corresponding request and mask bits."]
pub type TR_RX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type XIP_ALIGNMENT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_MMIO_FIFO_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RX_DATA_MMIO_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `DLP_FAIL` reader - Logical and of corresponding request and mask bits."]
pub type DLP_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `DLP_WARNING` reader - Logical and of corresponding request and mask bits."]
pub type DLP_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `DLL_LOCK` reader - Logical and of corresponding request and mask bits."]
pub type DLL_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `DLL_UNLOCK` reader - Logical and of corresponding request and mask bits."]
pub type DLL_UNLOCK_R = crate::BitReader<bool>;
#[doc = "Field `CRC_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `FS_STATUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type FS_STATUS_ERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_data_mmio_fifo_underflow(&self) -> RX_DATA_MMIO_FIFO_UNDERFLOW_R {
        RX_DATA_MMIO_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dlp_fail(&self) -> DLP_FAIL_R {
        DLP_FAIL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dlp_warning(&self) -> DLP_WARNING_R {
        DLP_WARNING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dll_lock(&self) -> DLL_LOCK_R {
        DLL_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dll_unlock(&self) -> DLL_UNLOCK_R {
        DLL_UNLOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn crc_error(&self) -> CRC_ERROR_R {
        CRC_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn fs_status_error(&self) -> FS_STATUS_ERROR_R {
        FS_STATUS_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
