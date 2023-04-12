#[doc = "Register `RX_LPI` reader"]
pub struct R(crate::R<RX_LPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_LPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_LPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_LPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_LPI` reader - Count of RX LPI transitions. A count of the number of times there is a transition from receiving normal idle to receiving low power idle. Cleared on read."]
pub type COUNT_LPI_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count of RX LPI transitions. A count of the number of times there is a transition from receiving normal idle to receiving low power idle. Cleared on read."]
    #[inline(always)]
    pub fn count_lpi(&self) -> COUNT_LPI_R {
        COUNT_LPI_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Received LPI transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_lpi](index.html) module"]
pub struct RX_LPI_SPEC;
impl crate::RegisterSpec for RX_LPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_lpi::R](R) reader structure"]
impl crate::Readable for RX_LPI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_LPI to value 0"]
impl crate::Resettable for RX_LPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
