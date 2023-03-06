#[doc = "Register `BIST_LB_STATUS` reader"]
pub struct R(crate::R<BIST_LB_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_LB_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_LB_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_LB_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_INT_LB_PASS` reader - Status bit showing the pass/fail result of the test"]
pub type D_INT_LB_PASS_R = crate::BitReader<bool>;
#[doc = "Field `D_LB_ACTIVE` reader - Status bit showing that the BIST loopback test is active and running"]
pub type D_LB_ACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Status bit showing the pass/fail result of the test"]
    #[inline(always)]
    pub fn d_int_lb_pass(&self) -> D_INT_LB_PASS_R {
        D_INT_LB_PASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Status bit showing that the BIST loopback test is active and running"]
    #[inline(always)]
    pub fn d_lb_active(&self) -> D_LB_ACTIVE_R {
        D_LB_ACTIVE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "BIST Loopback status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_lb_status](index.html) module"]
pub struct BIST_LB_STATUS_SPEC;
impl crate::RegisterSpec for BIST_LB_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_lb_status::R](R) reader structure"]
impl crate::Readable for BIST_LB_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BIST_LB_STATUS to value 0"]
impl crate::Resettable for BIST_LB_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
