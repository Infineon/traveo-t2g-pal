#[doc = "Register `LBHLINK4BUFFERSTATUS2` reader"]
pub struct R(crate::R<LBHLINK4BUFFERSTATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK4BUFFERSTATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK4BUFFERSTATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK4BUFFERSTATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHLINK4BUFFERREADYCYCLES` reader - This counter increments with every cycle a slice is allowed to be written to the line buffer."]
pub type LBHLINK4BUFFERREADYCYCLES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This counter increments with every cycle a slice is allowed to be written to the line buffer."]
    #[inline(always)]
    pub fn lbhlink4bufferreadycycles(&self) -> LBHLINK4BUFFERREADYCYCLES_R {
        LBHLINK4BUFFERREADYCYCLES_R::new(self.bits)
    }
}
#[doc = "Buffer status register 2 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink4bufferstatus2](index.html) module"]
pub struct LBHLINK4BUFFERSTATUS2_SPEC;
impl crate::RegisterSpec for LBHLINK4BUFFERSTATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink4bufferstatus2::R](R) reader structure"]
impl crate::Readable for LBHLINK4BUFFERSTATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHLINK4BUFFERSTATUS2 to value 0"]
impl crate::Resettable for LBHLINK4BUFFERSTATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
