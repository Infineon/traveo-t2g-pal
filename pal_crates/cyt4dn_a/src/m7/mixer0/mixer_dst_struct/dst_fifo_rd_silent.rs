#[doc = "Register `DST_FIFO_RD_SILENT` reader"]
pub struct R(crate::R<DST_FIFO_RD_SILENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_FIFO_RD_SILENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_FIFO_RD_SILENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_FIFO_RD_SILENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - N/A"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Destination FIFO silent read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_fifo_rd_silent](index.html) module"]
pub struct DST_FIFO_RD_SILENT_SPEC;
impl crate::RegisterSpec for DST_FIFO_RD_SILENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_fifo_rd_silent::R](R) reader structure"]
impl crate::Readable for DST_FIFO_RD_SILENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DST_FIFO_RD_SILENT to value 0"]
impl crate::Resettable for DST_FIFO_RD_SILENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
