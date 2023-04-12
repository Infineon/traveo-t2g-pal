#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GRP_DONE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type GRP_DONE_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `GRP_CANCELLED_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type GRP_CANCELLED_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `GRP_OVERFLOW_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type GRP_OVERFLOW_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `CH_RANGE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type CH_RANGE_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `CH_PULSE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type CH_PULSE_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `CH_OVERFLOW_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type CH_OVERFLOW_MASKED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn grp_done_masked(&self) -> GRP_DONE_MASKED_R {
        GRP_DONE_MASKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn grp_cancelled_masked(&self) -> GRP_CANCELLED_MASKED_R {
        GRP_CANCELLED_MASKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn grp_overflow_masked(&self) -> GRP_OVERFLOW_MASKED_R {
        GRP_OVERFLOW_MASKED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ch_range_masked(&self) -> CH_RANGE_MASKED_R {
        CH_RANGE_MASKED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ch_pulse_masked(&self) -> CH_PULSE_MASKED_R {
        CH_PULSE_MASKED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ch_overflow_masked(&self) -> CH_OVERFLOW_MASKED_R {
        CH_OVERFLOW_MASKED_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
