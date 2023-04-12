#[doc = "Register `LBHLINK0BUFFERSTATUS4` reader"]
pub struct R(crate::R<LBHLINK0BUFFERSTATUS4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK0BUFFERSTATUS4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK0BUFFERSTATUS4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK0BUFFERSTATUS4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHLINK0FETCHSTOPLINE` reader - This field describes the stopline of the display fetch."]
pub type LBHLINK0FETCHSTOPLINE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK0STORESTOPLINE` reader - This field describes the stopline of the store."]
pub type LBHLINK0STORESTOPLINE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - This field describes the stopline of the display fetch."]
    #[inline(always)]
    pub fn lbhlink0fetchstopline(&self) -> LBHLINK0FETCHSTOPLINE_R {
        LBHLINK0FETCHSTOPLINE_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - This field describes the stopline of the store."]
    #[inline(always)]
    pub fn lbhlink0storestopline(&self) -> LBHLINK0STORESTOPLINE_R {
        LBHLINK0STORESTOPLINE_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "Buffer status register 4 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink0bufferstatus4](index.html) module"]
pub struct LBHLINK0BUFFERSTATUS4_SPEC;
impl crate::RegisterSpec for LBHLINK0BUFFERSTATUS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink0bufferstatus4::R](R) reader structure"]
impl crate::Readable for LBHLINK0BUFFERSTATUS4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHLINK0BUFFERSTATUS4 to value 0"]
impl crate::Resettable for LBHLINK0BUFFERSTATUS4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
