#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STOP_ACK` reader - Clock Stop Acknowledge for each TTCAN IP. These bits are directly driven by m_ttcan_clkstop_ack of each TTCAN IP. When this bit is set the corresponding TTCAN IP clocks will be gated off, except HCLK will enabled for each AHB write"]
pub type STOP_ACK_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Clock Stop Acknowledge for each TTCAN IP. These bits are directly driven by m_ttcan_clkstop_ack of each TTCAN IP. When this bit is set the corresponding TTCAN IP clocks will be gated off, except HCLK will enabled for each AHB write"]
    #[inline(always)]
    pub fn stop_ack(&self) -> STOP_ACK_R {
        STOP_ACK_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Global CAN status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
