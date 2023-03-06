#[doc = "Register `DST_FADE_STATUS` reader"]
pub struct R(crate::R<DST_FADE_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_FADE_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_FADE_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_FADE_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PACE` reader - Current fading pace counter value. DST_FADE_CTL.PACE value of 'n' reuses a specific fade value 'n+1' times. The reuses are numbered \\[0, n\\]."]
pub type PACE_R = crate::FieldReader<u16, PACE_A>;
#[doc = "Current fading pace counter value. DST_FADE_CTL.PACE value of 'n' reuses a specific fade value 'n+1' times. The reuses are numbered \\[0, n\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PACE_A {
    #[doc = "0: N/A"]
    PACE_MIN = 0,
    #[doc = "1023: N/A"]
    PACE_MAX = 1023,
}
impl From<PACE_A> for u16 {
    #[inline(always)]
    fn from(variant: PACE_A) -> Self {
        variant as _
    }
}
impl PACE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PACE_A> {
        match self.bits {
            0 => Some(PACE_A::PACE_MIN),
            1023 => Some(PACE_A::PACE_MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PACE_MIN`"]
    #[inline(always)]
    pub fn is_pace_min(&self) -> bool {
        *self == PACE_A::PACE_MIN
    }
    #[doc = "Checks if the value of the field is `PACE_MAX`"]
    #[inline(always)]
    pub fn is_pace_max(&self) -> bool {
        *self == PACE_A::PACE_MAX
    }
}
impl R {
    #[doc = "Bits 16:25 - Current fading pace counter value. DST_FADE_CTL.PACE value of 'n' reuses a specific fade value 'n+1' times. The reuses are numbered \\[0, n\\]."]
    #[inline(always)]
    pub fn pace(&self) -> PACE_R {
        PACE_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "Destination fade status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_fade_status](index.html) module"]
pub struct DST_FADE_STATUS_SPEC;
impl crate::RegisterSpec for DST_FADE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_fade_status::R](R) reader structure"]
impl crate::Readable for DST_FADE_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DST_FADE_STATUS to value 0"]
impl crate::Resettable for DST_FADE_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
