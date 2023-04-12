#[doc = "Register `RX_OVERRUNS` reader"]
pub struct R(crate::R<RX_OVERRUNS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_OVERRUNS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_OVERRUNS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_OVERRUNS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_OVERRUN` reader - Receive overruns - a 10 bit register counting the number of frames that are address recognized but were not copied to memory due to a receive overrun."]
pub type COUNT_OVERRUN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Receive overruns - a 10 bit register counting the number of frames that are address recognized but were not copied to memory due to a receive overrun."]
    #[inline(always)]
    pub fn count_overrun(&self) -> COUNT_OVERRUN_R {
        COUNT_OVERRUN_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Receive Overruns\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_overruns](index.html) module"]
pub struct RX_OVERRUNS_SPEC;
impl crate::RegisterSpec for RX_OVERRUNS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_overruns::R](R) reader structure"]
impl crate::Readable for RX_OVERRUNS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_OVERRUNS to value 0"]
impl crate::Resettable for RX_OVERRUNS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
