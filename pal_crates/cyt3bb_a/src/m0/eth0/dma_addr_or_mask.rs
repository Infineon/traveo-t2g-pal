#[doc = "Register `DMA_ADDR_OR_MASK` reader"]
pub struct R(crate::R<DMA_ADDR_OR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ADDR_OR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ADDR_OR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ADDR_OR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ADDR_OR_MASK` writer"]
pub struct W(crate::W<DMA_ADDR_OR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ADDR_OR_MASK_SPEC>;
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
impl From<crate::W<DMA_ADDR_OR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ADDR_OR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_ENABLE` reader - Data Buffer Address Mask Enable. These bits are associated directly with bits\\[31:28\\].When bit 0 is set, the AXI/AHB address bit 28 used for accessing the receive data buffers will be forced to the value stored in bit 28 of this register. When bit 1 is set, the AXI/AHB address bit 29 used for accessing the receive data buffers will be forced to the value stored in bit 29 of this register. When bit 2 is set, the AXI/AHB address bit 30 used for accessing the receive data buffers will be forced to the value stored in bit 30 of this register. When bit 3 is set, the AXI/AHB address bit 31 used for accessing the receive data buffers will be forced to the value stored in bit 31 of this register. When these bits are clear, the associated value stored in bits 31:28 have no effect on the AXI/AHB address used for receive data buffer accesses. Any changes to this register will be ignored while the DMA is currently processing a receive packet. It will only affect the next full packet to be written to external memory."]
pub type MASK_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_ENABLE` writer - Data Buffer Address Mask Enable. These bits are associated directly with bits\\[31:28\\].When bit 0 is set, the AXI/AHB address bit 28 used for accessing the receive data buffers will be forced to the value stored in bit 28 of this register. When bit 1 is set, the AXI/AHB address bit 29 used for accessing the receive data buffers will be forced to the value stored in bit 29 of this register. When bit 2 is set, the AXI/AHB address bit 30 used for accessing the receive data buffers will be forced to the value stored in bit 30 of this register. When bit 3 is set, the AXI/AHB address bit 31 used for accessing the receive data buffers will be forced to the value stored in bit 31 of this register. When these bits are clear, the associated value stored in bits 31:28 have no effect on the AXI/AHB address used for receive data buffer accesses. Any changes to this register will be ignored while the DMA is currently processing a receive packet. It will only affect the next full packet to be written to external memory."]
pub type MASK_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_ADDR_OR_MASK_SPEC, u8, u8, 4, O>;
#[doc = "Field `MASK_VALUE_DA` reader - Data Buffer Address Mask Value. Values used to force bits 31:28 of the receive data buffer AHB address to a particular value when the associated enable bits stored in this register \\[3:0\\]
are set. Any changes to this register will be ignored while the DMA is currently processing a receive packet. It will only affect the next full packet to be written to external system memory."]
pub type MASK_VALUE_DA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_VALUE_DA` writer - Data Buffer Address Mask Value. Values used to force bits 31:28 of the receive data buffer AHB address to a particular value when the associated enable bits stored in this register \\[3:0\\]
are set. Any changes to this register will be ignored while the DMA is currently processing a receive packet. It will only affect the next full packet to be written to external system memory."]
pub type MASK_VALUE_DA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_ADDR_OR_MASK_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Data Buffer Address Mask Enable. These bits are associated directly with bits\\[31:28\\].When bit 0 is set, the AXI/AHB address bit 28 used for accessing the receive data buffers will be forced to the value stored in bit 28 of this register. When bit 1 is set, the AXI/AHB address bit 29 used for accessing the receive data buffers will be forced to the value stored in bit 29 of this register. When bit 2 is set, the AXI/AHB address bit 30 used for accessing the receive data buffers will be forced to the value stored in bit 30 of this register. When bit 3 is set, the AXI/AHB address bit 31 used for accessing the receive data buffers will be forced to the value stored in bit 31 of this register. When these bits are clear, the associated value stored in bits 31:28 have no effect on the AXI/AHB address used for receive data buffer accesses. Any changes to this register will be ignored while the DMA is currently processing a receive packet. It will only affect the next full packet to be written to external memory."]
    #[inline(always)]
    pub fn mask_enable(&self) -> MASK_ENABLE_R {
        MASK_ENABLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Data Buffer Address Mask Value. Values used to force bits 31:28 of the receive data buffer AHB address to a particular value when the associated enable bits stored in this register \\[3:0\\]
are set. Any changes to this register will be ignored while the DMA is currently processing a receive packet. It will only affect the next full packet to be written to external system memory."]
    #[inline(always)]
    pub fn mask_value_da(&self) -> MASK_VALUE_DA_R {
        MASK_VALUE_DA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Buffer Address Mask Enable. These bits are associated directly with bits\\[31:28\\].When bit 0 is set, the AXI/AHB address bit 28 used for accessing the receive data buffers will be forced to the value stored in bit 28 of this register. When bit 1 is set, the AXI/AHB address bit 29 used for accessing the receive data buffers will be forced to the value stored in bit 29 of this register. When bit 2 is set, the AXI/AHB address bit 30 used for accessing the receive data buffers will be forced to the value stored in bit 30 of this register. When bit 3 is set, the AXI/AHB address bit 31 used for accessing the receive data buffers will be forced to the value stored in bit 31 of this register. When these bits are clear, the associated value stored in bits 31:28 have no effect on the AXI/AHB address used for receive data buffer accesses. Any changes to this register will be ignored while the DMA is currently processing a receive packet. It will only affect the next full packet to be written to external memory."]
    #[inline(always)]
    #[must_use]
    pub fn mask_enable(&mut self) -> MASK_ENABLE_W<0> {
        MASK_ENABLE_W::new(self)
    }
    #[doc = "Bits 28:31 - Data Buffer Address Mask Value. Values used to force bits 31:28 of the receive data buffer AHB address to a particular value when the associated enable bits stored in this register \\[3:0\\]
are set. Any changes to this register will be ignored while the DMA is currently processing a receive packet. It will only affect the next full packet to be written to external system memory."]
    #[inline(always)]
    #[must_use]
    pub fn mask_value_da(&mut self) -> MASK_VALUE_DA_W<28> {
        MASK_VALUE_DA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive DMA Data Buffer Address Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_addr_or_mask](index.html) module"]
pub struct DMA_ADDR_OR_MASK_SPEC;
impl crate::RegisterSpec for DMA_ADDR_OR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_addr_or_mask::R](R) reader structure"]
impl crate::Readable for DMA_ADDR_OR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_addr_or_mask::W](W) writer structure"]
impl crate::Writable for DMA_ADDR_OR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_ADDR_OR_MASK to value 0"]
impl crate::Resettable for DMA_ADDR_OR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
