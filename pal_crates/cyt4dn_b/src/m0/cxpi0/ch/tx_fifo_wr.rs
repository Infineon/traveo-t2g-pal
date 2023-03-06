#[doc = "Register `TX_FIFO_WR` writer"]
pub struct W(crate::W<TX_FIFO_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_FIFO_WR_SPEC>;
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
impl From<crate::W<TX_FIFO_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_FIFO_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Transmit Data field. Transmission: To be transmitted data field. SW provides data field. HW shadows over the write data to TX FIFO after SW performs a write to this field. HW shadows the whole 8 bits to the TX FIFO and relies on the TXPID_FI.FI/TXPID_FI.DCLEXT to determine the number of bytes. SW needs to ensure that TX FIFO is not overwritten before the content is consumed by HW by checking TX_FIFO_STATUS.AVAIL. Otherwise, the previous content would be overwritten and resulting in TX FIFO's overflow error (INTR.TX_OVERFLOW_ERROR)."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_FIFO_WR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Transmit Data field. Transmission: To be transmitted data field. SW provides data field. HW shadows over the write data to TX FIFO after SW performs a write to this field. HW shadows the whole 8 bits to the TX FIFO and relies on the TXPID_FI.FI/TXPID_FI.DCLEXT to determine the number of bytes. SW needs to ensure that TX FIFO is not overwritten before the content is consumed by HW by checking TX_FIFO_STATUS.AVAIL. Otherwise, the previous content would be overwritten and resulting in TX FIFO's overflow error (INTR.TX_OVERFLOW_ERROR)."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO write\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_wr](index.html) module"]
pub struct TX_FIFO_WR_SPEC;
impl crate::RegisterSpec for TX_FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tx_fifo_wr::W](W) writer structure"]
impl crate::Writable for TX_FIFO_WR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_FIFO_WR to value 0"]
impl crate::Resettable for TX_FIFO_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
