#[doc = "Register `FSR` reader"]
pub struct R(crate::R<FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFNE_` reader - Receive FIFO Not Empty This flag is set by the CC when a received valid frame (data or null frame depending on rejection mask) was stored in the FIFO. In addition, interrupt flag SIR.RFNE is set. The bit is reset after the Host has read all message from the FIFO. 1 = Receive FIFO is not empty 0 = Receive FIFO is empty"]
pub type RFNE__R = crate::BitReader<RFNE__A>;
#[doc = "Receive FIFO Not Empty This flag is set by the CC when a received valid frame (data or null frame depending on rejection mask) was stored in the FIFO. In addition, interrupt flag SIR.RFNE is set. The bit is reset after the Host has read all message from the FIFO. 1 = Receive FIFO is not empty 0 = Receive FIFO is empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFNE__A {
    #[doc = "0: N/A"]
    RX_FIFO_EMPTY = 0,
    #[doc = "1: N/A"]
    RX_FIFO_NOT_EMPTY = 1,
}
impl From<RFNE__A> for bool {
    #[inline(always)]
    fn from(variant: RFNE__A) -> Self {
        variant as u8 != 0
    }
}
impl RFNE__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFNE__A {
        match self.bits {
            false => RFNE__A::RX_FIFO_EMPTY,
            true => RFNE__A::RX_FIFO_NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_EMPTY`"]
    #[inline(always)]
    pub fn is_rx_fifo_empty(&self) -> bool {
        *self == RFNE__A::RX_FIFO_EMPTY
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_rx_fifo_not_empty(&self) -> bool {
        *self == RFNE__A::RX_FIFO_NOT_EMPTY
    }
}
#[doc = "Field `RFCL_` reader - Receive FIFO Critical Level This flag is set when the receive FIFO fill level RFFL\\[7:0\\]
is equal or greater than the critical level as configured by FCL.CL\\[7:0\\]. The flag is cleared by the CC as soon as RFFL\\[7:0\\]
drops below FCL.CL\\[7:0\\]. When RFCL changes from '0' to '1' bit SIR.RFCL is set to '1', and if enabled, an interrupt is generated. 1 = Receive FIFO critical level reached 0 = Receive FIFO below critical level"]
pub type RFCL__R = crate::BitReader<RFCL__A>;
#[doc = "Receive FIFO Critical Level This flag is set when the receive FIFO fill level RFFL\\[7:0\\]
is equal or greater than the critical level as configured by FCL.CL\\[7:0\\]. The flag is cleared by the CC as soon as RFFL\\[7:0\\]
drops below FCL.CL\\[7:0\\]. When RFCL changes from '0' to '1' bit SIR.RFCL is set to '1', and if enabled, an interrupt is generated. 1 = Receive FIFO critical level reached 0 = Receive FIFO below critical level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCL__A {
    #[doc = "0: N/A"]
    RX_FIFO_LT_CRITICAL_LEVEL = 0,
    #[doc = "1: N/A"]
    RX_FIFO_CRITICAL_LEVEL = 1,
}
impl From<RFCL__A> for bool {
    #[inline(always)]
    fn from(variant: RFCL__A) -> Self {
        variant as u8 != 0
    }
}
impl RFCL__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCL__A {
        match self.bits {
            false => RFCL__A::RX_FIFO_LT_CRITICAL_LEVEL,
            true => RFCL__A::RX_FIFO_CRITICAL_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_LT_CRITICAL_LEVEL`"]
    #[inline(always)]
    pub fn is_rx_fifo_lt_critical_level(&self) -> bool {
        *self == RFCL__A::RX_FIFO_LT_CRITICAL_LEVEL
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_CRITICAL_LEVEL`"]
    #[inline(always)]
    pub fn is_rx_fifo_critical_level(&self) -> bool {
        *self == RFCL__A::RX_FIFO_CRITICAL_LEVEL
    }
}
#[doc = "Field `RFO_` reader - Receive FIFO Overrun The flag is set by the CC when a receive FIFO overrun is detected. When a receive FIFO overrun occurs, the oldest message is overwritten with the actual received message. In addition, interrupt flag EIR.RFO is set.The flag is cleared by the next FIFO read access issued by the Host. 1 = A receive FIFO overrun has been detected 0 = No receive FIFO overrun detected"]
pub type RFO__R = crate::BitReader<RFO__A>;
#[doc = "Receive FIFO Overrun The flag is set by the CC when a receive FIFO overrun is detected. When a receive FIFO overrun occurs, the oldest message is overwritten with the actual received message. In addition, interrupt flag EIR.RFO is set.The flag is cleared by the next FIFO read access issued by the Host. 1 = A receive FIFO overrun has been detected 0 = No receive FIFO overrun detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFO__A {
    #[doc = "0: N/A"]
    RX_FIFO_NO_OVERRUN = 0,
    #[doc = "1: N/A"]
    RX_FIFO_OVERRUN = 1,
}
impl From<RFO__A> for bool {
    #[inline(always)]
    fn from(variant: RFO__A) -> Self {
        variant as u8 != 0
    }
}
impl RFO__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFO__A {
        match self.bits {
            false => RFO__A::RX_FIFO_NO_OVERRUN,
            true => RFO__A::RX_FIFO_OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_rx_fifo_no_overrun(&self) -> bool {
        *self == RFO__A::RX_FIFO_NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_OVERRUN`"]
    #[inline(always)]
    pub fn is_rx_fifo_overrun(&self) -> bool {
        *self == RFO__A::RX_FIFO_OVERRUN
    }
}
#[doc = "Field `RFFL` reader - Receive FIFO Fill Level Number of FIFO buffers filled with new data not yet read by the Host. Maximum value is 128."]
pub type RFFL_R = crate::FieldReader<u8, RFFL_A>;
#[doc = "Receive FIFO Fill Level Number of FIFO buffers filled with new data not yet read by the Host. Maximum value is 128.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFFL_A {
    #[doc = "128: N/A"]
    MAX = 128,
}
impl From<RFFL_A> for u8 {
    #[inline(always)]
    fn from(variant: RFFL_A) -> Self {
        variant as _
    }
}
impl RFFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFFL_A> {
        match self.bits {
            128 => Some(RFFL_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == RFFL_A::MAX
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Not Empty This flag is set by the CC when a received valid frame (data or null frame depending on rejection mask) was stored in the FIFO. In addition, interrupt flag SIR.RFNE is set. The bit is reset after the Host has read all message from the FIFO. 1 = Receive FIFO is not empty 0 = Receive FIFO is empty"]
    #[inline(always)]
    pub fn rfne_(&self) -> RFNE__R {
        RFNE__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO Critical Level This flag is set when the receive FIFO fill level RFFL\\[7:0\\]
is equal or greater than the critical level as configured by FCL.CL\\[7:0\\]. The flag is cleared by the CC as soon as RFFL\\[7:0\\]
drops below FCL.CL\\[7:0\\]. When RFCL changes from '0' to '1' bit SIR.RFCL is set to '1', and if enabled, an interrupt is generated. 1 = Receive FIFO critical level reached 0 = Receive FIFO below critical level"]
    #[inline(always)]
    pub fn rfcl_(&self) -> RFCL__R {
        RFCL__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Overrun The flag is set by the CC when a receive FIFO overrun is detected. When a receive FIFO overrun occurs, the oldest message is overwritten with the actual received message. In addition, interrupt flag EIR.RFO is set.The flag is cleared by the next FIFO read access issued by the Host. 1 = A receive FIFO overrun has been detected 0 = No receive FIFO overrun detected"]
    #[inline(always)]
    pub fn rfo_(&self) -> RFO__R {
        RFO__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Receive FIFO Fill Level Number of FIFO buffers filled with new data not yet read by the Host. Maximum value is 128."]
    #[inline(always)]
    pub fn rffl(&self) -> RFFL_R {
        RFFL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](index.html) module"]
pub struct FSR_SPEC;
impl crate::RegisterSpec for FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsr::R](R) reader structure"]
impl crate::Readable for FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
