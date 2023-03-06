#[doc = "Register `READCHANNELSTATUS` reader"]
pub struct R(crate::R<READCHANNELSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READCHANNELSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READCHANNELSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READCHANNELSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READCHANNELBUSY` reader - Status indication for read channel interface."]
pub type READCHANNELBUSY_R = crate::BitReader<READCHANNELBUSY_A>;
#[doc = "Status indication for read channel interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READCHANNELBUSY_A {
    #[doc = "0: No operation."]
    IDLE = 0,
    #[doc = "1: Reading data."]
    BUSY = 1,
}
impl From<READCHANNELBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: READCHANNELBUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl READCHANNELBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READCHANNELBUSY_A {
        match self.bits {
            false => READCHANNELBUSY_A::IDLE,
            true => READCHANNELBUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == READCHANNELBUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == READCHANNELBUSY_A::BUSY
    }
}
#[doc = "Field `READCHANNELWORDSREMAINING` reader - Number of 32-bit words still to be read."]
pub type READCHANNELWORDSREMAINING_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0 - Status indication for read channel interface."]
    #[inline(always)]
    pub fn readchannelbusy(&self) -> READCHANNELBUSY_R {
        READCHANNELBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Number of 32-bit words still to be read."]
    #[inline(always)]
    pub fn readchannelwordsremaining(&self) -> READCHANNELWORDSREMAINING_R {
        READCHANNELWORDSREMAINING_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Read Channel Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readchannelstatus](index.html) module"]
pub struct READCHANNELSTATUS_SPEC;
impl crate::RegisterSpec for READCHANNELSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readchannelstatus::R](R) reader structure"]
impl crate::Readable for READCHANNELSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets READCHANNELSTATUS to value 0"]
impl crate::Resettable for READCHANNELSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
