#[doc = "Register `ULPS_CLK_STATUS` reader"]
pub struct R(crate::R<ULPS_CLK_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULPS_CLK_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULPS_CLK_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULPS_CLK_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ULPS_CLK_STATUS` reader - Status of RX Clock Lane. \\[0\\]
- Clock Lane in ULPS state when 1"]
pub type ULPS_CLK_STATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Status of RX Clock Lane. \\[0\\]
- Clock Lane in ULPS state when 1"]
    #[inline(always)]
    pub fn ulps_clk_status(&self) -> ULPS_CLK_STATUS_R {
        ULPS_CLK_STATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "ULPS_CLK_STATUS is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulps_clk_status](index.html) module"]
pub struct ULPS_CLK_STATUS_SPEC;
impl crate::RegisterSpec for ULPS_CLK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulps_clk_status::R](R) reader structure"]
impl crate::Readable for ULPS_CLK_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ULPS_CLK_STATUS to value 0"]
impl crate::Resettable for ULPS_CLK_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
