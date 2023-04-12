#[doc = "Register `AUTO_FLUSHED_PKTS` reader"]
pub struct R(crate::R<AUTO_FLUSHED_PKTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTO_FLUSHED_PKTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTO_FLUSHED_PKTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTO_FLUSHED_PKTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_FLUSHED` reader - Flushed RX packets counter. A 16 bit register counting the number of frames that have been flushed from the receive SRAM based packet buffer due to one of the following reasons .1. When partial store and forward mode is enabled or bit 24 of the DMA configuration register is enabled, a packet is received while there is no AMBA (AXI/AHB) resource. 2. When partial store and forward mode is enabled and an AMBA (AXI/AHB) error is encountered while writing the packet data to external memory. When bit 18 of the network control register(software action to flush a packet from the head of the PBUF queue) is pulsed and the GEM DMA is not currently busy."]
pub type COUNT_FLUSHED_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Flushed RX packets counter. A 16 bit register counting the number of frames that have been flushed from the receive SRAM based packet buffer due to one of the following reasons .1. When partial store and forward mode is enabled or bit 24 of the DMA configuration register is enabled, a packet is received while there is no AMBA (AXI/AHB) resource. 2. When partial store and forward mode is enabled and an AMBA (AXI/AHB) error is encountered while writing the packet data to external memory. When bit 18 of the network control register(software action to flush a packet from the head of the PBUF queue) is pulsed and the GEM DMA is not currently busy."]
    #[inline(always)]
    pub fn count_flushed(&self) -> COUNT_FLUSHED_R {
        COUNT_FLUSHED_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive DMA Flushed Packets\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auto_flushed_pkts](index.html) module"]
pub struct AUTO_FLUSHED_PKTS_SPEC;
impl crate::RegisterSpec for AUTO_FLUSHED_PKTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auto_flushed_pkts::R](R) reader structure"]
impl crate::Readable for AUTO_FLUSHED_PKTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AUTO_FLUSHED_PKTS to value 0"]
impl crate::Resettable for AUTO_FLUSHED_PKTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
