#[doc = "Register `RECEIVE_Q_PTR` reader"]
pub struct R(crate::R<RECEIVE_Q_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEIVE_Q_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEIVE_Q_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEIVE_Q_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECEIVE_Q_PTR` writer"]
pub struct W(crate::W<RECEIVE_Q_PTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEIVE_Q_PTR_SPEC>;
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
impl From<crate::W<RECEIVE_Q_PTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEIVE_Q_PTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_RX_DIS_Q` reader - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while receive is not enabled."]
pub type DMA_RX_DIS_Q_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RX_DIS_Q` writer - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while receive is not enabled."]
pub type DMA_RX_DIS_Q_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECEIVE_Q_PTR_SPEC, bool, O>;
#[doc = "Field `DMA_RX_Q_PTR` reader - Receive buffer queue base address - written with the address of the start of the receive queue."]
pub type DMA_RX_Q_PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMA_RX_Q_PTR` writer - Receive buffer queue base address - written with the address of the start of the receive queue."]
pub type DMA_RX_Q_PTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECEIVE_Q_PTR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while receive is not enabled."]
    #[inline(always)]
    pub fn dma_rx_dis_q(&self) -> DMA_RX_DIS_Q_R {
        DMA_RX_DIS_Q_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - Receive buffer queue base address - written with the address of the start of the receive queue."]
    #[inline(always)]
    pub fn dma_rx_q_ptr(&self) -> DMA_RX_Q_PTR_R {
        DMA_RX_Q_PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Disable queue if set to 1. This can be used to reduce the number of active queues and should only be changed while receive is not enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_dis_q(&mut self) -> DMA_RX_DIS_Q_W<0> {
        DMA_RX_DIS_Q_W::new(self)
    }
    #[doc = "Bits 2:31 - Receive buffer queue base address - written with the address of the start of the receive queue."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_q_ptr(&mut self) -> DMA_RX_Q_PTR_W<2> {
        DMA_RX_Q_PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds the start address of the receive buffer queue (receive buffers descriptor list). The receive buffer queue base address must be initialized before receive is enabled through bit 2 of the network control register. Once reception is enabled, any write to the receive buffer queue base address register is ignored. Reading this register returns the location of the descriptor currently being accessed. This value increments as buffers are used. Software should not use this register for determining where to remove received frames from the queue as it constantly changes as new frames are received. Software should instead work its way through the buffer descriptor queue checking the used bits. In terms of AMBA (AXI) operation, the receive descriptors are read from memory using a single 32bit AXI access. When the datapath is configured at 64bit, the receive descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is written to using a single 64bit AXI access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_q_ptr](index.html) module"]
pub struct RECEIVE_Q_PTR_SPEC;
impl crate::RegisterSpec for RECEIVE_Q_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receive_q_ptr::R](R) reader structure"]
impl crate::Readable for RECEIVE_Q_PTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receive_q_ptr::W](W) writer structure"]
impl crate::Writable for RECEIVE_Q_PTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECEIVE_Q_PTR to value 0"]
impl crate::Resettable for RECEIVE_Q_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
