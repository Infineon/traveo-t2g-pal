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
#[doc = "Field `DATA` writer - Data (PCM sample) written to the TX FIFO. Writing adds the data to the TX FIFO; i.e. behavior is similar to that of a PUSH operation (TX_FIFO_STATUS.WR_PTR is incremented and TX_FIFO_STATUS.USED is incremented). The write data (DATA) should be right aligned when it is written to the FIFO entry (data\\[23:0\\]): - 8 bit, pcm_data\\[23:0\\]
= DATA\\[7:0\\]
&lt;&lt; 16. - 10 bit, pcm_data\\[23:0\\]
= DATA\\[9:0\\]
&lt;&lt; 14. - 12 bit, pcm_data\\[23:0\\]
= DATA\\[11:0\\]
&lt;&lt; 12. - 14 bit, pcm_data\\[23:0\\]
= DATA\\[13:0\\]
&lt;&lt; 10. - 16 bit, pcm_data\\[23:0\\]
= DATA\\[15:0\\]
&lt;&lt; 8. - 18 bit, pcm_data\\[23:0\\]
= DATA\\[17:0\\]
&lt;&lt; 6. - 20 bit, pcm_data\\[23:0\\]
= DATA\\[19:0\\]
&lt;&lt; 4. - 24 bit, pcm_data\\[23:0\\]
= DATA\\[23:0\\]. - 32 bit, pcm_data\\[23:0\\]
= DATA\\[31:8\\]. Note: Writing to a full TX FIFO activates INTR.TX_FIFO_OVERFLOW."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_FIFO_WR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Data (PCM sample) written to the TX FIFO. Writing adds the data to the TX FIFO; i.e. behavior is similar to that of a PUSH operation (TX_FIFO_STATUS.WR_PTR is incremented and TX_FIFO_STATUS.USED is incremented). The write data (DATA) should be right aligned when it is written to the FIFO entry (data\\[23:0\\]): - 8 bit, pcm_data\\[23:0\\]
= DATA\\[7:0\\]
&lt;&lt; 16. - 10 bit, pcm_data\\[23:0\\]
= DATA\\[9:0\\]
&lt;&lt; 14. - 12 bit, pcm_data\\[23:0\\]
= DATA\\[11:0\\]
&lt;&lt; 12. - 14 bit, pcm_data\\[23:0\\]
= DATA\\[13:0\\]
&lt;&lt; 10. - 16 bit, pcm_data\\[23:0\\]
= DATA\\[15:0\\]
&lt;&lt; 8. - 18 bit, pcm_data\\[23:0\\]
= DATA\\[17:0\\]
&lt;&lt; 6. - 20 bit, pcm_data\\[23:0\\]
= DATA\\[19:0\\]
&lt;&lt; 4. - 24 bit, pcm_data\\[23:0\\]
= DATA\\[23:0\\]. - 32 bit, pcm_data\\[23:0\\]
= DATA\\[31:8\\]. Note: Writing to a full TX FIFO activates INTR.TX_FIFO_OVERFLOW."]
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
