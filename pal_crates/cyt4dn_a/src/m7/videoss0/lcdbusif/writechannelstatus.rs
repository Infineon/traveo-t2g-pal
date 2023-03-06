#[doc = "Register `WRITECHANNELSTATUS` reader"]
pub struct R(crate::R<WRITECHANNELSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITECHANNELSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITECHANNELSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITECHANNELSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRITECHANNELBUSY` reader - Status indication for write channel interface."]
pub type WRITECHANNELBUSY_R = crate::BitReader<WRITECHANNELBUSY_A>;
#[doc = "Status indication for write channel interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITECHANNELBUSY_A {
    #[doc = "0: No operation."]
    IDLE = 0,
    #[doc = "1: Writeing data."]
    BUSY = 1,
}
impl From<WRITECHANNELBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: WRITECHANNELBUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITECHANNELBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITECHANNELBUSY_A {
        match self.bits {
            false => WRITECHANNELBUSY_A::IDLE,
            true => WRITECHANNELBUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WRITECHANNELBUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == WRITECHANNELBUSY_A::BUSY
    }
}
impl R {
    #[doc = "Bit 0 - Status indication for write channel interface."]
    #[inline(always)]
    pub fn writechannelbusy(&self) -> WRITECHANNELBUSY_R {
        WRITECHANNELBUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Write Channel Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writechannelstatus](index.html) module"]
pub struct WRITECHANNELSTATUS_SPEC;
impl crate::RegisterSpec for WRITECHANNELSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writechannelstatus::R](R) reader structure"]
impl crate::Readable for WRITECHANNELSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRITECHANNELSTATUS to value 0"]
impl crate::Resettable for WRITECHANNELSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
