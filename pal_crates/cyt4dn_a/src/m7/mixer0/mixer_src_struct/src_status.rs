#[doc = "Register `SRC_STATUS` reader"]
pub struct R(crate::R<SRC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PHASE` reader - Current FIR phase value. A upscale ratio of 'n' uses a n polyphase FIR filter. The phases are numbered \\[0, n-1\\]."]
pub type PHASE_R = crate::FieldReader<u8, PHASE_A>;
#[doc = "Current FIR phase value. A upscale ratio of 'n' uses a n polyphase FIR filter. The phases are numbered \\[0, n-1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PHASE_A {
    #[doc = "0: N/A"]
    PHASE_0 = 0,
    #[doc = "1: N/A"]
    PHASE_1 = 1,
    #[doc = "11: N/A"]
    PHASE_11 = 11,
}
impl From<PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: PHASE_A) -> Self {
        variant as _
    }
}
impl PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PHASE_A> {
        match self.bits {
            0 => Some(PHASE_A::PHASE_0),
            1 => Some(PHASE_A::PHASE_1),
            11 => Some(PHASE_A::PHASE_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PHASE_0`"]
    #[inline(always)]
    pub fn is_phase_0(&self) -> bool {
        *self == PHASE_A::PHASE_0
    }
    #[doc = "Checks if the value of the field is `PHASE_1`"]
    #[inline(always)]
    pub fn is_phase_1(&self) -> bool {
        *self == PHASE_A::PHASE_1
    }
    #[doc = "Checks if the value of the field is `PHASE_11`"]
    #[inline(always)]
    pub fn is_phase_11(&self) -> bool {
        *self == PHASE_A::PHASE_11
    }
}
impl R {
    #[doc = "Bits 0:3 - Current FIR phase value. A upscale ratio of 'n' uses a n polyphase FIR filter. The phases are numbered \\[0, n-1\\]."]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Source status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_status](index.html) module"]
pub struct SRC_STATUS_SPEC;
impl crate::RegisterSpec for SRC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src_status::R](R) reader structure"]
impl crate::Readable for SRC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRC_STATUS to value 0"]
impl crate::Resettable for SRC_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
