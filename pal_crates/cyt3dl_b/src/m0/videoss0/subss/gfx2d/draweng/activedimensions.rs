#[doc = "Register `ACTIVEDIMENSIONS` reader"]
pub struct R(crate::R<ACTIVEDIMENSIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVEDIMENSIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVEDIMENSIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVEDIMENSIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVEWIDTH` reader - Active width of the resulting alpha frame minus one."]
pub type ACTIVEWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTIVEHEIGHT` reader - Active height of the resulting alpha frame minus one."]
pub type ACTIVEHEIGHT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Active width of the resulting alpha frame minus one."]
    #[inline(always)]
    pub fn activewidth(&self) -> ACTIVEWIDTH_R {
        ACTIVEWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Active height of the resulting alpha frame minus one."]
    #[inline(always)]
    pub fn activeheight(&self) -> ACTIVEHEIGHT_R {
        ACTIVEHEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Shows active dimensions of the output alpha frame. This register is aligned to the ClipWindowDimensions register of the fetch.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [activedimensions](index.html) module"]
pub struct ACTIVEDIMENSIONS_SPEC;
impl crate::RegisterSpec for ACTIVEDIMENSIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [activedimensions::R](R) reader structure"]
impl crate::Readable for ACTIVEDIMENSIONS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVEDIMENSIONS to value 0"]
impl crate::Resettable for ACTIVEDIMENSIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
