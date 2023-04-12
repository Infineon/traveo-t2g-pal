#[doc = "Register `SRC_FIFO_WR` writer"]
pub struct W(crate::W<SRC_FIFO_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_FIFO_WR_SPEC>;
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
impl From<crate::W<SRC_FIFO_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_FIFO_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Data (PCM sample pair of two 16-bit samples) written to the top source FIFO entry (data\\[31:0\\]
= DATA\\[31:0\\]). Typically, DATA\\[15:0\\]
is used for channel 0 and DATA\\[31:16\\]
is used for channel 1. Writing adds the data to the source FIFO; i.e. behavior is similar to that of a PUSH operation (SRC_FIFO_STATUS.WR_PTR is incremented and SRC_FIFO_STATUS.USED is incremented). Note: Writing to a full source FIFO activates INTR_SRC.FIFO_OVERFLOW."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRC_FIFO_WR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Data (PCM sample pair of two 16-bit samples) written to the top source FIFO entry (data\\[31:0\\]
= DATA\\[31:0\\]). Typically, DATA\\[15:0\\]
is used for channel 0 and DATA\\[31:16\\]
is used for channel 1. Writing adds the data to the source FIFO; i.e. behavior is similar to that of a PUSH operation (SRC_FIFO_STATUS.WR_PTR is incremented and SRC_FIFO_STATUS.USED is incremented). Note: Writing to a full source FIFO activates INTR_SRC.FIFO_OVERFLOW."]
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
#[doc = "Source FIFO write\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_fifo_wr](index.html) module"]
pub struct SRC_FIFO_WR_SPEC;
impl crate::RegisterSpec for SRC_FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [src_fifo_wr::W](W) writer structure"]
impl crate::Writable for SRC_FIFO_WR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRC_FIFO_WR to value 0"]
impl crate::Resettable for SRC_FIFO_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
