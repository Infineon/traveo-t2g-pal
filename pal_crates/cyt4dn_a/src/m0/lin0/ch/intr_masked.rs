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
#[doc = "Field `TX_HEADER_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_WAKEUP_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_HEADER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_RESPONSE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_BREAK_WAKEUP_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_BREAK_WAKEUP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_SYNC_DONE` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_HEADER_SYNC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RX_NOISE_DETECT` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_NOISE_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER_BIT_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_HEADER_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE_BIT_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type TX_RESPONSE_BIT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_FRAME_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_HEADER_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_SYNC_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_HEADER_SYNC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER_PARITY_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_HEADER_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_FRAME_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_RESPONSE_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE_CHECKSUM_ERROR` reader - Logical AND of corresponding INTR and INTR_MASK fields."]
pub type RX_RESPONSE_CHECKSUM_ERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_header_done(&self) -> TX_HEADER_DONE_R {
        TX_HEADER_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_response_done(&self) -> TX_RESPONSE_DONE_R {
        TX_RESPONSE_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_wakeup_done(&self) -> TX_WAKEUP_DONE_R {
        TX_WAKEUP_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_header_done(&self) -> RX_HEADER_DONE_R {
        RX_HEADER_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_response_done(&self) -> RX_RESPONSE_DONE_R {
        RX_RESPONSE_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_break_wakeup_done(&self) -> RX_BREAK_WAKEUP_DONE_R {
        RX_BREAK_WAKEUP_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_header_sync_done(&self) -> RX_HEADER_SYNC_DONE_R {
        RX_HEADER_SYNC_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_noise_detect(&self) -> RX_NOISE_DETECT_R {
        RX_NOISE_DETECT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_header_bit_error(&self) -> TX_HEADER_BIT_ERROR_R {
        TX_HEADER_BIT_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn tx_response_bit_error(&self) -> TX_RESPONSE_BIT_ERROR_R {
        TX_RESPONSE_BIT_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_header_frame_error(&self) -> RX_HEADER_FRAME_ERROR_R {
        RX_HEADER_FRAME_ERROR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_header_sync_error(&self) -> RX_HEADER_SYNC_ERROR_R {
        RX_HEADER_SYNC_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_header_parity_error(&self) -> RX_HEADER_PARITY_ERROR_R {
        RX_HEADER_PARITY_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_response_frame_error(&self) -> RX_RESPONSE_FRAME_ERROR_R {
        RX_RESPONSE_FRAME_ERROR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Logical AND of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn rx_response_checksum_error(&self) -> RX_RESPONSE_CHECKSUM_ERROR_R {
        RX_RESPONSE_CHECKSUM_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
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
