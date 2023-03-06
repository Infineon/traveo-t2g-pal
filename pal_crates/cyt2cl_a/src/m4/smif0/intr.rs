#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR_TX_REQ` reader - Activated when a TX data MMIO FIFO trigger 'tr_tx_req' is activated."]
pub type TR_TX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_TX_REQ` writer - Activated when a TX data MMIO FIFO trigger 'tr_tx_req' is activated."]
pub type TR_TX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TR_RX_REQ` reader - Activated when a RX data MMIO FIFO trigger 'tr_rx_req' is activated."]
pub type TR_RX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_RX_REQ` writer - Activated when a RX data MMIO FIFO trigger 'tr_rx_req' is activated."]
pub type TR_RX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Activated on a XIP access, when: - a write transfer is requested and - Dual-Quad SPI mode (selected device's ADDR_CTL.DIV2 is '1') is selected or - Octal SPI DDR mode (selected device's DATA_CTL.DDR_MODE = '1' and DATA_CTL.WIDTH = '3') or Hyperbus mode (selected device's ADDR_CTL.SIZE3 = '7') is selected without memory write byte masking (selected device's WR_DUMMY_CTL.RWDS_EN = '0') and - the AHB-Lite / AXI bus transfer address is not a multiple of '2' or - the requested AHB-Lite / AXI bus transfer size is NOT a multiple of '2'. Additionally, it is activated on a XIP access when the selected memory device does not support write byte masking (WR_DUMMY_CTL.RWDS_EN=0) and an AXI transfer occurs with not all of the AXI write strobes ('wstrb') enabled according to the transfer size ('assize'). Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. Write accesses are only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2. Octal SPI DDR mode or Hyperbus mode are 16-bit word based protocols, writing single bytes is only possible when write byte masking is supported (via RWDS). Read accesses are always possible by extending the transfer size and / or aligning the address as needed and discarding the non-relevant byte(s)."]
pub type XIP_ALIGNMENT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` writer - Activated on a XIP access, when: - a write transfer is requested and - Dual-Quad SPI mode (selected device's ADDR_CTL.DIV2 is '1') is selected or - Octal SPI DDR mode (selected device's DATA_CTL.DDR_MODE = '1' and DATA_CTL.WIDTH = '3') or Hyperbus mode (selected device's ADDR_CTL.SIZE3 = '7') is selected without memory write byte masking (selected device's WR_DUMMY_CTL.RWDS_EN = '0') and - the AHB-Lite / AXI bus transfer address is not a multiple of '2' or - the requested AHB-Lite / AXI bus transfer size is NOT a multiple of '2'. Additionally, it is activated on a XIP access when the selected memory device does not support write byte masking (WR_DUMMY_CTL.RWDS_EN=0) and an AXI transfer occurs with not all of the AXI write strobes ('wstrb') enabled according to the transfer size ('assize'). Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. Write accesses are only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2. Octal SPI DDR mode or Hyperbus mode are 16-bit word based protocols, writing single bytes is only possible when write byte masking is supported (via RWDS). Read accesses are always possible by extending the transfer size and / or aligning the address as needed and discarding the non-relevant byte(s)."]
pub type XIP_ALIGNMENT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Activated on an AHB-Lite write transfer to the TX command MMIO FIFO (TX_CMD_MMIO_FIFO_WR) with not enough free entries available."]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` writer - Activated on an AHB-Lite write transfer to the TX command MMIO FIFO (TX_CMD_MMIO_FIFO_WR) with not enough free entries available."]
pub type TX_CMD_FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Activated on an AHB-Lite write transfer to the TX data MMIO FIFO (TX_DATA_MMIO_FIFO_WR1, TX_DATA_MMIO_FIFO_WR2, TX_DATA_MMIO_FIFO_WR4, TX_DATA_MMIO_FIFO_WR1ODD) with not enough free entries available."]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` writer - Activated on an AHB-Lite write transfer to the TX data MMIO FIFO (TX_DATA_MMIO_FIFO_WR1, TX_DATA_MMIO_FIFO_WR2, TX_DATA_MMIO_FIFO_WR4, TX_DATA_MMIO_FIFO_WR1ODD) with not enough free entries available."]
pub type TX_DATA_FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_DATA_MMIO_FIFO_UNDERFLOW` reader - Activated on an AHB-Lite read transfer from the RX data MMIO FIFO (RX_DATA_MMIO_FIFO_RD1, RX_DATA_MMIO_FIFO_RD2, RX_DATA_MMIO_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
pub type RX_DATA_MMIO_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_MMIO_FIFO_UNDERFLOW` writer - Activated on an AHB-Lite read transfer from the RX data MMIO FIFO (RX_DATA_MMIO_FIFO_RD1, RX_DATA_MMIO_FIFO_RD2, RX_DATA_MMIO_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
pub type RX_DATA_MMIO_FIFO_UNDERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `DL_FAIL` reader - Data Learning Failed (no DLP match found on at least one of the input data lines when CTL.INT_CLOCK_DL_ENABLED = 1)."]
pub type DL_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `DL_FAIL` writer - Data Learning Failed (no DLP match found on at least one of the input data lines when CTL.INT_CLOCK_DL_ENABLED = 1)."]
pub type DL_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `DL_WARNING` reader - Data Learning Warning (for at least one input data line less then DLP.DL_WARNING_LEVEL delay line taps resulted in a correct DLP capturing when CTL.INT_CLOCK_DL_ENABLED = 1). This interrupt will be suppressed, though, if DL_FAIL also occurs during the same DLP evaluation cycle."]
pub type DL_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `DL_WARNING` writer - Data Learning Warning (for at least one input data line less then DLP.DL_WARNING_LEVEL delay line taps resulted in a correct DLP capturing when CTL.INT_CLOCK_DL_ENABLED = 1). This interrupt will be suppressed, though, if DL_FAIL also occurs during the same DLP evaluation cycle."]
pub type DL_WARNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `CRC_ERROR` reader - CRC Error. A read transfer data CRC check failed."]
pub type CRC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `CRC_ERROR` writer - CRC Error. A read transfer data CRC check failed."]
pub type CRC_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `FS_STATUS_ERROR` reader - Functional Safety Status Error. A read transfer Functional Safety Status check failed (see definition in DEVICE_ver2.RD_CRC_CTL.STATUS_CHECK_MASK and DEVICE_ver2.RD_CRC_CTL.STATUS_CHECK_MASK_POL)."]
pub type FS_STATUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `FS_STATUS_ERROR` writer - Functional Safety Status Error. A read transfer Functional Safety Status check failed (see definition in DEVICE_ver2.RD_CRC_CTL.STATUS_CHECK_MASK and DEVICE_ver2.RD_CRC_CTL.STATUS_CHECK_MASK_POL)."]
pub type FS_STATUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Activated when a TX data MMIO FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Activated when a RX data MMIO FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Activated on a XIP access, when: - a write transfer is requested and - Dual-Quad SPI mode (selected device's ADDR_CTL.DIV2 is '1') is selected or - Octal SPI DDR mode (selected device's DATA_CTL.DDR_MODE = '1' and DATA_CTL.WIDTH = '3') or Hyperbus mode (selected device's ADDR_CTL.SIZE3 = '7') is selected without memory write byte masking (selected device's WR_DUMMY_CTL.RWDS_EN = '0') and - the AHB-Lite / AXI bus transfer address is not a multiple of '2' or - the requested AHB-Lite / AXI bus transfer size is NOT a multiple of '2'. Additionally, it is activated on a XIP access when the selected memory device does not support write byte masking (WR_DUMMY_CTL.RWDS_EN=0) and an AXI transfer occurs with not all of the AXI write strobes ('wstrb') enabled according to the transfer size ('assize'). Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. Write accesses are only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2. Octal SPI DDR mode or Hyperbus mode are 16-bit word based protocols, writing single bytes is only possible when write byte masking is supported (via RWDS). Read accesses are always possible by extending the transfer size and / or aligning the address as needed and discarding the non-relevant byte(s)."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Activated on an AHB-Lite write transfer to the TX command MMIO FIFO (TX_CMD_MMIO_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Activated on an AHB-Lite write transfer to the TX data MMIO FIFO (TX_DATA_MMIO_FIFO_WR1, TX_DATA_MMIO_FIFO_WR2, TX_DATA_MMIO_FIFO_WR4, TX_DATA_MMIO_FIFO_WR1ODD) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Activated on an AHB-Lite read transfer from the RX data MMIO FIFO (RX_DATA_MMIO_FIFO_RD1, RX_DATA_MMIO_FIFO_RD2, RX_DATA_MMIO_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    pub fn rx_data_mmio_fifo_underflow(&self) -> RX_DATA_MMIO_FIFO_UNDERFLOW_R {
        RX_DATA_MMIO_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Data Learning Failed (no DLP match found on at least one of the input data lines when CTL.INT_CLOCK_DL_ENABLED = 1)."]
    #[inline(always)]
    pub fn dl_fail(&self) -> DL_FAIL_R {
        DL_FAIL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Learning Warning (for at least one input data line less then DLP.DL_WARNING_LEVEL delay line taps resulted in a correct DLP capturing when CTL.INT_CLOCK_DL_ENABLED = 1). This interrupt will be suppressed, though, if DL_FAIL also occurs during the same DLP evaluation cycle."]
    #[inline(always)]
    pub fn dl_warning(&self) -> DL_WARNING_R {
        DL_WARNING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CRC Error. A read transfer data CRC check failed."]
    #[inline(always)]
    pub fn crc_error(&self) -> CRC_ERROR_R {
        CRC_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Functional Safety Status Error. A read transfer Functional Safety Status check failed (see definition in DEVICE_ver2.RD_CRC_CTL.STATUS_CHECK_MASK and DEVICE_ver2.RD_CRC_CTL.STATUS_CHECK_MASK_POL)."]
    #[inline(always)]
    pub fn fs_status_error(&self) -> FS_STATUS_ERROR_R {
        FS_STATUS_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activated when a TX data MMIO FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    #[must_use]
    pub fn tr_tx_req(&mut self) -> TR_TX_REQ_W<0> {
        TR_TX_REQ_W::new(self)
    }
    #[doc = "Bit 1 - Activated when a RX data MMIO FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rx_req(&mut self) -> TR_RX_REQ_W<1> {
        TR_RX_REQ_W::new(self)
    }
    #[doc = "Bit 2 - Activated on a XIP access, when: - a write transfer is requested and - Dual-Quad SPI mode (selected device's ADDR_CTL.DIV2 is '1') is selected or - Octal SPI DDR mode (selected device's DATA_CTL.DDR_MODE = '1' and DATA_CTL.WIDTH = '3') or Hyperbus mode (selected device's ADDR_CTL.SIZE3 = '7') is selected without memory write byte masking (selected device's WR_DUMMY_CTL.RWDS_EN = '0') and - the AHB-Lite / AXI bus transfer address is not a multiple of '2' or - the requested AHB-Lite / AXI bus transfer size is NOT a multiple of '2'. Additionally, it is activated on a XIP access when the selected memory device does not support write byte masking (WR_DUMMY_CTL.RWDS_EN=0) and an AXI transfer occurs with not all of the AXI write strobes ('wstrb') enabled according to the transfer size ('assize'). Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. Write accesses are only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2. Octal SPI DDR mode or Hyperbus mode are 16-bit word based protocols, writing single bytes is only possible when write byte masking is supported (via RWDS). Read accesses are always possible by extending the transfer size and / or aligning the address as needed and discarding the non-relevant byte(s)."]
    #[inline(always)]
    #[must_use]
    pub fn xip_alignment_error(&mut self) -> XIP_ALIGNMENT_ERROR_W<2> {
        XIP_ALIGNMENT_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Activated on an AHB-Lite write transfer to the TX command MMIO FIFO (TX_CMD_MMIO_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    #[must_use]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TX_CMD_FIFO_OVERFLOW_W<3> {
        TX_CMD_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - Activated on an AHB-Lite write transfer to the TX data MMIO FIFO (TX_DATA_MMIO_FIFO_WR1, TX_DATA_MMIO_FIFO_WR2, TX_DATA_MMIO_FIFO_WR4, TX_DATA_MMIO_FIFO_WR1ODD) with not enough free entries available."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_fifo_overflow(&mut self) -> TX_DATA_FIFO_OVERFLOW_W<4> {
        TX_DATA_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 5 - Activated on an AHB-Lite read transfer from the RX data MMIO FIFO (RX_DATA_MMIO_FIFO_RD1, RX_DATA_MMIO_FIFO_RD2, RX_DATA_MMIO_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_mmio_fifo_underflow(&mut self) -> RX_DATA_MMIO_FIFO_UNDERFLOW_W<5> {
        RX_DATA_MMIO_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Data Learning Failed (no DLP match found on at least one of the input data lines when CTL.INT_CLOCK_DL_ENABLED = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn dl_fail(&mut self) -> DL_FAIL_W<8> {
        DL_FAIL_W::new(self)
    }
    #[doc = "Bit 12 - Data Learning Warning (for at least one input data line less then DLP.DL_WARNING_LEVEL delay line taps resulted in a correct DLP capturing when CTL.INT_CLOCK_DL_ENABLED = 1). This interrupt will be suppressed, though, if DL_FAIL also occurs during the same DLP evaluation cycle."]
    #[inline(always)]
    #[must_use]
    pub fn dl_warning(&mut self) -> DL_WARNING_W<12> {
        DL_WARNING_W::new(self)
    }
    #[doc = "Bit 16 - CRC Error. A read transfer data CRC check failed."]
    #[inline(always)]
    #[must_use]
    pub fn crc_error(&mut self) -> CRC_ERROR_W<16> {
        CRC_ERROR_W::new(self)
    }
    #[doc = "Bit 17 - Functional Safety Status Error. A read transfer Functional Safety Status check failed (see definition in DEVICE_ver2.RD_CRC_CTL.STATUS_CHECK_MASK and DEVICE_ver2.RD_CRC_CTL.STATUS_CHECK_MASK_POL)."]
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
#[doc = "Interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
