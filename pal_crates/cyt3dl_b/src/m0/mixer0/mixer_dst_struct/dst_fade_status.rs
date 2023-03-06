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
pub type PACE_R = crate::FieldReader<u16, u16>;
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
