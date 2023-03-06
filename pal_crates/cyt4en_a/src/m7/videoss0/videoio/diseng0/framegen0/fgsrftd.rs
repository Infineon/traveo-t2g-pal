#[doc = "Register `FGSRFTD` reader"]
pub struct R(crate::R<FGSRFTD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSRFTD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSRFTD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSRFTD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRTOT` reader - Measured value for frame total measured in display clock cycles. Updated with condition 'hlast AND vlast' when first FrTot value is valid. (For debugging purposes only)"]
pub type FRTOT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:27 - Measured value for frame total measured in display clock cycles. Updated with condition 'hlast AND vlast' when first FrTot value is valid. (For debugging purposes only)"]
    #[inline(always)]
    pub fn frtot(&self) -> FRTOT_R {
        FRTOT_R::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "FrameGen Skew Regulation Frame Total Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsrftd](index.html) module"]
pub struct FGSRFTD_SPEC;
impl crate::RegisterSpec for FGSRFTD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsrftd::R](R) reader structure"]
impl crate::Readable for FGSRFTD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGSRFTD to value 0"]
impl crate::Resettable for FGSRFTD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
