#[doc = "Register `FGSREPD` reader"]
pub struct R(crate::R<FGSREPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSREPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSREPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSREPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPVAL` reader - Calculated value for line skew extrapolation in blanking. Updated with condition 'hlast AND vlast'. (For debugging purposes only)"]
pub type EPVAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:28 - Calculated value for line skew extrapolation in blanking. Updated with condition 'hlast AND vlast'. (For debugging purposes only)"]
    #[inline(always)]
    pub fn epval(&self) -> EPVAL_R {
        EPVAL_R::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "FrameGen Skew Regulation ExtraPolation Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsrepd](index.html) module"]
pub struct FGSREPD_SPEC;
impl crate::RegisterSpec for FGSREPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsrepd::R](R) reader structure"]
impl crate::Readable for FGSREPD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGSREPD to value 0"]
impl crate::Resettable for FGSREPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
