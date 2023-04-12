#[doc = "Register `BIST_CNT_STATUS` reader"]
pub struct R(crate::R<BIST_CNT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_CNT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_CNT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_CNT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D_INT_LB_ERR_CNT` reader - Ten-bit status field that showing the number of failing bytes in case of a test failure."]
pub type D_INT_LB_ERR_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `D_INT_LB_BYTE_FLAG` reader - Status bit indicating that the LVDS RX received 1024 words and the test is over"]
pub type D_INT_LB_BYTE_FLAG_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:9 - Ten-bit status field that showing the number of failing bytes in case of a test failure."]
    #[inline(always)]
    pub fn d_int_lb_err_cnt(&self) -> D_INT_LB_ERR_CNT_R {
        D_INT_LB_ERR_CNT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Status bit indicating that the LVDS RX received 1024 words and the test is over"]
    #[inline(always)]
    pub fn d_int_lb_byte_flag(&self) -> D_INT_LB_BYTE_FLAG_R {
        D_INT_LB_BYTE_FLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "BIST loopback count status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_cnt_status](index.html) module"]
pub struct BIST_CNT_STATUS_SPEC;
impl crate::RegisterSpec for BIST_CNT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_cnt_status::R](R) reader structure"]
impl crate::Readable for BIST_CNT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BIST_CNT_STATUS to value 0"]
impl crate::Resettable for BIST_CNT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
