#[doc = "Register `SIZEY` reader"]
pub struct R(crate::R<SIZEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIZEY` reader - Y size value downloaded from SOF marker segment."]
pub type SIZEY_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Y size value downloaded from SOF marker segment."]
    #[inline(always)]
    pub fn sizey(&self) -> SIZEY_R {
        SIZEY_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Y size downloaded from the JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sizey](index.html) module"]
pub struct SIZEY_SPEC;
impl crate::RegisterSpec for SIZEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sizey::R](R) reader structure"]
impl crate::Readable for SIZEY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIZEY to value 0"]
impl crate::Resettable for SIZEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
