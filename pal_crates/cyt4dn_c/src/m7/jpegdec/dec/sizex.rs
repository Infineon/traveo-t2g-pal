#[doc = "Register `SIZEX` reader"]
pub struct R(crate::R<SIZEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIZEX` reader - X size value downloaded from SOF marker segment."]
pub type SIZEX_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - X size value downloaded from SOF marker segment."]
    #[inline(always)]
    pub fn sizex(&self) -> SIZEX_R {
        SIZEX_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "X size downloaded from the JPEG image.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sizex](index.html) module"]
pub struct SIZEX_SPEC;
impl crate::RegisterSpec for SIZEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sizex::R](R) reader structure"]
impl crate::Readable for SIZEX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIZEX to value 0"]
impl crate::Resettable for SIZEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
