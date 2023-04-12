#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PGM_CODE` reader - Indicates if active PGM operation to the Code flash is taking place 0: not running 1: running"]
pub type PGM_CODE_R = crate::BitReader<bool>;
#[doc = "Field `PGM_WORK` reader - Indicates if active PGM operation to the Work flash is taking place 0: not running 1: running"]
pub type PGM_WORK_R = crate::BitReader<bool>;
#[doc = "Field `ERASE_CODE` reader - Indicates if active Erase operation to the Code flash is taking place 0: not running 1: running"]
pub type ERASE_CODE_R = crate::BitReader<bool>;
#[doc = "Field `ERASE_WORK` reader - Indicates if active Erase operation to the Work flash is taking place 0: not running 1: running"]
pub type ERASE_WORK_R = crate::BitReader<bool>;
#[doc = "Field `ERS_SUSPEND` reader - Indicates if Erase operation (Code/Work) is currently being suspended 0: not suspended 1: suspended"]
pub type ERS_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `BLANK_CHECK_WORK` reader - Indicates if Blank Check mode is currently running on the work flash 0: not running 1: running"]
pub type BLANK_CHECK_WORK_R = crate::BitReader<bool>;
#[doc = "Field `BLANK_CHCEK_PASS` reader - Indicates the Blank check command result is PASS (Blank) 0: Not Blank 1: Blank (PASS)"]
pub type BLANK_CHCEK_PASS_R = crate::BitReader<bool>;
#[doc = "Field `POR_1B_ECC_CORRECTED` reader - Indicates internal ECC found 1b error while downloading info in POR from NVM to VM and fixed it. Valid after 2nd, 3rd and 4th POR phases (FUR, IREM &amp; MMR, SW DOWNLOAD). If Set it is not cleaned till additional POR (rst_hf_ac_t) 0: No error 1: 1b ECC Error corrected in POR"]
pub type POR_1B_ECC_CORRECTED_R = crate::BitReader<bool>;
#[doc = "Field `POR_2B_ECC_ERROR` reader - Indicates an internal ECC error of 2b while downloading info in POR from NVM to VM. Valid after 2nd, 3rd and 4th POR phases (FUR, IREM &amp; MMR, SW DOWNLOAD). If Set it is not cleaned till additional POR (rst_hf_ac_t) 0: No error 1: ECC 2b Error in POR"]
pub type POR_2B_ECC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `NATIVE_POR` reader - Indicates a Native Flash state (UV) or sorted one. Valid only after 2nd phase of POR (FUR DOWNLOAD). Comment: not a retained flop, therefore reset (rst_hf_act_n) puts it back to 0. If Set it is not cleaned till additional POR (rst_hf_ac_t) 0: SORTED DEVICE (Non - Native) 1: NATIVE"]
pub type NATIVE_POR_R = crate::BitReader<bool>;
#[doc = "Field `HANG` reader - After embedded operation (pgm/erase) this flag will tell if it was successful or failed 0: PASS 1: FAIL"]
pub type HANG_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - Whenever the device is in embedded mode the RDY goes low. Should be the same as c_interrupt pin of the IP (but inverted) 1: busy in embedded 0: rdy (high also in erase suspend)"]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates if active PGM operation to the Code flash is taking place 0: not running 1: running"]
    #[inline(always)]
    pub fn pgm_code(&self) -> PGM_CODE_R {
        PGM_CODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if active PGM operation to the Work flash is taking place 0: not running 1: running"]
    #[inline(always)]
    pub fn pgm_work(&self) -> PGM_WORK_R {
        PGM_WORK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if active Erase operation to the Code flash is taking place 0: not running 1: running"]
    #[inline(always)]
    pub fn erase_code(&self) -> ERASE_CODE_R {
        ERASE_CODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates if active Erase operation to the Work flash is taking place 0: not running 1: running"]
    #[inline(always)]
    pub fn erase_work(&self) -> ERASE_WORK_R {
        ERASE_WORK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates if Erase operation (Code/Work) is currently being suspended 0: not suspended 1: suspended"]
    #[inline(always)]
    pub fn ers_suspend(&self) -> ERS_SUSPEND_R {
        ERS_SUSPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if Blank Check mode is currently running on the work flash 0: not running 1: running"]
    #[inline(always)]
    pub fn blank_check_work(&self) -> BLANK_CHECK_WORK_R {
        BLANK_CHECK_WORK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the Blank check command result is PASS (Blank) 0: Not Blank 1: Blank (PASS)"]
    #[inline(always)]
    pub fn blank_chcek_pass(&self) -> BLANK_CHCEK_PASS_R {
        BLANK_CHCEK_PASS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicates internal ECC found 1b error while downloading info in POR from NVM to VM and fixed it. Valid after 2nd, 3rd and 4th POR phases (FUR, IREM &amp; MMR, SW DOWNLOAD). If Set it is not cleaned till additional POR (rst_hf_ac_t) 0: No error 1: 1b ECC Error corrected in POR"]
    #[inline(always)]
    pub fn por_1b_ecc_corrected(&self) -> POR_1B_ECC_CORRECTED_R {
        POR_1B_ECC_CORRECTED_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Indicates an internal ECC error of 2b while downloading info in POR from NVM to VM. Valid after 2nd, 3rd and 4th POR phases (FUR, IREM &amp; MMR, SW DOWNLOAD). If Set it is not cleaned till additional POR (rst_hf_ac_t) 0: No error 1: ECC 2b Error in POR"]
    #[inline(always)]
    pub fn por_2b_ecc_error(&self) -> POR_2B_ECC_ERROR_R {
        POR_2B_ECC_ERROR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Indicates a Native Flash state (UV) or sorted one. Valid only after 2nd phase of POR (FUR DOWNLOAD). Comment: not a retained flop, therefore reset (rst_hf_act_n) puts it back to 0. If Set it is not cleaned till additional POR (rst_hf_ac_t) 0: SORTED DEVICE (Non - Native) 1: NATIVE"]
    #[inline(always)]
    pub fn native_por(&self) -> NATIVE_POR_R {
        NATIVE_POR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - After embedded operation (pgm/erase) this flag will tell if it was successful or failed 0: PASS 1: FAIL"]
    #[inline(always)]
    pub fn hang(&self) -> HANG_R {
        HANG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Whenever the device is in embedded mode the RDY goes low. Should be the same as c_interrupt pin of the IP (but inverted) 1: busy in embedded 0: rdy (high also in erase suspend)"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status read from Flash Macro\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x8000_0000"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
