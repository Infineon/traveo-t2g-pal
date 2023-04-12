#[doc = "Register `MHDS` reader"]
pub struct R(crate::R<MHDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MHDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MHDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MHDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MHDS` writer"]
pub struct W(crate::W<MHDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MHDS_SPEC>;
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
impl From<crate::W<MHDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MHDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIBF` reader - Parity Error Input Buffer RAM 1,2 1 = Parity error occurred when reading Input Buffer RAM 1,2 0 = No parity error"]
pub type PIBF_R = crate::BitReader<PIBF_A>;
#[doc = "Parity Error Input Buffer RAM 1,2 1 = Parity error occurred when reading Input Buffer RAM 1,2 0 = No parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIBF_A {
    #[doc = "0: N/A"]
    NO_PARITY_ERROR = 0,
    #[doc = "1: N/A"]
    PARITY_ERROR_IN_IBF_RAM_1_2 = 1,
}
impl From<PIBF_A> for bool {
    #[inline(always)]
    fn from(variant: PIBF_A) -> Self {
        variant as u8 != 0
    }
}
impl PIBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIBF_A {
        match self.bits {
            false => PIBF_A::NO_PARITY_ERROR,
            true => PIBF_A::PARITY_ERROR_IN_IBF_RAM_1_2,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY_ERROR`"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        *self == PIBF_A::NO_PARITY_ERROR
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR_IN_IBF_RAM_1_2`"]
    #[inline(always)]
    pub fn is_parity_error_in_ibf_ram_1_2(&self) -> bool {
        *self == PIBF_A::PARITY_ERROR_IN_IBF_RAM_1_2
    }
}
#[doc = "Field `PIBF` writer - Parity Error Input Buffer RAM 1,2 1 = Parity error occurred when reading Input Buffer RAM 1,2 0 = No parity error"]
pub type PIBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDS_SPEC, PIBF_A, O>;
impl<'a, const O: u8> PIBF_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_parity_error(self) -> &'a mut W {
        self.variant(PIBF_A::NO_PARITY_ERROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn parity_error_in_ibf_ram_1_2(self) -> &'a mut W {
        self.variant(PIBF_A::PARITY_ERROR_IN_IBF_RAM_1_2)
    }
}
#[doc = "Field `POBF` reader - Parity Error Output Buffer RAM 1,2 1 = Parity error occurred when reading Output Buffer RAM 1,2 0 = No parity error"]
pub type POBF_R = crate::BitReader<POBF_A>;
#[doc = "Parity Error Output Buffer RAM 1,2 1 = Parity error occurred when reading Output Buffer RAM 1,2 0 = No parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POBF_A {
    #[doc = "0: N/A"]
    NO_PARITY_ERROR = 0,
    #[doc = "1: N/A"]
    PARITY_ERROR_IN_OBF_RAM_1_2 = 1,
}
impl From<POBF_A> for bool {
    #[inline(always)]
    fn from(variant: POBF_A) -> Self {
        variant as u8 != 0
    }
}
impl POBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POBF_A {
        match self.bits {
            false => POBF_A::NO_PARITY_ERROR,
            true => POBF_A::PARITY_ERROR_IN_OBF_RAM_1_2,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY_ERROR`"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        *self == POBF_A::NO_PARITY_ERROR
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR_IN_OBF_RAM_1_2`"]
    #[inline(always)]
    pub fn is_parity_error_in_obf_ram_1_2(&self) -> bool {
        *self == POBF_A::PARITY_ERROR_IN_OBF_RAM_1_2
    }
}
#[doc = "Field `POBF` writer - Parity Error Output Buffer RAM 1,2 1 = Parity error occurred when reading Output Buffer RAM 1,2 0 = No parity error"]
pub type POBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDS_SPEC, POBF_A, O>;
impl<'a, const O: u8> POBF_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_parity_error(self) -> &'a mut W {
        self.variant(POBF_A::NO_PARITY_ERROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn parity_error_in_obf_ram_1_2(self) -> &'a mut W {
        self.variant(POBF_A::PARITY_ERROR_IN_OBF_RAM_1_2)
    }
}
#[doc = "Field `PMR` reader - Parity Error Message RAM 1 = Parity error occurred when reading the Message RAM 0 = No parity error"]
pub type PMR_R = crate::BitReader<PMR_A>;
#[doc = "Parity Error Message RAM 1 = Parity error occurred when reading the Message RAM 0 = No parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMR_A {
    #[doc = "0: N/A"]
    NO_PARITY_ERROR = 0,
    #[doc = "1: N/A"]
    PARITY_ERROR_IN_MSG_RAM = 1,
}
impl From<PMR_A> for bool {
    #[inline(always)]
    fn from(variant: PMR_A) -> Self {
        variant as u8 != 0
    }
}
impl PMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMR_A {
        match self.bits {
            false => PMR_A::NO_PARITY_ERROR,
            true => PMR_A::PARITY_ERROR_IN_MSG_RAM,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY_ERROR`"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        *self == PMR_A::NO_PARITY_ERROR
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR_IN_MSG_RAM`"]
    #[inline(always)]
    pub fn is_parity_error_in_msg_ram(&self) -> bool {
        *self == PMR_A::PARITY_ERROR_IN_MSG_RAM
    }
}
#[doc = "Field `PMR` writer - Parity Error Message RAM 1 = Parity error occurred when reading the Message RAM 0 = No parity error"]
pub type PMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDS_SPEC, PMR_A, O>;
impl<'a, const O: u8> PMR_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_parity_error(self) -> &'a mut W {
        self.variant(PMR_A::NO_PARITY_ERROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn parity_error_in_msg_ram(self) -> &'a mut W {
        self.variant(PMR_A::PARITY_ERROR_IN_MSG_RAM)
    }
}
#[doc = "Field `PTBF1` reader - Parity Error Transient Buffer RAM A 1 = Parity error occurred when reading Transient Buffer RAM A 0 = No parity error"]
pub type PTBF1_R = crate::BitReader<PTBF1_A>;
#[doc = "Parity Error Transient Buffer RAM A 1 = Parity error occurred when reading Transient Buffer RAM A 0 = No parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTBF1_A {
    #[doc = "0: N/A"]
    NO_PARITY_ERROR = 0,
    #[doc = "1: N/A"]
    PARITY_ERROR_IN_TRAN_BUF_RAM_A = 1,
}
impl From<PTBF1_A> for bool {
    #[inline(always)]
    fn from(variant: PTBF1_A) -> Self {
        variant as u8 != 0
    }
}
impl PTBF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBF1_A {
        match self.bits {
            false => PTBF1_A::NO_PARITY_ERROR,
            true => PTBF1_A::PARITY_ERROR_IN_TRAN_BUF_RAM_A,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY_ERROR`"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        *self == PTBF1_A::NO_PARITY_ERROR
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR_IN_TRAN_BUF_RAM_A`"]
    #[inline(always)]
    pub fn is_parity_error_in_tran_buf_ram_a(&self) -> bool {
        *self == PTBF1_A::PARITY_ERROR_IN_TRAN_BUF_RAM_A
    }
}
#[doc = "Field `PTBF1` writer - Parity Error Transient Buffer RAM A 1 = Parity error occurred when reading Transient Buffer RAM A 0 = No parity error"]
pub type PTBF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDS_SPEC, PTBF1_A, O>;
impl<'a, const O: u8> PTBF1_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_parity_error(self) -> &'a mut W {
        self.variant(PTBF1_A::NO_PARITY_ERROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn parity_error_in_tran_buf_ram_a(self) -> &'a mut W {
        self.variant(PTBF1_A::PARITY_ERROR_IN_TRAN_BUF_RAM_A)
    }
}
#[doc = "Field `PTBF2` reader - Parity Error Transient Buffer RAM B 1 = Parity error occurred when reading Transient Buffer RAM B 0 = No parity error"]
pub type PTBF2_R = crate::BitReader<PTBF2_A>;
#[doc = "Parity Error Transient Buffer RAM B 1 = Parity error occurred when reading Transient Buffer RAM B 0 = No parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTBF2_A {
    #[doc = "0: N/A"]
    NO_PARITY_ERROR = 0,
    #[doc = "1: N/A"]
    PARITY_ERROR_IN_TRAN_BUF_RAM_B = 1,
}
impl From<PTBF2_A> for bool {
    #[inline(always)]
    fn from(variant: PTBF2_A) -> Self {
        variant as u8 != 0
    }
}
impl PTBF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBF2_A {
        match self.bits {
            false => PTBF2_A::NO_PARITY_ERROR,
            true => PTBF2_A::PARITY_ERROR_IN_TRAN_BUF_RAM_B,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY_ERROR`"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        *self == PTBF2_A::NO_PARITY_ERROR
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR_IN_TRAN_BUF_RAM_B`"]
    #[inline(always)]
    pub fn is_parity_error_in_tran_buf_ram_b(&self) -> bool {
        *self == PTBF2_A::PARITY_ERROR_IN_TRAN_BUF_RAM_B
    }
}
#[doc = "Field `PTBF2` writer - Parity Error Transient Buffer RAM B 1 = Parity error occurred when reading Transient Buffer RAM B 0 = No parity error"]
pub type PTBF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDS_SPEC, PTBF2_A, O>;
impl<'a, const O: u8> PTBF2_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_parity_error(self) -> &'a mut W {
        self.variant(PTBF2_A::NO_PARITY_ERROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn parity_error_in_tran_buf_ram_b(self) -> &'a mut W {
        self.variant(PTBF2_A::PARITY_ERROR_IN_TRAN_BUF_RAM_B)
    }
}
#[doc = "Field `FMBD` reader - Faulty Message Buffer Detected 1 = Message buffer referenced by FMB\\[6:0\\]
holds faulty data due to a parity error 0 = No faulty message buffer"]
pub type FMBD_R = crate::BitReader<FMBD_A>;
#[doc = "Faulty Message Buffer Detected 1 = Message buffer referenced by FMB\\[6:0\\]
holds faulty data due to a parity error 0 = No faulty message buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMBD_A {
    #[doc = "0: N/A"]
    NO_FAULTY_MSG_BUFFER = 0,
    #[doc = "1: N/A"]
    FAULTY_MSG_BUFFER_ERROR = 1,
}
impl From<FMBD_A> for bool {
    #[inline(always)]
    fn from(variant: FMBD_A) -> Self {
        variant as u8 != 0
    }
}
impl FMBD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMBD_A {
        match self.bits {
            false => FMBD_A::NO_FAULTY_MSG_BUFFER,
            true => FMBD_A::FAULTY_MSG_BUFFER_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FAULTY_MSG_BUFFER`"]
    #[inline(always)]
    pub fn is_no_faulty_msg_buffer(&self) -> bool {
        *self == FMBD_A::NO_FAULTY_MSG_BUFFER
    }
    #[doc = "Checks if the value of the field is `FAULTY_MSG_BUFFER_ERROR`"]
    #[inline(always)]
    pub fn is_faulty_msg_buffer_error(&self) -> bool {
        *self == FMBD_A::FAULTY_MSG_BUFFER_ERROR
    }
}
#[doc = "Field `FMBD` writer - Faulty Message Buffer Detected 1 = Message buffer referenced by FMB\\[6:0\\]
holds faulty data due to a parity error 0 = No faulty message buffer"]
pub type FMBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDS_SPEC, FMBD_A, O>;
impl<'a, const O: u8> FMBD_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_faulty_msg_buffer(self) -> &'a mut W {
        self.variant(FMBD_A::NO_FAULTY_MSG_BUFFER)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn faulty_msg_buffer_error(self) -> &'a mut W {
        self.variant(FMBD_A::FAULTY_MSG_BUFFER_ERROR)
    }
}
#[doc = "Field `MFMB` reader - Multiple Faulty Message Buffers detected 1 = Another faulty message buffer was detected while flag FMBD is set 0 = No additional faulty message buffer"]
pub type MFMB_R = crate::BitReader<MFMB_A>;
#[doc = "Multiple Faulty Message Buffers detected 1 = Another faulty message buffer was detected while flag FMBD is set 0 = No additional faulty message buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MFMB_A {
    #[doc = "0: N/A"]
    NO_ADDITIONAL_FAULTY_MSG_BUFFER = 0,
    #[doc = "1: N/A"]
    ADDITIONAL_FAULTY_MSG_BUFFER_ERROR = 1,
}
impl From<MFMB_A> for bool {
    #[inline(always)]
    fn from(variant: MFMB_A) -> Self {
        variant as u8 != 0
    }
}
impl MFMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MFMB_A {
        match self.bits {
            false => MFMB_A::NO_ADDITIONAL_FAULTY_MSG_BUFFER,
            true => MFMB_A::ADDITIONAL_FAULTY_MSG_BUFFER_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ADDITIONAL_FAULTY_MSG_BUFFER`"]
    #[inline(always)]
    pub fn is_no_additional_faulty_msg_buffer(&self) -> bool {
        *self == MFMB_A::NO_ADDITIONAL_FAULTY_MSG_BUFFER
    }
    #[doc = "Checks if the value of the field is `ADDITIONAL_FAULTY_MSG_BUFFER_ERROR`"]
    #[inline(always)]
    pub fn is_additional_faulty_msg_buffer_error(&self) -> bool {
        *self == MFMB_A::ADDITIONAL_FAULTY_MSG_BUFFER_ERROR
    }
}
#[doc = "Field `MFMB` writer - Multiple Faulty Message Buffers detected 1 = Another faulty message buffer was detected while flag FMBD is set 0 = No additional faulty message buffer"]
pub type MFMB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MHDS_SPEC, MFMB_A, O>;
impl<'a, const O: u8> MFMB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn no_additional_faulty_msg_buffer(self) -> &'a mut W {
        self.variant(MFMB_A::NO_ADDITIONAL_FAULTY_MSG_BUFFER)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn additional_faulty_msg_buffer_error(self) -> &'a mut W {
        self.variant(MFMB_A::ADDITIONAL_FAULTY_MSG_BUFFER_ERROR)
    }
}
#[doc = "Field `CRAM` reader - Clear all internal RAM's Signals that execution of the CHI command CLEAR_RAMS is ongoing (all bits of all internal RAM blocks are written to '0'). The bit is set by hard reset or by CHI command CLEAR_RAMS. 1 = Execution of the CHI command CLEAR_RAMS ongoing 0 = No execution of the CHI command CLEAR_RAMS"]
pub type CRAM_R = crate::BitReader<CRAM_A>;
#[doc = "Clear all internal RAM's Signals that execution of the CHI command CLEAR_RAMS is ongoing (all bits of all internal RAM blocks are written to '0'). The bit is set by hard reset or by CHI command CLEAR_RAMS. 1 = Execution of the CHI command CLEAR_RAMS ongoing 0 = No execution of the CHI command CLEAR_RAMS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRAM_A {
    #[doc = "0: N/A"]
    CLEAR_RAMS_NOT_IN_PROGRESS = 0,
    #[doc = "1: N/A"]
    CLEAR_RAMS_IN_PROGRESS = 1,
}
impl From<CRAM_A> for bool {
    #[inline(always)]
    fn from(variant: CRAM_A) -> Self {
        variant as u8 != 0
    }
}
impl CRAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRAM_A {
        match self.bits {
            false => CRAM_A::CLEAR_RAMS_NOT_IN_PROGRESS,
            true => CRAM_A::CLEAR_RAMS_IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_RAMS_NOT_IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_clear_rams_not_in_progress(&self) -> bool {
        *self == CRAM_A::CLEAR_RAMS_NOT_IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `CLEAR_RAMS_IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_clear_rams_in_progress(&self) -> bool {
        *self == CRAM_A::CLEAR_RAMS_IN_PROGRESS
    }
}
#[doc = "Field `FMB` reader - Faulty Message Buffer Parity error occurred when reading from the message buffer or when transferring data from Input Buffer or Transient Buffer 1,2 to the message buffer referenced by FMB\\[6:0\\]. Value only valid when one of the flags PIBF, PMR, PTBF1, PTBF2, and flag FMBD is set. Is not updated while flag FMBD is set."]
pub type FMB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBT` reader - Message Buffer Transmitted Number of last successfully transmitted message buffer. If the message buffer is configured for single-shot mode, the respective TXR flag in the TXRQ1/2/3/4 registers was reset."]
pub type MBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBU` reader - Message Buffer Updated Number of message buffer that was updated last by the CC. For this message buffer the respective ND and / or MBC flag in the NDAT1/2/3/4 registers and the MBSC1/2/3/4 registers are also set."]
pub type MBU_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Parity Error Input Buffer RAM 1,2 1 = Parity error occurred when reading Input Buffer RAM 1,2 0 = No parity error"]
    #[inline(always)]
    pub fn pibf(&self) -> PIBF_R {
        PIBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Output Buffer RAM 1,2 1 = Parity error occurred when reading Output Buffer RAM 1,2 0 = No parity error"]
    #[inline(always)]
    pub fn pobf(&self) -> POBF_R {
        POBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Message RAM 1 = Parity error occurred when reading the Message RAM 0 = No parity error"]
    #[inline(always)]
    pub fn pmr(&self) -> PMR_R {
        PMR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Error Transient Buffer RAM A 1 = Parity error occurred when reading Transient Buffer RAM A 0 = No parity error"]
    #[inline(always)]
    pub fn ptbf1(&self) -> PTBF1_R {
        PTBF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Error Transient Buffer RAM B 1 = Parity error occurred when reading Transient Buffer RAM B 0 = No parity error"]
    #[inline(always)]
    pub fn ptbf2(&self) -> PTBF2_R {
        PTBF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Faulty Message Buffer Detected 1 = Message buffer referenced by FMB\\[6:0\\]
holds faulty data due to a parity error 0 = No faulty message buffer"]
    #[inline(always)]
    pub fn fmbd(&self) -> FMBD_R {
        FMBD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multiple Faulty Message Buffers detected 1 = Another faulty message buffer was detected while flag FMBD is set 0 = No additional faulty message buffer"]
    #[inline(always)]
    pub fn mfmb(&self) -> MFMB_R {
        MFMB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear all internal RAM's Signals that execution of the CHI command CLEAR_RAMS is ongoing (all bits of all internal RAM blocks are written to '0'). The bit is set by hard reset or by CHI command CLEAR_RAMS. 1 = Execution of the CHI command CLEAR_RAMS ongoing 0 = No execution of the CHI command CLEAR_RAMS"]
    #[inline(always)]
    pub fn cram(&self) -> CRAM_R {
        CRAM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Faulty Message Buffer Parity error occurred when reading from the message buffer or when transferring data from Input Buffer or Transient Buffer 1,2 to the message buffer referenced by FMB\\[6:0\\]. Value only valid when one of the flags PIBF, PMR, PTBF1, PTBF2, and flag FMBD is set. Is not updated while flag FMBD is set."]
    #[inline(always)]
    pub fn fmb(&self) -> FMB_R {
        FMB_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Message Buffer Transmitted Number of last successfully transmitted message buffer. If the message buffer is configured for single-shot mode, the respective TXR flag in the TXRQ1/2/3/4 registers was reset."]
    #[inline(always)]
    pub fn mbt(&self) -> MBT_R {
        MBT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Message Buffer Updated Number of message buffer that was updated last by the CC. For this message buffer the respective ND and / or MBC flag in the NDAT1/2/3/4 registers and the MBSC1/2/3/4 registers are also set."]
    #[inline(always)]
    pub fn mbu(&self) -> MBU_R {
        MBU_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Input Buffer RAM 1,2 1 = Parity error occurred when reading Input Buffer RAM 1,2 0 = No parity error"]
    #[inline(always)]
    #[must_use]
    pub fn pibf(&mut self) -> PIBF_W<0> {
        PIBF_W::new(self)
    }
    #[doc = "Bit 1 - Parity Error Output Buffer RAM 1,2 1 = Parity error occurred when reading Output Buffer RAM 1,2 0 = No parity error"]
    #[inline(always)]
    #[must_use]
    pub fn pobf(&mut self) -> POBF_W<1> {
        POBF_W::new(self)
    }
    #[doc = "Bit 2 - Parity Error Message RAM 1 = Parity error occurred when reading the Message RAM 0 = No parity error"]
    #[inline(always)]
    #[must_use]
    pub fn pmr(&mut self) -> PMR_W<2> {
        PMR_W::new(self)
    }
    #[doc = "Bit 3 - Parity Error Transient Buffer RAM A 1 = Parity error occurred when reading Transient Buffer RAM A 0 = No parity error"]
    #[inline(always)]
    #[must_use]
    pub fn ptbf1(&mut self) -> PTBF1_W<3> {
        PTBF1_W::new(self)
    }
    #[doc = "Bit 4 - Parity Error Transient Buffer RAM B 1 = Parity error occurred when reading Transient Buffer RAM B 0 = No parity error"]
    #[inline(always)]
    #[must_use]
    pub fn ptbf2(&mut self) -> PTBF2_W<4> {
        PTBF2_W::new(self)
    }
    #[doc = "Bit 5 - Faulty Message Buffer Detected 1 = Message buffer referenced by FMB\\[6:0\\]
holds faulty data due to a parity error 0 = No faulty message buffer"]
    #[inline(always)]
    #[must_use]
    pub fn fmbd(&mut self) -> FMBD_W<5> {
        FMBD_W::new(self)
    }
    #[doc = "Bit 6 - Multiple Faulty Message Buffers detected 1 = Another faulty message buffer was detected while flag FMBD is set 0 = No additional faulty message buffer"]
    #[inline(always)]
    #[must_use]
    pub fn mfmb(&mut self) -> MFMB_W<6> {
        MFMB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Handler Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mhds](index.html) module"]
pub struct MHDS_SPEC;
impl crate::RegisterSpec for MHDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mhds::R](R) reader structure"]
impl crate::Readable for MHDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mhds::W](W) writer structure"]
impl crate::Writable for MHDS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MHDS to value 0x80"]
impl crate::Resettable for MHDS_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
