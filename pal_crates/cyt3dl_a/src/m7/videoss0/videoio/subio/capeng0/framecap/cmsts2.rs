#[doc = "Register `CMSTS2` reader"]
pub struct R(crate::R<CMSTS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMSTS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMSTS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMSTS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOTWIDTHT` reader - cycle number of the total part of a video frame line for calculating pixel frame rate. LSBs contained in this field, MSBs contained in FRLineCount.TotWidthT_MSBs."]
pub type TOTWIDTHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTWIDTHT` reader - cycle number of the active part of a video frame line for calculating pixel frame rate. LSBs contained in this field, MSBs contained in FRLineCount.ActWidthT_MSBs."]
pub type ACTWIDTHT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - cycle number of the total part of a video frame line for calculating pixel frame rate. LSBs contained in this field, MSBs contained in FRLineCount.TotWidthT_MSBs."]
    #[inline(always)]
    pub fn totwidtht(&self) -> TOTWIDTHT_R {
        TOTWIDTHT_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:28 - cycle number of the active part of a video frame line for calculating pixel frame rate. LSBs contained in this field, MSBs contained in FRLineCount.ActWidthT_MSBs."]
    #[inline(always)]
    pub fn actwidtht(&self) -> ACTWIDTHT_R {
        ACTWIDTHT_R::new(((self.bits >> 15) & 0x3fff) as u16)
    }
}
#[doc = "clk_axi cycle number of totwidth and actwidth of a frame. (bit locked when MdrCmrDone=1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmsts2](index.html) module"]
pub struct CMSTS2_SPEC;
impl crate::RegisterSpec for CMSTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmsts2::R](R) reader structure"]
impl crate::Readable for CMSTS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMSTS2 to value 0"]
impl crate::Resettable for CMSTS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
