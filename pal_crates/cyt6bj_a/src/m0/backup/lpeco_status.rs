#[doc = "Register `LPECO_STATUS` reader"]
pub struct R(crate::R<LPECO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPECO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPECO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPECO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPECO_AMPDET_OK` reader - Indicates sufficient oscillation amplitude reported by LPECO amplitude detector. This field will read as zero when the amplitude detector is off (see LPECO_CTL.LPECO_AMPDET_EN)."]
pub type LPECO_AMPDET_OK_R = crate::BitReader<bool>;
#[doc = "Field `LPECO_READY` reader - Indicates the LPECO has had enough time to start. This field is driven by a stabilization counter clocked by IMO."]
pub type LPECO_READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates sufficient oscillation amplitude reported by LPECO amplitude detector. This field will read as zero when the amplitude detector is off (see LPECO_CTL.LPECO_AMPDET_EN)."]
    #[inline(always)]
    pub fn lpeco_ampdet_ok(&self) -> LPECO_AMPDET_OK_R {
        LPECO_AMPDET_OK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the LPECO has had enough time to start. This field is driven by a stabilization counter clocked by IMO."]
    #[inline(always)]
    pub fn lpeco_ready(&self) -> LPECO_READY_R {
        LPECO_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Low-power external crystal oscillator status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpeco_status](index.html) module"]
pub struct LPECO_STATUS_SPEC;
impl crate::RegisterSpec for LPECO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpeco_status::R](R) reader structure"]
impl crate::Readable for LPECO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPECO_STATUS to value 0"]
impl crate::Resettable for LPECO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
