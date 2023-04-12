#[doc = "Register `TR_MON_RC_STATUS0` reader"]
pub struct R(crate::R<TR_MON_RC_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_MON_RC_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_MON_RC_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_MON_RC_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIT` reader - Current active bit value: '0': '0'. '1': '1'. This field is only valid when TR_MON_RC_STATUS1.REP_COUNT is NOT equal to '0'."]
pub type BIT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Current active bit value: '0': '0'. '1': '1'. This field is only valid when TR_MON_RC_STATUS1.REP_COUNT is NOT equal to '0'."]
    #[inline(always)]
    pub fn bit_(&self) -> BIT_R {
        BIT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "True random monitor RC status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_mon_rc_status0](index.html) module"]
pub struct TR_MON_RC_STATUS0_SPEC;
impl crate::RegisterSpec for TR_MON_RC_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_mon_rc_status0::R](R) reader structure"]
impl crate::Readable for TR_MON_RC_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TR_MON_RC_STATUS0 to value 0"]
impl crate::Resettable for TR_MON_RC_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
