#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUR_CHAN` reader - current channel being acquired, only valid if BUSY."]
pub type CUR_CHAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CUR_PRIO` reader - priority of current group/channel, only valid if BUSY."]
pub type CUR_PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CUR_PREEMPT_TYPE` reader - Preempting type of current group/channel, only valid if BUSY."]
pub type CUR_PREEMPT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBG_FREEZE` reader - If high then the SAR is prevented from starting a new acquisition, see DBG_FREEZE_EN."]
pub type DBG_FREEZE_R = crate::BitReader<bool>;
#[doc = "Field `PWRUP_BUSY` reader - If high then the SAR is waiting for PWRUP_TIME due to IDLE_PWRDWN"]
pub type PWRUP_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - If high then the SAR is busy with a conversion."]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4 - current channel being acquired, only valid if BUSY."]
    #[inline(always)]
    pub fn cur_chan(&self) -> CUR_CHAN_R {
        CUR_CHAN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - priority of current group/channel, only valid if BUSY."]
    #[inline(always)]
    pub fn cur_prio(&self) -> CUR_PRIO_R {
        CUR_PRIO_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Preempting type of current group/channel, only valid if BUSY."]
    #[inline(always)]
    pub fn cur_preempt_type(&self) -> CUR_PREEMPT_TYPE_R {
        CUR_PREEMPT_TYPE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 29 - If high then the SAR is prevented from starting a new acquisition, see DBG_FREEZE_EN."]
    #[inline(always)]
    pub fn dbg_freeze(&self) -> DBG_FREEZE_R {
        DBG_FREEZE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If high then the SAR is waiting for PWRUP_TIME due to IDLE_PWRDWN"]
    #[inline(always)]
    pub fn pwrup_busy(&self) -> PWRUP_BUSY_R {
        PWRUP_BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If high then the SAR is busy with a conversion."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Current status of internal SAR registers (mostly for debug)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
