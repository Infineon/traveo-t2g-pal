#[doc = "Register `INTSTT_CH0` reader"]
pub struct R(crate::R<INTSTT_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTT_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTT_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTT_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INT_GC_FSM` reader - Global Control FSM Error Interrupt Status - Channel 0"]
pub type INT_GC_FSM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Global Control FSM Error Interrupt Status - Channel 0"]
    #[inline(always)]
    pub fn int_gc_fsm(&self) -> INT_GC_FSM_R {
        INT_GC_FSM_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt Status Register - Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstt_ch0](index.html) module"]
pub struct INTSTT_CH0_SPEC;
impl crate::RegisterSpec for INTSTT_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstt_ch0::R](R) reader structure"]
impl crate::Readable for INTSTT_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTT_CH0 to value 0"]
impl crate::Resettable for INTSTT_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
