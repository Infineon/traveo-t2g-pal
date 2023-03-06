#[doc = "Register `RX_FIFO_RD` reader"]
pub struct R(crate::R<RX_FIFO_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data (PCM sample) read from the RX FIFO. Reading removes the data from the RX FIFO; i.e. behavior is similar to that of a POP operation (RX_FIFO_STATUS.RD_PTR is incremented and RX_FIFO_STATUS.USED is decremented). The read data (DATA) is right aligned (unused bit positions follow the specified sign extension per RX_CTL.WORD_SIGN_EXTEND) when it is read from the FIFO entry (data\\[31:0\\]): - 8 bit, DATA\\[7:0\\]
= data\\[31:24\\]. - 10 bit, DATA\\[9:0\\]
= data\\[31:22\\]. - 12 bit, DATA\\[11:0\\]
= data\\[31:20\\]. - 14 bit, DATA\\[13:0\\]
= data\\[31:18\\]. - 16 bit, DATA\\[15:0\\]
= data\\[31:16\\]. - 18 bit, DATA\\[17:0\\]
= data\\[31:14\\]. - 20 bit, DATA\\[19:0\\]
= data\\[31:12\\]. - 24 bit, DATA\\[23:0\\]
= data\\[31:8\\]. - 32 bit, DATA\\[31:0\\]
= data\\[31:0\\]. Note: Reading from an empty RX FIFO activates INTR_RX.FIFO_UNDERFLOW."]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data (PCM sample) read from the RX FIFO. Reading removes the data from the RX FIFO; i.e. behavior is similar to that of a POP operation (RX_FIFO_STATUS.RD_PTR is incremented and RX_FIFO_STATUS.USED is decremented). The read data (DATA) is right aligned (unused bit positions follow the specified sign extension per RX_CTL.WORD_SIGN_EXTEND) when it is read from the FIFO entry (data\\[31:0\\]): - 8 bit, DATA\\[7:0\\]
= data\\[31:24\\]. - 10 bit, DATA\\[9:0\\]
= data\\[31:22\\]. - 12 bit, DATA\\[11:0\\]
= data\\[31:20\\]. - 14 bit, DATA\\[13:0\\]
= data\\[31:18\\]. - 16 bit, DATA\\[15:0\\]
= data\\[31:16\\]. - 18 bit, DATA\\[17:0\\]
= data\\[31:14\\]. - 20 bit, DATA\\[19:0\\]
= data\\[31:12\\]. - 24 bit, DATA\\[23:0\\]
= data\\[31:8\\]. - 32 bit, DATA\\[31:0\\]
= data\\[31:0\\]. Note: Reading from an empty RX FIFO activates INTR_RX.FIFO_UNDERFLOW."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "RX FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd](index.html) module"]
pub struct RX_FIFO_RD_SPEC;
impl crate::RegisterSpec for RX_FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_rd::R](R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_RD to value 0"]
impl crate::Resettable for RX_FIFO_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
