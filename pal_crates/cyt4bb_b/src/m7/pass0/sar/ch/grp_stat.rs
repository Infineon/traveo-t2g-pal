#[doc = "Register `GRP_STAT` reader"]
pub struct R(crate::R<GRP_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRP_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRP_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRP_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GRP_COMPLETE` reader - Group acquisition complete. This is a copy of the INTR.GRP_DONE bit."]
pub type GRP_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `GRP_CANCELLED` reader - Group Cancelled. This is a copy of the INTR.GRP_CANCELLED bit."]
pub type GRP_CANCELLED_R = crate::BitReader<bool>;
#[doc = "Field `GRP_OVERFLOW` reader - Group Overflow. This is a copy of the INTR.GRP_OVERFLOW bit."]
pub type GRP_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `CH_RANGE_COMPLETE` reader - Channel Range complete. This is a copy of the INTR.CH_RANGE bit."]
pub type CH_RANGE_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `CH_PULSE_COMPLETE` reader - Channel Pulse complete. This is a copy of the INTR.CH_PULSE bit."]
pub type CH_PULSE_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `CH_OVERFLOW` reader - Channel Overflow. This is a copy of the INTR.CH_OVERFLOW bit."]
pub type CH_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `GRP_BUSY` reader - Group acquisition busy. This is a copy of the TR_PENDING bit of the first channel of the group."]
pub type GRP_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Group acquisition complete. This is a copy of the INTR.GRP_DONE bit."]
    #[inline(always)]
    pub fn grp_complete(&self) -> GRP_COMPLETE_R {
        GRP_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Group Cancelled. This is a copy of the INTR.GRP_CANCELLED bit."]
    #[inline(always)]
    pub fn grp_cancelled(&self) -> GRP_CANCELLED_R {
        GRP_CANCELLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Group Overflow. This is a copy of the INTR.GRP_OVERFLOW bit."]
    #[inline(always)]
    pub fn grp_overflow(&self) -> GRP_OVERFLOW_R {
        GRP_OVERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel Range complete. This is a copy of the INTR.CH_RANGE bit."]
    #[inline(always)]
    pub fn ch_range_complete(&self) -> CH_RANGE_COMPLETE_R {
        CH_RANGE_COMPLETE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Pulse complete. This is a copy of the INTR.CH_PULSE bit."]
    #[inline(always)]
    pub fn ch_pulse_complete(&self) -> CH_PULSE_COMPLETE_R {
        CH_PULSE_COMPLETE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Overflow. This is a copy of the INTR.CH_OVERFLOW bit."]
    #[inline(always)]
    pub fn ch_overflow(&self) -> CH_OVERFLOW_R {
        CH_OVERFLOW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Group acquisition busy. This is a copy of the TR_PENDING bit of the first channel of the group."]
    #[inline(always)]
    pub fn grp_busy(&self) -> GRP_BUSY_R {
        GRP_BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Group status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grp_stat](index.html) module"]
pub struct GRP_STAT_SPEC;
impl crate::RegisterSpec for GRP_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grp_stat::R](R) reader structure"]
impl crate::Readable for GRP_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GRP_STAT to value 0"]
impl crate::Resettable for GRP_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
