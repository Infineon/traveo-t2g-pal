#[doc = "Register `RXFTOP1_STAT` reader"]
pub struct R(crate::R<RXFTOP1_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFTOP1_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFTOP1_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFTOP1_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F1TA` reader - See F0TA description"]
pub type F1TA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - See F0TA description"]
    #[inline(always)]
    pub fn f1ta(&self) -> F1TA_R {
        F1TA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive FIFO 1 Top Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop1_stat](index.html) module"]
pub struct RXFTOP1_STAT_SPEC;
impl crate::RegisterSpec for RXFTOP1_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxftop1_stat::R](R) reader structure"]
impl crate::Readable for RXFTOP1_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFTOP1_STAT to value 0"]
impl crate::Resettable for RXFTOP1_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
