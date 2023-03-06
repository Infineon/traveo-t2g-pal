#[doc = "Register `LBHLINK4BUFFERSTATUS1` reader"]
pub struct R(crate::R<LBHLINK4BUFFERSTATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK4BUFFERSTATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK4BUFFERSTATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK4BUFFERSTATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHLINK4AVAILABLELINES` reader - The number of lines in the line buffer that the store can write."]
pub type LBHLINK4AVAILABLELINES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK4SAMEFRAME` reader - This status bit indicates if the fetch and the store are processing the same frame. If this bit is zero the store has already switched to the next frame"]
pub type LBHLINK4SAMEFRAME_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:14 - The number of lines in the line buffer that the store can write."]
    #[inline(always)]
    pub fn lbhlink4availablelines(&self) -> LBHLINK4AVAILABLELINES_R {
        LBHLINK4AVAILABLELINES_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - This status bit indicates if the fetch and the store are processing the same frame. If this bit is zero the store has already switched to the next frame"]
    #[inline(always)]
    pub fn lbhlink4sameframe(&self) -> LBHLINK4SAMEFRAME_R {
        LBHLINK4SAMEFRAME_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Buffer status register 1 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink4bufferstatus1](index.html) module"]
pub struct LBHLINK4BUFFERSTATUS1_SPEC;
impl crate::RegisterSpec for LBHLINK4BUFFERSTATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink4bufferstatus1::R](R) reader structure"]
impl crate::Readable for LBHLINK4BUFFERSTATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHLINK4BUFFERSTATUS1 to value 0x0001_0001"]
impl crate::Resettable for LBHLINK4BUFFERSTATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
