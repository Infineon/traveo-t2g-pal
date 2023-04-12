#[doc = "Register `CURPIXELCNT` reader"]
pub struct R(crate::R<CURPIXELCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURPIXELCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURPIXELCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURPIXELCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `C_XVAL` reader - value of horizontal pixel counter, internal counter counting from max-1 to 0"]
pub type C_XVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C_YVAL` reader - value of vertical line counter, internal counter counting from max-1 to 0"]
pub type C_YVAL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - value of horizontal pixel counter, internal counter counting from max-1 to 0"]
    #[inline(always)]
    pub fn c_xval(&self) -> C_XVAL_R {
        C_XVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - value of vertical line counter, internal counter counting from max-1 to 0"]
    #[inline(always)]
    pub fn c_yval(&self) -> C_YVAL_R {
        C_YVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "pixel count of currently running frame\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curpixelcnt](index.html) module"]
pub struct CURPIXELCNT_SPEC;
impl crate::RegisterSpec for CURPIXELCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [curpixelcnt::R](R) reader structure"]
impl crate::Readable for CURPIXELCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CURPIXELCNT to value 0"]
impl crate::Resettable for CURPIXELCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
