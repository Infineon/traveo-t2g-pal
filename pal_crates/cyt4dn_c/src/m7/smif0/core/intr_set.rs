#[doc = "Register `INTR_SET` reader"]
pub struct R(crate::R<INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SET` writer"]
pub struct W(crate::W<INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR_TX_REQ` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TR_TX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_TX_REQ` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TR_TX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TR_RX_REQ` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TR_RX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_RX_REQ` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TR_RX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type XIP_ALIGNMENT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type XIP_ALIGNMENT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TX_CMD_FIFO_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TX_DATA_FIFO_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_DATA_MMIO_FIFO_UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_DATA_MMIO_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_MMIO_FIFO_UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_DATA_MMIO_FIFO_UNDERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `DLP_FAIL` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type DLP_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `DLP_FAIL` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type DLP_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `DLP_WARNING` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type DLP_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `DLP_WARNING` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type DLP_WARNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `DLL_LOCK` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type DLL_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `DLL_LOCK` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type DLL_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `DLL_UNLOCK` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type DLL_UNLOCK_R = crate::BitReader<bool>;
#[doc = "Field `DLL_UNLOCK` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type DLL_UNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `CRC_ERROR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `CRC_ERROR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type CRC_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `FS_STATUS_ERROR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type FS_STATUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `FS_STATUS_ERROR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type FS_STATUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_data_mmio_fifo_underflow(&self) -> RX_DATA_MMIO_FIFO_UNDERFLOW_R {
        RX_DATA_MMIO_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dlp_fail(&self) -> DLP_FAIL_R {
        DLP_FAIL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dlp_warning(&self) -> DLP_WARNING_R {
        DLP_WARNING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dll_lock(&self) -> DLL_LOCK_R {
        DLL_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dll_unlock(&self) -> DLL_UNLOCK_R {
        DLL_UNLOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn crc_error(&self) -> CRC_ERROR_R {
        CRC_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fs_status_error(&self) -> FS_STATUS_ERROR_R {
        FS_STATUS_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_tx_req(&mut self) -> TR_TX_REQ_W<0> {
        TR_TX_REQ_W::new(self)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rx_req(&mut self) -> TR_RX_REQ_W<1> {
        TR_RX_REQ_W::new(self)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn xip_alignment_error(&mut self) -> XIP_ALIGNMENT_ERROR_W<2> {
        XIP_ALIGNMENT_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TX_CMD_FIFO_OVERFLOW_W<3> {
        TX_CMD_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_fifo_overflow(&mut self) -> TX_DATA_FIFO_OVERFLOW_W<4> {
        TX_DATA_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_mmio_fifo_underflow(&mut self) -> RX_DATA_MMIO_FIFO_UNDERFLOW_W<5> {
        RX_DATA_MMIO_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn dlp_fail(&mut self) -> DLP_FAIL_W<8> {
        DLP_FAIL_W::new(self)
    }
    #[doc = "Bit 12 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn dlp_warning(&mut self) -> DLP_WARNING_W<12> {
        DLP_WARNING_W::new(self)
    }
    #[doc = "Bit 13 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn dll_lock(&mut self) -> DLL_LOCK_W<13> {
        DLL_LOCK_W::new(self)
    }
    #[doc = "Bit 14 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn dll_unlock(&mut self) -> DLL_UNLOCK_W<14> {
        DLL_UNLOCK_W::new(self)
    }
    #[doc = "Bit 16 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn crc_error(&mut self) -> CRC_ERROR_W<16> {
        CRC_ERROR_W::new(self)
    }
    #[doc = "Bit 17 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn fs_status_error(&mut self) -> FS_STATUS_ERROR_W<17> {
        FS_STATUS_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_set::R](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_set::W](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
