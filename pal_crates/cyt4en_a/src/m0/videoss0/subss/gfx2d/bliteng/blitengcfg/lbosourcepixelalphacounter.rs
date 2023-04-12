#[doc = "Register `LBOSOURCEPIXELALPHACOUNTER` reader"]
pub struct R(crate::R<LBOSOURCEPIXELALPHACOUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBOSOURCEPIXELALPHACOUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBOSOURCEPIXELALPHACOUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBOSOURCEPIXELALPHACOUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBOSOURCEPIXELALPHACOUNTER` reader - Provides the number of processed source alpha pixels which should be multiplied by 256. Only active in LBO and when LBOSourcePixelMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
pub type LBOSOURCEPIXELALPHACOUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the number of processed source alpha pixels which should be multiplied by 256. Only active in LBO and when LBOSourcePixelMeasurementEn is set. This field is updated with SaveAndResetMeasurements pulse and internal counter is reset."]
    #[inline(always)]
    pub fn lbosourcepixelalphacounter(&self) -> LBOSOURCEPIXELALPHACOUNTER_R {
        LBOSOURCEPIXELALPHACOUNTER_R::new(self.bits)
    }
}
#[doc = "Alpha source pixel counter for LBO mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbosourcepixelalphacounter](index.html) module"]
pub struct LBOSOURCEPIXELALPHACOUNTER_SPEC;
impl crate::RegisterSpec for LBOSOURCEPIXELALPHACOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbosourcepixelalphacounter::R](R) reader structure"]
impl crate::Readable for LBOSOURCEPIXELALPHACOUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBOSOURCEPIXELALPHACOUNTER to value 0"]
impl crate::Resettable for LBOSOURCEPIXELALPHACOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
