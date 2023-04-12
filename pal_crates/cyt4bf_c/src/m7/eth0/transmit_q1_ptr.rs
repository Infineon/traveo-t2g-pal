#[doc = "Register `TRANSMIT_Q1_PTR` reader"]
pub struct R(crate::R<TRANSMIT_Q1_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_Q1_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_Q1_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_Q1_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSMIT_Q1_PTR` writer"]
pub struct W(crate::W<TRANSMIT_Q1_PTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_Q1_PTR_SPEC>;
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
impl From<crate::W<TRANSMIT_Q1_PTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_Q1_PTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_TX_DIS_Q` reader - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while transmit is not enabled."]
pub type DMA_TX_DIS_Q_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TX_DIS_Q` writer - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while transmit is not enabled."]
pub type DMA_TX_DIS_Q_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSMIT_Q1_PTR_SPEC, bool, O>;
#[doc = "Field `DMA_TX_Q_PTR` reader - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AHB/AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit or 128bit, the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AHB/AXI access. For 32bit datapaths, the descriptors should be aligned at 32-bit boundaries and the descriptors are read from memory using two individual 32-bit non sequential accesses."]
pub type DMA_TX_Q_PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMA_TX_Q_PTR` writer - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AHB/AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit or 128bit, the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AHB/AXI access. For 32bit datapaths, the descriptors should be aligned at 32-bit boundaries and the descriptors are read from memory using two individual 32-bit non sequential accesses."]
pub type DMA_TX_Q_PTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSMIT_Q1_PTR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while transmit is not enabled."]
    #[inline(always)]
    pub fn dma_tx_dis_q(&self) -> DMA_TX_DIS_Q_R {
        DMA_TX_DIS_Q_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AHB/AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit or 128bit, the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AHB/AXI access. For 32bit datapaths, the descriptors should be aligned at 32-bit boundaries and the descriptors are read from memory using two individual 32-bit non sequential accesses."]
    #[inline(always)]
    pub fn dma_tx_q_ptr(&self) -> DMA_TX_Q_PTR_R {
        DMA_TX_Q_PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while transmit is not enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_dis_q(&mut self) -> DMA_TX_DIS_Q_W<0> {
        DMA_TX_DIS_Q_W::new(self)
    }
    #[doc = "Bits 2:31 - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AHB/AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit or 128bit, the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AHB/AXI access. For 32bit datapaths, the descriptors should be aligned at 32-bit boundaries and the descriptors are read from memory using two individual 32-bit non sequential accesses."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_q_ptr(&mut self) -> DMA_TX_Q_PTR_W<2> {
        DMA_TX_Q_PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_q1_ptr](index.html) module"]
pub struct TRANSMIT_Q1_PTR_SPEC;
impl crate::RegisterSpec for TRANSMIT_Q1_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_q1_ptr::R](R) reader structure"]
impl crate::Readable for TRANSMIT_Q1_PTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_q1_ptr::W](W) writer structure"]
impl crate::Writable for TRANSMIT_Q1_PTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSMIT_Q1_PTR to value 0"]
impl crate::Resettable for TRANSMIT_Q1_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
