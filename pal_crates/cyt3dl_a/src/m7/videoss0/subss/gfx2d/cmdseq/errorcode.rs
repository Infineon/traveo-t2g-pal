#[doc = "Register `ERRORCODE` reader"]
pub struct R(crate::R<ERRORCODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRORCODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRORCODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRORCODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERRORSCHEDULEROPCODE` reader - Opcode error in scheduler"]
pub type ERRORSCHEDULEROPCODE_R = crate::BitReader<bool>;
#[doc = "Field `ERRORPROGRAMMEROPCODE` reader - Opcode error in programmer"]
pub type ERRORPROGRAMMEROPCODE_R = crate::BitReader<bool>;
#[doc = "Field `ERROREXECUTIONOFFSET` reader - Execution offset is outside of the task buffer"]
pub type ERROREXECUTIONOFFSET_R = crate::BitReader<bool>;
#[doc = "Field `ERRORSTOPOFFSETWITHININSTRUCTION` reader - Stop offset is within instruction in scheduler"]
pub type ERRORSTOPOFFSETWITHININSTRUCTION_R = crate::BitReader<bool>;
#[doc = "Field `ERRORAXISLV` reader - AXI slave error has been occurred. SchedulerAddress and ProgrammerAddress have to be evaluated"]
pub type ERRORAXISLV_R = crate::BitReader<bool>;
#[doc = "Field `ERRORAXIDEC` reader - AXI decoder error has been occurred. SchedulerAddress and ProgrammerAddress have to be evaluated"]
pub type ERRORAXIDEC_R = crate::BitReader<bool>;
#[doc = "Field `ERRORREGID` reader - Error accessing an register via id that is not bound to any GPR or SPR register"]
pub type ERRORREGID_R = crate::BitReader<bool>;
#[doc = "Field `ERRORDELAYEDSYNC` reader - SYNC is following delayed SYNC in Scheduler"]
pub type ERRORDELAYEDSYNC_R = crate::BitReader<bool>;
#[doc = "Field `ERRORSTOPOFFSETOUTSIDETB` reader - Stop offset value is bigger than or equals to task buffer size. If however both are zero, no error should be raised."]
pub type ERRORSTOPOFFSETOUTSIDETB_R = crate::BitReader<bool>;
#[doc = "Field `ERRORFUNCTIONOFFSET` reader - Function offset provided by the EXEC instruction is outside of task buffer"]
pub type ERRORFUNCTIONOFFSET_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Opcode error in scheduler"]
    #[inline(always)]
    pub fn errorscheduleropcode(&self) -> ERRORSCHEDULEROPCODE_R {
        ERRORSCHEDULEROPCODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Opcode error in programmer"]
    #[inline(always)]
    pub fn errorprogrammeropcode(&self) -> ERRORPROGRAMMEROPCODE_R {
        ERRORPROGRAMMEROPCODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Execution offset is outside of the task buffer"]
    #[inline(always)]
    pub fn errorexecutionoffset(&self) -> ERROREXECUTIONOFFSET_R {
        ERROREXECUTIONOFFSET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop offset is within instruction in scheduler"]
    #[inline(always)]
    pub fn errorstopoffsetwithininstruction(&self) -> ERRORSTOPOFFSETWITHININSTRUCTION_R {
        ERRORSTOPOFFSETWITHININSTRUCTION_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AXI slave error has been occurred. SchedulerAddress and ProgrammerAddress have to be evaluated"]
    #[inline(always)]
    pub fn erroraxislv(&self) -> ERRORAXISLV_R {
        ERRORAXISLV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AXI decoder error has been occurred. SchedulerAddress and ProgrammerAddress have to be evaluated"]
    #[inline(always)]
    pub fn erroraxidec(&self) -> ERRORAXIDEC_R {
        ERRORAXIDEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error accessing an register via id that is not bound to any GPR or SPR register"]
    #[inline(always)]
    pub fn errorregid(&self) -> ERRORREGID_R {
        ERRORREGID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYNC is following delayed SYNC in Scheduler"]
    #[inline(always)]
    pub fn errordelayedsync(&self) -> ERRORDELAYEDSYNC_R {
        ERRORDELAYEDSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop offset value is bigger than or equals to task buffer size. If however both are zero, no error should be raised."]
    #[inline(always)]
    pub fn errorstopoffsetoutsidetb(&self) -> ERRORSTOPOFFSETOUTSIDETB_R {
        ERRORSTOPOFFSETOUTSIDETB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Function offset provided by the EXEC instruction is outside of task buffer"]
    #[inline(always)]
    pub fn errorfunctionoffset(&self) -> ERRORFUNCTIONOFFSET_R {
        ERRORFUNCTIONOFFSET_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Error code register, each field of this register is one bit wide and corresponds to an error event. If the value of this register is zero, than there are no errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorcode](index.html) module"]
pub struct ERRORCODE_SPEC;
impl crate::RegisterSpec for ERRORCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errorcode::R](R) reader structure"]
impl crate::Readable for ERRORCODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERRORCODE to value 0"]
impl crate::Resettable for ERRORCODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
