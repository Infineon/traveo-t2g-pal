#[doc = "Register `CCEV` reader"]
pub struct R(crate::R<CCEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCFC` reader - Clock Correction Failed Counter (vClockCorrectionFailed) The Clock Correction Failed Counter is incremented by one at the end of any odd communication cycle where either the missing offset correction error or missing rate correction error are active. The Clock Correction Failed Counter is reset to '0' at the end of an odd communication cycle if neither the offset correction failed nor the rate correction failed errors are active. The Clock Correction Failed Counter stops at 15."]
pub type CCFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERRM` reader - N/A"]
pub type ERRM_R = crate::FieldReader<u8, ERRM_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERRM_A {
    #[doc = "0: N/A"]
    ACTIVE = 0,
    #[doc = "1: N/A"]
    PASSIVE = 1,
    #[doc = "2: N/A"]
    COMM_HALT = 2,
}
impl From<ERRM_A> for u8 {
    #[inline(always)]
    fn from(variant: ERRM_A) -> Self {
        variant as _
    }
}
impl ERRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERRM_A> {
        match self.bits {
            0 => Some(ERRM_A::ACTIVE),
            1 => Some(ERRM_A::PASSIVE),
            2 => Some(ERRM_A::COMM_HALT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ERRM_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `PASSIVE`"]
    #[inline(always)]
    pub fn is_passive(&self) -> bool {
        *self == ERRM_A::PASSIVE
    }
    #[doc = "Checks if the value of the field is `COMM_HALT`"]
    #[inline(always)]
    pub fn is_comm_halt(&self) -> bool {
        *self == ERRM_A::COMM_HALT
    }
}
#[doc = "Field `PTAC` reader - Passive to Active Count (vAllowPassiveToActive) Indicates the number of consecutive even / odd cycle pairs that have passed with valid rate and offset correction terms, while the node is waiting to transit from NORMAL_PASSIVE state to NORMAL_ACTIVE state. The transition takes place when PTAC\\[4:0\\]
equals SUCC1.PTA\\[4:0\\]
-1."]
pub type PTAC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Clock Correction Failed Counter (vClockCorrectionFailed) The Clock Correction Failed Counter is incremented by one at the end of any odd communication cycle where either the missing offset correction error or missing rate correction error are active. The Clock Correction Failed Counter is reset to '0' at the end of an odd communication cycle if neither the offset correction failed nor the rate correction failed errors are active. The Clock Correction Failed Counter stops at 15."]
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Passive to Active Count (vAllowPassiveToActive) Indicates the number of consecutive even / odd cycle pairs that have passed with valid rate and offset correction terms, while the node is waiting to transit from NORMAL_PASSIVE state to NORMAL_ACTIVE state. The transition takes place when PTAC\\[4:0\\]
equals SUCC1.PTA\\[4:0\\]
-1."]
    #[inline(always)]
    pub fn ptac(&self) -> PTAC_R {
        PTAC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "CC Error Vector\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccev](index.html) module"]
pub struct CCEV_SPEC;
impl crate::RegisterSpec for CCEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccev::R](R) reader structure"]
impl crate::Readable for CCEV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCEV to value 0"]
impl crate::Resettable for CCEV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
