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
#[doc = "Field `STATUSBUSY` reader - Indicates if Drawing Engine is busy with drawing a command list."]
pub type STATUSBUSY_R = crate::BitReader<bool>;
#[doc = "Field `WAITACK` reader - Indicates if Drawing Engine is waiting on an read acknowledge."]
pub type WAITACK_R = crate::BitReader<bool>;
#[doc = "Field `STATUSDIRTY` reader - Indicates if the linebuffer memory requires a clear prior to the next drawing operation."]
pub type STATUSDIRTY_R = crate::BitReader<bool>;
#[doc = "Field `STARTPENDING` reader - Indicates if another start trigger is pending."]
pub type STARTPENDING_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates if Drawing Engine is busy with drawing a command list."]
    #[inline(always)]
    pub fn statusbusy(&self) -> STATUSBUSY_R {
        STATUSBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if Drawing Engine is waiting on an read acknowledge."]
    #[inline(always)]
    pub fn waitack(&self) -> WAITACK_R {
        WAITACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if the linebuffer memory requires a clear prior to the next drawing operation."]
    #[inline(always)]
    pub fn statusdirty(&self) -> STATUSDIRTY_R {
        STATUSDIRTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates if another start trigger is pending."]
    #[inline(always)]
    pub fn startpending(&self) -> STARTPENDING_R {
        STARTPENDING_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status bits.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x04"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
