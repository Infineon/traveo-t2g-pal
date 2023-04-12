#[doc = "Register `ACS` reader"]
pub struct R(crate::R<ACS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACS` writer"]
pub struct W(crate::W<ACS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACS_SPEC>;
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
impl From<crate::W<ACS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VFRA_` reader - Valid Frame Received on Channel A (vSS!ValidFrameA) One or more valid frames were received on channel A in any static or dynamic slot during the observation period. 1 = Valid frame(s) received on channel A 0 = No valid frame received"]
pub type VFRA__R = crate::BitReader<VFRA__A>;
#[doc = "Valid Frame Received on Channel A (vSS!ValidFrameA) One or more valid frames were received on channel A in any static or dynamic slot during the observation period. 1 = Valid frame(s) received on channel A 0 = No valid frame received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VFRA__A {
    #[doc = "0: N/A"]
    NO_VALID_FRAME_RXD_ON_CH_A = 0,
    #[doc = "1: N/A"]
    VALID_FRAME_RXD_ON_CH_A = 1,
}
impl From<VFRA__A> for bool {
    #[inline(always)]
    fn from(variant: VFRA__A) -> Self {
        variant as u8 != 0
    }
}
impl VFRA__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VFRA__A {
        match self.bits {
            false => VFRA__A::NO_VALID_FRAME_RXD_ON_CH_A,
            true => VFRA__A::VALID_FRAME_RXD_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_VALID_FRAME_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_no_valid_frame_rxd_on_ch_a(&self) -> bool {
        *self == VFRA__A::NO_VALID_FRAME_RXD_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `VALID_FRAME_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_valid_frame_rxd_on_ch_a(&self) -> bool {
        *self == VFRA__A::VALID_FRAME_RXD_ON_CH_A
    }
}
#[doc = "Field `VFRA_` writer - Valid Frame Received on Channel A (vSS!ValidFrameA) One or more valid frames were received on channel A in any static or dynamic slot during the observation period. 1 = Valid frame(s) received on channel A 0 = No valid frame received"]
pub type VFRA__W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, VFRA__A, O>;
impl<'a, const O: u8> VFRA__W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_valid_frame_rxd_on_ch_a(self) -> &'a mut W {
        self.variant(VFRA__A::NO_VALID_FRAME_RXD_ON_CH_A)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn valid_frame_rxd_on_ch_a(self) -> &'a mut W {
        self.variant(VFRA__A::VALID_FRAME_RXD_ON_CH_A)
    }
}
#[doc = "Field `SEDA` reader - Syntax Error Detected on Channel A (vSS!SyntaxErrorA) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel A. 1 = Syntax error(s) observed on channel A 0 = No syntax error observed"]
pub type SEDA_R = crate::BitReader<SEDA_A>;
#[doc = "Syntax Error Detected on Channel A (vSS!SyntaxErrorA) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel A. 1 = Syntax error(s) observed on channel A 0 = No syntax error observed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEDA_A {
    #[doc = "0: N/A"]
    NO_SYNTAX_ERR_OBSVD_ON_CH_A = 0,
    #[doc = "1: N/A"]
    SYNTAX_ERR_OBSVD_ON_CH_A = 1,
}
impl From<SEDA_A> for bool {
    #[inline(always)]
    fn from(variant: SEDA_A) -> Self {
        variant as u8 != 0
    }
}
impl SEDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEDA_A {
        match self.bits {
            false => SEDA_A::NO_SYNTAX_ERR_OBSVD_ON_CH_A,
            true => SEDA_A::SYNTAX_ERR_OBSVD_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNTAX_ERR_OBSVD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_no_syntax_err_obsvd_on_ch_a(&self) -> bool {
        *self == SEDA_A::NO_SYNTAX_ERR_OBSVD_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `SYNTAX_ERR_OBSVD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_syntax_err_obsvd_on_ch_a(&self) -> bool {
        *self == SEDA_A::SYNTAX_ERR_OBSVD_ON_CH_A
    }
}
#[doc = "Field `SEDA` writer - Syntax Error Detected on Channel A (vSS!SyntaxErrorA) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel A. 1 = Syntax error(s) observed on channel A 0 = No syntax error observed"]
pub type SEDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, SEDA_A, O>;
impl<'a, const O: u8> SEDA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_syntax_err_obsvd_on_ch_a(self) -> &'a mut W {
        self.variant(SEDA_A::NO_SYNTAX_ERR_OBSVD_ON_CH_A)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn syntax_err_obsvd_on_ch_a(self) -> &'a mut W {
        self.variant(SEDA_A::SYNTAX_ERR_OBSVD_ON_CH_A)
    }
}
#[doc = "Field `CEDA` reader - Content Error Detected on Channel A (vSS!ContentErrorA) One or more frames with a content error were received on channel A in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel A 0 = No frame with content error received"]
pub type CEDA_R = crate::BitReader<CEDA_A>;
#[doc = "Content Error Detected on Channel A (vSS!ContentErrorA) One or more frames with a content error were received on channel A in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel A 0 = No frame with content error received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEDA_A {
    #[doc = "0: N/A"]
    NO_FRAME_WITH_CONTENT_ERR_ON_CH_A = 0,
    #[doc = "1: N/A"]
    FRAME_WITH_CONTENT_ERR_RXD_ON_CH_A = 1,
}
impl From<CEDA_A> for bool {
    #[inline(always)]
    fn from(variant: CEDA_A) -> Self {
        variant as u8 != 0
    }
}
impl CEDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEDA_A {
        match self.bits {
            false => CEDA_A::NO_FRAME_WITH_CONTENT_ERR_ON_CH_A,
            true => CEDA_A::FRAME_WITH_CONTENT_ERR_RXD_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FRAME_WITH_CONTENT_ERR_ON_CH_A`"]
    #[inline(always)]
    pub fn is_no_frame_with_content_err_on_ch_a(&self) -> bool {
        *self == CEDA_A::NO_FRAME_WITH_CONTENT_ERR_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `FRAME_WITH_CONTENT_ERR_RXD_ON_CH_A`"]
    #[inline(always)]
    pub fn is_frame_with_content_err_rxd_on_ch_a(&self) -> bool {
        *self == CEDA_A::FRAME_WITH_CONTENT_ERR_RXD_ON_CH_A
    }
}
#[doc = "Field `CEDA` writer - Content Error Detected on Channel A (vSS!ContentErrorA) One or more frames with a content error were received on channel A in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel A 0 = No frame with content error received"]
pub type CEDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, CEDA_A, O>;
impl<'a, const O: u8> CEDA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_frame_with_content_err_on_ch_a(self) -> &'a mut W {
        self.variant(CEDA_A::NO_FRAME_WITH_CONTENT_ERR_ON_CH_A)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn frame_with_content_err_rxd_on_ch_a(self) -> &'a mut W {
        self.variant(CEDA_A::FRAME_WITH_CONTENT_ERR_RXD_ON_CH_A)
    }
}
#[doc = "Field `CIA` reader - Communication Indicator Channel A One or more valid frames were received on channel A in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel A in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication"]
pub type CIA_R = crate::BitReader<CIA_A>;
#[doc = "Communication Indicator Channel A One or more valid frames were received on channel A in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel A in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIA_A {
    #[doc = "0: N/A"]
    NO_VALID_FRAME_RXD_ON_CH_A_IN_SLOTS = 0,
    #[doc = "1: N/A"]
    VALID_FRAME_RXD_ON_CH_A_IN_SLOTS = 1,
}
impl From<CIA_A> for bool {
    #[inline(always)]
    fn from(variant: CIA_A) -> Self {
        variant as u8 != 0
    }
}
impl CIA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIA_A {
        match self.bits {
            false => CIA_A::NO_VALID_FRAME_RXD_ON_CH_A_IN_SLOTS,
            true => CIA_A::VALID_FRAME_RXD_ON_CH_A_IN_SLOTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_VALID_FRAME_RXD_ON_CH_A_IN_SLOTS`"]
    #[inline(always)]
    pub fn is_no_valid_frame_rxd_on_ch_a_in_slots(&self) -> bool {
        *self == CIA_A::NO_VALID_FRAME_RXD_ON_CH_A_IN_SLOTS
    }
    #[doc = "Checks if the value of the field is `VALID_FRAME_RXD_ON_CH_A_IN_SLOTS`"]
    #[inline(always)]
    pub fn is_valid_frame_rxd_on_ch_a_in_slots(&self) -> bool {
        *self == CIA_A::VALID_FRAME_RXD_ON_CH_A_IN_SLOTS
    }
}
#[doc = "Field `CIA` writer - Communication Indicator Channel A One or more valid frames were received on channel A in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel A in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication"]
pub type CIA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, CIA_A, O>;
impl<'a, const O: u8> CIA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_valid_frame_rxd_on_ch_a_in_slots(self) -> &'a mut W {
        self.variant(CIA_A::NO_VALID_FRAME_RXD_ON_CH_A_IN_SLOTS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn valid_frame_rxd_on_ch_a_in_slots(self) -> &'a mut W {
        self.variant(CIA_A::VALID_FRAME_RXD_ON_CH_A_IN_SLOTS)
    }
}
#[doc = "Field `SBVA` reader - Slot Boundary Violation on Channel A (vSS!BViolationA) One or more slot boundary violations were observed on channel A at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel A 0 = No slot boundary violation observed"]
pub type SBVA_R = crate::BitReader<SBVA_A>;
#[doc = "Slot Boundary Violation on Channel A (vSS!BViolationA) One or more slot boundary violations were observed on channel A at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel A 0 = No slot boundary violation observed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBVA_A {
    #[doc = "0: N/A"]
    NO_BNDRY_VIOLATION_ERROR_IN_CH_A = 0,
    #[doc = "1: N/A"]
    BNDRY_VIOLATION_ERROR_IN_CH_A = 1,
}
impl From<SBVA_A> for bool {
    #[inline(always)]
    fn from(variant: SBVA_A) -> Self {
        variant as u8 != 0
    }
}
impl SBVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBVA_A {
        match self.bits {
            false => SBVA_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_A,
            true => SBVA_A::BNDRY_VIOLATION_ERROR_IN_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BNDRY_VIOLATION_ERROR_IN_CH_A`"]
    #[inline(always)]
    pub fn is_no_bndry_violation_error_in_ch_a(&self) -> bool {
        *self == SBVA_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_A
    }
    #[doc = "Checks if the value of the field is `BNDRY_VIOLATION_ERROR_IN_CH_A`"]
    #[inline(always)]
    pub fn is_bndry_violation_error_in_ch_a(&self) -> bool {
        *self == SBVA_A::BNDRY_VIOLATION_ERROR_IN_CH_A
    }
}
#[doc = "Field `SBVA` writer - Slot Boundary Violation on Channel A (vSS!BViolationA) One or more slot boundary violations were observed on channel A at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel A 0 = No slot boundary violation observed"]
pub type SBVA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, SBVA_A, O>;
impl<'a, const O: u8> SBVA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_bndry_violation_error_in_ch_a(self) -> &'a mut W {
        self.variant(SBVA_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_A)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn bndry_violation_error_in_ch_a(self) -> &'a mut W {
        self.variant(SBVA_A::BNDRY_VIOLATION_ERROR_IN_CH_A)
    }
}
#[doc = "Field `VFRB_` reader - Valid Frame Received on Channel B (vSS!ValidFrameB) One or more valid frames were received on channel B in any static or dynamic slot during the observation period. Reset under control of the Host. 1 = Valid frame(s) received on channel B 0 = No valid frame received"]
pub type VFRB__R = crate::BitReader<VFRB__A>;
#[doc = "Valid Frame Received on Channel B (vSS!ValidFrameB) One or more valid frames were received on channel B in any static or dynamic slot during the observation period. Reset under control of the Host. 1 = Valid frame(s) received on channel B 0 = No valid frame received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VFRB__A {
    #[doc = "0: N/A"]
    NO_VALID_FRAME_RXD_ON_CH_B = 0,
    #[doc = "1: N/A"]
    VALID_FRAME_RXD_ON_CH_B = 1,
}
impl From<VFRB__A> for bool {
    #[inline(always)]
    fn from(variant: VFRB__A) -> Self {
        variant as u8 != 0
    }
}
impl VFRB__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VFRB__A {
        match self.bits {
            false => VFRB__A::NO_VALID_FRAME_RXD_ON_CH_B,
            true => VFRB__A::VALID_FRAME_RXD_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_VALID_FRAME_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_no_valid_frame_rxd_on_ch_b(&self) -> bool {
        *self == VFRB__A::NO_VALID_FRAME_RXD_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `VALID_FRAME_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_valid_frame_rxd_on_ch_b(&self) -> bool {
        *self == VFRB__A::VALID_FRAME_RXD_ON_CH_B
    }
}
#[doc = "Field `VFRB_` writer - Valid Frame Received on Channel B (vSS!ValidFrameB) One or more valid frames were received on channel B in any static or dynamic slot during the observation period. Reset under control of the Host. 1 = Valid frame(s) received on channel B 0 = No valid frame received"]
pub type VFRB__W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, VFRB__A, O>;
impl<'a, const O: u8> VFRB__W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_valid_frame_rxd_on_ch_b(self) -> &'a mut W {
        self.variant(VFRB__A::NO_VALID_FRAME_RXD_ON_CH_B)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn valid_frame_rxd_on_ch_b(self) -> &'a mut W {
        self.variant(VFRB__A::VALID_FRAME_RXD_ON_CH_B)
    }
}
#[doc = "Field `SEDB` reader - Syntax Error Detected on Channel B (vSS!SyntaxErrorB) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel B. 1 = Syntax error(s) observed on channel B 0 = No syntax error observed"]
pub type SEDB_R = crate::BitReader<SEDB_A>;
#[doc = "Syntax Error Detected on Channel B (vSS!SyntaxErrorB) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel B. 1 = Syntax error(s) observed on channel B 0 = No syntax error observed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEDB_A {
    #[doc = "0: N/A"]
    NO_SYNTAX_ERR_OBSVD_ON_CH_B = 0,
    #[doc = "1: N/A"]
    SYNTAX_ERR_OBSVD_ON_CH_B = 1,
}
impl From<SEDB_A> for bool {
    #[inline(always)]
    fn from(variant: SEDB_A) -> Self {
        variant as u8 != 0
    }
}
impl SEDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEDB_A {
        match self.bits {
            false => SEDB_A::NO_SYNTAX_ERR_OBSVD_ON_CH_B,
            true => SEDB_A::SYNTAX_ERR_OBSVD_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNTAX_ERR_OBSVD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_no_syntax_err_obsvd_on_ch_b(&self) -> bool {
        *self == SEDB_A::NO_SYNTAX_ERR_OBSVD_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `SYNTAX_ERR_OBSVD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_syntax_err_obsvd_on_ch_b(&self) -> bool {
        *self == SEDB_A::SYNTAX_ERR_OBSVD_ON_CH_B
    }
}
#[doc = "Field `SEDB` writer - Syntax Error Detected on Channel B (vSS!SyntaxErrorB) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel B. 1 = Syntax error(s) observed on channel B 0 = No syntax error observed"]
pub type SEDB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, SEDB_A, O>;
impl<'a, const O: u8> SEDB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_syntax_err_obsvd_on_ch_b(self) -> &'a mut W {
        self.variant(SEDB_A::NO_SYNTAX_ERR_OBSVD_ON_CH_B)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn syntax_err_obsvd_on_ch_b(self) -> &'a mut W {
        self.variant(SEDB_A::SYNTAX_ERR_OBSVD_ON_CH_B)
    }
}
#[doc = "Field `CEDB` reader - Content Error Detected on Channel B (vSS!ContentErrorB) One or more frames with a content error were received on channel B in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel B 0 = No frame with content error received"]
pub type CEDB_R = crate::BitReader<CEDB_A>;
#[doc = "Content Error Detected on Channel B (vSS!ContentErrorB) One or more frames with a content error were received on channel B in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel B 0 = No frame with content error received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEDB_A {
    #[doc = "0: N/A"]
    NO_FRAME_WITH_CONTENT_ERR_ON_CH_B = 0,
    #[doc = "1: N/A"]
    FRAME_WITH_CONTENT_ERR_RXD_ON_CH_B = 1,
}
impl From<CEDB_A> for bool {
    #[inline(always)]
    fn from(variant: CEDB_A) -> Self {
        variant as u8 != 0
    }
}
impl CEDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEDB_A {
        match self.bits {
            false => CEDB_A::NO_FRAME_WITH_CONTENT_ERR_ON_CH_B,
            true => CEDB_A::FRAME_WITH_CONTENT_ERR_RXD_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FRAME_WITH_CONTENT_ERR_ON_CH_B`"]
    #[inline(always)]
    pub fn is_no_frame_with_content_err_on_ch_b(&self) -> bool {
        *self == CEDB_A::NO_FRAME_WITH_CONTENT_ERR_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `FRAME_WITH_CONTENT_ERR_RXD_ON_CH_B`"]
    #[inline(always)]
    pub fn is_frame_with_content_err_rxd_on_ch_b(&self) -> bool {
        *self == CEDB_A::FRAME_WITH_CONTENT_ERR_RXD_ON_CH_B
    }
}
#[doc = "Field `CEDB` writer - Content Error Detected on Channel B (vSS!ContentErrorB) One or more frames with a content error were received on channel B in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel B 0 = No frame with content error received"]
pub type CEDB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, CEDB_A, O>;
impl<'a, const O: u8> CEDB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_frame_with_content_err_on_ch_b(self) -> &'a mut W {
        self.variant(CEDB_A::NO_FRAME_WITH_CONTENT_ERR_ON_CH_B)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn frame_with_content_err_rxd_on_ch_b(self) -> &'a mut W {
        self.variant(CEDB_A::FRAME_WITH_CONTENT_ERR_RXD_ON_CH_B)
    }
}
#[doc = "Field `CIB` reader - Communication Indicator Channel B One or more valid frames were received on channel B in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel B in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication"]
pub type CIB_R = crate::BitReader<CIB_A>;
#[doc = "Communication Indicator Channel B One or more valid frames were received on channel B in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel B in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIB_A {
    #[doc = "0: N/A"]
    NO_VALID_FRAME_RXD_ON_CH_B_IN_SLOTS = 0,
    #[doc = "1: N/A"]
    VALID_FRAME_RXD_ON_CH_B_IN_SLOTS = 1,
}
impl From<CIB_A> for bool {
    #[inline(always)]
    fn from(variant: CIB_A) -> Self {
        variant as u8 != 0
    }
}
impl CIB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIB_A {
        match self.bits {
            false => CIB_A::NO_VALID_FRAME_RXD_ON_CH_B_IN_SLOTS,
            true => CIB_A::VALID_FRAME_RXD_ON_CH_B_IN_SLOTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_VALID_FRAME_RXD_ON_CH_B_IN_SLOTS`"]
    #[inline(always)]
    pub fn is_no_valid_frame_rxd_on_ch_b_in_slots(&self) -> bool {
        *self == CIB_A::NO_VALID_FRAME_RXD_ON_CH_B_IN_SLOTS
    }
    #[doc = "Checks if the value of the field is `VALID_FRAME_RXD_ON_CH_B_IN_SLOTS`"]
    #[inline(always)]
    pub fn is_valid_frame_rxd_on_ch_b_in_slots(&self) -> bool {
        *self == CIB_A::VALID_FRAME_RXD_ON_CH_B_IN_SLOTS
    }
}
#[doc = "Field `CIB` writer - Communication Indicator Channel B One or more valid frames were received on channel B in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel B in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication"]
pub type CIB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, CIB_A, O>;
impl<'a, const O: u8> CIB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_valid_frame_rxd_on_ch_b_in_slots(self) -> &'a mut W {
        self.variant(CIB_A::NO_VALID_FRAME_RXD_ON_CH_B_IN_SLOTS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn valid_frame_rxd_on_ch_b_in_slots(self) -> &'a mut W {
        self.variant(CIB_A::VALID_FRAME_RXD_ON_CH_B_IN_SLOTS)
    }
}
#[doc = "Field `SBVB` reader - Slot Boundary Violation on Channel B (vSS!BViolationB) One or more slot boundary violations were observed on channel B at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel B 0 = No slot boundary violation observed"]
pub type SBVB_R = crate::BitReader<SBVB_A>;
#[doc = "Slot Boundary Violation on Channel B (vSS!BViolationB) One or more slot boundary violations were observed on channel B at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel B 0 = No slot boundary violation observed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBVB_A {
    #[doc = "0: N/A"]
    NO_BNDRY_VIOLATION_ERROR_IN_CH_B = 0,
    #[doc = "1: N/A"]
    BNDRY_VIOLATION_ERROR_IN_CH_B = 1,
}
impl From<SBVB_A> for bool {
    #[inline(always)]
    fn from(variant: SBVB_A) -> Self {
        variant as u8 != 0
    }
}
impl SBVB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBVB_A {
        match self.bits {
            false => SBVB_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_B,
            true => SBVB_A::BNDRY_VIOLATION_ERROR_IN_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BNDRY_VIOLATION_ERROR_IN_CH_B`"]
    #[inline(always)]
    pub fn is_no_bndry_violation_error_in_ch_b(&self) -> bool {
        *self == SBVB_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_B
    }
    #[doc = "Checks if the value of the field is `BNDRY_VIOLATION_ERROR_IN_CH_B`"]
    #[inline(always)]
    pub fn is_bndry_violation_error_in_ch_b(&self) -> bool {
        *self == SBVB_A::BNDRY_VIOLATION_ERROR_IN_CH_B
    }
}
#[doc = "Field `SBVB` writer - Slot Boundary Violation on Channel B (vSS!BViolationB) One or more slot boundary violations were observed on channel B at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel B 0 = No slot boundary violation observed"]
pub type SBVB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACS_SPEC, SBVB_A, O>;
impl<'a, const O: u8> SBVB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_bndry_violation_error_in_ch_b(self) -> &'a mut W {
        self.variant(SBVB_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_B)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn bndry_violation_error_in_ch_b(self) -> &'a mut W {
        self.variant(SBVB_A::BNDRY_VIOLATION_ERROR_IN_CH_B)
    }
}
impl R {
    #[doc = "Bit 0 - Valid Frame Received on Channel A (vSS!ValidFrameA) One or more valid frames were received on channel A in any static or dynamic slot during the observation period. 1 = Valid frame(s) received on channel A 0 = No valid frame received"]
    #[inline(always)]
    pub fn vfra_(&self) -> VFRA__R {
        VFRA__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Syntax Error Detected on Channel A (vSS!SyntaxErrorA) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel A. 1 = Syntax error(s) observed on channel A 0 = No syntax error observed"]
    #[inline(always)]
    pub fn seda(&self) -> SEDA_R {
        SEDA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Content Error Detected on Channel A (vSS!ContentErrorA) One or more frames with a content error were received on channel A in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel A 0 = No frame with content error received"]
    #[inline(always)]
    pub fn ceda(&self) -> CEDA_R {
        CEDA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Communication Indicator Channel A One or more valid frames were received on channel A in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel A in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication"]
    #[inline(always)]
    pub fn cia(&self) -> CIA_R {
        CIA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slot Boundary Violation on Channel A (vSS!BViolationA) One or more slot boundary violations were observed on channel A at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel A 0 = No slot boundary violation observed"]
    #[inline(always)]
    pub fn sbva(&self) -> SBVA_R {
        SBVA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Valid Frame Received on Channel B (vSS!ValidFrameB) One or more valid frames were received on channel B in any static or dynamic slot during the observation period. Reset under control of the Host. 1 = Valid frame(s) received on channel B 0 = No valid frame received"]
    #[inline(always)]
    pub fn vfrb_(&self) -> VFRB__R {
        VFRB__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Syntax Error Detected on Channel B (vSS!SyntaxErrorB) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel B. 1 = Syntax error(s) observed on channel B 0 = No syntax error observed"]
    #[inline(always)]
    pub fn sedb(&self) -> SEDB_R {
        SEDB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Content Error Detected on Channel B (vSS!ContentErrorB) One or more frames with a content error were received on channel B in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel B 0 = No frame with content error received"]
    #[inline(always)]
    pub fn cedb(&self) -> CEDB_R {
        CEDB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Communication Indicator Channel B One or more valid frames were received on channel B in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel B in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication"]
    #[inline(always)]
    pub fn cib(&self) -> CIB_R {
        CIB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slot Boundary Violation on Channel B (vSS!BViolationB) One or more slot boundary violations were observed on channel B at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel B 0 = No slot boundary violation observed"]
    #[inline(always)]
    pub fn sbvb(&self) -> SBVB_R {
        SBVB_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Valid Frame Received on Channel A (vSS!ValidFrameA) One or more valid frames were received on channel A in any static or dynamic slot during the observation period. 1 = Valid frame(s) received on channel A 0 = No valid frame received"]
    #[inline(always)]
    #[must_use]
    pub fn vfra_(&mut self) -> VFRA__W<0> {
        VFRA__W::new(self)
    }
    #[doc = "Bit 1 - Syntax Error Detected on Channel A (vSS!SyntaxErrorA) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel A. 1 = Syntax error(s) observed on channel A 0 = No syntax error observed"]
    #[inline(always)]
    #[must_use]
    pub fn seda(&mut self) -> SEDA_W<1> {
        SEDA_W::new(self)
    }
    #[doc = "Bit 2 - Content Error Detected on Channel A (vSS!ContentErrorA) One or more frames with a content error were received on channel A in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel A 0 = No frame with content error received"]
    #[inline(always)]
    #[must_use]
    pub fn ceda(&mut self) -> CEDA_W<2> {
        CEDA_W::new(self)
    }
    #[doc = "Bit 3 - Communication Indicator Channel A One or more valid frames were received on channel A in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel A in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication"]
    #[inline(always)]
    #[must_use]
    pub fn cia(&mut self) -> CIA_W<3> {
        CIA_W::new(self)
    }
    #[doc = "Bit 4 - Slot Boundary Violation on Channel A (vSS!BViolationA) One or more slot boundary violations were observed on channel A at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel A 0 = No slot boundary violation observed"]
    #[inline(always)]
    #[must_use]
    pub fn sbva(&mut self) -> SBVA_W<4> {
        SBVA_W::new(self)
    }
    #[doc = "Bit 8 - Valid Frame Received on Channel B (vSS!ValidFrameB) One or more valid frames were received on channel B in any static or dynamic slot during the observation period. Reset under control of the Host. 1 = Valid frame(s) received on channel B 0 = No valid frame received"]
    #[inline(always)]
    #[must_use]
    pub fn vfrb_(&mut self) -> VFRB__W<8> {
        VFRB__W::new(self)
    }
    #[doc = "Bit 9 - Syntax Error Detected on Channel B (vSS!SyntaxErrorB) One or more syntax errors in static or dynamic slots, symbol window, and NIT were observed on channel B. 1 = Syntax error(s) observed on channel B 0 = No syntax error observed"]
    #[inline(always)]
    #[must_use]
    pub fn sedb(&mut self) -> SEDB_W<9> {
        SEDB_W::new(self)
    }
    #[doc = "Bit 10 - Content Error Detected on Channel B (vSS!ContentErrorB) One or more frames with a content error were received on channel B in any static or dynamic slot during the observation period. 1 = Frame(s) with content error received on channel B 0 = No frame with content error received"]
    #[inline(always)]
    #[must_use]
    pub fn cedb(&mut self) -> CEDB_W<10> {
        CEDB_W::new(self)
    }
    #[doc = "Bit 11 - Communication Indicator Channel B One or more valid frames were received on channel B in slots that also contained any additional communication during the observation period, i.e. one or more slots received a valid frame AND had any combination of either syntax error OR content error OR slot boundary violation. 1 = Valid frame(s) received on channel B in slots containing any additional communication 0 = No valid frame(s) received in slots containing any additional communication"]
    #[inline(always)]
    #[must_use]
    pub fn cib(&mut self) -> CIB_W<11> {
        CIB_W::new(self)
    }
    #[doc = "Bit 12 - Slot Boundary Violation on Channel B (vSS!BViolationB) One or more slot boundary violations were observed on channel B at any time during the observation period (static or dynamic slots, symbol window, and NIT). 1 = Slot boundary violation(s) observed on channel B 0 = No slot boundary violation observed"]
    #[inline(always)]
    #[must_use]
    pub fn sbvb(&mut self) -> SBVB_W<12> {
        SBVB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Aggregated Channel Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acs](index.html) module"]
pub struct ACS_SPEC;
impl crate::RegisterSpec for ACS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acs::R](R) reader structure"]
impl crate::Readable for ACS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acs::W](W) writer structure"]
impl crate::Writable for ACS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACS to value 0"]
impl crate::Resettable for ACS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
