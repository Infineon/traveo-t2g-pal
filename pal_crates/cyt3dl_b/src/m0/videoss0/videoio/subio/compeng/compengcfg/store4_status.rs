#[doc = "Register `STORE4_STATUS` reader"]
pub struct R(crate::R<STORE4_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE4_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE4_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE4_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STORE4_PIPELINE_STATUS` reader - Status of pipeline with endpoint store4"]
pub type STORE4_PIPELINE_STATUS_R = crate::FieldReader<u8, STORE4_PIPELINE_STATUS_A>;
#[doc = "Status of pipeline with endpoint store4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STORE4_PIPELINE_STATUS_A {
    #[doc = "0: Pipeline with endpoint store4 is empty"]
    EMPTY = 0,
    #[doc = "1: Pipeline with endpoint store4 is currently processing one operation"]
    RUNNING = 1,
    #[doc = "2: Pipeline with endpoint store4 is currently processing one operation with a second one already kicked to be processed afterwards"]
    RUNNING_RETRIGGERED = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<STORE4_PIPELINE_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: STORE4_PIPELINE_STATUS_A) -> Self {
        variant as _
    }
}
impl STORE4_PIPELINE_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STORE4_PIPELINE_STATUS_A {
        match self.bits {
            0 => STORE4_PIPELINE_STATUS_A::EMPTY,
            1 => STORE4_PIPELINE_STATUS_A::RUNNING,
            2 => STORE4_PIPELINE_STATUS_A::RUNNING_RETRIGGERED,
            3 => STORE4_PIPELINE_STATUS_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == STORE4_PIPELINE_STATUS_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STORE4_PIPELINE_STATUS_A::RUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING_RETRIGGERED`"]
    #[inline(always)]
    pub fn is_running_retriggered(&self) -> bool {
        *self == STORE4_PIPELINE_STATUS_A::RUNNING_RETRIGGERED
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == STORE4_PIPELINE_STATUS_A::RSVD
    }
}
#[doc = "Field `STORE4_SYNC_BUSY` reader - Synchronization busy status of store4 endpoint"]
pub type STORE4_SYNC_BUSY_R = crate::BitReader<STORE4_SYNC_BUSY_A>;
#[doc = "Synchronization busy status of store4 endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STORE4_SYNC_BUSY_A {
    #[doc = "0: store4 synchronizer is idle"]
    IDLE = 0,
    #[doc = "1: store4 synchronizer is busy"]
    BUSY = 1,
}
impl From<STORE4_SYNC_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: STORE4_SYNC_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl STORE4_SYNC_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STORE4_SYNC_BUSY_A {
        match self.bits {
            false => STORE4_SYNC_BUSY_A::IDLE,
            true => STORE4_SYNC_BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STORE4_SYNC_BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == STORE4_SYNC_BUSY_A::BUSY
    }
}
impl R {
    #[doc = "Bits 0:1 - Status of pipeline with endpoint store4"]
    #[inline(always)]
    pub fn store4_pipeline_status(&self) -> STORE4_PIPELINE_STATUS_R {
        STORE4_PIPELINE_STATUS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronization busy status of store4 endpoint"]
    #[inline(always)]
    pub fn store4_sync_busy(&self) -> STORE4_SYNC_BUSY_R {
        STORE4_SYNC_BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Status information for pixel engine configuration of store4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store4_status](index.html) module"]
pub struct STORE4_STATUS_SPEC;
impl crate::RegisterSpec for STORE4_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store4_status::R](R) reader structure"]
impl crate::Readable for STORE4_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STORE4_STATUS to value 0"]
impl crate::Resettable for STORE4_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
