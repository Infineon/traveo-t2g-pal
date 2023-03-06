#[doc = "Register `WRHS1_MIR2` reader"]
pub struct R(crate::R<WRHS1_MIR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRHS1_MIR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRHS1_MIR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRHS1_MIR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRHS1_MIR2` writer"]
pub struct W(crate::W<WRHS1_MIR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRHS1_MIR2_SPEC>;
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
impl From<crate::W<WRHS1_MIR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRHS1_MIR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FID` reader - Frame ID Frame ID of the selected message buffer. The frame ID defines the slot number for transmission reception of the respective message. Message buffers with frame ID = '0' are considered as not valid."]
pub type FID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FID` writer - Frame ID Frame ID of the selected message buffer. The frame ID defines the slot number for transmission reception of the respective message. Message buffers with frame ID = '0' are considered as not valid."]
pub type FID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRHS1_MIR2_SPEC, u16, u16, 11, O>;
#[doc = "Field `CYC` reader - Cycle Code The 7-bit cycle code determines the cycle set used for cycle counter filtering. For details about the configuration of the cycle code see \\[01\\]Section 5.7.2 Cycle Counter Filtering. CHA, CHB Channel Filter Control The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. CHA CHB. Transmit Buffer transmit frame on Receive Buffer store frame received from 1 1 both channels static segment only) channel A or B store first semantically valid frame, static segment only) 1 0 channel A channel A 0 1 channel B channel B 0 0 no transmission ignore frame"]
pub type CYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CYC` writer - Cycle Code The 7-bit cycle code determines the cycle set used for cycle counter filtering. For details about the configuration of the cycle code see \\[01\\]Section 5.7.2 Cycle Counter Filtering. CHA, CHB Channel Filter Control The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. CHA CHB. Transmit Buffer transmit frame on Receive Buffer store frame received from 1 1 both channels static segment only) channel A or B store first semantically valid frame, static segment only) 1 0 channel A channel A 0 1 channel B channel B 0 0 no transmission ignore frame"]
pub type CYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRHS1_MIR2_SPEC, u8, u8, 7, O>;
#[doc = "Field `CHA` reader - Channel Filter Control A The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')"]
pub type CHA_R = crate::BitReader<CHA_A>;
#[doc = "Channel Filter Control A The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHA_A {
    #[doc = "0: N/A"]
    CH_A_DISABLED = 0,
    #[doc = "1: N/A"]
    CH_A_ENABLED = 1,
}
impl From<CHA_A> for bool {
    #[inline(always)]
    fn from(variant: CHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHA_A {
        match self.bits {
            false => CHA_A::CH_A_DISABLED,
            true => CHA_A::CH_A_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CH_A_DISABLED`"]
    #[inline(always)]
    pub fn is_ch_a_disabled(&self) -> bool {
        *self == CHA_A::CH_A_DISABLED
    }
    #[doc = "Checks if the value of the field is `CH_A_ENABLED`"]
    #[inline(always)]
    pub fn is_ch_a_enabled(&self) -> bool {
        *self == CHA_A::CH_A_ENABLED
    }
}
#[doc = "Field `CHA` writer - Channel Filter Control A The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')"]
pub type CHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRHS1_MIR2_SPEC, CHA_A, O>;
impl<'a, const O: u8> CHA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_disabled(self) -> &'a mut W {
        self.variant(CHA_A::CH_A_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_enabled(self) -> &'a mut W {
        self.variant(CHA_A::CH_A_ENABLED)
    }
}
#[doc = "Field `CHB` reader - Channel Filter Control B The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')"]
pub type CHB_R = crate::BitReader<CHB_A>;
#[doc = "Channel Filter Control B The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHB_A {
    #[doc = "0: N/A"]
    CH_B_DISABLED = 0,
    #[doc = "1: N/A"]
    CH_B_ENABLED = 1,
}
impl From<CHB_A> for bool {
    #[inline(always)]
    fn from(variant: CHB_A) -> Self {
        variant as u8 != 0
    }
}
impl CHB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHB_A {
        match self.bits {
            false => CHB_A::CH_B_DISABLED,
            true => CHB_A::CH_B_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CH_B_DISABLED`"]
    #[inline(always)]
    pub fn is_ch_b_disabled(&self) -> bool {
        *self == CHB_A::CH_B_DISABLED
    }
    #[doc = "Checks if the value of the field is `CH_B_ENABLED`"]
    #[inline(always)]
    pub fn is_ch_b_enabled(&self) -> bool {
        *self == CHB_A::CH_B_ENABLED
    }
}
#[doc = "Field `CHB` writer - Channel Filter Control B The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')"]
pub type CHB_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRHS1_MIR2_SPEC, CHB_A, O>;
impl<'a, const O: u8> CHB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_disabled(self) -> &'a mut W {
        self.variant(CHB_A::CH_B_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_enabled(self) -> &'a mut W {
        self.variant(CHB_A::CH_B_ENABLED)
    }
}
#[doc = "Field `CFG` reader - Message Buffer Direction Configuration Bit This bit is used to configure the corresponding buffer as transmit buffer or as receive buffer. For message buffers belonging to the receive FIFO the bit is not evaluated. 1 = The corresponding buffer is configured as Transmit Buffer 0 = The corresponding buffer is configured as Receive Buffer"]
pub type CFG_R = crate::BitReader<CFG_A>;
#[doc = "Message Buffer Direction Configuration Bit This bit is used to configure the corresponding buffer as transmit buffer or as receive buffer. For message buffers belonging to the receive FIFO the bit is not evaluated. 1 = The corresponding buffer is configured as Transmit Buffer 0 = The corresponding buffer is configured as Receive Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFG_A {
    #[doc = "0: N/A"]
    RX_BUFF = 0,
    #[doc = "1: N/A"]
    TX_BUFF = 1,
}
impl From<CFG_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_A) -> Self {
        variant as u8 != 0
    }
}
impl CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG_A {
        match self.bits {
            false => CFG_A::RX_BUFF,
            true => CFG_A::TX_BUFF,
        }
    }
    #[doc = "Checks if the value of the field is `RX_BUFF`"]
    #[inline(always)]
    pub fn is_rx_buff(&self) -> bool {
        *self == CFG_A::RX_BUFF
    }
    #[doc = "Checks if the value of the field is `TX_BUFF`"]
    #[inline(always)]
    pub fn is_tx_buff(&self) -> bool {
        *self == CFG_A::TX_BUFF
    }
}
#[doc = "Field `CFG` writer - Message Buffer Direction Configuration Bit This bit is used to configure the corresponding buffer as transmit buffer or as receive buffer. For message buffers belonging to the receive FIFO the bit is not evaluated. 1 = The corresponding buffer is configured as Transmit Buffer 0 = The corresponding buffer is configured as Receive Buffer"]
pub type CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRHS1_MIR2_SPEC, CFG_A, O>;
impl<'a, const O: u8> CFG_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rx_buff(self) -> &'a mut W {
        self.variant(CFG_A::RX_BUFF)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tx_buff(self) -> &'a mut W {
        self.variant(CFG_A::TX_BUFF)
    }
}
#[doc = "Field `PPIT` reader - Payload Preamble Indicator Transmit This bit is used to control the state of the Payload Preamble Indicator in transmit frames. If the bit is set in a static message buffer, the respective message buffer holds network management information. If the bit is set in a dynamic message buffer the first two bytes of the payload segment may be used for message ID filtering by the receiver. Message ID filtering of received FlexRay frames is not supported by the E-Ray module, but can be done by the Host. 1 = Payload Preamble Indicator set 0 = Payload Preamble Indicator not set"]
pub type PPIT_R = crate::BitReader<PPIT_A>;
#[doc = "Payload Preamble Indicator Transmit This bit is used to control the state of the Payload Preamble Indicator in transmit frames. If the bit is set in a static message buffer, the respective message buffer holds network management information. If the bit is set in a dynamic message buffer the first two bytes of the payload segment may be used for message ID filtering by the receiver. Message ID filtering of received FlexRay frames is not supported by the E-Ray module, but can be done by the Host. 1 = Payload Preamble Indicator set 0 = Payload Preamble Indicator not set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPIT_A {
    #[doc = "0: N/A"]
    PAYLOAD_PREAMBLE_NOT_SET = 0,
    #[doc = "1: N/A"]
    PAYLOAD_PREAMBLE_SET = 1,
}
impl From<PPIT_A> for bool {
    #[inline(always)]
    fn from(variant: PPIT_A) -> Self {
        variant as u8 != 0
    }
}
impl PPIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPIT_A {
        match self.bits {
            false => PPIT_A::PAYLOAD_PREAMBLE_NOT_SET,
            true => PPIT_A::PAYLOAD_PREAMBLE_SET,
        }
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_PREAMBLE_NOT_SET`"]
    #[inline(always)]
    pub fn is_payload_preamble_not_set(&self) -> bool {
        *self == PPIT_A::PAYLOAD_PREAMBLE_NOT_SET
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_PREAMBLE_SET`"]
    #[inline(always)]
    pub fn is_payload_preamble_set(&self) -> bool {
        *self == PPIT_A::PAYLOAD_PREAMBLE_SET
    }
}
#[doc = "Field `PPIT` writer - Payload Preamble Indicator Transmit This bit is used to control the state of the Payload Preamble Indicator in transmit frames. If the bit is set in a static message buffer, the respective message buffer holds network management information. If the bit is set in a dynamic message buffer the first two bytes of the payload segment may be used for message ID filtering by the receiver. Message ID filtering of received FlexRay frames is not supported by the E-Ray module, but can be done by the Host. 1 = Payload Preamble Indicator set 0 = Payload Preamble Indicator not set"]
pub type PPIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRHS1_MIR2_SPEC, PPIT_A, O>;
impl<'a, const O: u8> PPIT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn payload_preamble_not_set(self) -> &'a mut W {
        self.variant(PPIT_A::PAYLOAD_PREAMBLE_NOT_SET)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn payload_preamble_set(self) -> &'a mut W {
        self.variant(PPIT_A::PAYLOAD_PREAMBLE_SET)
    }
}
#[doc = "Field `TXM` reader - Transmission Mode This bit is used to select the transmission mode (see \\[01\\]Section 5.8.3 Transmit Buffers). 1 = Single-shot mode 0 = Continuous mode"]
pub type TXM_R = crate::BitReader<TXM_A>;
#[doc = "Transmission Mode This bit is used to select the transmission mode (see \\[01\\]Section 5.8.3 Transmit Buffers). 1 = Single-shot mode 0 = Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXM_A {
    #[doc = "0: N/A"]
    CONTINUOUS_MODE = 0,
    #[doc = "1: N/A"]
    SINGLE_SHOT_MODE = 1,
}
impl From<TXM_A> for bool {
    #[inline(always)]
    fn from(variant: TXM_A) -> Self {
        variant as u8 != 0
    }
}
impl TXM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXM_A {
        match self.bits {
            false => TXM_A::CONTINUOUS_MODE,
            true => TXM_A::SINGLE_SHOT_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS_MODE`"]
    #[inline(always)]
    pub fn is_continuous_mode(&self) -> bool {
        *self == TXM_A::CONTINUOUS_MODE
    }
    #[doc = "Checks if the value of the field is `SINGLE_SHOT_MODE`"]
    #[inline(always)]
    pub fn is_single_shot_mode(&self) -> bool {
        *self == TXM_A::SINGLE_SHOT_MODE
    }
}
#[doc = "Field `TXM` writer - Transmission Mode This bit is used to select the transmission mode (see \\[01\\]Section 5.8.3 Transmit Buffers). 1 = Single-shot mode 0 = Continuous mode"]
pub type TXM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRHS1_MIR2_SPEC, TXM_A, O>;
impl<'a, const O: u8> TXM_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn continuous_mode(self) -> &'a mut W {
        self.variant(TXM_A::CONTINUOUS_MODE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn single_shot_mode(self) -> &'a mut W {
        self.variant(TXM_A::SINGLE_SHOT_MODE)
    }
}
#[doc = "Field `MBI` reader - Message Buffer Interrupt This bit enables the receive / transmit interrupt for the corresponding message buffer. After a dedicated receive buffer has been updated by the Message Handler, flag SIR.RXI and /or SIR.MBSI are set. After a transmission has completed flag SIR.TXI is set. 1 = The corresponding message buffer interrupt is enabled 0 = The corresponding message buffer interrupt is disabled"]
pub type MBI_R = crate::BitReader<MBI_A>;
#[doc = "Message Buffer Interrupt This bit enables the receive / transmit interrupt for the corresponding message buffer. After a dedicated receive buffer has been updated by the Message Handler, flag SIR.RXI and /or SIR.MBSI are set. After a transmission has completed flag SIR.TXI is set. 1 = The corresponding message buffer interrupt is enabled 0 = The corresponding message buffer interrupt is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBI_A {
    #[doc = "0: N/A"]
    MSG_BUFF_INTR_DISABLED = 0,
    #[doc = "1: N/A"]
    MSG_BUFF_INTR_ENABLED = 1,
}
impl From<MBI_A> for bool {
    #[inline(always)]
    fn from(variant: MBI_A) -> Self {
        variant as u8 != 0
    }
}
impl MBI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBI_A {
        match self.bits {
            false => MBI_A::MSG_BUFF_INTR_DISABLED,
            true => MBI_A::MSG_BUFF_INTR_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `MSG_BUFF_INTR_DISABLED`"]
    #[inline(always)]
    pub fn is_msg_buff_intr_disabled(&self) -> bool {
        *self == MBI_A::MSG_BUFF_INTR_DISABLED
    }
    #[doc = "Checks if the value of the field is `MSG_BUFF_INTR_ENABLED`"]
    #[inline(always)]
    pub fn is_msg_buff_intr_enabled(&self) -> bool {
        *self == MBI_A::MSG_BUFF_INTR_ENABLED
    }
}
#[doc = "Field `MBI` writer - Message Buffer Interrupt This bit enables the receive / transmit interrupt for the corresponding message buffer. After a dedicated receive buffer has been updated by the Message Handler, flag SIR.RXI and /or SIR.MBSI are set. After a transmission has completed flag SIR.TXI is set. 1 = The corresponding message buffer interrupt is enabled 0 = The corresponding message buffer interrupt is disabled"]
pub type MBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRHS1_MIR2_SPEC, MBI_A, O>;
impl<'a, const O: u8> MBI_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msg_buff_intr_disabled(self) -> &'a mut W {
        self.variant(MBI_A::MSG_BUFF_INTR_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msg_buff_intr_enabled(self) -> &'a mut W {
        self.variant(MBI_A::MSG_BUFF_INTR_ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame ID Frame ID of the selected message buffer. The frame ID defines the slot number for transmission reception of the respective message. Message buffers with frame ID = '0' are considered as not valid."]
    #[inline(always)]
    pub fn fid(&self) -> FID_R {
        FID_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Cycle Code The 7-bit cycle code determines the cycle set used for cycle counter filtering. For details about the configuration of the cycle code see \\[01\\]Section 5.7.2 Cycle Counter Filtering. CHA, CHB Channel Filter Control The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. CHA CHB. Transmit Buffer transmit frame on Receive Buffer store frame received from 1 1 both channels static segment only) channel A or B store first semantically valid frame, static segment only) 1 0 channel A channel A 0 1 channel B channel B 0 0 no transmission ignore frame"]
    #[inline(always)]
    pub fn cyc(&self) -> CYC_R {
        CYC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Channel Filter Control A The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')"]
    #[inline(always)]
    pub fn cha(&self) -> CHA_R {
        CHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel Filter Control B The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')"]
    #[inline(always)]
    pub fn chb(&self) -> CHB_R {
        CHB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Message Buffer Direction Configuration Bit This bit is used to configure the corresponding buffer as transmit buffer or as receive buffer. For message buffers belonging to the receive FIFO the bit is not evaluated. 1 = The corresponding buffer is configured as Transmit Buffer 0 = The corresponding buffer is configured as Receive Buffer"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Payload Preamble Indicator Transmit This bit is used to control the state of the Payload Preamble Indicator in transmit frames. If the bit is set in a static message buffer, the respective message buffer holds network management information. If the bit is set in a dynamic message buffer the first two bytes of the payload segment may be used for message ID filtering by the receiver. Message ID filtering of received FlexRay frames is not supported by the E-Ray module, but can be done by the Host. 1 = Payload Preamble Indicator set 0 = Payload Preamble Indicator not set"]
    #[inline(always)]
    pub fn ppit(&self) -> PPIT_R {
        PPIT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Mode This bit is used to select the transmission mode (see \\[01\\]Section 5.8.3 Transmit Buffers). 1 = Single-shot mode 0 = Continuous mode"]
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Message Buffer Interrupt This bit enables the receive / transmit interrupt for the corresponding message buffer. After a dedicated receive buffer has been updated by the Message Handler, flag SIR.RXI and /or SIR.MBSI are set. After a transmission has completed flag SIR.TXI is set. 1 = The corresponding message buffer interrupt is enabled 0 = The corresponding message buffer interrupt is disabled"]
    #[inline(always)]
    pub fn mbi(&self) -> MBI_R {
        MBI_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame ID Frame ID of the selected message buffer. The frame ID defines the slot number for transmission reception of the respective message. Message buffers with frame ID = '0' are considered as not valid."]
    #[inline(always)]
    #[must_use]
    pub fn fid(&mut self) -> FID_W<0> {
        FID_W::new(self)
    }
    #[doc = "Bits 16:22 - Cycle Code The 7-bit cycle code determines the cycle set used for cycle counter filtering. For details about the configuration of the cycle code see \\[01\\]Section 5.7.2 Cycle Counter Filtering. CHA, CHB Channel Filter Control The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. CHA CHB. Transmit Buffer transmit frame on Receive Buffer store frame received from 1 1 both channels static segment only) channel A or B store first semantically valid frame, static segment only) 1 0 channel A channel A 0 1 channel B channel B 0 0 no transmission ignore frame"]
    #[inline(always)]
    #[must_use]
    pub fn cyc(&mut self) -> CYC_W<16> {
        CYC_W::new(self)
    }
    #[doc = "Bit 24 - Channel Filter Control A The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')"]
    #[inline(always)]
    #[must_use]
    pub fn cha(&mut self) -> CHA_W<24> {
        CHA_W::new(self)
    }
    #[doc = "Bit 25 - Channel Filter Control B The 2-bit channel filtering field associated with each buffer serves as a filter for receive buffers, and as a control field for transmit buffers. Note: If a message buffer is configured for the dynamic segment and both bits of the channel filtering field are set to '1', no frames are transmitted resp. received frames are ignored (same function as CHA = CHB = '0')"]
    #[inline(always)]
    #[must_use]
    pub fn chb(&mut self) -> CHB_W<25> {
        CHB_W::new(self)
    }
    #[doc = "Bit 26 - Message Buffer Direction Configuration Bit This bit is used to configure the corresponding buffer as transmit buffer or as receive buffer. For message buffers belonging to the receive FIFO the bit is not evaluated. 1 = The corresponding buffer is configured as Transmit Buffer 0 = The corresponding buffer is configured as Receive Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<26> {
        CFG_W::new(self)
    }
    #[doc = "Bit 27 - Payload Preamble Indicator Transmit This bit is used to control the state of the Payload Preamble Indicator in transmit frames. If the bit is set in a static message buffer, the respective message buffer holds network management information. If the bit is set in a dynamic message buffer the first two bytes of the payload segment may be used for message ID filtering by the receiver. Message ID filtering of received FlexRay frames is not supported by the E-Ray module, but can be done by the Host. 1 = Payload Preamble Indicator set 0 = Payload Preamble Indicator not set"]
    #[inline(always)]
    #[must_use]
    pub fn ppit(&mut self) -> PPIT_W<27> {
        PPIT_W::new(self)
    }
    #[doc = "Bit 28 - Transmission Mode This bit is used to select the transmission mode (see \\[01\\]Section 5.8.3 Transmit Buffers). 1 = Single-shot mode 0 = Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn txm(&mut self) -> TXM_W<28> {
        TXM_W::new(self)
    }
    #[doc = "Bit 29 - Message Buffer Interrupt This bit enables the receive / transmit interrupt for the corresponding message buffer. After a dedicated receive buffer has been updated by the Message Handler, flag SIR.RXI and /or SIR.MBSI are set. After a transmission has completed flag SIR.TXI is set. 1 = The corresponding message buffer interrupt is enabled 0 = The corresponding message buffer interrupt is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn mbi(&mut self) -> MBI_W<29> {
        MBI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Header Section 1 (2nd mirror)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrhs1_mir2](index.html) module"]
pub struct WRHS1_MIR2_SPEC;
impl crate::RegisterSpec for WRHS1_MIR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrhs1_mir2::R](R) reader structure"]
impl crate::Readable for WRHS1_MIR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrhs1_mir2::W](W) writer structure"]
impl crate::Writable for WRHS1_MIR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRHS1_MIR2 to value 0"]
impl crate::Resettable for WRHS1_MIR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
