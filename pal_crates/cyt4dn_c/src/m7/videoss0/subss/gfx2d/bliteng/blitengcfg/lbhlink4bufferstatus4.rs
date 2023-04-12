#[doc = "Register `LBHLINK4BUFFERSTATUS4` reader"]
pub struct R(crate::R<LBHLINK4BUFFERSTATUS4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK4BUFFERSTATUS4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK4BUFFERSTATUS4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK4BUFFERSTATUS4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHLINK4FETCHSTOPLINE` reader - This field describes the stopline of the display fetch."]
pub type LBHLINK4FETCHSTOPLINE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK4STORESTOPLINE` reader - This field describes the stopline of the store."]
pub type LBHLINK4STORESTOPLINE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - This field describes the stopline of the display fetch."]
    #[inline(always)]
    pub fn lbhlink4fetchstopline(&self) -> LBHLINK4FETCHSTOPLINE_R {
        LBHLINK4FETCHSTOPLINE_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - This field describes the stopline of the store."]
    #[inline(always)]
    pub fn lbhlink4storestopline(&self) -> LBHLINK4STORESTOPLINE_R {
        LBHLINK4STORESTOPLINE_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "Buffer status register 4 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink4bufferstatus4](index.html) module"]
pub struct LBHLINK4BUFFERSTATUS4_SPEC;
impl crate::RegisterSpec for LBHLINK4BUFFERSTATUS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink4bufferstatus4::R](R) reader structure"]
impl crate::Readable for LBHLINK4BUFFERSTATUS4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHLINK4BUFFERSTATUS4 to value 0"]
impl crate::Resettable for LBHLINK4BUFFERSTATUS4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
