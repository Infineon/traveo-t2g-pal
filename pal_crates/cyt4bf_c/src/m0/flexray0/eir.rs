#[doc = "Register `EIR` reader"]
pub struct R(crate::R<EIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIR` writer"]
pub struct W(crate::W<EIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIR_SPEC>;
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
impl From<crate::W<EIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEMC` reader - POC Error Mode Changed This flag is set whenever the error mode signalled by CCEV.ERRM\\[1:0\\]
has changed. 1 = Error mode has changed 0 = Error mode has not changed"]
pub type PEMC_R = crate::BitReader<PEMC_A>;
#[doc = "POC Error Mode Changed This flag is set whenever the error mode signalled by CCEV.ERRM\\[1:0\\]
has changed. 1 = Error mode has changed 0 = Error mode has not changed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEMC_A {
    #[doc = "0: N/A"]
    POC_ERR_MODE_UNCHANGED = 0,
    #[doc = "1: N/A"]
    POC_ERR_MODE_CHANGED = 1,
}
impl From<PEMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEMC_A) -> Self {
        variant as u8 != 0
    }
}
impl PEMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEMC_A {
        match self.bits {
            false => PEMC_A::POC_ERR_MODE_UNCHANGED,
            true => PEMC_A::POC_ERR_MODE_CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `POC_ERR_MODE_UNCHANGED`"]
    #[inline(always)]
    pub fn is_poc_err_mode_unchanged(&self) -> bool {
        *self == PEMC_A::POC_ERR_MODE_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `POC_ERR_MODE_CHANGED`"]
    #[inline(always)]
    pub fn is_poc_err_mode_changed(&self) -> bool {
        *self == PEMC_A::POC_ERR_MODE_CHANGED
    }
}
#[doc = "Field `PEMC` writer - POC Error Mode Changed This flag is set whenever the error mode signalled by CCEV.ERRM\\[1:0\\]
has changed. 1 = Error mode has changed 0 = Error mode has not changed"]
pub type PEMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, PEMC_A, O>;
impl<'a, const O: u8> PEMC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn poc_err_mode_unchanged(self) -> &'a mut W {
        self.variant(PEMC_A::POC_ERR_MODE_UNCHANGED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn poc_err_mode_changed(self) -> &'a mut W {
        self.variant(PEMC_A::POC_ERR_MODE_CHANGED)
    }
}
#[doc = "Field `CNA` reader - Command Not Accepted The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the requested command was not valid in the actual POC state, or because the CHI command was locked (CCL = '1'). 1 = CHI command not accepted 0 = CHI command accepted"]
pub type CNA_R = crate::BitReader<CNA_A>;
#[doc = "Command Not Accepted The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the requested command was not valid in the actual POC state, or because the CHI command was locked (CCL = '1'). 1 = CHI command not accepted 0 = CHI command accepted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNA_A {
    #[doc = "0: N/A"]
    CHI_CMD_ACCEPTED = 0,
    #[doc = "1: N/A"]
    CHI_CMD_NOT_ACCEPTED = 1,
}
impl From<CNA_A> for bool {
    #[inline(always)]
    fn from(variant: CNA_A) -> Self {
        variant as u8 != 0
    }
}
impl CNA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNA_A {
        match self.bits {
            false => CNA_A::CHI_CMD_ACCEPTED,
            true => CNA_A::CHI_CMD_NOT_ACCEPTED,
        }
    }
    #[doc = "Checks if the value of the field is `CHI_CMD_ACCEPTED`"]
    #[inline(always)]
    pub fn is_chi_cmd_accepted(&self) -> bool {
        *self == CNA_A::CHI_CMD_ACCEPTED
    }
    #[doc = "Checks if the value of the field is `CHI_CMD_NOT_ACCEPTED`"]
    #[inline(always)]
    pub fn is_chi_cmd_not_accepted(&self) -> bool {
        *self == CNA_A::CHI_CMD_NOT_ACCEPTED
    }
}
#[doc = "Field `CNA` writer - Command Not Accepted The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the requested command was not valid in the actual POC state, or because the CHI command was locked (CCL = '1'). 1 = CHI command not accepted 0 = CHI command accepted"]
pub type CNA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, CNA_A, O>;
impl<'a, const O: u8> CNA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn chi_cmd_accepted(self) -> &'a mut W {
        self.variant(CNA_A::CHI_CMD_ACCEPTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn chi_cmd_not_accepted(self) -> &'a mut W {
        self.variant(CNA_A::CHI_CMD_NOT_ACCEPTED)
    }
}
#[doc = "Field `SFBM` reader - Sync Frames Below Minimum This flag signals that the number of sync frames received during the last communication cycle was below the limit required by the FlexRay protocol. May be set during startup and therefore should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Less than the required minimum of sync frames received 0 = Sync node: 1 or more sync frames received, non-sync node: 2 or more sync frames received"]
pub type SFBM_R = crate::BitReader<SFBM_A>;
#[doc = "Sync Frames Below Minimum This flag signals that the number of sync frames received during the last communication cycle was below the limit required by the FlexRay protocol. May be set during startup and therefore should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Less than the required minimum of sync frames received 0 = Sync node: 1 or more sync frames received, non-sync node: 2 or more sync frames received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFBM_A {
    #[doc = "0: N/A"]
    SYNC_FRAMES_NOT_BELOW_MIN = 0,
    #[doc = "1: N/A"]
    SYNC_FRAMES_BELOW_MIN = 1,
}
impl From<SFBM_A> for bool {
    #[inline(always)]
    fn from(variant: SFBM_A) -> Self {
        variant as u8 != 0
    }
}
impl SFBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFBM_A {
        match self.bits {
            false => SFBM_A::SYNC_FRAMES_NOT_BELOW_MIN,
            true => SFBM_A::SYNC_FRAMES_BELOW_MIN,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAMES_NOT_BELOW_MIN`"]
    #[inline(always)]
    pub fn is_sync_frames_not_below_min(&self) -> bool {
        *self == SFBM_A::SYNC_FRAMES_NOT_BELOW_MIN
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAMES_BELOW_MIN`"]
    #[inline(always)]
    pub fn is_sync_frames_below_min(&self) -> bool {
        *self == SFBM_A::SYNC_FRAMES_BELOW_MIN
    }
}
#[doc = "Field `SFBM` writer - Sync Frames Below Minimum This flag signals that the number of sync frames received during the last communication cycle was below the limit required by the FlexRay protocol. May be set during startup and therefore should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Less than the required minimum of sync frames received 0 = Sync node: 1 or more sync frames received, non-sync node: 2 or more sync frames received"]
pub type SFBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, SFBM_A, O>;
impl<'a, const O: u8> SFBM_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sync_frames_not_below_min(self) -> &'a mut W {
        self.variant(SFBM_A::SYNC_FRAMES_NOT_BELOW_MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sync_frames_below_min(self) -> &'a mut W {
        self.variant(SFBM_A::SYNC_FRAMES_BELOW_MIN)
    }
}
#[doc = "Field `SFO` reader - Sync Frame Overflow Set when either the number of sync frames received during the last communication cycle or the total number of different sync frame IDs received during the last double cycle exceeds the maximum number of sync frames as defined by GTUC2.SNM\\[3:0\\]. 1 = More sync frames received than configured by GTUC2.SNM\\[3:0\\]
0 = Number of received sync frames &lt;= GTUC2.SNM\\[3:0\\]"]
pub type SFO_R = crate::BitReader<SFO_A>;
#[doc = "Sync Frame Overflow Set when either the number of sync frames received during the last communication cycle or the total number of different sync frame IDs received during the last double cycle exceeds the maximum number of sync frames as defined by GTUC2.SNM\\[3:0\\]. 1 = More sync frames received than configured by GTUC2.SNM\\[3:0\\]
0 = Number of received sync frames &lt;= GTUC2.SNM\\[3:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFO_A {
    #[doc = "0: N/A"]
    NO_SYNC_FRAME_OVERFLOW = 0,
    #[doc = "1: N/A"]
    SYNC_FRAME_OVERFLOW = 1,
}
impl From<SFO_A> for bool {
    #[inline(always)]
    fn from(variant: SFO_A) -> Self {
        variant as u8 != 0
    }
}
impl SFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFO_A {
        match self.bits {
            false => SFO_A::NO_SYNC_FRAME_OVERFLOW,
            true => SFO_A::SYNC_FRAME_OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNC_FRAME_OVERFLOW`"]
    #[inline(always)]
    pub fn is_no_sync_frame_overflow(&self) -> bool {
        *self == SFO_A::NO_SYNC_FRAME_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `SYNC_FRAME_OVERFLOW`"]
    #[inline(always)]
    pub fn is_sync_frame_overflow(&self) -> bool {
        *self == SFO_A::SYNC_FRAME_OVERFLOW
    }
}
#[doc = "Field `SFO` writer - Sync Frame Overflow Set when either the number of sync frames received during the last communication cycle or the total number of different sync frame IDs received during the last double cycle exceeds the maximum number of sync frames as defined by GTUC2.SNM\\[3:0\\]. 1 = More sync frames received than configured by GTUC2.SNM\\[3:0\\]
0 = Number of received sync frames &lt;= GTUC2.SNM\\[3:0\\]"]
pub type SFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, SFO_A, O>;
impl<'a, const O: u8> SFO_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_sync_frame_overflow(self) -> &'a mut W {
        self.variant(SFO_A::NO_SYNC_FRAME_OVERFLOW)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sync_frame_overflow(self) -> &'a mut W {
        self.variant(SFO_A::SYNC_FRAME_OVERFLOW)
    }
}
#[doc = "Field `CCF` reader - Clock Correction Failure This flag is set at the end of the cycle whenever one of the following errors occurred: Missing offset and / or rate correction Clock correction limit reached The clock correction status is monitored in registers CCEV and SFS. A failure may occur during startup, therefore bit CCF should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Clock correction failed 0 = No clock correction error"]
pub type CCF_R = crate::BitReader<CCF_A>;
#[doc = "Clock Correction Failure This flag is set at the end of the cycle whenever one of the following errors occurred: Missing offset and / or rate correction Clock correction limit reached The clock correction status is monitored in registers CCEV and SFS. A failure may occur during startup, therefore bit CCF should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Clock correction failed 0 = No clock correction error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCF_A {
    #[doc = "0: N/A"]
    NO_CLK_CORRECTION_ERR = 0,
    #[doc = "1: N/A"]
    CLK_CORRECTION_ERR = 1,
}
impl From<CCF_A> for bool {
    #[inline(always)]
    fn from(variant: CCF_A) -> Self {
        variant as u8 != 0
    }
}
impl CCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCF_A {
        match self.bits {
            false => CCF_A::NO_CLK_CORRECTION_ERR,
            true => CCF_A::CLK_CORRECTION_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLK_CORRECTION_ERR`"]
    #[inline(always)]
    pub fn is_no_clk_correction_err(&self) -> bool {
        *self == CCF_A::NO_CLK_CORRECTION_ERR
    }
    #[doc = "Checks if the value of the field is `CLK_CORRECTION_ERR`"]
    #[inline(always)]
    pub fn is_clk_correction_err(&self) -> bool {
        *self == CCF_A::CLK_CORRECTION_ERR
    }
}
#[doc = "Field `CCF` writer - Clock Correction Failure This flag is set at the end of the cycle whenever one of the following errors occurred: Missing offset and / or rate correction Clock correction limit reached The clock correction status is monitored in registers CCEV and SFS. A failure may occur during startup, therefore bit CCF should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Clock correction failed 0 = No clock correction error"]
pub type CCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, CCF_A, O>;
impl<'a, const O: u8> CCF_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_clk_correction_err(self) -> &'a mut W {
        self.variant(CCF_A::NO_CLK_CORRECTION_ERR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn clk_correction_err(self) -> &'a mut W {
        self.variant(CCF_A::CLK_CORRECTION_ERR)
    }
}
#[doc = "Field `CCL` reader - CHI Command Locked The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the execution of the previous CHI command has not yet completed. In this case bit CNA is also set to '1'. 1 = CHI command not accepted 0 = CHI command accepted"]
pub type CCL_R = crate::BitReader<CCL_A>;
#[doc = "CHI Command Locked The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the execution of the previous CHI command has not yet completed. In this case bit CNA is also set to '1'. 1 = CHI command not accepted 0 = CHI command accepted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCL_A {
    #[doc = "0: N/A"]
    CHI_CMD_ACCEPTED = 0,
    #[doc = "1: N/A"]
    CHI_CMD_NOT_ACCEPTED = 1,
}
impl From<CCL_A> for bool {
    #[inline(always)]
    fn from(variant: CCL_A) -> Self {
        variant as u8 != 0
    }
}
impl CCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCL_A {
        match self.bits {
            false => CCL_A::CHI_CMD_ACCEPTED,
            true => CCL_A::CHI_CMD_NOT_ACCEPTED,
        }
    }
    #[doc = "Checks if the value of the field is `CHI_CMD_ACCEPTED`"]
    #[inline(always)]
    pub fn is_chi_cmd_accepted(&self) -> bool {
        *self == CCL_A::CHI_CMD_ACCEPTED
    }
    #[doc = "Checks if the value of the field is `CHI_CMD_NOT_ACCEPTED`"]
    #[inline(always)]
    pub fn is_chi_cmd_not_accepted(&self) -> bool {
        *self == CCL_A::CHI_CMD_NOT_ACCEPTED
    }
}
#[doc = "Field `CCL` writer - CHI Command Locked The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the execution of the previous CHI command has not yet completed. In this case bit CNA is also set to '1'. 1 = CHI command not accepted 0 = CHI command accepted"]
pub type CCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, CCL_A, O>;
impl<'a, const O: u8> CCL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn chi_cmd_accepted(self) -> &'a mut W {
        self.variant(CCL_A::CHI_CMD_ACCEPTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn chi_cmd_not_accepted(self) -> &'a mut W {
        self.variant(CCL_A::CHI_CMD_NOT_ACCEPTED)
    }
}
#[doc = "Field `PERR` reader - Parity Error The flag signals a parity error to the Host. It is set whenever one of the flags MHDS.PIBF, MHDS.POBF, MHDS.PMR, MHDS.PTBF1, MHDS.PTBF2 changes from '0' to '1'. 1 = Parity error detected 0 = No parity error detected"]
pub type PERR_R = crate::BitReader<PERR_A>;
#[doc = "Parity Error The flag signals a parity error to the Host. It is set whenever one of the flags MHDS.PIBF, MHDS.POBF, MHDS.PMR, MHDS.PTBF1, MHDS.PTBF2 changes from '0' to '1'. 1 = Parity error detected 0 = No parity error detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERR_A {
    #[doc = "0: N/A"]
    NO_PARITY_ERR = 0,
    #[doc = "1: N/A"]
    PARITY_ERR = 1,
}
impl From<PERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERR_A {
        match self.bits {
            false => PERR_A::NO_PARITY_ERR,
            true => PERR_A::PARITY_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY_ERR`"]
    #[inline(always)]
    pub fn is_no_parity_err(&self) -> bool {
        *self == PERR_A::NO_PARITY_ERR
    }
    #[doc = "Checks if the value of the field is `PARITY_ERR`"]
    #[inline(always)]
    pub fn is_parity_err(&self) -> bool {
        *self == PERR_A::PARITY_ERR
    }
}
#[doc = "Field `PERR` writer - Parity Error The flag signals a parity error to the Host. It is set whenever one of the flags MHDS.PIBF, MHDS.POBF, MHDS.PMR, MHDS.PTBF1, MHDS.PTBF2 changes from '0' to '1'. 1 = Parity error detected 0 = No parity error detected"]
pub type PERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, PERR_A, O>;
impl<'a, const O: u8> PERR_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_parity_err(self) -> &'a mut W {
        self.variant(PERR_A::NO_PARITY_ERR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn parity_err(self) -> &'a mut W {
        self.variant(PERR_A::PARITY_ERR)
    }
}
#[doc = "Field `RFO` reader - Receive FIFO Overrun The flag is set by the CC when a receive FIFO overrun is detected. When a receive FIFO overrun occurs, the oldest message is overwritten with the actual received message. The actual state of the FIFO is monitored in register FSR. 1 = A receive FIFO overrun has been detected 0 = No receive FIFO overrun detected"]
pub type RFO_R = crate::BitReader<RFO_A>;
#[doc = "Receive FIFO Overrun The flag is set by the CC when a receive FIFO overrun is detected. When a receive FIFO overrun occurs, the oldest message is overwritten with the actual received message. The actual state of the FIFO is monitored in register FSR. 1 = A receive FIFO overrun has been detected 0 = No receive FIFO overrun detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFO_A {
    #[doc = "0: N/A"]
    NO_RECEIVE_FIFO_OVERRUN = 0,
    #[doc = "1: N/A"]
    RECEIVE_FIFO_OVERRUN = 1,
}
impl From<RFO_A> for bool {
    #[inline(always)]
    fn from(variant: RFO_A) -> Self {
        variant as u8 != 0
    }
}
impl RFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFO_A {
        match self.bits {
            false => RFO_A::NO_RECEIVE_FIFO_OVERRUN,
            true => RFO_A::RECEIVE_FIFO_OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RECEIVE_FIFO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_receive_fifo_overrun(&self) -> bool {
        *self == RFO_A::NO_RECEIVE_FIFO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `RECEIVE_FIFO_OVERRUN`"]
    #[inline(always)]
    pub fn is_receive_fifo_overrun(&self) -> bool {
        *self == RFO_A::RECEIVE_FIFO_OVERRUN
    }
}
#[doc = "Field `RFO` writer - Receive FIFO Overrun The flag is set by the CC when a receive FIFO overrun is detected. When a receive FIFO overrun occurs, the oldest message is overwritten with the actual received message. The actual state of the FIFO is monitored in register FSR. 1 = A receive FIFO overrun has been detected 0 = No receive FIFO overrun detected"]
pub type RFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, RFO_A, O>;
impl<'a, const O: u8> RFO_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_receive_fifo_overrun(self) -> &'a mut W {
        self.variant(RFO_A::NO_RECEIVE_FIFO_OVERRUN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn receive_fifo_overrun(self) -> &'a mut W {
        self.variant(RFO_A::RECEIVE_FIFO_OVERRUN)
    }
}
#[doc = "Field `EFA` reader - Empty FIFO Access This flag is set by the CC when the Host requests the transfer of a message from the receive FIFO via Output Buffer while the receive FIFO is empty. 1 = Host access to empty FIFO occurred 0 = No Host access to empty FIFO occurred"]
pub type EFA_R = crate::BitReader<EFA_A>;
#[doc = "Empty FIFO Access This flag is set by the CC when the Host requests the transfer of a message from the receive FIFO via Output Buffer while the receive FIFO is empty. 1 = Host access to empty FIFO occurred 0 = No Host access to empty FIFO occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFA_A {
    #[doc = "0: N/A"]
    NO_EMPTY_FIFO_ACCESS = 0,
    #[doc = "1: N/A"]
    EMPTY_FIFO_ACCESS = 1,
}
impl From<EFA_A> for bool {
    #[inline(always)]
    fn from(variant: EFA_A) -> Self {
        variant as u8 != 0
    }
}
impl EFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFA_A {
        match self.bits {
            false => EFA_A::NO_EMPTY_FIFO_ACCESS,
            true => EFA_A::EMPTY_FIFO_ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EMPTY_FIFO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_empty_fifo_access(&self) -> bool {
        *self == EFA_A::NO_EMPTY_FIFO_ACCESS
    }
    #[doc = "Checks if the value of the field is `EMPTY_FIFO_ACCESS`"]
    #[inline(always)]
    pub fn is_empty_fifo_access(&self) -> bool {
        *self == EFA_A::EMPTY_FIFO_ACCESS
    }
}
#[doc = "Field `EFA` writer - Empty FIFO Access This flag is set by the CC when the Host requests the transfer of a message from the receive FIFO via Output Buffer while the receive FIFO is empty. 1 = Host access to empty FIFO occurred 0 = No Host access to empty FIFO occurred"]
pub type EFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, EFA_A, O>;
impl<'a, const O: u8> EFA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_empty_fifo_access(self) -> &'a mut W {
        self.variant(EFA_A::NO_EMPTY_FIFO_ACCESS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn empty_fifo_access(self) -> &'a mut W {
        self.variant(EFA_A::EMPTY_FIFO_ACCESS)
    }
}
#[doc = "Field `IIBA` reader - Illegal Input Buffer Access This flag is set by the CC when the Host wants to modify a message buffer via Input Buffer and one of the following conditions applies: 1) The CC is not in CONFIG or DEFAULT_CONFIG state and the Host writes to the Input Buffer Command Request register to modify the Header section of message buffer 0, 1 if configured for transmission in key slot Header section of static message buffers with buffer number &lt; MRC.FDB\\[7:0\\]
while MRC.SEC\\[1:0\\]
= '01' Header section of any static or dynamic message buffer while MRC.SEC\\[1:0\\]
= '1x' Header and / or data section of any message buffer belonging to the receive FIFO 2) The Host writes to any register of the Input Buffer while IBCR.IBSYH is set to '1'. 1 = Illegal Host access to Input Buffer occurred 0 = No illegal Host access to Input Buffer occurred"]
pub type IIBA_R = crate::BitReader<IIBA_A>;
#[doc = "Illegal Input Buffer Access This flag is set by the CC when the Host wants to modify a message buffer via Input Buffer and one of the following conditions applies: 1) The CC is not in CONFIG or DEFAULT_CONFIG state and the Host writes to the Input Buffer Command Request register to modify the Header section of message buffer 0, 1 if configured for transmission in key slot Header section of static message buffers with buffer number &lt; MRC.FDB\\[7:0\\]
while MRC.SEC\\[1:0\\]
= '01' Header section of any static or dynamic message buffer while MRC.SEC\\[1:0\\]
= '1x' Header and / or data section of any message buffer belonging to the receive FIFO 2) The Host writes to any register of the Input Buffer while IBCR.IBSYH is set to '1'. 1 = Illegal Host access to Input Buffer occurred 0 = No illegal Host access to Input Buffer occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIBA_A {
    #[doc = "0: N/A"]
    NO_ILLEGAL_IBF_ACCESS = 0,
    #[doc = "1: N/A"]
    ILLEGAL_IBF_ACCESS = 1,
}
impl From<IIBA_A> for bool {
    #[inline(always)]
    fn from(variant: IIBA_A) -> Self {
        variant as u8 != 0
    }
}
impl IIBA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IIBA_A {
        match self.bits {
            false => IIBA_A::NO_ILLEGAL_IBF_ACCESS,
            true => IIBA_A::ILLEGAL_IBF_ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ILLEGAL_IBF_ACCESS`"]
    #[inline(always)]
    pub fn is_no_illegal_ibf_access(&self) -> bool {
        *self == IIBA_A::NO_ILLEGAL_IBF_ACCESS
    }
    #[doc = "Checks if the value of the field is `ILLEGAL_IBF_ACCESS`"]
    #[inline(always)]
    pub fn is_illegal_ibf_access(&self) -> bool {
        *self == IIBA_A::ILLEGAL_IBF_ACCESS
    }
}
#[doc = "Field `IIBA` writer - Illegal Input Buffer Access This flag is set by the CC when the Host wants to modify a message buffer via Input Buffer and one of the following conditions applies: 1) The CC is not in CONFIG or DEFAULT_CONFIG state and the Host writes to the Input Buffer Command Request register to modify the Header section of message buffer 0, 1 if configured for transmission in key slot Header section of static message buffers with buffer number &lt; MRC.FDB\\[7:0\\]
while MRC.SEC\\[1:0\\]
= '01' Header section of any static or dynamic message buffer while MRC.SEC\\[1:0\\]
= '1x' Header and / or data section of any message buffer belonging to the receive FIFO 2) The Host writes to any register of the Input Buffer while IBCR.IBSYH is set to '1'. 1 = Illegal Host access to Input Buffer occurred 0 = No illegal Host access to Input Buffer occurred"]
pub type IIBA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, IIBA_A, O>;
impl<'a, const O: u8> IIBA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_illegal_ibf_access(self) -> &'a mut W {
        self.variant(IIBA_A::NO_ILLEGAL_IBF_ACCESS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn illegal_ibf_access(self) -> &'a mut W {
        self.variant(IIBA_A::ILLEGAL_IBF_ACCESS)
    }
}
#[doc = "Field `IOBA` reader - Illegal Output buffer Access This flag is set by the CC when the Host requests the transfer of a message buffer from the Message RAM to the Output Buffer while OBCR.OBSYS is set to '1'. 1 = Illegal Host access to Output Buffer occurred 0 = No illegal Host access to Output Buffer occurred"]
pub type IOBA_R = crate::BitReader<IOBA_A>;
#[doc = "Illegal Output buffer Access This flag is set by the CC when the Host requests the transfer of a message buffer from the Message RAM to the Output Buffer while OBCR.OBSYS is set to '1'. 1 = Illegal Host access to Output Buffer occurred 0 = No illegal Host access to Output Buffer occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOBA_A {
    #[doc = "0: N/A"]
    NO_ILLEGAL_OBF_ACCESS = 0,
    #[doc = "1: N/A"]
    ILLEGAL_OBF_ACCESS = 1,
}
impl From<IOBA_A> for bool {
    #[inline(always)]
    fn from(variant: IOBA_A) -> Self {
        variant as u8 != 0
    }
}
impl IOBA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOBA_A {
        match self.bits {
            false => IOBA_A::NO_ILLEGAL_OBF_ACCESS,
            true => IOBA_A::ILLEGAL_OBF_ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ILLEGAL_OBF_ACCESS`"]
    #[inline(always)]
    pub fn is_no_illegal_obf_access(&self) -> bool {
        *self == IOBA_A::NO_ILLEGAL_OBF_ACCESS
    }
    #[doc = "Checks if the value of the field is `ILLEGAL_OBF_ACCESS`"]
    #[inline(always)]
    pub fn is_illegal_obf_access(&self) -> bool {
        *self == IOBA_A::ILLEGAL_OBF_ACCESS
    }
}
#[doc = "Field `IOBA` writer - Illegal Output buffer Access This flag is set by the CC when the Host requests the transfer of a message buffer from the Message RAM to the Output Buffer while OBCR.OBSYS is set to '1'. 1 = Illegal Host access to Output Buffer occurred 0 = No illegal Host access to Output Buffer occurred"]
pub type IOBA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, IOBA_A, O>;
impl<'a, const O: u8> IOBA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_illegal_obf_access(self) -> &'a mut W {
        self.variant(IOBA_A::NO_ILLEGAL_OBF_ACCESS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn illegal_obf_access(self) -> &'a mut W {
        self.variant(IOBA_A::ILLEGAL_OBF_ACCESS)
    }
}
#[doc = "Field `MHF` reader - Message Handler Constraints Flag The flag signals a Message Handler constraints violation condition. It is set whenever one of the flags MHDF.SNUA, MHDF.SNUB, MHDF.FNFA, MHDF.FNFB, MHDF.TBFA, MHDF.TBFB, MHDF.WAHP changes from '0' to '1'. 1 = Message Handler failure detected 0 = No Message Handler failure detected"]
pub type MHF_R = crate::BitReader<MHF_A>;
#[doc = "Message Handler Constraints Flag The flag signals a Message Handler constraints violation condition. It is set whenever one of the flags MHDF.SNUA, MHDF.SNUB, MHDF.FNFA, MHDF.FNFB, MHDF.TBFA, MHDF.TBFB, MHDF.WAHP changes from '0' to '1'. 1 = Message Handler failure detected 0 = No Message Handler failure detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MHF_A {
    #[doc = "0: N/A"]
    NO_MSG_HANDLER_FAIL_DETECT = 0,
    #[doc = "1: N/A"]
    MSG_HANDLER_FAIL_DETECT = 1,
}
impl From<MHF_A> for bool {
    #[inline(always)]
    fn from(variant: MHF_A) -> Self {
        variant as u8 != 0
    }
}
impl MHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MHF_A {
        match self.bits {
            false => MHF_A::NO_MSG_HANDLER_FAIL_DETECT,
            true => MHF_A::MSG_HANDLER_FAIL_DETECT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MSG_HANDLER_FAIL_DETECT`"]
    #[inline(always)]
    pub fn is_no_msg_handler_fail_detect(&self) -> bool {
        *self == MHF_A::NO_MSG_HANDLER_FAIL_DETECT
    }
    #[doc = "Checks if the value of the field is `MSG_HANDLER_FAIL_DETECT`"]
    #[inline(always)]
    pub fn is_msg_handler_fail_detect(&self) -> bool {
        *self == MHF_A::MSG_HANDLER_FAIL_DETECT
    }
}
#[doc = "Field `MHF` writer - Message Handler Constraints Flag The flag signals a Message Handler constraints violation condition. It is set whenever one of the flags MHDF.SNUA, MHDF.SNUB, MHDF.FNFA, MHDF.FNFB, MHDF.TBFA, MHDF.TBFB, MHDF.WAHP changes from '0' to '1'. 1 = Message Handler failure detected 0 = No Message Handler failure detected"]
pub type MHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, MHF_A, O>;
impl<'a, const O: u8> MHF_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_msg_handler_fail_detect(self) -> &'a mut W {
        self.variant(MHF_A::NO_MSG_HANDLER_FAIL_DETECT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn msg_handler_fail_detect(self) -> &'a mut W {
        self.variant(MHF_A::MSG_HANDLER_FAIL_DETECT)
    }
}
#[doc = "Field `EDA` reader - Error Detected on Channel A This bit is set whenever one of the flags ACS.SEDA, ACS.CEDA, ACS.CIA, ACS.SBVA changes from '0' to '1'. 1 = Error detected on channel A 0 = No error detected on channel A"]
pub type EDA_R = crate::BitReader<EDA_A>;
#[doc = "Error Detected on Channel A This bit is set whenever one of the flags ACS.SEDA, ACS.CEDA, ACS.CIA, ACS.SBVA changes from '0' to '1'. 1 = Error detected on channel A 0 = No error detected on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDA_A {
    #[doc = "0: N/A"]
    NO_CH_A_ERR_DETECTED = 0,
    #[doc = "1: N/A"]
    CH_A_ERR_DETECTED = 1,
}
impl From<EDA_A> for bool {
    #[inline(always)]
    fn from(variant: EDA_A) -> Self {
        variant as u8 != 0
    }
}
impl EDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDA_A {
        match self.bits {
            false => EDA_A::NO_CH_A_ERR_DETECTED,
            true => EDA_A::CH_A_ERR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_A_ERR_DETECTED`"]
    #[inline(always)]
    pub fn is_no_ch_a_err_detected(&self) -> bool {
        *self == EDA_A::NO_CH_A_ERR_DETECTED
    }
    #[doc = "Checks if the value of the field is `CH_A_ERR_DETECTED`"]
    #[inline(always)]
    pub fn is_ch_a_err_detected(&self) -> bool {
        *self == EDA_A::CH_A_ERR_DETECTED
    }
}
#[doc = "Field `EDA` writer - Error Detected on Channel A This bit is set whenever one of the flags ACS.SEDA, ACS.CEDA, ACS.CIA, ACS.SBVA changes from '0' to '1'. 1 = Error detected on channel A 0 = No error detected on channel A"]
pub type EDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, EDA_A, O>;
impl<'a, const O: u8> EDA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_a_err_detected(self) -> &'a mut W {
        self.variant(EDA_A::NO_CH_A_ERR_DETECTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_err_detected(self) -> &'a mut W {
        self.variant(EDA_A::CH_A_ERR_DETECTED)
    }
}
#[doc = "Field `LTVA` reader - Latest Transmit Violation Channel A The flag signals a latest transmit violation on channel A to the Host. 1 = Latest transmit violation detected on channel A 0 = No latest transmit violation detected on channel A"]
pub type LTVA_R = crate::BitReader<LTVA_A>;
#[doc = "Latest Transmit Violation Channel A The flag signals a latest transmit violation on channel A to the Host. 1 = Latest transmit violation detected on channel A 0 = No latest transmit violation detected on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTVA_A {
    #[doc = "0: N/A"]
    NO_CH_A_LATEST_TX_VIOLATION_DETECTED = 0,
    #[doc = "1: N/A"]
    CH_A_LATEST_TX_VIOLATION_DETECTED = 1,
}
impl From<LTVA_A> for bool {
    #[inline(always)]
    fn from(variant: LTVA_A) -> Self {
        variant as u8 != 0
    }
}
impl LTVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTVA_A {
        match self.bits {
            false => LTVA_A::NO_CH_A_LATEST_TX_VIOLATION_DETECTED,
            true => LTVA_A::CH_A_LATEST_TX_VIOLATION_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_A_LATEST_TX_VIOLATION_DETECTED`"]
    #[inline(always)]
    pub fn is_no_ch_a_latest_tx_violation_detected(&self) -> bool {
        *self == LTVA_A::NO_CH_A_LATEST_TX_VIOLATION_DETECTED
    }
    #[doc = "Checks if the value of the field is `CH_A_LATEST_TX_VIOLATION_DETECTED`"]
    #[inline(always)]
    pub fn is_ch_a_latest_tx_violation_detected(&self) -> bool {
        *self == LTVA_A::CH_A_LATEST_TX_VIOLATION_DETECTED
    }
}
#[doc = "Field `LTVA` writer - Latest Transmit Violation Channel A The flag signals a latest transmit violation on channel A to the Host. 1 = Latest transmit violation detected on channel A 0 = No latest transmit violation detected on channel A"]
pub type LTVA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, LTVA_A, O>;
impl<'a, const O: u8> LTVA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_a_latest_tx_violation_detected(self) -> &'a mut W {
        self.variant(LTVA_A::NO_CH_A_LATEST_TX_VIOLATION_DETECTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_latest_tx_violation_detected(self) -> &'a mut W {
        self.variant(LTVA_A::CH_A_LATEST_TX_VIOLATION_DETECTED)
    }
}
#[doc = "Field `TABA` reader - Transmission Across Boundary Channel A The flag signals to the Host that a transmission across a slot boundary occurred for channel A. 1 = Transmission across slot boundary detected on channel A 0 = No transmission across slot boundary detected on channel A"]
pub type TABA_R = crate::BitReader<TABA_A>;
#[doc = "Transmission Across Boundary Channel A The flag signals to the Host that a transmission across a slot boundary occurred for channel A. 1 = Transmission across slot boundary detected on channel A 0 = No transmission across slot boundary detected on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABA_A {
    #[doc = "0: N/A"]
    NO_CH_A_TX_ACROSS_SLOT_BOUND_DETECTED = 0,
    #[doc = "1: N/A"]
    CH_A_TX_ACROSS_SLOT_BOUND_DETECTED = 1,
}
impl From<TABA_A> for bool {
    #[inline(always)]
    fn from(variant: TABA_A) -> Self {
        variant as u8 != 0
    }
}
impl TABA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABA_A {
        match self.bits {
            false => TABA_A::NO_CH_A_TX_ACROSS_SLOT_BOUND_DETECTED,
            true => TABA_A::CH_A_TX_ACROSS_SLOT_BOUND_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_A_TX_ACROSS_SLOT_BOUND_DETECTED`"]
    #[inline(always)]
    pub fn is_no_ch_a_tx_across_slot_bound_detected(&self) -> bool {
        *self == TABA_A::NO_CH_A_TX_ACROSS_SLOT_BOUND_DETECTED
    }
    #[doc = "Checks if the value of the field is `CH_A_TX_ACROSS_SLOT_BOUND_DETECTED`"]
    #[inline(always)]
    pub fn is_ch_a_tx_across_slot_bound_detected(&self) -> bool {
        *self == TABA_A::CH_A_TX_ACROSS_SLOT_BOUND_DETECTED
    }
}
#[doc = "Field `TABA` writer - Transmission Across Boundary Channel A The flag signals to the Host that a transmission across a slot boundary occurred for channel A. 1 = Transmission across slot boundary detected on channel A 0 = No transmission across slot boundary detected on channel A"]
pub type TABA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, TABA_A, O>;
impl<'a, const O: u8> TABA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_a_tx_across_slot_bound_detected(self) -> &'a mut W {
        self.variant(TABA_A::NO_CH_A_TX_ACROSS_SLOT_BOUND_DETECTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_a_tx_across_slot_bound_detected(self) -> &'a mut W {
        self.variant(TABA_A::CH_A_TX_ACROSS_SLOT_BOUND_DETECTED)
    }
}
#[doc = "Field `EDB` reader - Error Detected on Channel B This bit is set whenever one of the flags ACS.SEDB, ACS.CEDB, ACS.CIB, ACS.SBVB changes from '0' to '1'. 1 = Error detected on channel B 0 = No error detected on channel B"]
pub type EDB_R = crate::BitReader<EDB_A>;
#[doc = "Error Detected on Channel B This bit is set whenever one of the flags ACS.SEDB, ACS.CEDB, ACS.CIB, ACS.SBVB changes from '0' to '1'. 1 = Error detected on channel B 0 = No error detected on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDB_A {
    #[doc = "0: N/A"]
    NO_CH_B_ERR_DETECTED = 0,
    #[doc = "1: N/A"]
    CH_B_ERR_DETECTED = 1,
}
impl From<EDB_A> for bool {
    #[inline(always)]
    fn from(variant: EDB_A) -> Self {
        variant as u8 != 0
    }
}
impl EDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDB_A {
        match self.bits {
            false => EDB_A::NO_CH_B_ERR_DETECTED,
            true => EDB_A::CH_B_ERR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_B_ERR_DETECTED`"]
    #[inline(always)]
    pub fn is_no_ch_b_err_detected(&self) -> bool {
        *self == EDB_A::NO_CH_B_ERR_DETECTED
    }
    #[doc = "Checks if the value of the field is `CH_B_ERR_DETECTED`"]
    #[inline(always)]
    pub fn is_ch_b_err_detected(&self) -> bool {
        *self == EDB_A::CH_B_ERR_DETECTED
    }
}
#[doc = "Field `EDB` writer - Error Detected on Channel B This bit is set whenever one of the flags ACS.SEDB, ACS.CEDB, ACS.CIB, ACS.SBVB changes from '0' to '1'. 1 = Error detected on channel B 0 = No error detected on channel B"]
pub type EDB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, EDB_A, O>;
impl<'a, const O: u8> EDB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_b_err_detected(self) -> &'a mut W {
        self.variant(EDB_A::NO_CH_B_ERR_DETECTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_err_detected(self) -> &'a mut W {
        self.variant(EDB_A::CH_B_ERR_DETECTED)
    }
}
#[doc = "Field `LTVB` reader - Latest Transmit Violation Channel B The flag signals a latest transmit violation on channel B to the Host. 1 = Latest transmit violation detected on channel B 0 = No latest transmit violation detected on channel B"]
pub type LTVB_R = crate::BitReader<LTVB_A>;
#[doc = "Latest Transmit Violation Channel B The flag signals a latest transmit violation on channel B to the Host. 1 = Latest transmit violation detected on channel B 0 = No latest transmit violation detected on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTVB_A {
    #[doc = "0: N/A"]
    NO_CH_B_LATEST_TX_VIOLATION_DETECTED = 0,
    #[doc = "1: N/A"]
    CH_B_LATEST_TX_VIOLATION_DETECTED = 1,
}
impl From<LTVB_A> for bool {
    #[inline(always)]
    fn from(variant: LTVB_A) -> Self {
        variant as u8 != 0
    }
}
impl LTVB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTVB_A {
        match self.bits {
            false => LTVB_A::NO_CH_B_LATEST_TX_VIOLATION_DETECTED,
            true => LTVB_A::CH_B_LATEST_TX_VIOLATION_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_B_LATEST_TX_VIOLATION_DETECTED`"]
    #[inline(always)]
    pub fn is_no_ch_b_latest_tx_violation_detected(&self) -> bool {
        *self == LTVB_A::NO_CH_B_LATEST_TX_VIOLATION_DETECTED
    }
    #[doc = "Checks if the value of the field is `CH_B_LATEST_TX_VIOLATION_DETECTED`"]
    #[inline(always)]
    pub fn is_ch_b_latest_tx_violation_detected(&self) -> bool {
        *self == LTVB_A::CH_B_LATEST_TX_VIOLATION_DETECTED
    }
}
#[doc = "Field `LTVB` writer - Latest Transmit Violation Channel B The flag signals a latest transmit violation on channel B to the Host. 1 = Latest transmit violation detected on channel B 0 = No latest transmit violation detected on channel B"]
pub type LTVB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, LTVB_A, O>;
impl<'a, const O: u8> LTVB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_b_latest_tx_violation_detected(self) -> &'a mut W {
        self.variant(LTVB_A::NO_CH_B_LATEST_TX_VIOLATION_DETECTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_latest_tx_violation_detected(self) -> &'a mut W {
        self.variant(LTVB_A::CH_B_LATEST_TX_VIOLATION_DETECTED)
    }
}
#[doc = "Field `TABB` reader - Transmission Across Boundary Channel B The flag signals to the Host that a transmission across a slot boundary occurred for channel B. 1 = Transmission across slot boundary detected on channel B 0 = No transmission across slot boundary detected on channel B"]
pub type TABB_R = crate::BitReader<TABB_A>;
#[doc = "Transmission Across Boundary Channel B The flag signals to the Host that a transmission across a slot boundary occurred for channel B. 1 = Transmission across slot boundary detected on channel B 0 = No transmission across slot boundary detected on channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABB_A {
    #[doc = "0: N/A"]
    NO_CH_B_TX_ACROSS_SLOT_BOUND_DETECTED = 0,
    #[doc = "1: N/A"]
    CH_B_TX_ACROSS_SLOT_BOUND_DETECTED = 1,
}
impl From<TABB_A> for bool {
    #[inline(always)]
    fn from(variant: TABB_A) -> Self {
        variant as u8 != 0
    }
}
impl TABB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABB_A {
        match self.bits {
            false => TABB_A::NO_CH_B_TX_ACROSS_SLOT_BOUND_DETECTED,
            true => TABB_A::CH_B_TX_ACROSS_SLOT_BOUND_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CH_B_TX_ACROSS_SLOT_BOUND_DETECTED`"]
    #[inline(always)]
    pub fn is_no_ch_b_tx_across_slot_bound_detected(&self) -> bool {
        *self == TABB_A::NO_CH_B_TX_ACROSS_SLOT_BOUND_DETECTED
    }
    #[doc = "Checks if the value of the field is `CH_B_TX_ACROSS_SLOT_BOUND_DETECTED`"]
    #[inline(always)]
    pub fn is_ch_b_tx_across_slot_bound_detected(&self) -> bool {
        *self == TABB_A::CH_B_TX_ACROSS_SLOT_BOUND_DETECTED
    }
}
#[doc = "Field `TABB` writer - Transmission Across Boundary Channel B The flag signals to the Host that a transmission across a slot boundary occurred for channel B. 1 = Transmission across slot boundary detected on channel B 0 = No transmission across slot boundary detected on channel B"]
pub type TABB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, TABB_A, O>;
impl<'a, const O: u8> TABB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_ch_b_tx_across_slot_bound_detected(self) -> &'a mut W {
        self.variant(TABB_A::NO_CH_B_TX_ACROSS_SLOT_BOUND_DETECTED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ch_b_tx_across_slot_bound_detected(self) -> &'a mut W {
        self.variant(TABB_A::CH_B_TX_ACROSS_SLOT_BOUND_DETECTED)
    }
}
impl R {
    #[doc = "Bit 0 - POC Error Mode Changed This flag is set whenever the error mode signalled by CCEV.ERRM\\[1:0\\]
has changed. 1 = Error mode has changed 0 = Error mode has not changed"]
    #[inline(always)]
    pub fn pemc(&self) -> PEMC_R {
        PEMC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Not Accepted The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the requested command was not valid in the actual POC state, or because the CHI command was locked (CCL = '1'). 1 = CHI command not accepted 0 = CHI command accepted"]
    #[inline(always)]
    pub fn cna(&self) -> CNA_R {
        CNA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sync Frames Below Minimum This flag signals that the number of sync frames received during the last communication cycle was below the limit required by the FlexRay protocol. May be set during startup and therefore should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Less than the required minimum of sync frames received 0 = Sync node: 1 or more sync frames received, non-sync node: 2 or more sync frames received"]
    #[inline(always)]
    pub fn sfbm(&self) -> SFBM_R {
        SFBM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync Frame Overflow Set when either the number of sync frames received during the last communication cycle or the total number of different sync frame IDs received during the last double cycle exceeds the maximum number of sync frames as defined by GTUC2.SNM\\[3:0\\]. 1 = More sync frames received than configured by GTUC2.SNM\\[3:0\\]
0 = Number of received sync frames &lt;= GTUC2.SNM\\[3:0\\]"]
    #[inline(always)]
    pub fn sfo(&self) -> SFO_R {
        SFO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Correction Failure This flag is set at the end of the cycle whenever one of the following errors occurred: Missing offset and / or rate correction Clock correction limit reached The clock correction status is monitored in registers CCEV and SFS. A failure may occur during startup, therefore bit CCF should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Clock correction failed 0 = No clock correction error"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CHI Command Locked The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the execution of the previous CHI command has not yet completed. In this case bit CNA is also set to '1'. 1 = CHI command not accepted 0 = CHI command accepted"]
    #[inline(always)]
    pub fn ccl(&self) -> CCL_R {
        CCL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Parity Error The flag signals a parity error to the Host. It is set whenever one of the flags MHDS.PIBF, MHDS.POBF, MHDS.PMR, MHDS.PTBF1, MHDS.PTBF2 changes from '0' to '1'. 1 = Parity error detected 0 = No parity error detected"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Overrun The flag is set by the CC when a receive FIFO overrun is detected. When a receive FIFO overrun occurs, the oldest message is overwritten with the actual received message. The actual state of the FIFO is monitored in register FSR. 1 = A receive FIFO overrun has been detected 0 = No receive FIFO overrun detected"]
    #[inline(always)]
    pub fn rfo(&self) -> RFO_R {
        RFO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Empty FIFO Access This flag is set by the CC when the Host requests the transfer of a message from the receive FIFO via Output Buffer while the receive FIFO is empty. 1 = Host access to empty FIFO occurred 0 = No Host access to empty FIFO occurred"]
    #[inline(always)]
    pub fn efa(&self) -> EFA_R {
        EFA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Illegal Input Buffer Access This flag is set by the CC when the Host wants to modify a message buffer via Input Buffer and one of the following conditions applies: 1) The CC is not in CONFIG or DEFAULT_CONFIG state and the Host writes to the Input Buffer Command Request register to modify the Header section of message buffer 0, 1 if configured for transmission in key slot Header section of static message buffers with buffer number &lt; MRC.FDB\\[7:0\\]
while MRC.SEC\\[1:0\\]
= '01' Header section of any static or dynamic message buffer while MRC.SEC\\[1:0\\]
= '1x' Header and / or data section of any message buffer belonging to the receive FIFO 2) The Host writes to any register of the Input Buffer while IBCR.IBSYH is set to '1'. 1 = Illegal Host access to Input Buffer occurred 0 = No illegal Host access to Input Buffer occurred"]
    #[inline(always)]
    pub fn iiba(&self) -> IIBA_R {
        IIBA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Illegal Output buffer Access This flag is set by the CC when the Host requests the transfer of a message buffer from the Message RAM to the Output Buffer while OBCR.OBSYS is set to '1'. 1 = Illegal Host access to Output Buffer occurred 0 = No illegal Host access to Output Buffer occurred"]
    #[inline(always)]
    pub fn ioba(&self) -> IOBA_R {
        IOBA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Message Handler Constraints Flag The flag signals a Message Handler constraints violation condition. It is set whenever one of the flags MHDF.SNUA, MHDF.SNUB, MHDF.FNFA, MHDF.FNFB, MHDF.TBFA, MHDF.TBFB, MHDF.WAHP changes from '0' to '1'. 1 = Message Handler failure detected 0 = No Message Handler failure detected"]
    #[inline(always)]
    pub fn mhf(&self) -> MHF_R {
        MHF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Detected on Channel A This bit is set whenever one of the flags ACS.SEDA, ACS.CEDA, ACS.CIA, ACS.SBVA changes from '0' to '1'. 1 = Error detected on channel A 0 = No error detected on channel A"]
    #[inline(always)]
    pub fn eda(&self) -> EDA_R {
        EDA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Latest Transmit Violation Channel A The flag signals a latest transmit violation on channel A to the Host. 1 = Latest transmit violation detected on channel A 0 = No latest transmit violation detected on channel A"]
    #[inline(always)]
    pub fn ltva(&self) -> LTVA_R {
        LTVA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmission Across Boundary Channel A The flag signals to the Host that a transmission across a slot boundary occurred for channel A. 1 = Transmission across slot boundary detected on channel A 0 = No transmission across slot boundary detected on channel A"]
    #[inline(always)]
    pub fn taba(&self) -> TABA_R {
        TABA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Error Detected on Channel B This bit is set whenever one of the flags ACS.SEDB, ACS.CEDB, ACS.CIB, ACS.SBVB changes from '0' to '1'. 1 = Error detected on channel B 0 = No error detected on channel B"]
    #[inline(always)]
    pub fn edb(&self) -> EDB_R {
        EDB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Latest Transmit Violation Channel B The flag signals a latest transmit violation on channel B to the Host. 1 = Latest transmit violation detected on channel B 0 = No latest transmit violation detected on channel B"]
    #[inline(always)]
    pub fn ltvb(&self) -> LTVB_R {
        LTVB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Across Boundary Channel B The flag signals to the Host that a transmission across a slot boundary occurred for channel B. 1 = Transmission across slot boundary detected on channel B 0 = No transmission across slot boundary detected on channel B"]
    #[inline(always)]
    pub fn tabb(&self) -> TABB_R {
        TABB_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POC Error Mode Changed This flag is set whenever the error mode signalled by CCEV.ERRM\\[1:0\\]
has changed. 1 = Error mode has changed 0 = Error mode has not changed"]
    #[inline(always)]
    #[must_use]
    pub fn pemc(&mut self) -> PEMC_W<0> {
        PEMC_W::new(self)
    }
    #[doc = "Bit 1 - Command Not Accepted The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the requested command was not valid in the actual POC state, or because the CHI command was locked (CCL = '1'). 1 = CHI command not accepted 0 = CHI command accepted"]
    #[inline(always)]
    #[must_use]
    pub fn cna(&mut self) -> CNA_W<1> {
        CNA_W::new(self)
    }
    #[doc = "Bit 2 - Sync Frames Below Minimum This flag signals that the number of sync frames received during the last communication cycle was below the limit required by the FlexRay protocol. May be set during startup and therefore should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Less than the required minimum of sync frames received 0 = Sync node: 1 or more sync frames received, non-sync node: 2 or more sync frames received"]
    #[inline(always)]
    #[must_use]
    pub fn sfbm(&mut self) -> SFBM_W<2> {
        SFBM_W::new(self)
    }
    #[doc = "Bit 3 - Sync Frame Overflow Set when either the number of sync frames received during the last communication cycle or the total number of different sync frame IDs received during the last double cycle exceeds the maximum number of sync frames as defined by GTUC2.SNM\\[3:0\\]. 1 = More sync frames received than configured by GTUC2.SNM\\[3:0\\]
0 = Number of received sync frames &lt;= GTUC2.SNM\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sfo(&mut self) -> SFO_W<3> {
        SFO_W::new(self)
    }
    #[doc = "Bit 4 - Clock Correction Failure This flag is set at the end of the cycle whenever one of the following errors occurred: Missing offset and / or rate correction Clock correction limit reached The clock correction status is monitored in registers CCEV and SFS. A failure may occur during startup, therefore bit CCF should be cleared by the Host after the CC entered NORMAL_ACTIVE state. 1 = Clock correction failed 0 = No clock correction error"]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<4> {
        CCF_W::new(self)
    }
    #[doc = "Bit 5 - CHI Command Locked The flag signals that the write access to the CHI command vector SUCC1.CMD\\[3:0\\]
was not successful because the execution of the previous CHI command has not yet completed. In this case bit CNA is also set to '1'. 1 = CHI command not accepted 0 = CHI command accepted"]
    #[inline(always)]
    #[must_use]
    pub fn ccl(&mut self) -> CCL_W<5> {
        CCL_W::new(self)
    }
    #[doc = "Bit 6 - Parity Error The flag signals a parity error to the Host. It is set whenever one of the flags MHDS.PIBF, MHDS.POBF, MHDS.PMR, MHDS.PTBF1, MHDS.PTBF2 changes from '0' to '1'. 1 = Parity error detected 0 = No parity error detected"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<6> {
        PERR_W::new(self)
    }
    #[doc = "Bit 7 - Receive FIFO Overrun The flag is set by the CC when a receive FIFO overrun is detected. When a receive FIFO overrun occurs, the oldest message is overwritten with the actual received message. The actual state of the FIFO is monitored in register FSR. 1 = A receive FIFO overrun has been detected 0 = No receive FIFO overrun detected"]
    #[inline(always)]
    #[must_use]
    pub fn rfo(&mut self) -> RFO_W<7> {
        RFO_W::new(self)
    }
    #[doc = "Bit 8 - Empty FIFO Access This flag is set by the CC when the Host requests the transfer of a message from the receive FIFO via Output Buffer while the receive FIFO is empty. 1 = Host access to empty FIFO occurred 0 = No Host access to empty FIFO occurred"]
    #[inline(always)]
    #[must_use]
    pub fn efa(&mut self) -> EFA_W<8> {
        EFA_W::new(self)
    }
    #[doc = "Bit 9 - Illegal Input Buffer Access This flag is set by the CC when the Host wants to modify a message buffer via Input Buffer and one of the following conditions applies: 1) The CC is not in CONFIG or DEFAULT_CONFIG state and the Host writes to the Input Buffer Command Request register to modify the Header section of message buffer 0, 1 if configured for transmission in key slot Header section of static message buffers with buffer number &lt; MRC.FDB\\[7:0\\]
while MRC.SEC\\[1:0\\]
= '01' Header section of any static or dynamic message buffer while MRC.SEC\\[1:0\\]
= '1x' Header and / or data section of any message buffer belonging to the receive FIFO 2) The Host writes to any register of the Input Buffer while IBCR.IBSYH is set to '1'. 1 = Illegal Host access to Input Buffer occurred 0 = No illegal Host access to Input Buffer occurred"]
    #[inline(always)]
    #[must_use]
    pub fn iiba(&mut self) -> IIBA_W<9> {
        IIBA_W::new(self)
    }
    #[doc = "Bit 10 - Illegal Output buffer Access This flag is set by the CC when the Host requests the transfer of a message buffer from the Message RAM to the Output Buffer while OBCR.OBSYS is set to '1'. 1 = Illegal Host access to Output Buffer occurred 0 = No illegal Host access to Output Buffer occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ioba(&mut self) -> IOBA_W<10> {
        IOBA_W::new(self)
    }
    #[doc = "Bit 11 - Message Handler Constraints Flag The flag signals a Message Handler constraints violation condition. It is set whenever one of the flags MHDF.SNUA, MHDF.SNUB, MHDF.FNFA, MHDF.FNFB, MHDF.TBFA, MHDF.TBFB, MHDF.WAHP changes from '0' to '1'. 1 = Message Handler failure detected 0 = No Message Handler failure detected"]
    #[inline(always)]
    #[must_use]
    pub fn mhf(&mut self) -> MHF_W<11> {
        MHF_W::new(self)
    }
    #[doc = "Bit 16 - Error Detected on Channel A This bit is set whenever one of the flags ACS.SEDA, ACS.CEDA, ACS.CIA, ACS.SBVA changes from '0' to '1'. 1 = Error detected on channel A 0 = No error detected on channel A"]
    #[inline(always)]
    #[must_use]
    pub fn eda(&mut self) -> EDA_W<16> {
        EDA_W::new(self)
    }
    #[doc = "Bit 17 - Latest Transmit Violation Channel A The flag signals a latest transmit violation on channel A to the Host. 1 = Latest transmit violation detected on channel A 0 = No latest transmit violation detected on channel A"]
    #[inline(always)]
    #[must_use]
    pub fn ltva(&mut self) -> LTVA_W<17> {
        LTVA_W::new(self)
    }
    #[doc = "Bit 18 - Transmission Across Boundary Channel A The flag signals to the Host that a transmission across a slot boundary occurred for channel A. 1 = Transmission across slot boundary detected on channel A 0 = No transmission across slot boundary detected on channel A"]
    #[inline(always)]
    #[must_use]
    pub fn taba(&mut self) -> TABA_W<18> {
        TABA_W::new(self)
    }
    #[doc = "Bit 24 - Error Detected on Channel B This bit is set whenever one of the flags ACS.SEDB, ACS.CEDB, ACS.CIB, ACS.SBVB changes from '0' to '1'. 1 = Error detected on channel B 0 = No error detected on channel B"]
    #[inline(always)]
    #[must_use]
    pub fn edb(&mut self) -> EDB_W<24> {
        EDB_W::new(self)
    }
    #[doc = "Bit 25 - Latest Transmit Violation Channel B The flag signals a latest transmit violation on channel B to the Host. 1 = Latest transmit violation detected on channel B 0 = No latest transmit violation detected on channel B"]
    #[inline(always)]
    #[must_use]
    pub fn ltvb(&mut self) -> LTVB_W<25> {
        LTVB_W::new(self)
    }
    #[doc = "Bit 26 - Transmission Across Boundary Channel B The flag signals to the Host that a transmission across a slot boundary occurred for channel B. 1 = Transmission across slot boundary detected on channel B 0 = No transmission across slot boundary detected on channel B"]
    #[inline(always)]
    #[must_use]
    pub fn tabb(&mut self) -> TABB_W<26> {
        TABB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eir](index.html) module"]
pub struct EIR_SPEC;
impl crate::RegisterSpec for EIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eir::R](R) reader structure"]
impl crate::Readable for EIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eir::W](W) writer structure"]
impl crate::Writable for EIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIR to value 0"]
impl crate::Resettable for EIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
