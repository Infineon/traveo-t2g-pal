#[doc = "Register `CLOCK_STAT` reader"]
pub struct R(crate::R<CLOCK_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MCLK_DIV_OFF` reader - MCLK Divider OFF 0: Indicates MCLK Divider not a reset state 1: Indicates MCLK Divider at reset state. This is ONLY set when IP is enabled (CTL.RX_ENABLED=1 or CTL.TX_ENABLED=1), CLOCK_CTL.MCLK_EN=0, and MCLK_DIV reaches it's reset state. The intent is that this bit indicates successful shutdown of the MCLK_DIVIDER after CLOCK_CTL.MCLK_EN transitions from 1 to 0."]
pub type MCLK_DIV_OFF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - MCLK Divider OFF 0: Indicates MCLK Divider not a reset state 1: Indicates MCLK Divider at reset state. This is ONLY set when IP is enabled (CTL.RX_ENABLED=1 or CTL.TX_ENABLED=1), CLOCK_CTL.MCLK_EN=0, and MCLK_DIV reaches it's reset state. The intent is that this bit indicates successful shutdown of the MCLK_DIVIDER after CLOCK_CTL.MCLK_EN transitions from 1 to 0."]
    #[inline(always)]
    pub fn mclk_div_off(&self) -> MCLK_DIV_OFF_R {
        MCLK_DIV_OFF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Clock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_stat](index.html) module"]
pub struct CLOCK_STAT_SPEC;
impl crate::RegisterSpec for CLOCK_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_stat::R](R) reader structure"]
impl crate::Readable for CLOCK_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLOCK_STAT to value 0"]
impl crate::Resettable for CLOCK_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
