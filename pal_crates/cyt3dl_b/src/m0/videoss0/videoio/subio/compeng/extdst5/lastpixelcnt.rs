#[doc = "Register `LASTPIXELCNT` reader"]
pub struct R(crate::R<LASTPIXELCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LASTPIXELCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LASTPIXELCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LASTPIXELCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L_XVAL` reader - value of horizontal pixel counter"]
pub type L_XVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L_YVAL` reader - value of vertical line counter"]
pub type L_YVAL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - value of horizontal pixel counter"]
    #[inline(always)]
    pub fn l_xval(&self) -> L_XVAL_R {
        L_XVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - value of vertical line counter"]
    #[inline(always)]
    pub fn l_yval(&self) -> L_YVAL_R {
        L_YVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "pixel count between last two control words\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lastpixelcnt](index.html) module"]
pub struct LASTPIXELCNT_SPEC;
impl crate::RegisterSpec for LASTPIXELCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lastpixelcnt::R](R) reader structure"]
impl crate::Readable for LASTPIXELCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LASTPIXELCNT to value 0"]
impl crate::Resettable for LASTPIXELCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
