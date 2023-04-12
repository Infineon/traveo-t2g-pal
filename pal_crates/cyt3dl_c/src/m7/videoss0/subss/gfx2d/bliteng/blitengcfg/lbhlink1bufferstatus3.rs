#[doc = "Register `LBHLINK1BUFFERSTATUS3` reader"]
pub struct R(crate::R<LBHLINK1BUFFERSTATUS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK1BUFFERSTATUS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK1BUFFERSTATUS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK1BUFFERSTATUS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHLINK1FETCHLINE` reader - This field describes the line currently read by the display fetch."]
pub type LBHLINK1FETCHLINE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK1STORELINE` reader - This field describes the line currently written by the store."]
pub type LBHLINK1STORELINE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - This field describes the line currently read by the display fetch."]
    #[inline(always)]
    pub fn lbhlink1fetchline(&self) -> LBHLINK1FETCHLINE_R {
        LBHLINK1FETCHLINE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - This field describes the line currently written by the store."]
    #[inline(always)]
    pub fn lbhlink1storeline(&self) -> LBHLINK1STORELINE_R {
        LBHLINK1STORELINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Buffer status register 3 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink1bufferstatus3](index.html) module"]
pub struct LBHLINK1BUFFERSTATUS3_SPEC;
impl crate::RegisterSpec for LBHLINK1BUFFERSTATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink1bufferstatus3::R](R) reader structure"]
impl crate::Readable for LBHLINK1BUFFERSTATUS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHLINK1BUFFERSTATUS3 to value 0"]
impl crate::Resettable for LBHLINK1BUFFERSTATUS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
