#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LEC` reader - Last Error Code, Set on Read0 The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. 0= No Error: No error occurred since LEC has been reset by successful reception or transmission. 1= Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2= Form Error: A fixed format part of a received frame has the wrong format. 3= AckError: The message transmitted by the M_TTCAN was not acknowledged by another node. 4= Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant. 5= Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0'), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6= CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7= NoChange: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows the value '7', no CAN bus event was detected since the last CPU read access to the Protocol Status Register."]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT` reader - Activity Monitors the module's CAN communication state. 00= Synchronizing - node is synchronizing on CAN communication 01= Idle - node is neither receiver nor transmitter 10= Receiver - node is operating as receiver 11= Transmitter - node is operating as transmitter"]
pub type ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP` reader - Error Passive 0= The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1= The M_CAN is in the Error_Passive state"]
pub type EP_R = crate::BitReader<bool>;
#[doc = "Field `EW` reader - Warning Status 0= Both error counters are below the Error_Warning limit of 96 1= At least one of error counter has reached the Error_Warning limit of 96"]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `BO` reader - Bus_Off Status 0= The M_CAN is not Bus_Off 1= The M_CAN is in Bus_Off state"]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `DLEC` reader - Data Phase Last Error Code , Set on Read Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set. Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error."]
pub type DLEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESI` reader - ESI flag of last received CAN FD Message , Reset on Read This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its ESI flag set 1= Last received CAN FD message had its ESI flag set"]
pub type RESI_R = crate::BitReader<bool>;
#[doc = "Field `RBRS` reader - BRS flag of last received CAN FD Message , Reset on Read This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its BRS flag set 1= Last received CAN FD message had its BRS flag set"]
pub type RBRS_R = crate::BitReader<bool>;
#[doc = "Field `RFDF` reader - Received a CAN FD Message , Reset on Read This bit is set independent of acceptance filtering. 0= Since this bit was reset by the CPU, no CAN FD message has been received 1= Message in CAN FD format with FDF flag set has been received"]
pub type RFDF_R = crate::BitReader<bool>;
#[doc = "Field `PXE` reader - Protocol Exception Event , Reset on Read 0= No protocol exception event occurred since last read access 1= Protocol exception event occurred"]
pub type PXE_R = crate::BitReader<bool>;
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value 0x00-0x7F Position of the secondary sample point, defined by the sum of the measured delay from m_can_tx to m_can_rx and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
pub type TDCV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Last Error Code, Set on Read0 The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to '0' when a message has been transferred (reception or transmission) without error. 0= No Error: No error occurred since LEC has been reset by successful reception or transmission. 1= Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2= Form Error: A fixed format part of a received frame has the wrong format. 3= AckError: The message transmitted by the M_TTCAN was not acknowledged by another node. 4= Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value '1'), but the monitored bus value was dominant. 5= Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0'), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6= CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7= NoChange: Any read access to the Protocol Status Register re-initializes the LEC to '7'. When the LEC shows the value '7', no CAN bus event was detected since the last CPU read access to the Protocol Status Register."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity Monitors the module's CAN communication state. 00= Synchronizing - node is synchronizing on CAN communication 01= Idle - node is neither receiver nor transmitter 10= Receiver - node is operating as receiver 11= Transmitter - node is operating as transmitter"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive 0= The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1= The M_CAN is in the Error_Passive state"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status 0= Both error counters are below the Error_Warning limit of 96 1= At least one of error counter has reached the Error_Warning limit of 96"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status 0= The M_CAN is not Bus_Off 1= The M_CAN is in Bus_Off state"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Phase Last Error Code , Set on Read Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set. Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error."]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received CAN FD Message , Reset on Read This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its ESI flag set 1= Last received CAN FD message had its ESI flag set"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received CAN FD Message , Reset on Read This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its BRS flag set 1= Last received CAN FD message had its BRS flag set"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD Message , Reset on Read This bit is set independent of acceptance filtering. 0= Since this bit was reset by the CPU, no CAN FD message has been received 1= Message in CAN FD format with FDF flag set has been received"]
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event , Reset on Read 0= No protocol exception event occurred since last read access 1= Protocol exception event occurred"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value 0x00-0x7F Position of the secondary sample point, defined by the sum of the measured delay from m_can_tx to m_can_rx and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSR to value 0x0707"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707;
}
