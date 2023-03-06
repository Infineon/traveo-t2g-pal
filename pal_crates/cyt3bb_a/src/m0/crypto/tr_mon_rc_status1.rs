#[doc = "Register `TR_MON_RC_STATUS1` reader"]
pub struct R(crate::R<TR_MON_RC_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_MON_RC_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_MON_RC_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_MON_RC_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REP_COUNT` reader - Number of repetitions of the current active bit counter: '0': 0 repetitions. ... '255': 255 repetitions."]
pub type REP_COUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of repetitions of the current active bit counter: '0': 0 repetitions. ... '255': 255 repetitions."]
    #[inline(always)]
    pub fn rep_count(&self) -> REP_COUNT_R {
        REP_COUNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "True random monitor RC status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_mon_rc_status1](index.html) module"]
pub struct TR_MON_RC_STATUS1_SPEC;
impl crate::RegisterSpec for TR_MON_RC_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_mon_rc_status1::R](R) reader structure"]
impl crate::Readable for TR_MON_RC_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TR_MON_RC_STATUS1 to value 0"]
impl crate::Resettable for TR_MON_RC_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
