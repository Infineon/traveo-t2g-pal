#[doc = "Register `TR_MON_AP_STATUS1` reader"]
pub struct R(crate::R<TR_MON_AP_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_MON_AP_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_MON_AP_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_MON_AP_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OCC_COUNT` reader - Number of occurrences of the current active bit counter: '0': 0 occurrences ... '65535': 65535 occurrences"]
pub type OCC_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINDOW_INDEX` reader - Counter to keep track of the current index in the window (counts from '0' to TR_MON_AP_CTL.WINDOW_SIZE to '0')."]
pub type WINDOW_INDEX_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of occurrences of the current active bit counter: '0': 0 occurrences ... '65535': 65535 occurrences"]
    #[inline(always)]
    pub fn occ_count(&self) -> OCC_COUNT_R {
        OCC_COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Counter to keep track of the current index in the window (counts from '0' to TR_MON_AP_CTL.WINDOW_SIZE to '0')."]
    #[inline(always)]
    pub fn window_index(&self) -> WINDOW_INDEX_R {
        WINDOW_INDEX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "True random monitor AP status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_mon_ap_status1](index.html) module"]
pub struct TR_MON_AP_STATUS1_SPEC;
impl crate::RegisterSpec for TR_MON_AP_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_mon_ap_status1::R](R) reader structure"]
impl crate::Readable for TR_MON_AP_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TR_MON_AP_STATUS1 to value 0"]
impl crate::Resettable for TR_MON_AP_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
