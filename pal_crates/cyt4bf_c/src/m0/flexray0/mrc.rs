#[doc = "Register `MRC` reader"]
pub struct R(crate::R<MRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRC` writer"]
pub struct W(crate::W<MRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRC_SPEC>;
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
impl From<crate::W<MRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDB` reader - N/A"]
pub type FDB_R = crate::FieldReader<u8, FDB_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDB_A {
    #[doc = "128: N/A"]
    MAX = 128,
}
impl From<FDB_A> for u8 {
    #[inline(always)]
    fn from(variant: FDB_A) -> Self {
        variant as _
    }
}
impl FDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FDB_A> {
        match self.bits {
            128 => Some(FDB_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == FDB_A::MAX
    }
}
#[doc = "Field `FDB` writer - N/A"]
pub type FDB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRC_SPEC, u8, FDB_A, 8, O>;
impl<'a, const O: u8> FDB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(FDB_A::MAX)
    }
}
#[doc = "Field `FFB` reader - First Buffer of FIFO 0 = All message buffers assigned to the FIFO 1...127 = Message buffers from FFB to LCB assigned to the FIFO 128 = No message buffer assigned to the FIFO"]
pub type FFB_R = crate::FieldReader<u8, FFB_A>;
#[doc = "First Buffer of FIFO 0 = All message buffers assigned to the FIFO 1...127 = Message buffers from FFB to LCB assigned to the FIFO 128 = No message buffer assigned to the FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FFB_A {
    #[doc = "128: N/A"]
    MAX = 128,
}
impl From<FFB_A> for u8 {
    #[inline(always)]
    fn from(variant: FFB_A) -> Self {
        variant as _
    }
}
impl FFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FFB_A> {
        match self.bits {
            128 => Some(FFB_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == FFB_A::MAX
    }
}
#[doc = "Field `FFB` writer - First Buffer of FIFO 0 = All message buffers assigned to the FIFO 1...127 = Message buffers from FFB to LCB assigned to the FIFO 128 = No message buffer assigned to the FIFO"]
pub type FFB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRC_SPEC, u8, FFB_A, 8, O>;
impl<'a, const O: u8> FFB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(FFB_A::MAX)
    }
}
#[doc = "Field `LCB` reader - Last Configured Buffer 0...127 = Number of message buffers is LCB + 1 128 = No message buffer configured"]
pub type LCB_R = crate::FieldReader<u8, LCB_A>;
#[doc = "Last Configured Buffer 0...127 = Number of message buffers is LCB + 1 128 = No message buffer configured\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LCB_A {
    #[doc = "128: N/A"]
    MAX = 128,
}
impl From<LCB_A> for u8 {
    #[inline(always)]
    fn from(variant: LCB_A) -> Self {
        variant as _
    }
}
impl LCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LCB_A> {
        match self.bits {
            128 => Some(LCB_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == LCB_A::MAX
    }
}
#[doc = "Field `LCB` writer - Last Configured Buffer 0...127 = Number of message buffers is LCB + 1 128 = No message buffer configured"]
pub type LCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRC_SPEC, u8, LCB_A, 8, O>;
impl<'a, const O: u8> LCB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(LCB_A::MAX)
    }
}
#[doc = "Field `SEC` reader - Secure Buffers Not evaluated when the CC is in DEFAULT_CONFIG or CONFIG state. For temporary unlocking see 5.12.4 Host Handling of Parity Errors. 00 = Reconfiguration of message buffers enabled with numbers &lt; FFB enabled Exception: In nodes configured for sync frame transmission or for single slot mode operation message buffer 0 (and if SPLM = '1', also message buffer 1) is always locked 01 = Reconfiguration of message buffers with numbers &lt; FDB and with numbers >= FFB locked and transmission of message buffers for static segment with numbers >= FDB disabled 10 = Reconfiguration of all message buffers locked 11 = Reconfiguration of all message buffers locked and transmission of message buffers for static segment with numbers => FDB disabled"]
pub type SEC_R = crate::FieldReader<u8, SEC_A>;
#[doc = "Secure Buffers Not evaluated when the CC is in DEFAULT_CONFIG or CONFIG state. For temporary unlocking see 5.12.4 Host Handling of Parity Errors. 00 = Reconfiguration of message buffers enabled with numbers &lt; FFB enabled Exception: In nodes configured for sync frame transmission or for single slot mode operation message buffer 0 (and if SPLM = '1', also message buffer 1) is always locked 01 = Reconfiguration of message buffers with numbers &lt; FDB and with numbers >= FFB locked and transmission of message buffers for static segment with numbers >= FDB disabled 10 = Reconfiguration of all message buffers locked 11 = Reconfiguration of all message buffers locked and transmission of message buffers for static segment with numbers => FDB disabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_A {
    #[doc = "0: N/A"]
    MSG_BUFFERS_LT_FFB_ENABLED = 0,
    #[doc = "1: N/A"]
    MSG_BUFFERS_LT_FDB_ENABLED_AND_GT_FFB_LOCKED = 1,
    #[doc = "2: N/A"]
    ALL_MSG_BUFFERS_LOCKED = 2,
    #[doc = "3: N/A"]
    ALL_MSG_BUFFERS_LOCKED_AND_TXMN_OF_GT_FDB_LOCKED = 3,
}
impl From<SEC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as _
    }
}
impl SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_A {
        match self.bits {
            0 => SEC_A::MSG_BUFFERS_LT_FFB_ENABLED,
            1 => SEC_A::MSG_BUFFERS_LT_FDB_ENABLED_AND_GT_FFB_LOCKED,
            2 => SEC_A::ALL_MSG_BUFFERS_LOCKED,
            3 => SEC_A::ALL_MSG_BUFFERS_LOCKED_AND_TXMN_OF_GT_FDB_LOCKED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSG_BUFFERS_LT_FFB_ENABLED`"]
    #[inline(always)]
    pub fn is_msg_buffers_lt_ffb_enabled(&self) -> bool {
        *self == SEC_A::MSG_BUFFERS_LT_FFB_ENABLED
    }
    #[doc = "Checks if the value of the field is `MSG_BUFFERS_LT_FDB_ENABLED_AND_GT_FFB_LOCKED`"]
    #[inline(always)]
    pub fn is_msg_buffers_lt_fdb_enabled_and_gt_ffb_locked(&self) -> bool {
        *self == SEC_A::MSG_BUFFERS_LT_FDB_ENABLED_AND_GT_FFB_LOCKED
    }
    #[doc = "Checks if the value of the field is `ALL_MSG_BUFFERS_LOCKED`"]
    #[inline(always)]
    pub fn is_all_msg_buffers_locked(&self) -> bool {
        *self == SEC_A::ALL_MSG_BUFFERS_LOCKED
    }
    #[doc = "Checks if the value of the field is `ALL_MSG_BUFFERS_LOCKED_AND_TXMN_OF_GT_FDB_LOCKED`"]
    #[inline(always)]
    pub fn is_all_msg_buffers_locked_and_txmn_of_gt_fdb_locked(&self) -> bool {
        *self == SEC_A::ALL_MSG_BUFFERS_LOCKED_AND_TXMN_OF_GT_FDB_LOCKED
    }
}
#[doc = "Field `SEC` writer - Secure Buffers Not evaluated when the CC is in DEFAULT_CONFIG or CONFIG state. For temporary unlocking see 5.12.4 Host Handling of Parity Errors. 00 = Reconfiguration of message buffers enabled with numbers &lt; FFB enabled Exception: In nodes configured for sync frame transmission or for single slot mode operation message buffer 0 (and if SPLM = '1', also message buffer 1) is always locked 01 = Reconfiguration of message buffers with numbers &lt; FDB and with numbers >= FFB locked and transmission of message buffers for static segment with numbers >= FDB disabled 10 = Reconfiguration of all message buffers locked 11 = Reconfiguration of all message buffers locked and transmission of message buffers for static segment with numbers => FDB disabled"]
pub type SEC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MRC_SPEC, u8, SEC_A, 2, O>;
impl<'a, const O: u8> SEC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msg_buffers_lt_ffb_enabled(self) -> &'a mut W {
        self.variant(SEC_A::MSG_BUFFERS_LT_FFB_ENABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msg_buffers_lt_fdb_enabled_and_gt_ffb_locked(self) -> &'a mut W {
        self.variant(SEC_A::MSG_BUFFERS_LT_FDB_ENABLED_AND_GT_FFB_LOCKED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn all_msg_buffers_locked(self) -> &'a mut W {
        self.variant(SEC_A::ALL_MSG_BUFFERS_LOCKED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn all_msg_buffers_locked_and_txmn_of_gt_fdb_locked(self) -> &'a mut W {
        self.variant(SEC_A::ALL_MSG_BUFFERS_LOCKED_AND_TXMN_OF_GT_FDB_LOCKED)
    }
}
#[doc = "Field `SPLM` reader - Sync Frame Payload Multiplex This bit is only evaluated if the node is configured as sync node (SUCC1.TXSY = '1') or for single slot mode operation (SUCC1.TSM = '1'). When this bit is set to '1' message buffers 0 and 1 are dedicated for sync frame transmission with different payload data on channel A and B. When this bit is set to '0', sync frames are transmitted from message buffer 0 with the same payload data on both channels. Note that the channel filter configuration for message buffer 0 resp. message buffer 1 has to be chosen accordingly. 1 = Both message buffers 0 and 1 are locked against reconfiguration 0 = Only message buffer 0 locked against reconfiguration"]
pub type SPLM_R = crate::BitReader<SPLM_A>;
#[doc = "Sync Frame Payload Multiplex This bit is only evaluated if the node is configured as sync node (SUCC1.TXSY = '1') or for single slot mode operation (SUCC1.TSM = '1'). When this bit is set to '1' message buffers 0 and 1 are dedicated for sync frame transmission with different payload data on channel A and B. When this bit is set to '0', sync frames are transmitted from message buffer 0 with the same payload data on both channels. Note that the channel filter configuration for message buffer 0 resp. message buffer 1 has to be chosen accordingly. 1 = Both message buffers 0 and 1 are locked against reconfiguration 0 = Only message buffer 0 locked against reconfiguration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLM_A {
    #[doc = "0: N/A"]
    MSG_BUFFER_0_LOCKED = 0,
    #[doc = "1: N/A"]
    MSG_BUFFER_0_AND_1_LOCKED = 1,
}
impl From<SPLM_A> for bool {
    #[inline(always)]
    fn from(variant: SPLM_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLM_A {
        match self.bits {
            false => SPLM_A::MSG_BUFFER_0_LOCKED,
            true => SPLM_A::MSG_BUFFER_0_AND_1_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `MSG_BUFFER_0_LOCKED`"]
    #[inline(always)]
    pub fn is_msg_buffer_0_locked(&self) -> bool {
        *self == SPLM_A::MSG_BUFFER_0_LOCKED
    }
    #[doc = "Checks if the value of the field is `MSG_BUFFER_0_AND_1_LOCKED`"]
    #[inline(always)]
    pub fn is_msg_buffer_0_and_1_locked(&self) -> bool {
        *self == SPLM_A::MSG_BUFFER_0_AND_1_LOCKED
    }
}
#[doc = "Field `SPLM` writer - Sync Frame Payload Multiplex This bit is only evaluated if the node is configured as sync node (SUCC1.TXSY = '1') or for single slot mode operation (SUCC1.TSM = '1'). When this bit is set to '1' message buffers 0 and 1 are dedicated for sync frame transmission with different payload data on channel A and B. When this bit is set to '0', sync frames are transmitted from message buffer 0 with the same payload data on both channels. Note that the channel filter configuration for message buffer 0 resp. message buffer 1 has to be chosen accordingly. 1 = Both message buffers 0 and 1 are locked against reconfiguration 0 = Only message buffer 0 locked against reconfiguration"]
pub type SPLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRC_SPEC, SPLM_A, O>;
impl<'a, const O: u8> SPLM_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msg_buffer_0_locked(self) -> &'a mut W {
        self.variant(SPLM_A::MSG_BUFFER_0_LOCKED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msg_buffer_0_and_1_locked(self) -> &'a mut W {
        self.variant(SPLM_A::MSG_BUFFER_0_AND_1_LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn fdb(&self) -> FDB_R {
        FDB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - First Buffer of FIFO 0 = All message buffers assigned to the FIFO 1...127 = Message buffers from FFB to LCB assigned to the FIFO 128 = No message buffer assigned to the FIFO"]
    #[inline(always)]
    pub fn ffb(&self) -> FFB_R {
        FFB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Last Configured Buffer 0...127 = Number of message buffers is LCB + 1 128 = No message buffer configured"]
    #[inline(always)]
    pub fn lcb(&self) -> LCB_R {
        LCB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Secure Buffers Not evaluated when the CC is in DEFAULT_CONFIG or CONFIG state. For temporary unlocking see 5.12.4 Host Handling of Parity Errors. 00 = Reconfiguration of message buffers enabled with numbers &lt; FFB enabled Exception: In nodes configured for sync frame transmission or for single slot mode operation message buffer 0 (and if SPLM = '1', also message buffer 1) is always locked 01 = Reconfiguration of message buffers with numbers &lt; FDB and with numbers >= FFB locked and transmission of message buffers for static segment with numbers >= FDB disabled 10 = Reconfiguration of all message buffers locked 11 = Reconfiguration of all message buffers locked and transmission of message buffers for static segment with numbers => FDB disabled"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Sync Frame Payload Multiplex This bit is only evaluated if the node is configured as sync node (SUCC1.TXSY = '1') or for single slot mode operation (SUCC1.TSM = '1'). When this bit is set to '1' message buffers 0 and 1 are dedicated for sync frame transmission with different payload data on channel A and B. When this bit is set to '0', sync frames are transmitted from message buffer 0 with the same payload data on both channels. Note that the channel filter configuration for message buffer 0 resp. message buffer 1 has to be chosen accordingly. 1 = Both message buffers 0 and 1 are locked against reconfiguration 0 = Only message buffer 0 locked against reconfiguration"]
    #[inline(always)]
    pub fn splm(&self) -> SPLM_R {
        SPLM_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fdb(&mut self) -> FDB_W<0> {
        FDB_W::new(self)
    }
    #[doc = "Bits 8:15 - First Buffer of FIFO 0 = All message buffers assigned to the FIFO 1...127 = Message buffers from FFB to LCB assigned to the FIFO 128 = No message buffer assigned to the FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ffb(&mut self) -> FFB_W<8> {
        FFB_W::new(self)
    }
    #[doc = "Bits 16:23 - Last Configured Buffer 0...127 = Number of message buffers is LCB + 1 128 = No message buffer configured"]
    #[inline(always)]
    #[must_use]
    pub fn lcb(&mut self) -> LCB_W<16> {
        LCB_W::new(self)
    }
    #[doc = "Bits 24:25 - Secure Buffers Not evaluated when the CC is in DEFAULT_CONFIG or CONFIG state. For temporary unlocking see 5.12.4 Host Handling of Parity Errors. 00 = Reconfiguration of message buffers enabled with numbers &lt; FFB enabled Exception: In nodes configured for sync frame transmission or for single slot mode operation message buffer 0 (and if SPLM = '1', also message buffer 1) is always locked 01 = Reconfiguration of message buffers with numbers &lt; FDB and with numbers >= FFB locked and transmission of message buffers for static segment with numbers >= FDB disabled 10 = Reconfiguration of all message buffers locked 11 = Reconfiguration of all message buffers locked and transmission of message buffers for static segment with numbers => FDB disabled"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<24> {
        SEC_W::new(self)
    }
    #[doc = "Bit 26 - Sync Frame Payload Multiplex This bit is only evaluated if the node is configured as sync node (SUCC1.TXSY = '1') or for single slot mode operation (SUCC1.TSM = '1'). When this bit is set to '1' message buffers 0 and 1 are dedicated for sync frame transmission with different payload data on channel A and B. When this bit is set to '0', sync frames are transmitted from message buffer 0 with the same payload data on both channels. Note that the channel filter configuration for message buffer 0 resp. message buffer 1 has to be chosen accordingly. 1 = Both message buffers 0 and 1 are locked against reconfiguration 0 = Only message buffer 0 locked against reconfiguration"]
    #[inline(always)]
    #[must_use]
    pub fn splm(&mut self) -> SPLM_W<26> {
        SPLM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message RAM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrc](index.html) module"]
pub struct MRC_SPEC;
impl crate::RegisterSpec for MRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrc::R](R) reader structure"]
impl crate::Readable for MRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrc::W](W) writer structure"]
impl crate::Writable for MRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRC to value 0x0180_0000"]
impl crate::Resettable for MRC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0180_0000;
}
