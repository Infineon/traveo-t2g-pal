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
#[doc = "Field `TASKBUFFERDONE` reader - Each bit corresponds to one task buffer. '1' means that the ExecOffset equals to the StopOffset"]
pub type TASKBUFFERDONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TASKBUFFERSYNC` reader - Each bit corresponds to one task buffer. '1' means that the scheduling of corresponding TB is blocked until some SW or HW state is reached."]
pub type TASKBUFFERSYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCHEDULERACTIVETB` reader - Task buffer beeing executed in scheduler. If scheduler is in idle state, the value of this field is not valid."]
pub type SCHEDULERACTIVETB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCHEDULERIDLE` reader - Scheduler is not processing any instructions and there are no outstanding AXI transactions."]
pub type SCHEDULERIDLE_R = crate::BitReader<bool>;
#[doc = "Field `FETCHEDACTIVETB` reader - Task buffer beeing fetch last in programmer. If programmer is in idle state, the value of this field is not valid."]
pub type FETCHEDACTIVETB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REQUESTEDAXITRANSACTIONS` reader - There are some outstanding axi transactions that have been requested but are not received yet."]
pub type REQUESTEDAXITRANSACTIONS_R = crate::BitReader<bool>;
#[doc = "Field `PROGRAMMERSTATE` reader - Execution state of programmer"]
pub type PROGRAMMERSTATE_R = crate::FieldReader<u8, PROGRAMMERSTATE_A>;
#[doc = "Execution state of programmer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROGRAMMERSTATE_A {
    #[doc = "0: When all fetched instructions have been executed and there are no functions scheduled for the execution"]
    IDLE = 0,
    #[doc = "1: During execution of the SYNC instruction"]
    SYNC = 1,
    #[doc = "2: Execution of instructions"]
    ACTIVE = 2,
    #[doc = "3: There is function scheduled for the execution, but the internal instruction buffer is empty."]
    AXI_WAIT = 3,
}
impl From<PROGRAMMERSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: PROGRAMMERSTATE_A) -> Self {
        variant as _
    }
}
impl PROGRAMMERSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROGRAMMERSTATE_A {
        match self.bits {
            0 => PROGRAMMERSTATE_A::IDLE,
            1 => PROGRAMMERSTATE_A::SYNC,
            2 => PROGRAMMERSTATE_A::ACTIVE,
            3 => PROGRAMMERSTATE_A::AXI_WAIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PROGRAMMERSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == PROGRAMMERSTATE_A::SYNC
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == PROGRAMMERSTATE_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `AXI_WAIT`"]
    #[inline(always)]
    pub fn is_axi_wait(&self) -> bool {
        *self == PROGRAMMERSTATE_A::AXI_WAIT
    }
}
#[doc = "Field `CMDSEQIDLE` reader - CmdSeq is not processing any instructions and there are no outstanding AXI transactions."]
pub type CMDSEQIDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Each bit corresponds to one task buffer. '1' means that the ExecOffset equals to the StopOffset"]
    #[inline(always)]
    pub fn taskbufferdone(&self) -> TASKBUFFERDONE_R {
        TASKBUFFERDONE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Each bit corresponds to one task buffer. '1' means that the scheduling of corresponding TB is blocked until some SW or HW state is reached."]
    #[inline(always)]
    pub fn taskbuffersync(&self) -> TASKBUFFERSYNC_R {
        TASKBUFFERSYNC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Task buffer beeing executed in scheduler. If scheduler is in idle state, the value of this field is not valid."]
    #[inline(always)]
    pub fn scheduleractivetb(&self) -> SCHEDULERACTIVETB_R {
        SCHEDULERACTIVETB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Scheduler is not processing any instructions and there are no outstanding AXI transactions."]
    #[inline(always)]
    pub fn scheduleridle(&self) -> SCHEDULERIDLE_R {
        SCHEDULERIDLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Task buffer beeing fetch last in programmer. If programmer is in idle state, the value of this field is not valid."]
    #[inline(always)]
    pub fn fetchedactivetb(&self) -> FETCHEDACTIVETB_R {
        FETCHEDACTIVETB_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - There are some outstanding axi transactions that have been requested but are not received yet."]
    #[inline(always)]
    pub fn requestedaxitransactions(&self) -> REQUESTEDAXITRANSACTIONS_R {
        REQUESTEDAXITRANSACTIONS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Execution state of programmer"]
    #[inline(always)]
    pub fn programmerstate(&self) -> PROGRAMMERSTATE_R {
        PROGRAMMERSTATE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - CmdSeq is not processing any instructions and there are no outstanding AXI transactions."]
    #[inline(always)]
    pub fn cmdseqidle(&self) -> CMDSEQIDLE_R {
        CMDSEQIDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
