#[doc = "Register `FGSKEWMON` reader"]
pub struct R(crate::R<FGSKEWMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSKEWMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSKEWMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSKEWMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SKEWMON` reader - Current skew value monitor for secondary channel skew control. Updated with hlast. (For debugging purposes only)."]
pub type SKEWMON_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:28 - Current skew value monitor for secondary channel skew control. Updated with hlast. (For debugging purposes only)."]
    #[inline(always)]
    pub fn skewmon(&self) -> SKEWMON_R {
        SKEWMON_R::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "FrameGen Skew Monitor Register for Secondary Channel Skew Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgskewmon](index.html) module"]
pub struct FGSKEWMON_SPEC;
impl crate::RegisterSpec for FGSKEWMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgskewmon::R](R) reader structure"]
impl crate::Readable for FGSKEWMON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGSKEWMON to value 0"]
impl crate::Resettable for FGSKEWMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
