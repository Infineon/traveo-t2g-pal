#[doc = "Register `FGTIMESTAMP` reader"]
pub struct R(crate::R<FGTIMESTAMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGTIMESTAMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGTIMESTAMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGTIMESTAMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINEINDEX` reader - Index of the output line that is currently generated (starts with 0 for first active output line)."]
pub type LINEINDEX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEINDEX` reader - Index of the output frame that is currently generated (starts with 0 after reset for first output frame). Wraps around to 0 when maximum value is reached."]
pub type FRAMEINDEX_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:13 - Index of the output line that is currently generated (starts with 0 for first active output line)."]
    #[inline(always)]
    pub fn lineindex(&self) -> LINEINDEX_R {
        LINEINDEX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - Index of the output frame that is currently generated (starts with 0 after reset for first output frame). Wraps around to 0 when maximum value is reached."]
    #[inline(always)]
    pub fn frameindex(&self) -> FRAMEINDEX_R {
        FRAMEINDEX_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
#[doc = "Time stamp status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgtimestamp](index.html) module"]
pub struct FGTIMESTAMP_SPEC;
impl crate::RegisterSpec for FGTIMESTAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgtimestamp::R](R) reader structure"]
impl crate::Readable for FGTIMESTAMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGTIMESTAMP to value 0"]
impl crate::Resettable for FGTIMESTAMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
