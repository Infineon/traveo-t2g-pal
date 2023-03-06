#[doc = "Register `DRI` reader"]
pub struct R(crate::R<DRI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DRI` reader - Value downloaded from DRI marker segment."]
pub type DRI_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Value downloaded from DRI marker segment."]
    #[inline(always)]
    pub fn dri(&self) -> DRI_R {
        DRI_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "The DRI value downloaded from the JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dri](index.html) module"]
pub struct DRI_SPEC;
impl crate::RegisterSpec for DRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dri::R](R) reader structure"]
impl crate::Readable for DRI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DRI to value 0"]
impl crate::Resettable for DRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
