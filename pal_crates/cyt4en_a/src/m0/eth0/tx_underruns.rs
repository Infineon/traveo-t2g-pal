#[doc = "Register `TX_UNDERRUNS` reader"]
pub struct R(crate::R<TX_UNDERRUNS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_UNDERRUNS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_UNDERRUNS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_UNDERRUNS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT_UN` reader - Transmit under runs - a 10 bit register counting the number of frames not transmitted due to a transmit under run. If this register is incremented then no other statistics register is incremented."]
pub type COUNT_UN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit under runs - a 10 bit register counting the number of frames not transmitted due to a transmit under run. If this register is incremented then no other statistics register is incremented."]
    #[inline(always)]
    pub fn count_un(&self) -> COUNT_UN_R {
        COUNT_UN_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Transmit Under Runs\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_underruns](index.html) module"]
pub struct TX_UNDERRUNS_SPEC;
impl crate::RegisterSpec for TX_UNDERRUNS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_underruns::R](R) reader structure"]
impl crate::Readable for TX_UNDERRUNS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_UNDERRUNS to value 0"]
impl crate::Resettable for TX_UNDERRUNS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
