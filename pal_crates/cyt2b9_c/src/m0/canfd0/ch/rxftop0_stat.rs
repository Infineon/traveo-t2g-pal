#[doc = "Register `RXFTOP0_STAT` reader"]
pub struct R(crate::R<RXFTOP0_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFTOP0_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFTOP0_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFTOP0_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F0TA` reader - Current FIFO 0 Top Address. This is a pointer to the next word in the message buffer defined by the FIFO Start Address (FnSA), Get Index (FnGI), the FIFO message size (FnDS) and the message word counter (FnMWC) FnTA = FnSA + FnGI * msg_size\\[FnDS\\]
+ FnMWC"]
pub type F0TA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current FIFO 0 Top Address. This is a pointer to the next word in the message buffer defined by the FIFO Start Address (FnSA), Get Index (FnGI), the FIFO message size (FnDS) and the message word counter (FnMWC) FnTA = FnSA + FnGI * msg_size\\[FnDS\\]
+ FnMWC"]
    #[inline(always)]
    pub fn f0ta(&self) -> F0TA_R {
        F0TA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive FIFO 0 Top Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxftop0_stat](index.html) module"]
pub struct RXFTOP0_STAT_SPEC;
impl crate::RegisterSpec for RXFTOP0_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxftop0_stat::R](R) reader structure"]
impl crate::Readable for RXFTOP0_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFTOP0_STAT to value 0"]
impl crate::Resettable for RXFTOP0_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
