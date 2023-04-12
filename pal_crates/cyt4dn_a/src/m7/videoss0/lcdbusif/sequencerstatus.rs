#[doc = "Register `SEQUENCERSTATUS` reader"]
pub struct R(crate::R<SEQUENCERSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQUENCERSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQUENCERSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQUENCERSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OPERATIONSTATE` reader - Indicate operation status."]
pub type OPERATIONSTATE_R = crate::FieldReader<u8, OPERATIONSTATE_A>;
#[doc = "Indicate operation status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPERATIONSTATE_A {
    #[doc = "0: Processing finished, nothing to do."]
    IDLE = 0,
    #[doc = "1: Currently processing instruction list."]
    BUSY = 1,
    #[doc = "2: Waiting for next instruction or data."]
    WAIT = 2,
    #[doc = "3: An error has occured"]
    ERROR = 3,
}
impl From<OPERATIONSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPERATIONSTATE_A) -> Self {
        variant as _
    }
}
impl OPERATIONSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPERATIONSTATE_A {
        match self.bits {
            0 => OPERATIONSTATE_A::IDLE,
            1 => OPERATIONSTATE_A::BUSY,
            2 => OPERATIONSTATE_A::WAIT,
            3 => OPERATIONSTATE_A::ERROR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == OPERATIONSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == OPERATIONSTATE_A::BUSY
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == OPERATIONSTATE_A::WAIT
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPERATIONSTATE_A::ERROR
    }
}
#[doc = "Field `WAITACK` reader - Indicate if a WAIT_ACK instruction is currently pending."]
pub type WAITACK_R = crate::BitReader<WAITACK_A>;
#[doc = "Indicate if a WAIT_ACK instruction is currently pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITACK_A {
    #[doc = "0: No waiting for acknowledge."]
    DONE = 0,
    #[doc = "1: Acknowledge from CPU is pending."]
    PENDING = 1,
}
impl From<WAITACK_A> for bool {
    #[inline(always)]
    fn from(variant: WAITACK_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITACK_A {
        match self.bits {
            false => WAITACK_A::DONE,
            true => WAITACK_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == WAITACK_A::DONE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == WAITACK_A::PENDING
    }
}
#[doc = "Field `ERRORCODE` reader - Type of error that has occured"]
pub type ERRORCODE_R = crate::FieldReader<u8, ERRORCODE_A>;
#[doc = "Type of error that has occured\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERRORCODE_A {
    #[doc = "0: No error."]
    NONE = 0,
    #[doc = "1: Illegal opcode error."]
    OPCODE = 1,
    #[doc = "2: Too many outstanding kicks error."]
    KICK = 2,
    #[doc = "3: Sequencer timout reached."]
    TIMEOUT = 3,
}
impl From<ERRORCODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERRORCODE_A) -> Self {
        variant as _
    }
}
impl ERRORCODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERRORCODE_A> {
        match self.bits {
            0 => Some(ERRORCODE_A::NONE),
            1 => Some(ERRORCODE_A::OPCODE),
            2 => Some(ERRORCODE_A::KICK),
            3 => Some(ERRORCODE_A::TIMEOUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ERRORCODE_A::NONE
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        *self == ERRORCODE_A::OPCODE
    }
    #[doc = "Checks if the value of the field is `KICK`"]
    #[inline(always)]
    pub fn is_kick(&self) -> bool {
        *self == ERRORCODE_A::KICK
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == ERRORCODE_A::TIMEOUT
    }
}
#[doc = "Field `INSTRUCTIONCOUNTER` reader - Number of instructions processed in current command list."]
pub type INSTRUCTIONCOUNTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTRUCTIONREGISTER` reader - Current instruction executed by sequencer."]
pub type INSTRUCTIONREGISTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRISINTERFACESTATUS` reader - Status of Iris interface."]
pub type IRISINTERFACESTATUS_R = crate::FieldReader<u8, IRISINTERFACESTATUS_A>;
#[doc = "Status of Iris interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IRISINTERFACESTATUS_A {
    #[doc = "0: Nothing pending."]
    IDLE = 0,
    #[doc = "1: Kick sent, frame pending."]
    KICK = 1,
    #[doc = "2: Command received, pixel transmission pending."]
    CMD = 2,
    #[doc = "3: Frame transmission ongoing."]
    FRAME = 3,
}
impl From<IRISINTERFACESTATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: IRISINTERFACESTATUS_A) -> Self {
        variant as _
    }
}
impl IRISINTERFACESTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRISINTERFACESTATUS_A {
        match self.bits {
            0 => IRISINTERFACESTATUS_A::IDLE,
            1 => IRISINTERFACESTATUS_A::KICK,
            2 => IRISINTERFACESTATUS_A::CMD,
            3 => IRISINTERFACESTATUS_A::FRAME,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IRISINTERFACESTATUS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `KICK`"]
    #[inline(always)]
    pub fn is_kick(&self) -> bool {
        *self == IRISINTERFACESTATUS_A::KICK
    }
    #[doc = "Checks if the value of the field is `CMD`"]
    #[inline(always)]
    pub fn is_cmd(&self) -> bool {
        *self == IRISINTERFACESTATUS_A::CMD
    }
    #[doc = "Checks if the value of the field is `FRAME`"]
    #[inline(always)]
    pub fn is_frame(&self) -> bool {
        *self == IRISINTERFACESTATUS_A::FRAME
    }
}
#[doc = "Field `LCDINTERFACESTATE` reader - Indicate LCDBus status."]
pub type LCDINTERFACESTATE_R = crate::BitReader<LCDINTERFACESTATE_A>;
#[doc = "Indicate LCDBus status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDINTERFACESTATE_A {
    #[doc = "0: No transactions pending."]
    IDLE = 0,
    #[doc = "1: Currently sending transactions."]
    BUSY = 1,
}
impl From<LCDINTERFACESTATE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDINTERFACESTATE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDINTERFACESTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDINTERFACESTATE_A {
        match self.bits {
            false => LCDINTERFACESTATE_A::IDLE,
            true => LCDINTERFACESTATE_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == LCDINTERFACESTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == LCDINTERFACESTATE_A::BUSY
    }
}
impl R {
    #[doc = "Bits 0:1 - Indicate operation status."]
    #[inline(always)]
    pub fn operationstate(&self) -> OPERATIONSTATE_R {
        OPERATIONSTATE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Indicate if a WAIT_ACK instruction is currently pending."]
    #[inline(always)]
    pub fn waitack(&self) -> WAITACK_R {
        WAITACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Type of error that has occured"]
    #[inline(always)]
    pub fn errorcode(&self) -> ERRORCODE_R {
        ERRORCODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Number of instructions processed in current command list."]
    #[inline(always)]
    pub fn instructioncounter(&self) -> INSTRUCTIONCOUNTER_R {
        INSTRUCTIONCOUNTER_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Current instruction executed by sequencer."]
    #[inline(always)]
    pub fn instructionregister(&self) -> INSTRUCTIONREGISTER_R {
        INSTRUCTIONREGISTER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Status of Iris interface."]
    #[inline(always)]
    pub fn irisinterfacestatus(&self) -> IRISINTERFACESTATUS_R {
        IRISINTERFACESTATUS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Indicate LCDBus status."]
    #[inline(always)]
    pub fn lcdinterfacestate(&self) -> LCDINTERFACESTATE_R {
        LCDINTERFACESTATE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Status of the internal sequencer.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sequencerstatus](index.html) module"]
pub struct SEQUENCERSTATUS_SPEC;
impl crate::RegisterSpec for SEQUENCERSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sequencerstatus::R](R) reader structure"]
impl crate::Readable for SEQUENCERSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SEQUENCERSTATUS to value 0"]
impl crate::Resettable for SEQUENCERSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
