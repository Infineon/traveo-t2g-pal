#[doc = "Register `RXFTOP1_DATA` reader"]
pub struct R(crate::R<RXFTOP1_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFTOP1_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFTOP1_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFTOP1_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F1TD` reader - See F0TD description"]
pub type F1TD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - See F0TD description"]
    #[inline(always)]
    pub fn f1td(&self) -> F1TD_R {
        F1TD_R::new(self.bits)
    }
}
#[doc = "Receive FIFO 1 Top Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop1_data](index.html) module"]
pub struct RXFTOP1_DATA_SPEC;
impl crate::RegisterSpec for RXFTOP1_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxftop1_data::R](R) reader structure"]
impl crate::Readable for RXFTOP1_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFTOP1_DATA to value 0"]
impl crate::Resettable for RXFTOP1_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
