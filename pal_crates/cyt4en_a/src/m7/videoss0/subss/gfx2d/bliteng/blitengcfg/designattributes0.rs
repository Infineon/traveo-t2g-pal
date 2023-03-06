#[doc = "Register `DESIGNATTRIBUTES0` reader"]
pub struct R(crate::R<DESIGNATTRIBUTES0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESIGNATTRIBUTES0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESIGNATTRIBUTES0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESIGNATTRIBUTES0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLICEHEIGHT` reader - Returns the slice height mode minus one."]
pub type SLICEHEIGHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROCESSINGPIPELINES` reader - Returns the maximum number of pixel processing pipelines LBO mode minus one."]
pub type PROCESSINGPIPELINES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLICEWIDTH` reader - Returns the maximum width that can be rendered in LBO mode minus one."]
pub type SLICEWIDTH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - Returns the slice height mode minus one."]
    #[inline(always)]
    pub fn sliceheight(&self) -> SLICEHEIGHT_R {
        SLICEHEIGHT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Returns the maximum number of pixel processing pipelines LBO mode minus one."]
    #[inline(always)]
    pub fn processingpipelines(&self) -> PROCESSINGPIPELINES_R {
        PROCESSINGPIPELINES_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:29 - Returns the maximum width that can be rendered in LBO mode minus one."]
    #[inline(always)]
    pub fn slicewidth(&self) -> SLICEWIDTH_R {
        SLICEWIDTH_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "This register contains attributes of Bliteng. They are not parameters since they cannot be easily changed, however here we still have them to provide some static status on design.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [designattributes0](index.html) module"]
pub struct DESIGNATTRIBUTES0_SPEC;
impl crate::RegisterSpec for DESIGNATTRIBUTES0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [designattributes0::R](R) reader structure"]
impl crate::Readable for DESIGNATTRIBUTES0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESIGNATTRIBUTES0 to value 0"]
impl crate::Resettable for DESIGNATTRIBUTES0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
