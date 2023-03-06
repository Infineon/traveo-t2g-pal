#[doc = "Register `SWNIT` reader"]
pub struct R(crate::R<SWNIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWNIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWNIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWNIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SESA` reader - Syntax Error in Symbol Window Channel A (vSS!SyntaxErrorA) 1 = Syntax error during symbol window detected on channel A 0 = No syntax error detected"]
pub type SESA_R = crate::BitReader<SESA_A>;
#[doc = "Syntax Error in Symbol Window Channel A (vSS!SyntaxErrorA) 1 = Syntax error during symbol window detected on channel A 0 = No syntax error detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SESA_A {
    #[doc = "0: N/A"]
    NO_SYNTAX_ERROR_IN_CH_A = 0,
    #[doc = "1: N/A"]
    SYNTAX_ERR_IN_CH_A = 1,
}
impl From<SESA_A> for bool {
    #[inline(always)]
    fn from(variant: SESA_A) -> Self {
        variant as u8 != 0
    }
}
impl SESA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESA_A {
        match self.bits {
            false => SESA_A::NO_SYNTAX_ERROR_IN_CH_A,
            true => SESA_A::SYNTAX_ERR_IN_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNTAX_ERROR_IN_CH_A`"]
    #[inline(always)]
    pub fn is_no_syntax_error_in_ch_a(&self) -> bool {
        *self == SESA_A::NO_SYNTAX_ERROR_IN_CH_A
    }
    #[doc = "Checks if the value of the field is `SYNTAX_ERR_IN_CH_A`"]
    #[inline(always)]
    pub fn is_syntax_err_in_ch_a(&self) -> bool {
        *self == SESA_A::SYNTAX_ERR_IN_CH_A
    }
}
#[doc = "Field `SBSA` reader - Slot Boundary Violation in Symbol Window Channel A (vSS!BViolationA) 1 = Slot boundary violation during symbol window detected on channel A 0 = No slot boundary violation detected"]
pub type SBSA_R = crate::BitReader<SBSA_A>;
#[doc = "Slot Boundary Violation in Symbol Window Channel A (vSS!BViolationA) 1 = Slot boundary violation during symbol window detected on channel A 0 = No slot boundary violation detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSA_A {
    #[doc = "0: N/A"]
    NO_BNDRY_VIOLATION_ERROR_IN_CH_A = 0,
    #[doc = "1: N/A"]
    BNDRY_VIOLATION_ERROR_IN_CH_A = 1,
}
impl From<SBSA_A> for bool {
    #[inline(always)]
    fn from(variant: SBSA_A) -> Self {
        variant as u8 != 0
    }
}
impl SBSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBSA_A {
        match self.bits {
            false => SBSA_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_A,
            true => SBSA_A::BNDRY_VIOLATION_ERROR_IN_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BNDRY_VIOLATION_ERROR_IN_CH_A`"]
    #[inline(always)]
    pub fn is_no_bndry_violation_error_in_ch_a(&self) -> bool {
        *self == SBSA_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_A
    }
    #[doc = "Checks if the value of the field is `BNDRY_VIOLATION_ERROR_IN_CH_A`"]
    #[inline(always)]
    pub fn is_bndry_violation_error_in_ch_a(&self) -> bool {
        *self == SBSA_A::BNDRY_VIOLATION_ERROR_IN_CH_A
    }
}
#[doc = "Field `TCSA` reader - Transmission Conflict in Symbol Window Channel A (vSS!TxConflictA) 1 = Transmission conflict in symbol window detected on channel A 0 = No transmission conflict detected"]
pub type TCSA_R = crate::BitReader<TCSA_A>;
#[doc = "Transmission Conflict in Symbol Window Channel A (vSS!TxConflictA) 1 = Transmission conflict in symbol window detected on channel A 0 = No transmission conflict detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCSA_A {
    #[doc = "0: N/A"]
    NO_TXMN_CONFLICT_IN_CH_A = 0,
    #[doc = "1: N/A"]
    TXMN_CONFLICT_IN_CH_A = 1,
}
impl From<TCSA_A> for bool {
    #[inline(always)]
    fn from(variant: TCSA_A) -> Self {
        variant as u8 != 0
    }
}
impl TCSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCSA_A {
        match self.bits {
            false => TCSA_A::NO_TXMN_CONFLICT_IN_CH_A,
            true => TCSA_A::TXMN_CONFLICT_IN_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TXMN_CONFLICT_IN_CH_A`"]
    #[inline(always)]
    pub fn is_no_txmn_conflict_in_ch_a(&self) -> bool {
        *self == TCSA_A::NO_TXMN_CONFLICT_IN_CH_A
    }
    #[doc = "Checks if the value of the field is `TXMN_CONFLICT_IN_CH_A`"]
    #[inline(always)]
    pub fn is_txmn_conflict_in_ch_a(&self) -> bool {
        *self == TCSA_A::TXMN_CONFLICT_IN_CH_A
    }
}
#[doc = "Field `SESB` reader - Syntax Error in Symbol Window Channel B (vSS!SyntaxErrorB) 1 = Syntax error during symbol window detected on channel B 0 = No syntax error detected"]
pub type SESB_R = crate::BitReader<SESB_A>;
#[doc = "Syntax Error in Symbol Window Channel B (vSS!SyntaxErrorB) 1 = Syntax error during symbol window detected on channel B 0 = No syntax error detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SESB_A {
    #[doc = "0: N/A"]
    NO_SYNTAX_ERROR_IN_CH_B = 0,
    #[doc = "1: N/A"]
    SYNTAX_ERR_IN_CH_B = 1,
}
impl From<SESB_A> for bool {
    #[inline(always)]
    fn from(variant: SESB_A) -> Self {
        variant as u8 != 0
    }
}
impl SESB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESB_A {
        match self.bits {
            false => SESB_A::NO_SYNTAX_ERROR_IN_CH_B,
            true => SESB_A::SYNTAX_ERR_IN_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNTAX_ERROR_IN_CH_B`"]
    #[inline(always)]
    pub fn is_no_syntax_error_in_ch_b(&self) -> bool {
        *self == SESB_A::NO_SYNTAX_ERROR_IN_CH_B
    }
    #[doc = "Checks if the value of the field is `SYNTAX_ERR_IN_CH_B`"]
    #[inline(always)]
    pub fn is_syntax_err_in_ch_b(&self) -> bool {
        *self == SESB_A::SYNTAX_ERR_IN_CH_B
    }
}
#[doc = "Field `SBSB` reader - Slot Boundary Violation in Symbol Window Channel B (vSS!BViolationB) 1 = Slot boundary violation during symbol window detected on channel B 0 = No slot boundary violation detected"]
pub type SBSB_R = crate::BitReader<SBSB_A>;
#[doc = "Slot Boundary Violation in Symbol Window Channel B (vSS!BViolationB) 1 = Slot boundary violation during symbol window detected on channel B 0 = No slot boundary violation detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSB_A {
    #[doc = "0: N/A"]
    NO_BNDRY_VIOLATION_ERROR_IN_CH_B = 0,
    #[doc = "1: N/A"]
    BNDRY_VIOLATION_ERROR_IN_CH_B = 1,
}
impl From<SBSB_A> for bool {
    #[inline(always)]
    fn from(variant: SBSB_A) -> Self {
        variant as u8 != 0
    }
}
impl SBSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBSB_A {
        match self.bits {
            false => SBSB_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_B,
            true => SBSB_A::BNDRY_VIOLATION_ERROR_IN_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BNDRY_VIOLATION_ERROR_IN_CH_B`"]
    #[inline(always)]
    pub fn is_no_bndry_violation_error_in_ch_b(&self) -> bool {
        *self == SBSB_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_B
    }
    #[doc = "Checks if the value of the field is `BNDRY_VIOLATION_ERROR_IN_CH_B`"]
    #[inline(always)]
    pub fn is_bndry_violation_error_in_ch_b(&self) -> bool {
        *self == SBSB_A::BNDRY_VIOLATION_ERROR_IN_CH_B
    }
}
#[doc = "Field `TCSB` reader - Transmission Conflict in Symbol Window Channel B (vSS!TxConflictB) 1 = Transmission conflict in symbol window detected on channel B 0 = No transmission conflict detected"]
pub type TCSB_R = crate::BitReader<TCSB_A>;
#[doc = "Transmission Conflict in Symbol Window Channel B (vSS!TxConflictB) 1 = Transmission conflict in symbol window detected on channel B 0 = No transmission conflict detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCSB_A {
    #[doc = "0: N/A"]
    NO_TXMN_CONFLICT_IN_CH_B = 0,
    #[doc = "1: N/A"]
    TXMN_CONFLICT_IN_CH_B = 1,
}
impl From<TCSB_A> for bool {
    #[inline(always)]
    fn from(variant: TCSB_A) -> Self {
        variant as u8 != 0
    }
}
impl TCSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCSB_A {
        match self.bits {
            false => TCSB_A::NO_TXMN_CONFLICT_IN_CH_B,
            true => TCSB_A::TXMN_CONFLICT_IN_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TXMN_CONFLICT_IN_CH_B`"]
    #[inline(always)]
    pub fn is_no_txmn_conflict_in_ch_b(&self) -> bool {
        *self == TCSB_A::NO_TXMN_CONFLICT_IN_CH_B
    }
    #[doc = "Checks if the value of the field is `TXMN_CONFLICT_IN_CH_B`"]
    #[inline(always)]
    pub fn is_txmn_conflict_in_ch_b(&self) -> bool {
        *self == TCSB_A::TXMN_CONFLICT_IN_CH_B
    }
}
#[doc = "Field `MTSA__` reader - MTS Received on Channel A (vSS!ValidMTSA) Media Access Test symbol received on channel A during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. When this bit is set to '1', also interrupt flag SIR.MTSA is set to '1'. 1 = MTS symbol received on channel A 0 = No MTS symbol received on channel A"]
pub type MTSA___R = crate::BitReader<MTSA___A>;
#[doc = "MTS Received on Channel A (vSS!ValidMTSA) Media Access Test symbol received on channel A during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. When this bit is set to '1', also interrupt flag SIR.MTSA is set to '1'. 1 = MTS symbol received on channel A 0 = No MTS symbol received on channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTSA___A {
    #[doc = "0: N/A"]
    NO_MTS_SYMBOL_RXD_IN_CH_A = 0,
    #[doc = "1: N/A"]
    MTS_SYMBOL_RXD_IN_CH_A = 1,
}
impl From<MTSA___A> for bool {
    #[inline(always)]
    fn from(variant: MTSA___A) -> Self {
        variant as u8 != 0
    }
}
impl MTSA___R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSA___A {
        match self.bits {
            false => MTSA___A::NO_MTS_SYMBOL_RXD_IN_CH_A,
            true => MTSA___A::MTS_SYMBOL_RXD_IN_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MTS_SYMBOL_RXD_IN_CH_A`"]
    #[inline(always)]
    pub fn is_no_mts_symbol_rxd_in_ch_a(&self) -> bool {
        *self == MTSA___A::NO_MTS_SYMBOL_RXD_IN_CH_A
    }
    #[doc = "Checks if the value of the field is `MTS_SYMBOL_RXD_IN_CH_A`"]
    #[inline(always)]
    pub fn is_mts_symbol_rxd_in_ch_a(&self) -> bool {
        *self == MTSA___A::MTS_SYMBOL_RXD_IN_CH_A
    }
}
#[doc = "Field `MTSB__` reader - MTS Received on Channel B (vSS!ValidMTSB) Media Access Test symbol received on channel B during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. When this bit is set to '1', also interrupt flag SIR.MTSB is set to '1'. 1 = MTS symbol received on channel B 0 = No MTS symbol received on channel B NIT related status information. Updated by the CC at the end of the NIT for each channel:"]
pub type MTSB___R = crate::BitReader<MTSB___A>;
#[doc = "MTS Received on Channel B (vSS!ValidMTSB) Media Access Test symbol received on channel B during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. When this bit is set to '1', also interrupt flag SIR.MTSB is set to '1'. 1 = MTS symbol received on channel B 0 = No MTS symbol received on channel B NIT related status information. Updated by the CC at the end of the NIT for each channel:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTSB___A {
    #[doc = "0: N/A"]
    NO_MTS_SYMBOL_RXD_IN_CH_B = 0,
    #[doc = "1: N/A"]
    MTS_SYMBOL_RXD_IN_CH_B = 1,
}
impl From<MTSB___A> for bool {
    #[inline(always)]
    fn from(variant: MTSB___A) -> Self {
        variant as u8 != 0
    }
}
impl MTSB___R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTSB___A {
        match self.bits {
            false => MTSB___A::NO_MTS_SYMBOL_RXD_IN_CH_B,
            true => MTSB___A::MTS_SYMBOL_RXD_IN_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MTS_SYMBOL_RXD_IN_CH_B`"]
    #[inline(always)]
    pub fn is_no_mts_symbol_rxd_in_ch_b(&self) -> bool {
        *self == MTSB___A::NO_MTS_SYMBOL_RXD_IN_CH_B
    }
    #[doc = "Checks if the value of the field is `MTS_SYMBOL_RXD_IN_CH_B`"]
    #[inline(always)]
    pub fn is_mts_symbol_rxd_in_ch_b(&self) -> bool {
        *self == MTSB___A::MTS_SYMBOL_RXD_IN_CH_B
    }
}
#[doc = "Field `SENA` reader - Syntax Error during NIT Channel A (vSS!SyntaxErrorA) 1 = Syntax error during NIT detected on channel A 0 = No syntax error detected"]
pub type SENA_R = crate::BitReader<SENA_A>;
#[doc = "Syntax Error during NIT Channel A (vSS!SyntaxErrorA) 1 = Syntax error during NIT detected on channel A 0 = No syntax error detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENA_A {
    #[doc = "0: N/A"]
    NO_SYNTAX_ERROR_DETECTED_ON_CH_A = 0,
    #[doc = "1: N/A"]
    SYNTAX_ERROR_DETECTED_ON_CH_A = 1,
}
impl From<SENA_A> for bool {
    #[inline(always)]
    fn from(variant: SENA_A) -> Self {
        variant as u8 != 0
    }
}
impl SENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SENA_A {
        match self.bits {
            false => SENA_A::NO_SYNTAX_ERROR_DETECTED_ON_CH_A,
            true => SENA_A::SYNTAX_ERROR_DETECTED_ON_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNTAX_ERROR_DETECTED_ON_CH_A`"]
    #[inline(always)]
    pub fn is_no_syntax_error_detected_on_ch_a(&self) -> bool {
        *self == SENA_A::NO_SYNTAX_ERROR_DETECTED_ON_CH_A
    }
    #[doc = "Checks if the value of the field is `SYNTAX_ERROR_DETECTED_ON_CH_A`"]
    #[inline(always)]
    pub fn is_syntax_error_detected_on_ch_a(&self) -> bool {
        *self == SENA_A::SYNTAX_ERROR_DETECTED_ON_CH_A
    }
}
#[doc = "Field `SBNA` reader - Slot Boundary Violation during NIT Channel A (vSS!BViolationA) 1 = Slot boundary violation during NIT detected on channel A 0 = No slot boundary violation detected"]
pub type SBNA_R = crate::BitReader<SBNA_A>;
#[doc = "Slot Boundary Violation during NIT Channel A (vSS!BViolationA) 1 = Slot boundary violation during NIT detected on channel A 0 = No slot boundary violation detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBNA_A {
    #[doc = "0: N/A"]
    NO_BNDRY_VIOLATION_ERROR_IN_CH_A = 0,
    #[doc = "1: N/A"]
    BNDRY_VIOLATION_ERROR_IN_CH_A = 1,
}
impl From<SBNA_A> for bool {
    #[inline(always)]
    fn from(variant: SBNA_A) -> Self {
        variant as u8 != 0
    }
}
impl SBNA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBNA_A {
        match self.bits {
            false => SBNA_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_A,
            true => SBNA_A::BNDRY_VIOLATION_ERROR_IN_CH_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BNDRY_VIOLATION_ERROR_IN_CH_A`"]
    #[inline(always)]
    pub fn is_no_bndry_violation_error_in_ch_a(&self) -> bool {
        *self == SBNA_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_A
    }
    #[doc = "Checks if the value of the field is `BNDRY_VIOLATION_ERROR_IN_CH_A`"]
    #[inline(always)]
    pub fn is_bndry_violation_error_in_ch_a(&self) -> bool {
        *self == SBNA_A::BNDRY_VIOLATION_ERROR_IN_CH_A
    }
}
#[doc = "Field `SENB` reader - Syntax Error during NIT Channel B (vSS!SyntaxErrorB) 1 = Syntax error during NIT detected on channel B 0 = No syntax error detected"]
pub type SENB_R = crate::BitReader<SENB_A>;
#[doc = "Syntax Error during NIT Channel B (vSS!SyntaxErrorB) 1 = Syntax error during NIT detected on channel B 0 = No syntax error detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SENB_A {
    #[doc = "0: N/A"]
    NO_SYNTAX_ERROR_DETECTED_ON_CH_B = 0,
    #[doc = "1: N/A"]
    SYNTAX_ERROR_DETECTED_ON_CH_B = 1,
}
impl From<SENB_A> for bool {
    #[inline(always)]
    fn from(variant: SENB_A) -> Self {
        variant as u8 != 0
    }
}
impl SENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SENB_A {
        match self.bits {
            false => SENB_A::NO_SYNTAX_ERROR_DETECTED_ON_CH_B,
            true => SENB_A::SYNTAX_ERROR_DETECTED_ON_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNTAX_ERROR_DETECTED_ON_CH_B`"]
    #[inline(always)]
    pub fn is_no_syntax_error_detected_on_ch_b(&self) -> bool {
        *self == SENB_A::NO_SYNTAX_ERROR_DETECTED_ON_CH_B
    }
    #[doc = "Checks if the value of the field is `SYNTAX_ERROR_DETECTED_ON_CH_B`"]
    #[inline(always)]
    pub fn is_syntax_error_detected_on_ch_b(&self) -> bool {
        *self == SENB_A::SYNTAX_ERROR_DETECTED_ON_CH_B
    }
}
#[doc = "Field `SBNB` reader - Slot Boundary Violation during NIT Channel B (vSS!BViolationB) 1 = Slot boundary violation during NIT detected on channel B 0 = No slot boundary violation detected"]
pub type SBNB_R = crate::BitReader<SBNB_A>;
#[doc = "Slot Boundary Violation during NIT Channel B (vSS!BViolationB) 1 = Slot boundary violation during NIT detected on channel B 0 = No slot boundary violation detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBNB_A {
    #[doc = "0: N/A"]
    NO_BNDRY_VIOLATION_ERROR_IN_CH_B = 0,
    #[doc = "1: N/A"]
    BNDRY_VIOLATION_ERROR_IN_CH_B = 1,
}
impl From<SBNB_A> for bool {
    #[inline(always)]
    fn from(variant: SBNB_A) -> Self {
        variant as u8 != 0
    }
}
impl SBNB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBNB_A {
        match self.bits {
            false => SBNB_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_B,
            true => SBNB_A::BNDRY_VIOLATION_ERROR_IN_CH_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BNDRY_VIOLATION_ERROR_IN_CH_B`"]
    #[inline(always)]
    pub fn is_no_bndry_violation_error_in_ch_b(&self) -> bool {
        *self == SBNB_A::NO_BNDRY_VIOLATION_ERROR_IN_CH_B
    }
    #[doc = "Checks if the value of the field is `BNDRY_VIOLATION_ERROR_IN_CH_B`"]
    #[inline(always)]
    pub fn is_bndry_violation_error_in_ch_b(&self) -> bool {
        *self == SBNB_A::BNDRY_VIOLATION_ERROR_IN_CH_B
    }
}
impl R {
    #[doc = "Bit 0 - Syntax Error in Symbol Window Channel A (vSS!SyntaxErrorA) 1 = Syntax error during symbol window detected on channel A 0 = No syntax error detected"]
    #[inline(always)]
    pub fn sesa(&self) -> SESA_R {
        SESA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slot Boundary Violation in Symbol Window Channel A (vSS!BViolationA) 1 = Slot boundary violation during symbol window detected on channel A 0 = No slot boundary violation detected"]
    #[inline(always)]
    pub fn sbsa(&self) -> SBSA_R {
        SBSA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Conflict in Symbol Window Channel A (vSS!TxConflictA) 1 = Transmission conflict in symbol window detected on channel A 0 = No transmission conflict detected"]
    #[inline(always)]
    pub fn tcsa(&self) -> TCSA_R {
        TCSA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Syntax Error in Symbol Window Channel B (vSS!SyntaxErrorB) 1 = Syntax error during symbol window detected on channel B 0 = No syntax error detected"]
    #[inline(always)]
    pub fn sesb(&self) -> SESB_R {
        SESB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slot Boundary Violation in Symbol Window Channel B (vSS!BViolationB) 1 = Slot boundary violation during symbol window detected on channel B 0 = No slot boundary violation detected"]
    #[inline(always)]
    pub fn sbsb(&self) -> SBSB_R {
        SBSB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Conflict in Symbol Window Channel B (vSS!TxConflictB) 1 = Transmission conflict in symbol window detected on channel B 0 = No transmission conflict detected"]
    #[inline(always)]
    pub fn tcsb(&self) -> TCSB_R {
        TCSB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MTS Received on Channel A (vSS!ValidMTSA) Media Access Test symbol received on channel A during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. When this bit is set to '1', also interrupt flag SIR.MTSA is set to '1'. 1 = MTS symbol received on channel A 0 = No MTS symbol received on channel A"]
    #[inline(always)]
    pub fn mtsa__(&self) -> MTSA___R {
        MTSA___R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MTS Received on Channel B (vSS!ValidMTSB) Media Access Test symbol received on channel B during the preceding symbol window. Updated by the CC for each channel at the end of the symbol window. When this bit is set to '1', also interrupt flag SIR.MTSB is set to '1'. 1 = MTS symbol received on channel B 0 = No MTS symbol received on channel B NIT related status information. Updated by the CC at the end of the NIT for each channel:"]
    #[inline(always)]
    pub fn mtsb__(&self) -> MTSB___R {
        MTSB___R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Syntax Error during NIT Channel A (vSS!SyntaxErrorA) 1 = Syntax error during NIT detected on channel A 0 = No syntax error detected"]
    #[inline(always)]
    pub fn sena(&self) -> SENA_R {
        SENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slot Boundary Violation during NIT Channel A (vSS!BViolationA) 1 = Slot boundary violation during NIT detected on channel A 0 = No slot boundary violation detected"]
    #[inline(always)]
    pub fn sbna(&self) -> SBNA_R {
        SBNA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Syntax Error during NIT Channel B (vSS!SyntaxErrorB) 1 = Syntax error during NIT detected on channel B 0 = No syntax error detected"]
    #[inline(always)]
    pub fn senb(&self) -> SENB_R {
        SENB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slot Boundary Violation during NIT Channel B (vSS!BViolationB) 1 = Slot boundary violation during NIT detected on channel B 0 = No slot boundary violation detected"]
    #[inline(always)]
    pub fn sbnb(&self) -> SBNB_R {
        SBNB_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Symbol Window and NIT Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swnit](index.html) module"]
pub struct SWNIT_SPEC;
impl crate::RegisterSpec for SWNIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swnit::R](R) reader structure"]
impl crate::Readable for SWNIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SWNIT to value 0"]
impl crate::Resettable for SWNIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
