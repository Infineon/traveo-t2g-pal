#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INSTR_FF_LEVEL` reader - Logical and of corresponding request and mask bits."]
pub type INSTR_FF_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_FF_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type INSTR_FF_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TR_INITIALIZED` reader - Logical and of corresponding request and mask bits."]
pub type TR_INITIALIZED_R = crate::BitReader<bool>;
#[doc = "Field `TR_DATA_AVAILABLE` reader - Logical and of corresponding request and mask bits."]
pub type TR_DATA_AVAILABLE_R = crate::BitReader<bool>;
#[doc = "Field `PR_DATA_AVAILABLE` reader - Logical and of corresponding request and mask bits."]
pub type PR_DATA_AVAILABLE_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_OPC_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type INSTR_OPC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_CC_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type INSTR_CC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TR_AP_DETECT_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type TR_AP_DETECT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TR_RC_DETECT_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type TR_RC_DETECT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `INSTR_DEV_KEY_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type INSTR_DEV_KEY_ERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_ff_level(&self) -> INSTR_FF_LEVEL_R {
        INSTR_FF_LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_ff_overflow(&self) -> INSTR_FF_OVERFLOW_R {
        INSTR_FF_OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_initialized(&self) -> TR_INITIALIZED_R {
        TR_INITIALIZED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_data_available(&self) -> TR_DATA_AVAILABLE_R {
        TR_DATA_AVAILABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn pr_data_available(&self) -> PR_DATA_AVAILABLE_R {
        PR_DATA_AVAILABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_opc_error(&self) -> INSTR_OPC_ERROR_R {
        INSTR_OPC_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_cc_error(&self) -> INSTR_CC_ERROR_R {
        INSTR_CC_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn bus_error(&self) -> BUS_ERROR_R {
        BUS_ERROR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_ap_detect_error(&self) -> TR_AP_DETECT_ERROR_R {
        TR_AP_DETECT_ERROR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_rc_detect_error(&self) -> TR_RC_DETECT_ERROR_R {
        TR_RC_DETECT_ERROR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn instr_dev_key_error(&self) -> INSTR_DEV_KEY_ERROR_R {
        INSTR_DEV_KEY_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
