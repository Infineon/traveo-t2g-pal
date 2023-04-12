#[doc = "Register `SIR` reader"]
pub struct R(crate::R<SIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIR` writer"]
pub struct W(crate::W<SIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WST` reader - Wakeup Status This flag is set when CCSV.WSV\\[2:0\\]
changes to a value other than UNDEFINED. 1 = Wakeup status changed 0 = Wakeup status unchanged"]
pub type WST_R = crate::BitReader<WST_A>;
#[doc = "Wakeup Status This flag is set when CCSV.WSV\\[2:0\\]
changes to a value other than UNDEFINED. 1 = Wakeup status changed 0 = Wakeup status unchanged\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WST_A {
    #[doc = "0: N/A"]
    WAKEUP_STATUS_UNCHANGED = 0,
    #[doc = "1: N/A"]
    WAKEUP_STATUS_CHANGED = 1,
}
impl From<WST_A> for bool {
    #[inline(always)]
    fn from(variant: WST_A) -> Self {
        variant as u8 != 0
    }
}
impl WST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WST_A {
        match self.bits {
            false => WST_A::WAKEUP_STATUS_UNCHANGED,
            true => WST_A::WAKEUP_STATUS_CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP_STATUS_UNCHANGED`"]
    #[inline(always)]
    pub fn is_wakeup_status_unchanged(&self) -> bool {
        *self == WST_A::WAKEUP_STATUS_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `WAKEUP_STATUS_CHANGED`"]
    #[inline(always)]
    pub fn is_wakeup_status_changed(&self) -> bool {
        *self == WST_A::WAKEUP_STATUS_CHANGED
    }
}
#[doc = "Field `WST` writer - Wakeup Status This flag is set when CCSV.WSV\\[2:0\\]
changes to a value other than UNDEFINED. 1 = Wakeup status changed 0 = Wakeup status unchanged"]
pub type WST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, WST_A, O>;
impl<'a, const O: u8> WST_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn wakeup_status_unchanged(self) -> &'a mut W {
        self.variant(WST_A::WAKEUP_STATUS_UNCHANGED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn wakeup_status_changed(self) -> &'a mut W {
        self.variant(WST_A::WAKEUP_STATUS_CHANGED)
    }
}
#[doc = "Field `CAS` reader - Collision Avoidance Symbol This flag is set by the CC during STARTUP state when a CAS or a potential CAS was received. 1 = Bit pattern matching the CAS symbol received 0 = No bit pattern matching the CAS symbol received"]
pub type CAS_R = crate::BitReader<CAS_A>;
#[doc = "Collision Avoidance Symbol This flag is set by the CC during STARTUP state when a CAS or a potential CAS was received. 1 = Bit pattern matching the CAS symbol received 0 = No bit pattern matching the CAS symbol received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAS_A {
    #[doc = "0: N/A"]
    NO_CAS_BIT_PATTERN_MATCH = 0,
    #[doc = "1: N/A"]
    CAS_BIT_PATTERN_MATCH = 1,
}
impl From<CAS_A> for bool {
    #[inline(always)]
    fn from(variant: CAS_A) -> Self {
        variant as u8 != 0
    }
}
impl CAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAS_A {
        match self.bits {
            false => CAS_A::NO_CAS_BIT_PATTERN_MATCH,
            true => CAS_A::CAS_BIT_PATTERN_MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CAS_BIT_PATTERN_MATCH`"]
    #[inline(always)]
    pub fn is_no_cas_bit_pattern_match(&self) -> bool {
        *self == CAS_A::NO_CAS_BIT_PATTERN_MATCH
    }
    #[doc = "Checks if the value of the field is `CAS_BIT_PATTERN_MATCH`"]
    #[inline(always)]
    pub fn is_cas_bit_pattern_match(&self) -> bool {
        *self == CAS_A::CAS_BIT_PATTERN_MATCH
    }
}
#[doc = "Field `CAS` writer - Collision Avoidance Symbol This flag is set by the CC during STARTUP state when a CAS or a potential CAS was received. 1 = Bit pattern matching the CAS symbol received 0 = No bit pattern matching the CAS symbol received"]
pub type CAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, CAS_A, O>;
impl<'a, const O: u8> CAS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_cas_bit_pattern_match(self) -> &'a mut W {
        self.variant(CAS_A::NO_CAS_BIT_PATTERN_MATCH)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn cas_bit_pattern_match(self) -> &'a mut W {
        self.variant(CAS_A::CAS_BIT_PATTERN_MATCH)
    }
}
#[doc = "Field `CYCS` reader - Cycle Start Interrupt This flag is set by the CC when a communication cycle starts. 1 = Communication cycle started 0 = No communication cycle started"]
pub type CYCS_R = crate::BitReader<CYCS_A>;
#[doc = "Cycle Start Interrupt This flag is set by the CC when a communication cycle starts. 1 = Communication cycle started 0 = No communication cycle started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCS_A {
    #[doc = "0: N/A"]
    NO_COMM_CYCLE_STARTED = 0,
    #[doc = "1: N/A"]
    COMM_CYCLE_STARTED = 1,
}
impl From<CYCS_A> for bool {
    #[inline(always)]
    fn from(variant: CYCS_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCS_A {
        match self.bits {
            false => CYCS_A::NO_COMM_CYCLE_STARTED,
            true => CYCS_A::COMM_CYCLE_STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_COMM_CYCLE_STARTED`"]
    #[inline(always)]
    pub fn is_no_comm_cycle_started(&self) -> bool {
        *self == CYCS_A::NO_COMM_CYCLE_STARTED
    }
    #[doc = "Checks if the value of the field is `COMM_CYCLE_STARTED`"]
    #[inline(always)]
    pub fn is_comm_cycle_started(&self) -> bool {
        *self == CYCS_A::COMM_CYCLE_STARTED
    }
}
#[doc = "Field `CYCS` writer - Cycle Start Interrupt This flag is set by the CC when a communication cycle starts. 1 = Communication cycle started 0 = No communication cycle started"]
pub type CYCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, CYCS_A, O>;
impl<'a, const O: u8> CYCS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_comm_cycle_started(self) -> &'a mut W {
        self.variant(CYCS_A::NO_COMM_CYCLE_STARTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn comm_cycle_started(self) -> &'a mut W {
        self.variant(CYCS_A::COMM_CYCLE_STARTED)
    }
}
#[doc = "Field `TXI` reader - Transmit Interrupt This flag is set by the CC at the end of frame transmission if bit MBI in the respective message buffer is set to '1' (see Table 17). 1 = At least one frame was transmitted from a transmit buffer with MBI = '1' 0 = No frame transmitted from a transmit buffer with MBI = '1'"]
pub type TXI_R = crate::BitReader<TXI_A>;
#[doc = "Transmit Interrupt This flag is set by the CC at the end of frame transmission if bit MBI in the respective message buffer is set to '1' (see Table 17). 1 = At least one frame was transmitted from a transmit buffer with MBI = '1' 0 = No frame transmitted from a transmit buffer with MBI = '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXI_A {
    #[doc = "0: N/A"]
    NO_FRAME_TXED_FROM_TX_MBI_BUFF = 0,
    #[doc = "1: N/A"]
    FRAME_TXED_FROM_TX_MBI_BUFF = 1,
}
impl From<TXI_A> for bool {
    #[inline(always)]
    fn from(variant: TXI_A) -> Self {
        variant as u8 != 0
    }
}
impl TXI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXI_A {
        match self.bits {
            false => TXI_A::NO_FRAME_TXED_FROM_TX_MBI_BUFF,
            true => TXI_A::FRAME_TXED_FROM_TX_MBI_BUFF,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FRAME_TXED_FROM_TX_MBI_BUFF`"]
    #[inline(always)]
    pub fn is_no_frame_txed_from_tx_mbi_buff(&self) -> bool {
        *self == TXI_A::NO_FRAME_TXED_FROM_TX_MBI_BUFF
    }
    #[doc = "Checks if the value of the field is `FRAME_TXED_FROM_TX_MBI_BUFF`"]
    #[inline(always)]
    pub fn is_frame_txed_from_tx_mbi_buff(&self) -> bool {
        *self == TXI_A::FRAME_TXED_FROM_TX_MBI_BUFF
    }
}
#[doc = "Field `TXI` writer - Transmit Interrupt This flag is set by the CC at the end of frame transmission if bit MBI in the respective message buffer is set to '1' (see Table 17). 1 = At least one frame was transmitted from a transmit buffer with MBI = '1' 0 = No frame transmitted from a transmit buffer with MBI = '1'"]
pub type TXI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, TXI_A, O>;
impl<'a, const O: u8> TXI_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_frame_txed_from_tx_mbi_buff(self) -> &'a mut W {
        self.variant(TXI_A::NO_FRAME_TXED_FROM_TX_MBI_BUFF)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn frame_txed_from_tx_mbi_buff(self) -> &'a mut W {
        self.variant(TXI_A::FRAME_TXED_FROM_TX_MBI_BUFF)
    }
}
#[doc = "Field `RXI` reader - Receive Interrupt This flag is set by the CC whenever the set condition of a message buffers ND flag is fulfilled see 4.8.6 New Data 1/2/3/4 (NDAT1/2/3/4)), and if bit MBI of that message buffer is set to '1' see Table 17). 1 = At least one ND flag of a receive buffer with MBI = '1' has been set to '1' 0 = No ND flag of a receive buffer with MBI = '1' has been set to '1'"]
pub type RXI_R = crate::BitReader<RXI_A>;
#[doc = "Receive Interrupt This flag is set by the CC whenever the set condition of a message buffers ND flag is fulfilled see 4.8.6 New Data 1/2/3/4 (NDAT1/2/3/4)), and if bit MBI of that message buffer is set to '1' see Table 17). 1 = At least one ND flag of a receive buffer with MBI = '1' has been set to '1' 0 = No ND flag of a receive buffer with MBI = '1' has been set to '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXI_A {
    #[doc = "0: N/A"]
    NO_ND_FLAG_OF_RX_MBI_BUFF_SET = 0,
    #[doc = "1: N/A"]
    ND_FLAG_OF_RX_MBI_BUFF_SET = 1,
}
impl From<RXI_A> for bool {
    #[inline(always)]
    fn from(variant: RXI_A) -> Self {
        variant as u8 != 0
    }
}
impl RXI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXI_A {
        match self.bits {
            false => RXI_A::NO_ND_FLAG_OF_RX_MBI_BUFF_SET,
            true => RXI_A::ND_FLAG_OF_RX_MBI_BUFF_SET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ND_FLAG_OF_RX_MBI_BUFF_SET`"]
    #[inline(always)]
    pub fn is_no_nd_flag_of_rx_mbi_buff_set(&self) -> bool {
        *self == RXI_A::NO_ND_FLAG_OF_RX_MBI_BUFF_SET
    }
    #[doc = "Checks if the value of the field is `ND_FLAG_OF_RX_MBI_BUFF_SET`"]
    #[inline(always)]
    pub fn is_nd_flag_of_rx_mbi_buff_set(&self) -> bool {
        *self == RXI_A::ND_FLAG_OF_RX_MBI_BUFF_SET
    }
}
#[doc = "Field `RXI` writer - Receive Interrupt This flag is set by the CC whenever the set condition of a message buffers ND flag is fulfilled see 4.8.6 New Data 1/2/3/4 (NDAT1/2/3/4)), and if bit MBI of that message buffer is set to '1' see Table 17). 1 = At least one ND flag of a receive buffer with MBI = '1' has been set to '1' 0 = No ND flag of a receive buffer with MBI = '1' has been set to '1'"]
pub type RXI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, RXI_A, O>;
impl<'a, const O: u8> RXI_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_nd_flag_of_rx_mbi_buff_set(self) -> &'a mut W {
        self.variant(RXI_A::NO_ND_FLAG_OF_RX_MBI_BUFF_SET)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn nd_flag_of_rx_mbi_buff_set(self) -> &'a mut W {
        self.variant(RXI_A::ND_FLAG_OF_RX_MBI_BUFF_SET)
    }
}
#[doc = "Field `RFNE` reader - Receive FIFO Not Empty This flag is set by the CC when a received valid frame was stored into the empty receive FIFO. The actual state of the receive FIFO is monitored in register FSR. 1 = Receive FIFO is not empty 0 = Receive FIFO is empty"]
pub type RFNE_R = crate::BitReader<RFNE_A>;
#[doc = "Receive FIFO Not Empty This flag is set by the CC when a received valid frame was stored into the empty receive FIFO. The actual state of the receive FIFO is monitored in register FSR. 1 = Receive FIFO is not empty 0 = Receive FIFO is empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFNE_A {
    #[doc = "0: N/A"]
    RX_FIFO_EMPTY = 0,
    #[doc = "1: N/A"]
    RX_FIFO_NOT_EMPTY = 1,
}
impl From<RFNE_A> for bool {
    #[inline(always)]
    fn from(variant: RFNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFNE_A {
        match self.bits {
            false => RFNE_A::RX_FIFO_EMPTY,
            true => RFNE_A::RX_FIFO_NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_EMPTY`"]
    #[inline(always)]
    pub fn is_rx_fifo_empty(&self) -> bool {
        *self == RFNE_A::RX_FIFO_EMPTY
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_rx_fifo_not_empty(&self) -> bool {
        *self == RFNE_A::RX_FIFO_NOT_EMPTY
    }
}
#[doc = "Field `RFNE` writer - Receive FIFO Not Empty This flag is set by the CC when a received valid frame was stored into the empty receive FIFO. The actual state of the receive FIFO is monitored in register FSR. 1 = Receive FIFO is not empty 0 = Receive FIFO is empty"]
pub type RFNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, RFNE_A, O>;
impl<'a, const O: u8> RFNE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rx_fifo_empty(self) -> &'a mut W {
        self.variant(RFNE_A::RX_FIFO_EMPTY)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(self) -> &'a mut W {
        self.variant(RFNE_A::RX_FIFO_NOT_EMPTY)
    }
}
#[doc = "Field `RFCL` reader - Receive FIFO Critical Level This flag is set when the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level as configured by FCL.CL\\[7:0\\]. 1 = Receive FIFO critical level reached 0 = Receive FIFO below critical level"]
pub type RFCL_R = crate::BitReader<RFCL_A>;
#[doc = "Receive FIFO Critical Level This flag is set when the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level as configured by FCL.CL\\[7:0\\]. 1 = Receive FIFO critical level reached 0 = Receive FIFO below critical level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCL_A {
    #[doc = "0: N/A"]
    RX_FIFO_BELOW_CRITICAL_LEVEL = 0,
    #[doc = "1: N/A"]
    RX_FIFO_NOT_BELOW_CRITICAL_LEVEL = 1,
}
impl From<RFCL_A> for bool {
    #[inline(always)]
    fn from(variant: RFCL_A) -> Self {
        variant as u8 != 0
    }
}
impl RFCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCL_A {
        match self.bits {
            false => RFCL_A::RX_FIFO_BELOW_CRITICAL_LEVEL,
            true => RFCL_A::RX_FIFO_NOT_BELOW_CRITICAL_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_BELOW_CRITICAL_LEVEL`"]
    #[inline(always)]
    pub fn is_rx_fifo_below_critical_level(&self) -> bool {
        *self == RFCL_A::RX_FIFO_BELOW_CRITICAL_LEVEL
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_NOT_BELOW_CRITICAL_LEVEL`"]
    #[inline(always)]
    pub fn is_rx_fifo_not_below_critical_level(&self) -> bool {
        *self == RFCL_A::RX_FIFO_NOT_BELOW_CRITICAL_LEVEL
    }
}
#[doc = "Field `RFCL` writer - Receive FIFO Critical Level This flag is set when the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level as configured by FCL.CL\\[7:0\\]. 1 = Receive FIFO critical level reached 0 = Receive FIFO below critical level"]
pub type RFCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, RFCL_A, O>;
impl<'a, const O: u8> RFCL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rx_fifo_below_critical_level(self) -> &'a mut W {
        self.variant(RFCL_A::RX_FIFO_BELOW_CRITICAL_LEVEL)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rx_fifo_not_below_critical_level(self) -> &'a mut W {
        self.variant(RFCL_A::RX_FIFO_NOT_BELOW_CRITICAL_LEVEL)
    }
}
#[doc = "Field `NMVC` reader - Network Management Vector Changed This interrupt flag signals a change in the Network Management Vector visible to the Host. 1 = Network management vector changed 0 = No change in the network management vector"]
pub type NMVC_R = crate::BitReader<NMVC_A>;
#[doc = "Network Management Vector Changed This interrupt flag signals a change in the Network Management Vector visible to the Host. 1 = Network management vector changed 0 = No change in the network management vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMVC_A {
    #[doc = "0: N/A"]
    NTWK_MGMT_VECTOR_UNCHANGED = 0,
    #[doc = "1: N/A"]
    NTWK_MGMT_VECTOR_CHANGED = 1,
}
impl From<NMVC_A> for bool {
    #[inline(always)]
    fn from(variant: NMVC_A) -> Self {
        variant as u8 != 0
    }
}
impl NMVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMVC_A {
        match self.bits {
            false => NMVC_A::NTWK_MGMT_VECTOR_UNCHANGED,
            true => NMVC_A::NTWK_MGMT_VECTOR_CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `NTWK_MGMT_VECTOR_UNCHANGED`"]
    #[inline(always)]
    pub fn is_ntwk_mgmt_vector_unchanged(&self) -> bool {
        *self == NMVC_A::NTWK_MGMT_VECTOR_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `NTWK_MGMT_VECTOR_CHANGED`"]
    #[inline(always)]
    pub fn is_ntwk_mgmt_vector_changed(&self) -> bool {
        *self == NMVC_A::NTWK_MGMT_VECTOR_CHANGED
    }
}
#[doc = "Field `NMVC` writer - Network Management Vector Changed This interrupt flag signals a change in the Network Management Vector visible to the Host. 1 = Network management vector changed 0 = No change in the network management vector"]
pub type NMVC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, NMVC_A, O>;
impl<'a, const O: u8> NMVC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ntwk_mgmt_vector_unchanged(self) -> &'a mut W {
        self.variant(NMVC_A::NTWK_MGMT_VECTOR_UNCHANGED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ntwk_mgmt_vector_changed(self) -> &'a mut W {
        self.variant(NMVC_A::NTWK_MGMT_VECTOR_CHANGED)
    }
}
#[doc = "Field `TI0` reader - Timer Interrupt 0 This flag is set whenever timer 0 matches the conditions configured in register T0C. A Timer Interrupt 0 is also signalled on pin eray_tint0. 1 = Timer Interrupt 0 occurred 0 = No Timer Interrupt 0"]
pub type TI0_R = crate::BitReader<TI0_A>;
#[doc = "Timer Interrupt 0 This flag is set whenever timer 0 matches the conditions configured in register T0C. A Timer Interrupt 0 is also signalled on pin eray_tint0. 1 = Timer Interrupt 0 occurred 0 = No Timer Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI0_A {
    #[doc = "0: N/A"]
    NO_TIMER_INTERRUPT_0 = 0,
    #[doc = "1: N/A"]
    TIMER_INTERRUPT_0_OCCURED = 1,
}
impl From<TI0_A> for bool {
    #[inline(always)]
    fn from(variant: TI0_A) -> Self {
        variant as u8 != 0
    }
}
impl TI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI0_A {
        match self.bits {
            false => TI0_A::NO_TIMER_INTERRUPT_0,
            true => TI0_A::TIMER_INTERRUPT_0_OCCURED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMER_INTERRUPT_0`"]
    #[inline(always)]
    pub fn is_no_timer_interrupt_0(&self) -> bool {
        *self == TI0_A::NO_TIMER_INTERRUPT_0
    }
    #[doc = "Checks if the value of the field is `TIMER_INTERRUPT_0_OCCURED`"]
    #[inline(always)]
    pub fn is_timer_interrupt_0_occured(&self) -> bool {
        *self == TI0_A::TIMER_INTERRUPT_0_OCCURED
    }
}
#[doc = "Field `TI0` writer - Timer Interrupt 0 This flag is set whenever timer 0 matches the conditions configured in register T0C. A Timer Interrupt 0 is also signalled on pin eray_tint0. 1 = Timer Interrupt 0 occurred 0 = No Timer Interrupt 0"]
pub type TI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, TI0_A, O>;
impl<'a, const O: u8> TI0_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_timer_interrupt_0(self) -> &'a mut W {
        self.variant(TI0_A::NO_TIMER_INTERRUPT_0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_interrupt_0_occured(self) -> &'a mut W {
        self.variant(TI0_A::TIMER_INTERRUPT_0_OCCURED)
    }
}
#[doc = "Field `TI1` reader - Timer Interrupt 1 This flag is set whenever timer 1 matches the conditions configured in register T1C. A Timer Interrupt 1 is also signalled on pin eray_tint1. 1 = Timer Interrupt 1 occurred 0 = No Timer Interrupt 1"]
pub type TI1_R = crate::BitReader<TI1_A>;
#[doc = "Timer Interrupt 1 This flag is set whenever timer 1 matches the conditions configured in register T1C. A Timer Interrupt 1 is also signalled on pin eray_tint1. 1 = Timer Interrupt 1 occurred 0 = No Timer Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1_A {
    #[doc = "0: N/A"]
    NO_TIMER_INTERRUPT_1 = 0,
    #[doc = "1: N/A"]
    TIMER_INTERRUPT_1_OCCURED = 1,
}
impl From<TI1_A> for bool {
    #[inline(always)]
    fn from(variant: TI1_A) -> Self {
        variant as u8 != 0
    }
}
impl TI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI1_A {
        match self.bits {
            false => TI1_A::NO_TIMER_INTERRUPT_1,
            true => TI1_A::TIMER_INTERRUPT_1_OCCURED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMER_INTERRUPT_1`"]
    #[inline(always)]
    pub fn is_no_timer_interrupt_1(&self) -> bool {
        *self == TI1_A::NO_TIMER_INTERRUPT_1
    }
    #[doc = "Checks if the value of the field is `TIMER_INTERRUPT_1_OCCURED`"]
    #[inline(always)]
    pub fn is_timer_interrupt_1_occured(&self) -> bool {
        *self == TI1_A::TIMER_INTERRUPT_1_OCCURED
    }
}
#[doc = "Field `TI1` writer - Timer Interrupt 1 This flag is set whenever timer 1 matches the conditions configured in register T1C. A Timer Interrupt 1 is also signalled on pin eray_tint1. 1 = Timer Interrupt 1 occurred 0 = No Timer Interrupt 1"]
pub type TI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, TI1_A, O>;
impl<'a, const O: u8> TI1_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_timer_interrupt_1(self) -> &'a mut W {
        self.variant(TI1_A::NO_TIMER_INTERRUPT_1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn timer_interrupt_1_occured(self) -> &'a mut W {
        self.variant(TI1_A::TIMER_INTERRUPT_1_OCCURED)
    }
}
#[doc = "Field `TIBC` reader - Transfer Input Buffer Completed This flag is set whenever a transfer from Input Buffer to the Message RAM has completed and IBCR.IBSYS has been reset by the Message Handler. 1 = Transfer between Input Buffer and Message RAM completed 0 = No transfer completed"]
pub type TIBC_R = crate::BitReader<TIBC_A>;
#[doc = "Transfer Input Buffer Completed This flag is set whenever a transfer from Input Buffer to the Message RAM has completed and IBCR.IBSYS has been reset by the Message Handler. 1 = Transfer between Input Buffer and Message RAM completed 0 = No transfer completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIBC_A {
    #[doc = "0: N/A"]
    NO_TRANSFER_COMPLETED = 0,
    #[doc = "1: N/A"]
    IBF_TO_MSGRAM_TRANSFER_COMPLETED = 1,
}
impl From<TIBC_A> for bool {
    #[inline(always)]
    fn from(variant: TIBC_A) -> Self {
        variant as u8 != 0
    }
}
impl TIBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIBC_A {
        match self.bits {
            false => TIBC_A::NO_TRANSFER_COMPLETED,
            true => TIBC_A::IBF_TO_MSGRAM_TRANSFER_COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRANSFER_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_transfer_completed(&self) -> bool {
        *self == TIBC_A::NO_TRANSFER_COMPLETED
    }
    #[doc = "Checks if the value of the field is `IBF_TO_MSGRAM_TRANSFER_COMPLETED`"]
    #[inline(always)]
    pub fn is_ibf_to_msgram_transfer_completed(&self) -> bool {
        *self == TIBC_A::IBF_TO_MSGRAM_TRANSFER_COMPLETED
    }
}
#[doc = "Field `TIBC` writer - Transfer Input Buffer Completed This flag is set whenever a transfer from Input Buffer to the Message RAM has completed and IBCR.IBSYS has been reset by the Message Handler. 1 = Transfer between Input Buffer and Message RAM completed 0 = No transfer completed"]
pub type TIBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, TIBC_A, O>;
impl<'a, const O: u8> TIBC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_transfer_completed(self) -> &'a mut W {
        self.variant(TIBC_A::NO_TRANSFER_COMPLETED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ibf_to_msgram_transfer_completed(self) -> &'a mut W {
        self.variant(TIBC_A::IBF_TO_MSGRAM_TRANSFER_COMPLETED)
    }
}
#[doc = "Field `TOBC` reader - Transfer Output Buffer Completed This flag is set whenever a transfer from the Message RAM to the Output Buffer has completed and OBCR.OBSYS has been reset by the Message Handler. 1 = Transfer between Message RAM and Output Buffer completed 0 = No transfer completed"]
pub type TOBC_R = crate::BitReader<TOBC_A>;
#[doc = "Transfer Output Buffer Completed This flag is set whenever a transfer from the Message RAM to the Output Buffer has completed and OBCR.OBSYS has been reset by the Message Handler. 1 = Transfer between Message RAM and Output Buffer completed 0 = No transfer completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOBC_A {
    #[doc = "0: N/A"]
    NO_TRANSFER_COMPLETED = 0,
    #[doc = "1: N/A"]
    MSGRAM_TO_OBF_TRANSFER_COMPLETED = 1,
}
impl From<TOBC_A> for bool {
    #[inline(always)]
    fn from(variant: TOBC_A) -> Self {
        variant as u8 != 0
    }
}
impl TOBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOBC_A {
        match self.bits {
            false => TOBC_A::NO_TRANSFER_COMPLETED,
            true => TOBC_A::MSGRAM_TO_OBF_TRANSFER_COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRANSFER_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_transfer_completed(&self) -> bool {
        *self == TOBC_A::NO_TRANSFER_COMPLETED
    }
    #[doc = "Checks if the value of the field is `MSGRAM_TO_OBF_TRANSFER_COMPLETED`"]
    #[inline(always)]
    pub fn is_msgram_to_obf_transfer_completed(&self) -> bool {
        *self == TOBC_A::MSGRAM_TO_OBF_TRANSFER_COMPLETED
    }
}
#[doc = "Field `TOBC` writer - Transfer Output Buffer Completed This flag is set whenever a transfer from the Message RAM to the Output Buffer has completed and OBCR.OBSYS has been reset by the Message Handler. 1 = Transfer between Message RAM and Output Buffer completed 0 = No transfer completed"]
pub type TOBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, TOBC_A, O>;
impl<'a, const O: u8> TOBC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_transfer_completed(self) -> &'a mut W {
        self.variant(TOBC_A::NO_TRANSFER_COMPLETED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msgram_to_obf_transfer_completed(self) -> &'a mut W {
        self.variant(TOBC_A::MSGRAM_TO_OBF_TRANSFER_COMPLETED)
    }
}
#[doc = "Field `SWE` reader - Stop Watch Event This flag is set after a stop watch activation when the actual cycle counter and macrotick value are stored in the Stop Watch register (see \\[01\\]Section 4.4.10 Stop Watch Register 1 (STPW1)). 1 = Stop Watch Event occurred 0 = No Stop Watch Event"]
pub type SWE_R = crate::BitReader<SWE_A>;
#[doc = "Stop Watch Event This flag is set after a stop watch activation when the actual cycle counter and macrotick value are stored in the Stop Watch register (see \\[01\\]Section 4.4.10 Stop Watch Register 1 (STPW1)). 1 = Stop Watch Event occurred 0 = No Stop Watch Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWE_A {
    #[doc = "0: N/A"]
    NO_STOP_WATCH_EVENT = 0,
    #[doc = "1: N/A"]
    STOP_WATCH_EVENT_OCCURRED = 1,
}
impl From<SWE_A> for bool {
    #[inline(always)]
    fn from(variant: SWE_A) -> Self {
        variant as u8 != 0
    }
}
impl SWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWE_A {
        match self.bits {
            false => SWE_A::NO_STOP_WATCH_EVENT,
            true => SWE_A::STOP_WATCH_EVENT_OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STOP_WATCH_EVENT`"]
    #[inline(always)]
    pub fn is_no_stop_watch_event(&self) -> bool {
        *self == SWE_A::NO_STOP_WATCH_EVENT
    }
    #[doc = "Checks if the value of the field is `STOP_WATCH_EVENT_OCCURRED`"]
    #[inline(always)]
    pub fn is_stop_watch_event_occurred(&self) -> bool {
        *self == SWE_A::STOP_WATCH_EVENT_OCCURRED
    }
}
#[doc = "Field `SWE` writer - Stop Watch Event This flag is set after a stop watch activation when the actual cycle counter and macrotick value are stored in the Stop Watch register (see \\[01\\]Section 4.4.10 Stop Watch Register 1 (STPW1)). 1 = Stop Watch Event occurred 0 = No Stop Watch Event"]
pub type SWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, SWE_A, O>;
impl<'a, const O: u8> SWE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_stop_watch_event(self) -> &'a mut W {
        self.variant(SWE_A::NO_STOP_WATCH_EVENT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn stop_watch_event_occurred(self) -> &'a mut W {
        self.variant(SWE_A::STOP_WATCH_EVENT_OCCURRED)
    }
}
#[doc = "Field `SUCS` reader - Startup Completed Successfully This flag is set whenever a startup completed successfully and the CC entered NORMAL_ACTIVE state. 1 = Startup completed successfully 0 = No startup completed successfully"]
pub type SUCS_R = crate::BitReader<SUCS_A>;
#[doc = "Startup Completed Successfully This flag is set whenever a startup completed successfully and the CC entered NORMAL_ACTIVE state. 1 = Startup completed successfully 0 = No startup completed successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUCS_A {
    #[doc = "0: N/A"]
    NO_STARTUP_COMPLETED = 0,
    #[doc = "1: N/A"]
    STARTUP_COMPLETED = 1,
}
impl From<SUCS_A> for bool {
    #[inline(always)]
    fn from(variant: SUCS_A) -> Self {
        variant as u8 != 0
    }
}
impl SUCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUCS_A {
        match self.bits {
            false => SUCS_A::NO_STARTUP_COMPLETED,
            true => SUCS_A::STARTUP_COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STARTUP_COMPLETED`"]
    #[inline(always)]
    pub fn is_no_startup_completed(&self) -> bool {
        *self == SUCS_A::NO_STARTUP_COMPLETED
    }
    #[doc = "Checks if the value of the field is `STARTUP_COMPLETED`"]
    #[inline(always)]
    pub fn is_startup_completed(&self) -> bool {
        *self == SUCS_A::STARTUP_COMPLETED
    }
}
#[doc = "Field `SUCS` writer - Startup Completed Successfully This flag is set whenever a startup completed successfully and the CC entered NORMAL_ACTIVE state. 1 = Startup completed successfully 0 = No startup completed successfully"]
pub type SUCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, SUCS_A, O>;
impl<'a, const O: u8> SUCS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_startup_completed(self) -> &'a mut W {
        self.variant(SUCS_A::NO_STARTUP_COMPLETED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn startup_completed(self) -> &'a mut W {
        self.variant(SUCS_A::STARTUP_COMPLETED)
    }
}
#[doc = "Field `MBSI` reader - Message Buffer Status Interrupt This flag is set by the CC when the message buffer status MBS has changed and if bit MBI of that message buffer is set (see Table 17). 1 = Message buffer status of at least one message buffer with MBI = '1' has changed 0 = No message buffer status change of message buffer with MBI = '1'"]
pub type MBSI_R = crate::BitReader<MBSI_A>;
#[doc = "Message Buffer Status Interrupt This flag is set by the CC when the message buffer status MBS has changed and if bit MBI of that message buffer is set (see Table 17). 1 = Message buffer status of at least one message buffer with MBI = '1' has changed 0 = No message buffer status change of message buffer with MBI = '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBSI_A {
    #[doc = "0: N/A"]
    NO_MSG_BUFF_STATUS_CHANGED = 0,
    #[doc = "1: N/A"]
    MSG_BUFF_STATUS_CHANGED = 1,
}
impl From<MBSI_A> for bool {
    #[inline(always)]
    fn from(variant: MBSI_A) -> Self {
        variant as u8 != 0
    }
}
impl MBSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBSI_A {
        match self.bits {
            false => MBSI_A::NO_MSG_BUFF_STATUS_CHANGED,
            true => MBSI_A::MSG_BUFF_STATUS_CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MSG_BUFF_STATUS_CHANGED`"]
    #[inline(always)]
    pub fn is_no_msg_buff_status_changed(&self) -> bool {
        *self == MBSI_A::NO_MSG_BUFF_STATUS_CHANGED
    }
    #[doc = "Checks if the value of the field is `MSG_BUFF_STATUS_CHANGED`"]
    #[inline(always)]
    pub fn is_msg_buff_status_changed(&self) -> bool {
        *self == MBSI_A::MSG_BUFF_STATUS_CHANGED
    }
}
#[doc = "Field `MBSI` writer - Message Buffer Status Interrupt This flag is set by the CC when the message buffer status MBS has changed and if bit MBI of that message buffer is set (see Table 17). 1 = Message buffer status of at least one message buffer with MBI = '1' has changed 0 = No message buffer status change of message buffer with MBI = '1'"]
pub type MBSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, MBSI_A, O>;
impl<'a, const O: u8> MBSI_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_msg_buff_status_changed(self) -> &'a mut W {
        self.variant(MBSI_A::NO_MSG_BUFF_STATUS_CHANGED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msg_buff_status_changed(self) -> &'a mut W {
        self.variant(MBSI_A::MSG_BUFF_STATUS_CHANGED)
    }
}
#[doc = "Field `SDS` reader - Start of Dynamic Segment This flag is set by the CC when the dynamic segment starts. 1 = Dynamic segment started 0 = Dynamic segment not yet started"]
pub type SDS_R = crate::BitReader<SDS_A>;
#[doc = "Start of Dynamic Segment This flag is set by the CC when the dynamic segment starts. 1 = Dynamic segment started 0 = Dynamic segment not yet started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDS_A {
    #[doc = "0: N/A"]
    DYNAMIC_SEGMENT_NOT_STARTED = 0,
    #[doc = "1: N/A"]
    DYNAMIC_SEGMENT_STARTED = 1,
}
impl From<SDS_A> for bool {
    #[inline(always)]
    fn from(variant: SDS_A) -> Self {
        variant as u8 != 0
    }
}
impl SDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDS_A {
        match self.bits {
            false => SDS_A::DYNAMIC_SEGMENT_NOT_STARTED,
            true => SDS_A::DYNAMIC_SEGMENT_STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `DYNAMIC_SEGMENT_NOT_STARTED`"]
    #[inline(always)]
    pub fn is_dynamic_segment_not_started(&self) -> bool {
        *self == SDS_A::DYNAMIC_SEGMENT_NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `DYNAMIC_SEGMENT_STARTED`"]
    #[inline(always)]
    pub fn is_dynamic_segment_started(&self) -> bool {
        *self == SDS_A::DYNAMIC_SEGMENT_STARTED
    }
}
#[doc = "Field `SDS` writer - Start of Dynamic Segment This flag is set by the CC when the dynamic segment starts. 1 = Dynamic segment started 0 = Dynamic segment not yet started"]
pub type SDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, SDS_A, O>;
impl<'a, const O: u8> SDS_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn dynamic_segment_not_started(self) -> &'a mut W {
        self.variant(SDS_A::DYNAMIC_SEGMENT_NOT_STARTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn dynamic_segment_started(self) -> &'a mut W {
        self.variant(SDS_A::DYNAMIC_SEGMENT_STARTED)
    }
}
#[doc = "Field `WUPA` reader - Wakeup Pattern Channel A This flag is set by the CC when a wakeup pattern was received on channel A. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel A 0 = No wakeup pattern received on channel A"]
pub type WUPA_R = crate::BitReader<WUPA_A>;
#[doc = "Wakeup Pattern Channel A This flag is set by the CC when a wakeup pattern was received on channel A. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel A 0 = No wakeup pattern received on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPA_A {
    #[doc = "0: N/A"]
    NO_WAKEUP_PATTERN_RXD_ON_CH_A = 0,
    #[doc = "1: N/A"]
    WAKEUP_PATTERN_RXD_ON_CH_A = 1,
}
impl From<WUPA_A> for bool {
    #[inline(always)]
    fn from(variant: WUPA_A) -> Self {
        variant as u8 != 0
    }
}
impl WUPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPA_A {
        match self.bits {
            false => WUPA_A::NO_WAKEUP_PATTERN_RXD_ON_CH_A,
            true => WUPA_A::WAKEUP_PATTERN_RXD_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WAKEUP_PATTERN_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_no_wakeup_pattern_rxd_on_ch_a(&self) -> bool {
        *self == WUPA_A::NO_WAKEUP_PATTERN_RXD_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `WAKEUP_PATTERN_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_wakeup_pattern_rxd_on_ch_a(&self) -> bool {
        *self == WUPA_A::WAKEUP_PATTERN_RXD_ON_CH_A
    }
}
#[doc = "Field `WUPA` writer - Wakeup Pattern Channel A This flag is set by the CC when a wakeup pattern was received on channel A. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel A 0 = No wakeup pattern received on channel A"]
pub type WUPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, WUPA_A, O>;
impl<'a, const O: u8> WUPA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_wakeup_pattern_rxd_on_ch_a(self) -> &'a mut W {
        self.variant(WUPA_A::NO_WAKEUP_PATTERN_RXD_ON_CH_A)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn wakeup_pattern_rxd_on_ch_a(self) -> &'a mut W {
        self.variant(WUPA_A::WAKEUP_PATTERN_RXD_ON_CH_A)
    }
}
#[doc = "Field `MTSA` reader - MTS Received on Channel A (vSS!ValidMTSA) Media Access Test symbol received on channel A during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel A 0 = No MTS symbol received on channel A"]
pub type MTSA_R = crate::BitReader<MTSA_A>;
#[doc = "MTS Received on Channel A (vSS!ValidMTSA) Media Access Test symbol received on channel A during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel A 0 = No MTS symbol received on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTSA_A {
    #[doc = "0: N/A"]
    NO_MTS_SYMBOL_RXD_ON_CH_A = 0,
    #[doc = "1: N/A"]
    MTS_SYMBOL_RXD_ON_CH_A = 1,
}
impl From<MTSA_A> for bool {
    #[inline(always)]
    fn from(variant: MTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl MTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSA_A {
        match self.bits {
            false => MTSA_A::NO_MTS_SYMBOL_RXD_ON_CH_A,
            true => MTSA_A::MTS_SYMBOL_RXD_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MTS_SYMBOL_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_no_mts_symbol_rxd_on_ch_a(&self) -> bool {
        *self == MTSA_A::NO_MTS_SYMBOL_RXD_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `MTS_SYMBOL_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_mts_symbol_rxd_on_ch_a(&self) -> bool {
        *self == MTSA_A::MTS_SYMBOL_RXD_ON_CH_A
    }
}
#[doc = "Field `MTSA` writer - MTS Received on Channel A (vSS!ValidMTSA) Media Access Test symbol received on channel A during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel A 0 = No MTS symbol received on channel A"]
pub type MTSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, MTSA_A, O>;
impl<'a, const O: u8> MTSA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_mts_symbol_rxd_on_ch_a(self) -> &'a mut W {
        self.variant(MTSA_A::NO_MTS_SYMBOL_RXD_ON_CH_A)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn mts_symbol_rxd_on_ch_a(self) -> &'a mut W {
        self.variant(MTSA_A::MTS_SYMBOL_RXD_ON_CH_A)
    }
}
#[doc = "Field `WUPB` reader - Wakeup Pattern Channel B This flag is set by the CC when a wakeup pattern was received on channel B. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel B 0 = No wakeup pattern received on channel B"]
pub type WUPB_R = crate::BitReader<WUPB_A>;
#[doc = "Wakeup Pattern Channel B This flag is set by the CC when a wakeup pattern was received on channel B. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel B 0 = No wakeup pattern received on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPB_A {
    #[doc = "0: N/A"]
    NO_WAKEUP_PATTERN_RXD_ON_CH_B = 0,
    #[doc = "1: N/A"]
    WAKEUP_PATTERN_RXD_ON_CH_B = 1,
}
impl From<WUPB_A> for bool {
    #[inline(always)]
    fn from(variant: WUPB_A) -> Self {
        variant as u8 != 0
    }
}
impl WUPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPB_A {
        match self.bits {
            false => WUPB_A::NO_WAKEUP_PATTERN_RXD_ON_CH_B,
            true => WUPB_A::WAKEUP_PATTERN_RXD_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WAKEUP_PATTERN_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_no_wakeup_pattern_rxd_on_ch_b(&self) -> bool {
        *self == WUPB_A::NO_WAKEUP_PATTERN_RXD_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `WAKEUP_PATTERN_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_wakeup_pattern_rxd_on_ch_b(&self) -> bool {
        *self == WUPB_A::WAKEUP_PATTERN_RXD_ON_CH_B
    }
}
#[doc = "Field `WUPB` writer - Wakeup Pattern Channel B This flag is set by the CC when a wakeup pattern was received on channel B. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel B 0 = No wakeup pattern received on channel B"]
pub type WUPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, WUPB_A, O>;
impl<'a, const O: u8> WUPB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_wakeup_pattern_rxd_on_ch_b(self) -> &'a mut W {
        self.variant(WUPB_A::NO_WAKEUP_PATTERN_RXD_ON_CH_B)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn wakeup_pattern_rxd_on_ch_b(self) -> &'a mut W {
        self.variant(WUPB_A::WAKEUP_PATTERN_RXD_ON_CH_B)
    }
}
#[doc = "Field `MTSB` reader - MTS Received on Channel B (vSS!ValidMTSB) Media Access Test symbol received on channel B during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel B 0 = No MTS symbol received on channel B"]
pub type MTSB_R = crate::BitReader<MTSB_A>;
#[doc = "MTS Received on Channel B (vSS!ValidMTSB) Media Access Test symbol received on channel B during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel B 0 = No MTS symbol received on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTSB_A {
    #[doc = "0: N/A"]
    NO_MTS_SYMBOL_RXD_ON_CH_B = 0,
    #[doc = "1: N/A"]
    MTS_SYMBOL_RXD_ON_CH_B = 1,
}
impl From<MTSB_A> for bool {
    #[inline(always)]
    fn from(variant: MTSB_A) -> Self {
        variant as u8 != 0
    }
}
impl MTSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSB_A {
        match self.bits {
            false => MTSB_A::NO_MTS_SYMBOL_RXD_ON_CH_B,
            true => MTSB_A::MTS_SYMBOL_RXD_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MTS_SYMBOL_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_no_mts_symbol_rxd_on_ch_b(&self) -> bool {
        *self == MTSB_A::NO_MTS_SYMBOL_RXD_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `MTS_SYMBOL_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_mts_symbol_rxd_on_ch_b(&self) -> bool {
        *self == MTSB_A::MTS_SYMBOL_RXD_ON_CH_B
    }
}
#[doc = "Field `MTSB` writer - MTS Received on Channel B (vSS!ValidMTSB) Media Access Test symbol received on channel B during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel B 0 = No MTS symbol received on channel B"]
pub type MTSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIR_SPEC, MTSB_A, O>;
impl<'a, const O: u8> MTSB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_mts_symbol_rxd_on_ch_b(self) -> &'a mut W {
        self.variant(MTSB_A::NO_MTS_SYMBOL_RXD_ON_CH_B)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn mts_symbol_rxd_on_ch_b(self) -> &'a mut W {
        self.variant(MTSB_A::MTS_SYMBOL_RXD_ON_CH_B)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Status This flag is set when CCSV.WSV\\[2:0\\]
changes to a value other than UNDEFINED. 1 = Wakeup status changed 0 = Wakeup status unchanged"]
    #[inline(always)]
    pub fn wst(&self) -> WST_R {
        WST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision Avoidance Symbol This flag is set by the CC during STARTUP state when a CAS or a potential CAS was received. 1 = Bit pattern matching the CAS symbol received 0 = No bit pattern matching the CAS symbol received"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cycle Start Interrupt This flag is set by the CC when a communication cycle starts. 1 = Communication cycle started 0 = No communication cycle started"]
    #[inline(always)]
    pub fn cycs(&self) -> CYCS_R {
        CYCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Interrupt This flag is set by the CC at the end of frame transmission if bit MBI in the respective message buffer is set to '1' (see Table 17). 1 = At least one frame was transmitted from a transmit buffer with MBI = '1' 0 = No frame transmitted from a transmit buffer with MBI = '1'"]
    #[inline(always)]
    pub fn txi(&self) -> TXI_R {
        TXI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Interrupt This flag is set by the CC whenever the set condition of a message buffers ND flag is fulfilled see 4.8.6 New Data 1/2/3/4 (NDAT1/2/3/4)), and if bit MBI of that message buffer is set to '1' see Table 17). 1 = At least one ND flag of a receive buffer with MBI = '1' has been set to '1' 0 = No ND flag of a receive buffer with MBI = '1' has been set to '1'"]
    #[inline(always)]
    pub fn rxi(&self) -> RXI_R {
        RXI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Not Empty This flag is set by the CC when a received valid frame was stored into the empty receive FIFO. The actual state of the receive FIFO is monitored in register FSR. 1 = Receive FIFO is not empty 0 = Receive FIFO is empty"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Critical Level This flag is set when the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level as configured by FCL.CL\\[7:0\\]. 1 = Receive FIFO critical level reached 0 = Receive FIFO below critical level"]
    #[inline(always)]
    pub fn rfcl(&self) -> RFCL_R {
        RFCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Network Management Vector Changed This interrupt flag signals a change in the Network Management Vector visible to the Host. 1 = Network management vector changed 0 = No change in the network management vector"]
    #[inline(always)]
    pub fn nmvc(&self) -> NMVC_R {
        NMVC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer Interrupt 0 This flag is set whenever timer 0 matches the conditions configured in register T0C. A Timer Interrupt 0 is also signalled on pin eray_tint0. 1 = Timer Interrupt 0 occurred 0 = No Timer Interrupt 0"]
    #[inline(always)]
    pub fn ti0(&self) -> TI0_R {
        TI0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer Interrupt 1 This flag is set whenever timer 1 matches the conditions configured in register T1C. A Timer Interrupt 1 is also signalled on pin eray_tint1. 1 = Timer Interrupt 1 occurred 0 = No Timer Interrupt 1"]
    #[inline(always)]
    pub fn ti1(&self) -> TI1_R {
        TI1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer Input Buffer Completed This flag is set whenever a transfer from Input Buffer to the Message RAM has completed and IBCR.IBSYS has been reset by the Message Handler. 1 = Transfer between Input Buffer and Message RAM completed 0 = No transfer completed"]
    #[inline(always)]
    pub fn tibc(&self) -> TIBC_R {
        TIBC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transfer Output Buffer Completed This flag is set whenever a transfer from the Message RAM to the Output Buffer has completed and OBCR.OBSYS has been reset by the Message Handler. 1 = Transfer between Message RAM and Output Buffer completed 0 = No transfer completed"]
    #[inline(always)]
    pub fn tobc(&self) -> TOBC_R {
        TOBC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Stop Watch Event This flag is set after a stop watch activation when the actual cycle counter and macrotick value are stored in the Stop Watch register (see \\[01\\]Section 4.4.10 Stop Watch Register 1 (STPW1)). 1 = Stop Watch Event occurred 0 = No Stop Watch Event"]
    #[inline(always)]
    pub fn swe(&self) -> SWE_R {
        SWE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Startup Completed Successfully This flag is set whenever a startup completed successfully and the CC entered NORMAL_ACTIVE state. 1 = Startup completed successfully 0 = No startup completed successfully"]
    #[inline(always)]
    pub fn sucs(&self) -> SUCS_R {
        SUCS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Buffer Status Interrupt This flag is set by the CC when the message buffer status MBS has changed and if bit MBI of that message buffer is set (see Table 17). 1 = Message buffer status of at least one message buffer with MBI = '1' has changed 0 = No message buffer status change of message buffer with MBI = '1'"]
    #[inline(always)]
    pub fn mbsi(&self) -> MBSI_R {
        MBSI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Start of Dynamic Segment This flag is set by the CC when the dynamic segment starts. 1 = Dynamic segment started 0 = Dynamic segment not yet started"]
    #[inline(always)]
    pub fn sds(&self) -> SDS_R {
        SDS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup Pattern Channel A This flag is set by the CC when a wakeup pattern was received on channel A. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel A 0 = No wakeup pattern received on channel A"]
    #[inline(always)]
    pub fn wupa(&self) -> WUPA_R {
        WUPA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MTS Received on Channel A (vSS!ValidMTSA) Media Access Test symbol received on channel A during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel A 0 = No MTS symbol received on channel A"]
    #[inline(always)]
    pub fn mtsa(&self) -> MTSA_R {
        MTSA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup Pattern Channel B This flag is set by the CC when a wakeup pattern was received on channel B. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel B 0 = No wakeup pattern received on channel B"]
    #[inline(always)]
    pub fn wupb(&self) -> WUPB_R {
        WUPB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MTS Received on Channel B (vSS!ValidMTSB) Media Access Test symbol received on channel B during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel B 0 = No MTS symbol received on channel B"]
    #[inline(always)]
    pub fn mtsb(&self) -> MTSB_R {
        MTSB_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Status This flag is set when CCSV.WSV\\[2:0\\]
changes to a value other than UNDEFINED. 1 = Wakeup status changed 0 = Wakeup status unchanged"]
    #[inline(always)]
    #[must_use]
    pub fn wst(&mut self) -> WST_W<0> {
        WST_W::new(self)
    }
    #[doc = "Bit 1 - Collision Avoidance Symbol This flag is set by the CC during STARTUP state when a CAS or a potential CAS was received. 1 = Bit pattern matching the CAS symbol received 0 = No bit pattern matching the CAS symbol received"]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CAS_W<1> {
        CAS_W::new(self)
    }
    #[doc = "Bit 2 - Cycle Start Interrupt This flag is set by the CC when a communication cycle starts. 1 = Communication cycle started 0 = No communication cycle started"]
    #[inline(always)]
    #[must_use]
    pub fn cycs(&mut self) -> CYCS_W<2> {
        CYCS_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Interrupt This flag is set by the CC at the end of frame transmission if bit MBI in the respective message buffer is set to '1' (see Table 17). 1 = At least one frame was transmitted from a transmit buffer with MBI = '1' 0 = No frame transmitted from a transmit buffer with MBI = '1'"]
    #[inline(always)]
    #[must_use]
    pub fn txi(&mut self) -> TXI_W<3> {
        TXI_W::new(self)
    }
    #[doc = "Bit 4 - Receive Interrupt This flag is set by the CC whenever the set condition of a message buffers ND flag is fulfilled see 4.8.6 New Data 1/2/3/4 (NDAT1/2/3/4)), and if bit MBI of that message buffer is set to '1' see Table 17). 1 = At least one ND flag of a receive buffer with MBI = '1' has been set to '1' 0 = No ND flag of a receive buffer with MBI = '1' has been set to '1'"]
    #[inline(always)]
    #[must_use]
    pub fn rxi(&mut self) -> RXI_W<4> {
        RXI_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO Not Empty This flag is set by the CC when a received valid frame was stored into the empty receive FIFO. The actual state of the receive FIFO is monitored in register FSR. 1 = Receive FIFO is not empty 0 = Receive FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn rfne(&mut self) -> RFNE_W<5> {
        RFNE_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO Critical Level This flag is set when the receive FIFO fill level FSR.RFFL\\[7:0\\]
is equal or greater than the critical level as configured by FCL.CL\\[7:0\\]. 1 = Receive FIFO critical level reached 0 = Receive FIFO below critical level"]
    #[inline(always)]
    #[must_use]
    pub fn rfcl(&mut self) -> RFCL_W<6> {
        RFCL_W::new(self)
    }
    #[doc = "Bit 7 - Network Management Vector Changed This interrupt flag signals a change in the Network Management Vector visible to the Host. 1 = Network management vector changed 0 = No change in the network management vector"]
    #[inline(always)]
    #[must_use]
    pub fn nmvc(&mut self) -> NMVC_W<7> {
        NMVC_W::new(self)
    }
    #[doc = "Bit 8 - Timer Interrupt 0 This flag is set whenever timer 0 matches the conditions configured in register T0C. A Timer Interrupt 0 is also signalled on pin eray_tint0. 1 = Timer Interrupt 0 occurred 0 = No Timer Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn ti0(&mut self) -> TI0_W<8> {
        TI0_W::new(self)
    }
    #[doc = "Bit 9 - Timer Interrupt 1 This flag is set whenever timer 1 matches the conditions configured in register T1C. A Timer Interrupt 1 is also signalled on pin eray_tint1. 1 = Timer Interrupt 1 occurred 0 = No Timer Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn ti1(&mut self) -> TI1_W<9> {
        TI1_W::new(self)
    }
    #[doc = "Bit 10 - Transfer Input Buffer Completed This flag is set whenever a transfer from Input Buffer to the Message RAM has completed and IBCR.IBSYS has been reset by the Message Handler. 1 = Transfer between Input Buffer and Message RAM completed 0 = No transfer completed"]
    #[inline(always)]
    #[must_use]
    pub fn tibc(&mut self) -> TIBC_W<10> {
        TIBC_W::new(self)
    }
    #[doc = "Bit 11 - Transfer Output Buffer Completed This flag is set whenever a transfer from the Message RAM to the Output Buffer has completed and OBCR.OBSYS has been reset by the Message Handler. 1 = Transfer between Message RAM and Output Buffer completed 0 = No transfer completed"]
    #[inline(always)]
    #[must_use]
    pub fn tobc(&mut self) -> TOBC_W<11> {
        TOBC_W::new(self)
    }
    #[doc = "Bit 12 - Stop Watch Event This flag is set after a stop watch activation when the actual cycle counter and macrotick value are stored in the Stop Watch register (see \\[01\\]Section 4.4.10 Stop Watch Register 1 (STPW1)). 1 = Stop Watch Event occurred 0 = No Stop Watch Event"]
    #[inline(always)]
    #[must_use]
    pub fn swe(&mut self) -> SWE_W<12> {
        SWE_W::new(self)
    }
    #[doc = "Bit 13 - Startup Completed Successfully This flag is set whenever a startup completed successfully and the CC entered NORMAL_ACTIVE state. 1 = Startup completed successfully 0 = No startup completed successfully"]
    #[inline(always)]
    #[must_use]
    pub fn sucs(&mut self) -> SUCS_W<13> {
        SUCS_W::new(self)
    }
    #[doc = "Bit 14 - Message Buffer Status Interrupt This flag is set by the CC when the message buffer status MBS has changed and if bit MBI of that message buffer is set (see Table 17). 1 = Message buffer status of at least one message buffer with MBI = '1' has changed 0 = No message buffer status change of message buffer with MBI = '1'"]
    #[inline(always)]
    #[must_use]
    pub fn mbsi(&mut self) -> MBSI_W<14> {
        MBSI_W::new(self)
    }
    #[doc = "Bit 15 - Start of Dynamic Segment This flag is set by the CC when the dynamic segment starts. 1 = Dynamic segment started 0 = Dynamic segment not yet started"]
    #[inline(always)]
    #[must_use]
    pub fn sds(&mut self) -> SDS_W<15> {
        SDS_W::new(self)
    }
    #[doc = "Bit 16 - Wakeup Pattern Channel A This flag is set by the CC when a wakeup pattern was received on channel A. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel A 0 = No wakeup pattern received on channel A"]
    #[inline(always)]
    #[must_use]
    pub fn wupa(&mut self) -> WUPA_W<16> {
        WUPA_W::new(self)
    }
    #[doc = "Bit 17 - MTS Received on Channel A (vSS!ValidMTSA) Media Access Test symbol received on channel A during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel A 0 = No MTS symbol received on channel A"]
    #[inline(always)]
    #[must_use]
    pub fn mtsa(&mut self) -> MTSA_W<17> {
        MTSA_W::new(self)
    }
    #[doc = "Bit 24 - Wakeup Pattern Channel B This flag is set by the CC when a wakeup pattern was received on channel B. Only set when the CC is in WAKEUP, READY, or STARTUP state, or when in Monitor mode. 1 = Wakeup pattern received on channel B 0 = No wakeup pattern received on channel B"]
    #[inline(always)]
    #[must_use]
    pub fn wupb(&mut self) -> WUPB_W<24> {
        WUPB_W::new(self)
    }
    #[doc = "Bit 25 - MTS Received on Channel B (vSS!ValidMTSB) Media Access Test symbol received on channel B during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. 1 = MTS symbol received on channel B 0 = No MTS symbol received on channel B"]
    #[inline(always)]
    #[must_use]
    pub fn mtsb(&mut self) -> MTSB_W<25> {
        MTSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sir](index.html) module"]
pub struct SIR_SPEC;
impl crate::RegisterSpec for SIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sir::R](R) reader structure"]
impl crate::Readable for SIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sir::W](W) writer structure"]
impl crate::Writable for SIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIR to value 0"]
impl crate::Resettable for SIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
