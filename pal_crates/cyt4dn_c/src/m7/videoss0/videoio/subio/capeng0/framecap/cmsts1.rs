#[doc = "Register `CMSTS1` reader"]
pub struct R(crate::R<CMSTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMSTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMSTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMSTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOTAL_TIME` reader - clk_axi cycle number of a frame starting from vsync to another vsync."]
pub type TOTAL_TIME_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - clk_axi cycle number of a frame starting from vsync to another vsync."]
    #[inline(always)]
    pub fn total_time(&self) -> TOTAL_TIME_R {
        TOTAL_TIME_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "clk_axi cycle number of a frame. (bit locked when MdrCmrDone=1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmsts1](index.html) module"]
pub struct CMSTS1_SPEC;
impl crate::RegisterSpec for CMSTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmsts1::R](R) reader structure"]
impl crate::Readable for CMSTS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMSTS1 to value 0"]
impl crate::Resettable for CMSTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
