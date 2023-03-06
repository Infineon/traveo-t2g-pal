#[doc = "Register `EXTDST4_STATUS` reader"]
pub struct R(crate::R<EXTDST4_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTDST4_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTDST4_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTDST4_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTDST4_PIPELINE_STATUS` reader - Status of pipeline with endpoint extdst4"]
pub type EXTDST4_PIPELINE_STATUS_R = crate::FieldReader<u8, EXTDST4_PIPELINE_STATUS_A>;
#[doc = "Status of pipeline with endpoint extdst4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTDST4_PIPELINE_STATUS_A {
    #[doc = "0: Pipeline with endpoint extdst4 is empty"]
    EMPTY = 0,
    #[doc = "1: Pipeline with endpoint extdst4 is currently processing one operation"]
    RUNNING = 1,
    #[doc = "2: Pipeline with endpoint extdst4 is currently processing one operation with a second one already kicked to be processed afterwards"]
    RUNNING_RETRIGGERED = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<EXTDST4_PIPELINE_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTDST4_PIPELINE_STATUS_A) -> Self {
        variant as _
    }
}
impl EXTDST4_PIPELINE_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDST4_PIPELINE_STATUS_A {
        match self.bits {
            0 => EXTDST4_PIPELINE_STATUS_A::EMPTY,
            1 => EXTDST4_PIPELINE_STATUS_A::RUNNING,
            2 => EXTDST4_PIPELINE_STATUS_A::RUNNING_RETRIGGERED,
            3 => EXTDST4_PIPELINE_STATUS_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == EXTDST4_PIPELINE_STATUS_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == EXTDST4_PIPELINE_STATUS_A::RUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING_RETRIGGERED`"]
    #[inline(always)]
    pub fn is_running_retriggered(&self) -> bool {
        *self == EXTDST4_PIPELINE_STATUS_A::RUNNING_RETRIGGERED
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == EXTDST4_PIPELINE_STATUS_A::RSVD
    }
}
#[doc = "Field `EXTDST4_SYNC_BUSY` reader - Synchronization busy status of extdst4 endpoint"]
pub type EXTDST4_SYNC_BUSY_R = crate::BitReader<EXTDST4_SYNC_BUSY_A>;
#[doc = "Synchronization busy status of extdst4 endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTDST4_SYNC_BUSY_A {
    #[doc = "0: extdst4 synchronizer is idle"]
    IDLE = 0,
    #[doc = "1: extdst4 synchronizer is busy"]
    BUSY = 1,
}
impl From<EXTDST4_SYNC_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: EXTDST4_SYNC_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTDST4_SYNC_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDST4_SYNC_BUSY_A {
        match self.bits {
            false => EXTDST4_SYNC_BUSY_A::IDLE,
            true => EXTDST4_SYNC_BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == EXTDST4_SYNC_BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == EXTDST4_SYNC_BUSY_A::BUSY
    }
}
impl R {
    #[doc = "Bits 0:1 - Status of pipeline with endpoint extdst4"]
    #[inline(always)]
    pub fn extdst4_pipeline_status(&self) -> EXTDST4_PIPELINE_STATUS_R {
        EXTDST4_PIPELINE_STATUS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronization busy status of extdst4 endpoint"]
    #[inline(always)]
    pub fn extdst4_sync_busy(&self) -> EXTDST4_SYNC_BUSY_R {
        EXTDST4_SYNC_BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Status information for pixel engine configuration of extdst4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extdst4_status](index.html) module"]
pub struct EXTDST4_STATUS_SPEC;
impl crate::RegisterSpec for EXTDST4_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extdst4_status::R](R) reader structure"]
impl crate::Readable for EXTDST4_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTDST4_STATUS to value 0"]
impl crate::Resettable for EXTDST4_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
