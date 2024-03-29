#[doc = "Register `LBHLINK1BUFFERSTATUS2` reader"]
pub struct R(crate::R<LBHLINK1BUFFERSTATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK1BUFFERSTATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK1BUFFERSTATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK1BUFFERSTATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHLINK1BUFFERREADYCYCLES` reader - This counter increments with every cycle a slice is allowed to be written to the line buffer."]
pub type LBHLINK1BUFFERREADYCYCLES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This counter increments with every cycle a slice is allowed to be written to the line buffer."]
    #[inline(always)]
    pub fn lbhlink1bufferreadycycles(&self) -> LBHLINK1BUFFERREADYCYCLES_R {
        LBHLINK1BUFFERREADYCYCLES_R::new(self.bits)
    }
}
#[doc = "Buffer status register 2 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink1bufferstatus2](index.html) module"]
pub struct LBHLINK1BUFFERSTATUS2_SPEC;
impl crate::RegisterSpec for LBHLINK1BUFFERSTATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink1bufferstatus2::R](R) reader structure"]
impl crate::Readable for LBHLINK1BUFFERSTATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHLINK1BUFFERSTATUS2 to value 0"]
impl crate::Resettable for LBHLINK1BUFFERSTATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
