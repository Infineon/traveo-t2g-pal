#[doc = "Register `DESIGNATTRIBUTES1` reader"]
pub struct R(crate::R<DESIGNATTRIBUTES1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNATTRIBUTES1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNATTRIBUTES1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNATTRIBUTES1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPSCALERWIDTH` reader - Returns the maximum width that can be rendered in LBO mode minus one."]
pub type GPSCALERWIDTH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Returns the maximum width that can be rendered in LBO mode minus one."]
    #[inline(always)]
    pub fn gpscalerwidth(&self) -> GPSCALERWIDTH_R {
        GPSCALERWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "This register contains attributes of Bliteng. They are not parameters since they cannot be easily changed, however here we still have them to provide some static status on design.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designattributes1](index.html) module"]
pub struct DESIGNATTRIBUTES1_SPEC;
impl crate::RegisterSpec for DESIGNATTRIBUTES1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designattributes1::R](R) reader structure"]
impl crate::Readable for DESIGNATTRIBUTES1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNATTRIBUTES1 to value 0"]
impl crate::Resettable for DESIGNATTRIBUTES1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
