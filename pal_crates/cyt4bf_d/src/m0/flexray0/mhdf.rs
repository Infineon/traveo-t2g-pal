#[doc = "Register `MHDF` reader"]
pub struct R(crate::R<MHDF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MHDF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MHDF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MHDF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MHDF` writer"]
pub struct W(crate::W<MHDF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MHDF_SPEC>;
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
impl From<crate::W<MHDF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MHDF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNUA` reader - Status Not Updated Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel A. 1 = MBS for channel A not updated 0 = No overload condition occurred when updating MBS for channel A"]
pub type SNUA_R = crate::BitReader<SNUA_A>;
#[doc = "Status Not Updated Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel A. 1 = MBS for channel A not updated 0 = No overload condition occurred when updating MBS for channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNUA_A {
    #[doc = "0: N/A"]
    MBS_FOR_CH_A_UPDATED = 0,
    #[doc = "1: N/A"]
    MBS_FOR_CH_A_NOT_UPDATED = 1,
}
impl From<SNUA_A> for bool {
    #[inline(always)]
    fn from(variant: SNUA_A) -> Self {
        variant as u8 != 0
    }
}
impl SNUA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNUA_A {
        match self.bits {
            false => SNUA_A::MBS_FOR_CH_A_UPDATED,
            true => SNUA_A::MBS_FOR_CH_A_NOT_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `MBS_FOR_CH_A_UPDATED`"]
    #[inline(always)]
    pub fn is_mbs_for_ch_a_updated(&self) -> bool {
        *self == SNUA_A::MBS_FOR_CH_A_UPDATED
    }
    #[doc = "Checks if the value of the field is `MBS_FOR_CH_A_NOT_UPDATED`"]
    #[inline(always)]
    pub fn is_mbs_for_ch_a_not_updated(&self) -> bool {
        *self == SNUA_A::MBS_FOR_CH_A_NOT_UPDATED
    }
}
#[doc = "Field `SNUA` writer - Status Not Updated Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel A. 1 = MBS for channel A not updated 0 = No overload condition occurred when updating MBS for channel A"]
pub type SNUA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, SNUA_A, O>;
impl<'a, const O: u8> SNUA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn mbs_for_ch_a_updated(self) -> &'a mut W {
        self.variant(SNUA_A::MBS_FOR_CH_A_UPDATED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn mbs_for_ch_a_not_updated(self) -> &'a mut W {
        self.variant(SNUA_A::MBS_FOR_CH_A_NOT_UPDATED)
    }
}
#[doc = "Field `SNUB` reader - Status Not Updated Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel B. 1 = MBS for channel B not updated 0 = No overload condition occurred when updating MBS for channel B"]
pub type SNUB_R = crate::BitReader<SNUB_A>;
#[doc = "Status Not Updated Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel B. 1 = MBS for channel B not updated 0 = No overload condition occurred when updating MBS for channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNUB_A {
    #[doc = "0: N/A"]
    MBS_FOR_CH_B_UPDATED = 0,
    #[doc = "1: N/A"]
    MBS_FOR_CH_B_NOT_UPDATED = 1,
}
impl From<SNUB_A> for bool {
    #[inline(always)]
    fn from(variant: SNUB_A) -> Self {
        variant as u8 != 0
    }
}
impl SNUB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNUB_A {
        match self.bits {
            false => SNUB_A::MBS_FOR_CH_B_UPDATED,
            true => SNUB_A::MBS_FOR_CH_B_NOT_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `MBS_FOR_CH_B_UPDATED`"]
    #[inline(always)]
    pub fn is_mbs_for_ch_b_updated(&self) -> bool {
        *self == SNUB_A::MBS_FOR_CH_B_UPDATED
    }
    #[doc = "Checks if the value of the field is `MBS_FOR_CH_B_NOT_UPDATED`"]
    #[inline(always)]
    pub fn is_mbs_for_ch_b_not_updated(&self) -> bool {
        *self == SNUB_A::MBS_FOR_CH_B_NOT_UPDATED
    }
}
#[doc = "Field `SNUB` writer - Status Not Updated Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel B. 1 = MBS for channel B not updated 0 = No overload condition occurred when updating MBS for channel B"]
pub type SNUB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, SNUB_A, O>;
impl<'a, const O: u8> SNUB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn mbs_for_ch_b_updated(self) -> &'a mut W {
        self.variant(SNUB_A::MBS_FOR_CH_B_UPDATED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn mbs_for_ch_b_not_updated(self) -> &'a mut W {
        self.variant(SNUB_A::MBS_FOR_CH_B_NOT_UPDATED)
    }
}
#[doc = "Field `FNFA` reader - Find Sequence Not Finished Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel A. 1 = Find sequence not finished for channel A 0 = No find sequence not finished for channel A"]
pub type FNFA_R = crate::BitReader<FNFA_A>;
#[doc = "Find Sequence Not Finished Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel A. 1 = Find sequence not finished for channel A 0 = No find sequence not finished for channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FNFA_A {
    #[doc = "0: N/A"]
    NO_CH_A_FIND_SEQ_NOT_FINISHED = 0,
    #[doc = "1: N/A"]
    CH_A_FIND_SEQ_NOT_FINISHED = 1,
}
impl From<FNFA_A> for bool {
    #[inline(always)]
    fn from(variant: FNFA_A) -> Self {
        variant as u8 != 0
    }
}
impl FNFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FNFA_A {
        match self.bits {
            false => FNFA_A::NO_CH_A_FIND_SEQ_NOT_FINISHED,
            true => FNFA_A::CH_A_FIND_SEQ_NOT_FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_A_FIND_SEQ_NOT_FINISHED`"]
    #[inline(always)]
    pub fn is_no_ch_a_find_seq_not_finished(&self) -> bool {
        *self == FNFA_A::NO_CH_A_FIND_SEQ_NOT_FINISHED
    }
    #[doc = "Checks if the value of the field is `CH_A_FIND_SEQ_NOT_FINISHED`"]
    #[inline(always)]
    pub fn is_ch_a_find_seq_not_finished(&self) -> bool {
        *self == FNFA_A::CH_A_FIND_SEQ_NOT_FINISHED
    }
}
#[doc = "Field `FNFA` writer - Find Sequence Not Finished Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel A. 1 = Find sequence not finished for channel A 0 = No find sequence not finished for channel A"]
pub type FNFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, FNFA_A, O>;
impl<'a, const O: u8> FNFA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_a_find_seq_not_finished(self) -> &'a mut W {
        self.variant(FNFA_A::NO_CH_A_FIND_SEQ_NOT_FINISHED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_find_seq_not_finished(self) -> &'a mut W {
        self.variant(FNFA_A::CH_A_FIND_SEQ_NOT_FINISHED)
    }
}
#[doc = "Field `FNFB` reader - Find Sequence Not Finished Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel B. 1 = Find sequence not finished for channel B 0 = No find sequence not finished for channel B"]
pub type FNFB_R = crate::BitReader<FNFB_A>;
#[doc = "Find Sequence Not Finished Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel B. 1 = Find sequence not finished for channel B 0 = No find sequence not finished for channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FNFB_A {
    #[doc = "0: N/A"]
    NO_CH_B_FIND_SEQ_NOT_FINISHED = 0,
    #[doc = "1: N/A"]
    CH_B_FIND_SEQ_NOT_FINISHED = 1,
}
impl From<FNFB_A> for bool {
    #[inline(always)]
    fn from(variant: FNFB_A) -> Self {
        variant as u8 != 0
    }
}
impl FNFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FNFB_A {
        match self.bits {
            false => FNFB_A::NO_CH_B_FIND_SEQ_NOT_FINISHED,
            true => FNFB_A::CH_B_FIND_SEQ_NOT_FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_B_FIND_SEQ_NOT_FINISHED`"]
    #[inline(always)]
    pub fn is_no_ch_b_find_seq_not_finished(&self) -> bool {
        *self == FNFB_A::NO_CH_B_FIND_SEQ_NOT_FINISHED
    }
    #[doc = "Checks if the value of the field is `CH_B_FIND_SEQ_NOT_FINISHED`"]
    #[inline(always)]
    pub fn is_ch_b_find_seq_not_finished(&self) -> bool {
        *self == FNFB_A::CH_B_FIND_SEQ_NOT_FINISHED
    }
}
#[doc = "Field `FNFB` writer - Find Sequence Not Finished Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel B. 1 = Find sequence not finished for channel B 0 = No find sequence not finished for channel B"]
pub type FNFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, FNFB_A, O>;
impl<'a, const O: u8> FNFB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_b_find_seq_not_finished(self) -> &'a mut W {
        self.variant(FNFB_A::NO_CH_B_FIND_SEQ_NOT_FINISHED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_find_seq_not_finished(self) -> &'a mut W {
        self.variant(FNFB_A::CH_B_FIND_SEQ_NOT_FINISHED)
    }
}
#[doc = "Field `TBFA` reader - Transient Buffer Access Failure A This flag is set by the CC when a read or write access to TBF A requested by PRT A could not complete within the available time. 1 = TBF A access failure 0 = No TBF A access failure"]
pub type TBFA_R = crate::BitReader<TBFA_A>;
#[doc = "Transient Buffer Access Failure A This flag is set by the CC when a read or write access to TBF A requested by PRT A could not complete within the available time. 1 = TBF A access failure 0 = No TBF A access failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBFA_A {
    #[doc = "0: N/A"]
    TBF_CH_A_ACCESS_NOT_FAILED = 0,
    #[doc = "1: N/A"]
    TBF_CH_A_ACCESS_FAILED = 1,
}
impl From<TBFA_A> for bool {
    #[inline(always)]
    fn from(variant: TBFA_A) -> Self {
        variant as u8 != 0
    }
}
impl TBFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBFA_A {
        match self.bits {
            false => TBFA_A::TBF_CH_A_ACCESS_NOT_FAILED,
            true => TBFA_A::TBF_CH_A_ACCESS_FAILED,
        }
    }
    #[doc = "Checks if the value of the field is `TBF_CH_A_ACCESS_NOT_FAILED`"]
    #[inline(always)]
    pub fn is_tbf_ch_a_access_not_failed(&self) -> bool {
        *self == TBFA_A::TBF_CH_A_ACCESS_NOT_FAILED
    }
    #[doc = "Checks if the value of the field is `TBF_CH_A_ACCESS_FAILED`"]
    #[inline(always)]
    pub fn is_tbf_ch_a_access_failed(&self) -> bool {
        *self == TBFA_A::TBF_CH_A_ACCESS_FAILED
    }
}
#[doc = "Field `TBFA` writer - Transient Buffer Access Failure A This flag is set by the CC when a read or write access to TBF A requested by PRT A could not complete within the available time. 1 = TBF A access failure 0 = No TBF A access failure"]
pub type TBFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, TBFA_A, O>;
impl<'a, const O: u8> TBFA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tbf_ch_a_access_not_failed(self) -> &'a mut W {
        self.variant(TBFA_A::TBF_CH_A_ACCESS_NOT_FAILED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tbf_ch_a_access_failed(self) -> &'a mut W {
        self.variant(TBFA_A::TBF_CH_A_ACCESS_FAILED)
    }
}
#[doc = "Field `TBFB` reader - Transient Buffer Access Failure B This flag is set by the CC when a read or write access to TBF B requested by PRT B could not complete within the available time. 1 = TBF B access failure 0 = No TBF B access failure"]
pub type TBFB_R = crate::BitReader<TBFB_A>;
#[doc = "Transient Buffer Access Failure B This flag is set by the CC when a read or write access to TBF B requested by PRT B could not complete within the available time. 1 = TBF B access failure 0 = No TBF B access failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBFB_A {
    #[doc = "0: N/A"]
    TBF_CH_B_ACCESS_NOT_FAILED = 0,
    #[doc = "1: N/A"]
    TBF_CH_B_ACCESS_FAILED = 1,
}
impl From<TBFB_A> for bool {
    #[inline(always)]
    fn from(variant: TBFB_A) -> Self {
        variant as u8 != 0
    }
}
impl TBFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBFB_A {
        match self.bits {
            false => TBFB_A::TBF_CH_B_ACCESS_NOT_FAILED,
            true => TBFB_A::TBF_CH_B_ACCESS_FAILED,
        }
    }
    #[doc = "Checks if the value of the field is `TBF_CH_B_ACCESS_NOT_FAILED`"]
    #[inline(always)]
    pub fn is_tbf_ch_b_access_not_failed(&self) -> bool {
        *self == TBFB_A::TBF_CH_B_ACCESS_NOT_FAILED
    }
    #[doc = "Checks if the value of the field is `TBF_CH_B_ACCESS_FAILED`"]
    #[inline(always)]
    pub fn is_tbf_ch_b_access_failed(&self) -> bool {
        *self == TBFB_A::TBF_CH_B_ACCESS_FAILED
    }
}
#[doc = "Field `TBFB` writer - Transient Buffer Access Failure B This flag is set by the CC when a read or write access to TBF B requested by PRT B could not complete within the available time. 1 = TBF B access failure 0 = No TBF B access failure"]
pub type TBFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, TBFB_A, O>;
impl<'a, const O: u8> TBFB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tbf_ch_b_access_not_failed(self) -> &'a mut W {
        self.variant(TBFB_A::TBF_CH_B_ACCESS_NOT_FAILED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn tbf_ch_b_access_failed(self) -> &'a mut W {
        self.variant(TBFB_A::TBF_CH_B_ACCESS_FAILED)
    }
}
#[doc = "Field `TNSA` reader - Transmission Not Started Channel A This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel A at the action point of the configured slot. 1 = Transmission not started on channel A 0 = No transmission not started on channel A"]
pub type TNSA_R = crate::BitReader<TNSA_A>;
#[doc = "Transmission Not Started Channel A This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel A at the action point of the configured slot. 1 = Transmission not started on channel A 0 = No transmission not started on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TNSA_A {
    #[doc = "0: N/A"]
    NO_CH_A_TXMN_NOT_STARTED = 0,
    #[doc = "1: N/A"]
    CH_A_TXMN_NOT_STARTED = 1,
}
impl From<TNSA_A> for bool {
    #[inline(always)]
    fn from(variant: TNSA_A) -> Self {
        variant as u8 != 0
    }
}
impl TNSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNSA_A {
        match self.bits {
            false => TNSA_A::NO_CH_A_TXMN_NOT_STARTED,
            true => TNSA_A::CH_A_TXMN_NOT_STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_A_TXMN_NOT_STARTED`"]
    #[inline(always)]
    pub fn is_no_ch_a_txmn_not_started(&self) -> bool {
        *self == TNSA_A::NO_CH_A_TXMN_NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `CH_A_TXMN_NOT_STARTED`"]
    #[inline(always)]
    pub fn is_ch_a_txmn_not_started(&self) -> bool {
        *self == TNSA_A::CH_A_TXMN_NOT_STARTED
    }
}
#[doc = "Field `TNSA` writer - Transmission Not Started Channel A This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel A at the action point of the configured slot. 1 = Transmission not started on channel A 0 = No transmission not started on channel A"]
pub type TNSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, TNSA_A, O>;
impl<'a, const O: u8> TNSA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_a_txmn_not_started(self) -> &'a mut W {
        self.variant(TNSA_A::NO_CH_A_TXMN_NOT_STARTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_txmn_not_started(self) -> &'a mut W {
        self.variant(TNSA_A::CH_A_TXMN_NOT_STARTED)
    }
}
#[doc = "Field `TNSB` reader - Transmission Not Started Channel B This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel B at the action point of the configured slot. 1 = Transmission not started on channel B 0 = No transmission not started on channel B"]
pub type TNSB_R = crate::BitReader<TNSB_A>;
#[doc = "Transmission Not Started Channel B This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel B at the action point of the configured slot. 1 = Transmission not started on channel B 0 = No transmission not started on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TNSB_A {
    #[doc = "0: N/A"]
    NO_CH_B_TXMN_NOT_STARTED = 0,
    #[doc = "1: N/A"]
    CH_B_TXMN_NOT_STARTED = 1,
}
impl From<TNSB_A> for bool {
    #[inline(always)]
    fn from(variant: TNSB_A) -> Self {
        variant as u8 != 0
    }
}
impl TNSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNSB_A {
        match self.bits {
            false => TNSB_A::NO_CH_B_TXMN_NOT_STARTED,
            true => TNSB_A::CH_B_TXMN_NOT_STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_B_TXMN_NOT_STARTED`"]
    #[inline(always)]
    pub fn is_no_ch_b_txmn_not_started(&self) -> bool {
        *self == TNSB_A::NO_CH_B_TXMN_NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `CH_B_TXMN_NOT_STARTED`"]
    #[inline(always)]
    pub fn is_ch_b_txmn_not_started(&self) -> bool {
        *self == TNSB_A::CH_B_TXMN_NOT_STARTED
    }
}
#[doc = "Field `TNSB` writer - Transmission Not Started Channel B This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel B at the action point of the configured slot. 1 = Transmission not started on channel B 0 = No transmission not started on channel B"]
pub type TNSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, TNSB_A, O>;
impl<'a, const O: u8> TNSB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_b_txmn_not_started(self) -> &'a mut W {
        self.variant(TNSB_A::NO_CH_B_TXMN_NOT_STARTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_txmn_not_started(self) -> &'a mut W {
        self.variant(TNSB_A::CH_B_TXMN_NOT_STARTED)
    }
}
#[doc = "Field `WAHP` reader - Write Attempt to Header Partition Outside DEFAULT_CONFIG and CONFIG state this flag is set by the CC when the message handler tries to write message data into the header partition of the Message RAM due to faulty configuration of a message buffer. The write attempt is not executed, to protect the header partition from unintended write accesses. 1 = Write attempt to header partition 0 = No write attempt to header partition"]
pub type WAHP_R = crate::BitReader<WAHP_A>;
#[doc = "Write Attempt to Header Partition Outside DEFAULT_CONFIG and CONFIG state this flag is set by the CC when the message handler tries to write message data into the header partition of the Message RAM due to faulty configuration of a message buffer. The write attempt is not executed, to protect the header partition from unintended write accesses. 1 = Write attempt to header partition 0 = No write attempt to header partition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAHP_A {
    #[doc = "0: N/A"]
    HEADER_PARTITION_NO_WRITE = 0,
    #[doc = "1: N/A"]
    HEADER_PARTITION_WRITE = 1,
}
impl From<WAHP_A> for bool {
    #[inline(always)]
    fn from(variant: WAHP_A) -> Self {
        variant as u8 != 0
    }
}
impl WAHP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAHP_A {
        match self.bits {
            false => WAHP_A::HEADER_PARTITION_NO_WRITE,
            true => WAHP_A::HEADER_PARTITION_WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `HEADER_PARTITION_NO_WRITE`"]
    #[inline(always)]
    pub fn is_header_partition_no_write(&self) -> bool {
        *self == WAHP_A::HEADER_PARTITION_NO_WRITE
    }
    #[doc = "Checks if the value of the field is `HEADER_PARTITION_WRITE`"]
    #[inline(always)]
    pub fn is_header_partition_write(&self) -> bool {
        *self == WAHP_A::HEADER_PARTITION_WRITE
    }
}
#[doc = "Field `WAHP` writer - Write Attempt to Header Partition Outside DEFAULT_CONFIG and CONFIG state this flag is set by the CC when the message handler tries to write message data into the header partition of the Message RAM due to faulty configuration of a message buffer. The write attempt is not executed, to protect the header partition from unintended write accesses. 1 = Write attempt to header partition 0 = No write attempt to header partition"]
pub type WAHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDF_SPEC, WAHP_A, O>;
impl<'a, const O: u8> WAHP_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn header_partition_no_write(self) -> &'a mut W {
        self.variant(WAHP_A::HEADER_PARTITION_NO_WRITE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn header_partition_write(self) -> &'a mut W {
        self.variant(WAHP_A::HEADER_PARTITION_WRITE)
    }
}
impl R {
    #[doc = "Bit 0 - Status Not Updated Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel A. 1 = MBS for channel A not updated 0 = No overload condition occurred when updating MBS for channel A"]
    #[inline(always)]
    pub fn snua(&self) -> SNUA_R {
        SNUA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status Not Updated Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel B. 1 = MBS for channel B not updated 0 = No overload condition occurred when updating MBS for channel B"]
    #[inline(always)]
    pub fn snub(&self) -> SNUB_R {
        SNUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Find Sequence Not Finished Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel A. 1 = Find sequence not finished for channel A 0 = No find sequence not finished for channel A"]
    #[inline(always)]
    pub fn fnfa(&self) -> FNFA_R {
        FNFA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Find Sequence Not Finished Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel B. 1 = Find sequence not finished for channel B 0 = No find sequence not finished for channel B"]
    #[inline(always)]
    pub fn fnfb(&self) -> FNFB_R {
        FNFB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transient Buffer Access Failure A This flag is set by the CC when a read or write access to TBF A requested by PRT A could not complete within the available time. 1 = TBF A access failure 0 = No TBF A access failure"]
    #[inline(always)]
    pub fn tbfa(&self) -> TBFA_R {
        TBFA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transient Buffer Access Failure B This flag is set by the CC when a read or write access to TBF B requested by PRT B could not complete within the available time. 1 = TBF B access failure 0 = No TBF B access failure"]
    #[inline(always)]
    pub fn tbfb(&self) -> TBFB_R {
        TBFB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Not Started Channel A This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel A at the action point of the configured slot. 1 = Transmission not started on channel A 0 = No transmission not started on channel A"]
    #[inline(always)]
    pub fn tnsa(&self) -> TNSA_R {
        TNSA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission Not Started Channel B This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel B at the action point of the configured slot. 1 = Transmission not started on channel B 0 = No transmission not started on channel B"]
    #[inline(always)]
    pub fn tnsb(&self) -> TNSB_R {
        TNSB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Attempt to Header Partition Outside DEFAULT_CONFIG and CONFIG state this flag is set by the CC when the message handler tries to write message data into the header partition of the Message RAM due to faulty configuration of a message buffer. The write attempt is not executed, to protect the header partition from unintended write accesses. 1 = Write attempt to header partition 0 = No write attempt to header partition"]
    #[inline(always)]
    pub fn wahp(&self) -> WAHP_R {
        WAHP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status Not Updated Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel A. 1 = MBS for channel A not updated 0 = No overload condition occurred when updating MBS for channel A"]
    #[inline(always)]
    #[must_use]
    pub fn snua(&mut self) -> SNUA_W<0> {
        SNUA_W::new(self)
    }
    #[doc = "Bit 1 - Status Not Updated Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to update a message buffer's status MBS with respect to channel B. 1 = MBS for channel B not updated 0 = No overload condition occurred when updating MBS for channel B"]
    #[inline(always)]
    #[must_use]
    pub fn snub(&mut self) -> SNUB_W<1> {
        SNUB_W::new(self)
    }
    #[doc = "Bit 2 - Find Sequence Not Finished Channel A This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel A. 1 = Find sequence not finished for channel A 0 = No find sequence not finished for channel A"]
    #[inline(always)]
    #[must_use]
    pub fn fnfa(&mut self) -> FNFA_W<2> {
        FNFA_W::new(self)
    }
    #[doc = "Bit 3 - Find Sequence Not Finished Channel B This flag is set by the CC when the Message Handler, due to overload condition, was not able to finish a find sequence (scan of Message RAM for matching message buffer) with respect to channel B. 1 = Find sequence not finished for channel B 0 = No find sequence not finished for channel B"]
    #[inline(always)]
    #[must_use]
    pub fn fnfb(&mut self) -> FNFB_W<3> {
        FNFB_W::new(self)
    }
    #[doc = "Bit 4 - Transient Buffer Access Failure A This flag is set by the CC when a read or write access to TBF A requested by PRT A could not complete within the available time. 1 = TBF A access failure 0 = No TBF A access failure"]
    #[inline(always)]
    #[must_use]
    pub fn tbfa(&mut self) -> TBFA_W<4> {
        TBFA_W::new(self)
    }
    #[doc = "Bit 5 - Transient Buffer Access Failure B This flag is set by the CC when a read or write access to TBF B requested by PRT B could not complete within the available time. 1 = TBF B access failure 0 = No TBF B access failure"]
    #[inline(always)]
    #[must_use]
    pub fn tbfb(&mut self) -> TBFB_W<5> {
        TBFB_W::new(self)
    }
    #[doc = "Bit 6 - Transmission Not Started Channel A This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel A at the action point of the configured slot. 1 = Transmission not started on channel A 0 = No transmission not started on channel A"]
    #[inline(always)]
    #[must_use]
    pub fn tnsa(&mut self) -> TNSA_W<6> {
        TNSA_W::new(self)
    }
    #[doc = "Bit 7 - Transmission Not Started Channel B This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel B at the action point of the configured slot. 1 = Transmission not started on channel B 0 = No transmission not started on channel B"]
    #[inline(always)]
    #[must_use]
    pub fn tnsb(&mut self) -> TNSB_W<7> {
        TNSB_W::new(self)
    }
    #[doc = "Bit 8 - Write Attempt to Header Partition Outside DEFAULT_CONFIG and CONFIG state this flag is set by the CC when the message handler tries to write message data into the header partition of the Message RAM due to faulty configuration of a message buffer. The write attempt is not executed, to protect the header partition from unintended write accesses. 1 = Write attempt to header partition 0 = No write attempt to header partition"]
    #[inline(always)]
    #[must_use]
    pub fn wahp(&mut self) -> WAHP_W<8> {
        WAHP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Handler Constraints Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mhdf](index.html) module"]
pub struct MHDF_SPEC;
impl crate::RegisterSpec for MHDF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mhdf::R](R) reader structure"]
impl crate::Readable for MHDF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mhdf::W](W) writer structure"]
impl crate::Writable for MHDF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MHDF to value 0"]
impl crate::Resettable for MHDF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
