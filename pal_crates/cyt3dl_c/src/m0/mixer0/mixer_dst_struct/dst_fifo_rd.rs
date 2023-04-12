#[doc = "Register `DST_FIFO_RD` reader"]
pub struct R(crate::R<DST_FIFO_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_FIFO_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_FIFO_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_FIFO_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data (PCM sample pair) read from the top destination FIFO entry (DATA\\[31:0\\]
= {ch1_data\\[15:0\\], ch0_data\\[15:0\\]). Reading removes the data from the destination FIFO; i.e. behavior is similar to that of a POP operation (DST_FIFO_STATUS.RD_PTR is incremented and DST_FIFO_STATUS.USED is decremented). Note: Reading from an empty destination FIFO activates INTR_DST.FIFO_UNDERFLOW (if DST_CTL.ENABLED is '1' and TX_CTL.ENABLED is ''0')."]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data (PCM sample pair) read from the top destination FIFO entry (DATA\\[31:0\\]
= {ch1_data\\[15:0\\], ch0_data\\[15:0\\]). Reading removes the data from the destination FIFO; i.e. behavior is similar to that of a POP operation (DST_FIFO_STATUS.RD_PTR is incremented and DST_FIFO_STATUS.USED is decremented). Note: Reading from an empty destination FIFO activates INTR_DST.FIFO_UNDERFLOW (if DST_CTL.ENABLED is '1' and TX_CTL.ENABLED is ''0')."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Destination FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_fifo_rd](index.html) module"]
pub struct DST_FIFO_RD_SPEC;
impl crate::RegisterSpec for DST_FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_fifo_rd::R](R) reader structure"]
impl crate::Readable for DST_FIFO_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DST_FIFO_RD to value 0"]
impl crate::Resettable for DST_FIFO_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
