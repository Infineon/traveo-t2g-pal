#[doc = "Register `DESIGNCFG_DEBUG6` reader"]
pub struct R(crate::R<DESIGNCFG_DEBUG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNCFG_DEBUG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNCFG_DEBUG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNCFG_DEBUG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_PRIORITY_QUEUE1` reader - Takes the value of the `dma_priority_queue1 DEFINE"]
pub type DMA_PRIORITY_QUEUE1_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE2` reader - Takes the value of the `dma_priority_queue2 DEFINE"]
pub type DMA_PRIORITY_QUEUE2_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE3` reader - Takes the value of the `dma_priority_queue3 DEFINE"]
pub type DMA_PRIORITY_QUEUE3_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE4` reader - Takes the value of the `dma_priority_queue4 DEFINE"]
pub type DMA_PRIORITY_QUEUE4_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE5` reader - Takes the value of the `dma_priority_queue5 DEFINE"]
pub type DMA_PRIORITY_QUEUE5_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE6` reader - Takes the value of the `dma_priority_queue6 DEFINE"]
pub type DMA_PRIORITY_QUEUE6_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE7` reader - Takes the value of the `dma_priority_queue7 DEFINE"]
pub type DMA_PRIORITY_QUEUE7_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE8` reader - Takes the value of the `dma_priority_queue8 DEFINE"]
pub type DMA_PRIORITY_QUEUE8_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE9` reader - Takes the value of the `dma_priority_queue9 DEFINE"]
pub type DMA_PRIORITY_QUEUE9_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE10` reader - Takes the value of the `dma_priority_queue10 DEFINE"]
pub type DMA_PRIORITY_QUEUE10_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE11` reader - Takes the value of the `dma_priority_queue11 DEFINE"]
pub type DMA_PRIORITY_QUEUE11_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE12` reader - Takes the value of the `dma_priority_queue12 DEFINE"]
pub type DMA_PRIORITY_QUEUE12_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE13` reader - Takes the value of the `dma_priority_queue13 DEFINE"]
pub type DMA_PRIORITY_QUEUE13_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE14` reader - Takes the value of the `dma_priority_queue14 DEFINE"]
pub type DMA_PRIORITY_QUEUE14_R = crate::BitReader<bool>;
#[doc = "Field `DMA_PRIORITY_QUEUE15` reader - Takes the value of the `dma_priority_queue15 DEFINE"]
pub type DMA_PRIORITY_QUEUE15_R = crate::BitReader<bool>;
#[doc = "Field `TX_PBUF_QUEUE_SEGMENT_SIZE` reader - Takes the value of the `gem_tx_pbuf_queue_segment_size DEFINE"]
pub type TX_PBUF_QUEUE_SEGMENT_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXT_TSU_TIMER` reader - Takes the value of the `gem_ext_tsu_timer DEFINE"]
pub type EXT_TSU_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `TX_ADD_FIFO_IF` reader - Takes the value of the `gem_tx_add_fifo_if DEFINE"]
pub type TX_ADD_FIFO_IF_R = crate::BitReader<bool>;
#[doc = "Field `HOST_IF_SOFT_SELECT` reader - Takes the value of the `gem_host_if_soft_select DEFINE"]
pub type HOST_IF_SOFT_SELECT_R = crate::BitReader<bool>;
#[doc = "Field `DMA_ADDR_WIDTH_IS_64B` reader - Takes the value of the `gem_dma_addr_width_is_64b DEFINE"]
pub type DMA_ADDR_WIDTH_IS_64B_R = crate::BitReader<bool>;
#[doc = "Field `PFC_MULTI_QUANTUM` reader - Takes the value of the `gem_pfc_multi_quantum DEFINE"]
pub type PFC_MULTI_QUANTUM_R = crate::BitReader<bool>;
#[doc = "Field `PBUF_CUTTHRU` reader - Takes the value of the `gem_pbuf_cutthru DEFINE"]
pub type PBUF_CUTTHRU_R = crate::BitReader<bool>;
#[doc = "Field `PBUF_RSC` reader - Takes the value of the `gem_pbuf_rsc DEFINE"]
pub type PBUF_RSC_R = crate::BitReader<bool>;
#[doc = "Field `PBUF_LSO` reader - Takes the value of the `gem_pbuf_lso DEFINE"]
pub type PBUF_LSO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Takes the value of the `dma_priority_queue1 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue1(&self) -> DMA_PRIORITY_QUEUE1_R {
        DMA_PRIORITY_QUEUE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Takes the value of the `dma_priority_queue2 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue2(&self) -> DMA_PRIORITY_QUEUE2_R {
        DMA_PRIORITY_QUEUE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Takes the value of the `dma_priority_queue3 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue3(&self) -> DMA_PRIORITY_QUEUE3_R {
        DMA_PRIORITY_QUEUE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Takes the value of the `dma_priority_queue4 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue4(&self) -> DMA_PRIORITY_QUEUE4_R {
        DMA_PRIORITY_QUEUE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Takes the value of the `dma_priority_queue5 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue5(&self) -> DMA_PRIORITY_QUEUE5_R {
        DMA_PRIORITY_QUEUE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Takes the value of the `dma_priority_queue6 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue6(&self) -> DMA_PRIORITY_QUEUE6_R {
        DMA_PRIORITY_QUEUE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Takes the value of the `dma_priority_queue7 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue7(&self) -> DMA_PRIORITY_QUEUE7_R {
        DMA_PRIORITY_QUEUE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Takes the value of the `dma_priority_queue8 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue8(&self) -> DMA_PRIORITY_QUEUE8_R {
        DMA_PRIORITY_QUEUE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Takes the value of the `dma_priority_queue9 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue9(&self) -> DMA_PRIORITY_QUEUE9_R {
        DMA_PRIORITY_QUEUE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Takes the value of the `dma_priority_queue10 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue10(&self) -> DMA_PRIORITY_QUEUE10_R {
        DMA_PRIORITY_QUEUE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Takes the value of the `dma_priority_queue11 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue11(&self) -> DMA_PRIORITY_QUEUE11_R {
        DMA_PRIORITY_QUEUE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Takes the value of the `dma_priority_queue12 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue12(&self) -> DMA_PRIORITY_QUEUE12_R {
        DMA_PRIORITY_QUEUE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Takes the value of the `dma_priority_queue13 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue13(&self) -> DMA_PRIORITY_QUEUE13_R {
        DMA_PRIORITY_QUEUE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Takes the value of the `dma_priority_queue14 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue14(&self) -> DMA_PRIORITY_QUEUE14_R {
        DMA_PRIORITY_QUEUE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Takes the value of the `dma_priority_queue15 DEFINE"]
    #[inline(always)]
    pub fn dma_priority_queue15(&self) -> DMA_PRIORITY_QUEUE15_R {
        DMA_PRIORITY_QUEUE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Takes the value of the `gem_tx_pbuf_queue_segment_size DEFINE"]
    #[inline(always)]
    pub fn tx_pbuf_queue_segment_size(&self) -> TX_PBUF_QUEUE_SEGMENT_SIZE_R {
        TX_PBUF_QUEUE_SEGMENT_SIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Takes the value of the `gem_ext_tsu_timer DEFINE"]
    #[inline(always)]
    pub fn ext_tsu_timer(&self) -> EXT_TSU_TIMER_R {
        EXT_TSU_TIMER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Takes the value of the `gem_tx_add_fifo_if DEFINE"]
    #[inline(always)]
    pub fn tx_add_fifo_if(&self) -> TX_ADD_FIFO_IF_R {
        TX_ADD_FIFO_IF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Takes the value of the `gem_host_if_soft_select DEFINE"]
    #[inline(always)]
    pub fn host_if_soft_select(&self) -> HOST_IF_SOFT_SELECT_R {
        HOST_IF_SOFT_SELECT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Takes the value of the `gem_dma_addr_width_is_64b DEFINE"]
    #[inline(always)]
    pub fn dma_addr_width_is_64b(&self) -> DMA_ADDR_WIDTH_IS_64B_R {
        DMA_ADDR_WIDTH_IS_64B_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Takes the value of the `gem_pfc_multi_quantum DEFINE"]
    #[inline(always)]
    pub fn pfc_multi_quantum(&self) -> PFC_MULTI_QUANTUM_R {
        PFC_MULTI_QUANTUM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Takes the value of the `gem_pbuf_cutthru DEFINE"]
    #[inline(always)]
    pub fn pbuf_cutthru(&self) -> PBUF_CUTTHRU_R {
        PBUF_CUTTHRU_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Takes the value of the `gem_pbuf_rsc DEFINE"]
    #[inline(always)]
    pub fn pbuf_rsc(&self) -> PBUF_RSC_R {
        PBUF_RSC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Takes the value of the `gem_pbuf_lso DEFINE"]
    #[inline(always)]
    pub fn pbuf_lso(&self) -> PBUF_LSO_R {
        PBUF_LSO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Design Configuration Register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designcfg_debug6](index.html) module"]
pub struct DESIGNCFG_DEBUG6_SPEC;
impl crate::RegisterSpec for DESIGNCFG_DEBUG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designcfg_debug6::R](R) reader structure"]
impl crate::Readable for DESIGNCFG_DEBUG6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNCFG_DEBUG6 to value 0x0302_0006"]
impl crate::Resettable for DESIGNCFG_DEBUG6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0302_0006;
}
