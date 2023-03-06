#[doc = "Register `ULPS_MARK_STATUS` reader"]
pub struct R(crate::R<ULPS_MARK_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULPS_MARK_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULPS_MARK_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULPS_MARK_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ULPS_MARK_STATUS` reader - Mark status of RX D-PHY ULPS state. \\[0\\]
- Data Lane 0 in Mark state when 1 \\[1\\]
- Data Lane 1 in Mark state when 1 \\[2\\]
- Data Lane 2 in Mark state when 1 \\[3\\]
- Data Lane 3 in Mark state when 1"]
pub type ULPS_MARK_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Mark status of RX D-PHY ULPS state. \\[0\\]
- Data Lane 0 in Mark state when 1 \\[1\\]
- Data Lane 1 in Mark state when 1 \\[2\\]
- Data Lane 2 in Mark state when 1 \\[3\\]
- Data Lane 3 in Mark state when 1"]
    #[inline(always)]
    pub fn ulps_mark_status(&self) -> ULPS_MARK_STATUS_R {
        ULPS_MARK_STATUS_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "ULPS_MARK_STATUS is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulps_mark_status](index.html) module"]
pub struct ULPS_MARK_STATUS_SPEC;
impl crate::RegisterSpec for ULPS_MARK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulps_mark_status::R](R) reader structure"]
impl crate::Readable for ULPS_MARK_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ULPS_MARK_STATUS to value 0"]
impl crate::Resettable for ULPS_MARK_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
