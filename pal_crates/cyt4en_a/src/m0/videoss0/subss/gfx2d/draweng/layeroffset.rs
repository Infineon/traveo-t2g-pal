#[doc = "Register `LAYEROFFSET` reader"]
pub struct R(crate::R<LAYEROFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYEROFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYEROFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYEROFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `XOFFSET` reader - Horizontal offset (X). (format is signed integer)"]
pub type XOFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YOFFSET` reader - Vertical offset (Y). (format is signed integer)"]
pub type YOFFSET_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Horizontal offset (X). (format is signed integer)"]
    #[inline(always)]
    pub fn xoffset(&self) -> XOFFSET_R {
        XOFFSET_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Vertical offset (Y). (format is signed integer)"]
    #[inline(always)]
    pub fn yoffset(&self) -> YOFFSET_R {
        YOFFSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Desired position of the alpha frame for the following Blit operation. This register is aligned to the LayerOffset of the fetch and given as signed 16.0 values.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layeroffset](index.html) module"]
pub struct LAYEROFFSET_SPEC;
impl crate::RegisterSpec for LAYEROFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layeroffset::R](R) reader structure"]
impl crate::Readable for LAYEROFFSET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LAYEROFFSET to value 0"]
impl crate::Resettable for LAYEROFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
