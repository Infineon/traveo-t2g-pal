#[doc = "Register `COUNTER_STATUS` reader"]
pub struct R(crate::R<COUNTER_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTER_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTER_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID` reader - Active counter validity: '0': Invalid. '1': Valid. The COUNTER register field INT32 is only valid when VALID is '1'. The COUNTER_STATUS and COUNTER registers are non-retention registers; i.e. the COUNTER_STATUS and COUNTER registers are reset during DeepSleep power mode. After entering the Active power mode, the Active counter is initialized with the DeepSleep counter. This initialization may take up to 1 clk_lf cycle."]
pub type VALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 31 - Active counter validity: '0': Invalid. '1': Valid. The COUNTER register field INT32 is only valid when VALID is '1'. The COUNTER_STATUS and COUNTER registers are non-retention registers; i.e. the COUNTER_STATUS and COUNTER registers are reset during DeepSleep power mode. After entering the Active power mode, the Active counter is initialized with the DeepSleep counter. This initialization may take up to 1 clk_lf cycle."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Counter status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter_status](index.html) module"]
pub struct COUNTER_STATUS_SPEC;
impl crate::RegisterSpec for COUNTER_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [counter_status::R](R) reader structure"]
impl crate::Readable for COUNTER_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNTER_STATUS to value 0"]
impl crate::Resettable for COUNTER_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
