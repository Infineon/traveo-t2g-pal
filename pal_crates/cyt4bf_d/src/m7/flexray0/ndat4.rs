#[doc = "Register `NDAT4` reader"]
pub struct R(crate::R<NDAT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDAT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDAT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDAT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ND` reader - New Data ND\\[127:96\\]
The flags are set when a valid received data frame matches the message buffer's filter configuration, independent of the payload length received or the payload length configured for that message buffer. The flags are not set after reception of null frames except for message buffers belonging to the receive FIFO. An ND flag is reset when the header section of the corresponding message buffer is reconfigured or when the data section has been transferred to the Output Buffer."]
pub type ND_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - New Data ND\\[127:96\\]
The flags are set when a valid received data frame matches the message buffer's filter configuration, independent of the payload length received or the payload length configured for that message buffer. The flags are not set after reception of null frames except for message buffers belonging to the receive FIFO. An ND flag is reset when the header section of the corresponding message buffer is reconfigured or when the data section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd(&self) -> ND_R {
        ND_R::new(self.bits)
    }
}
#[doc = "New Data 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat4](index.html) module"]
pub struct NDAT4_SPEC;
impl crate::RegisterSpec for NDAT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ndat4::R](R) reader structure"]
impl crate::Readable for NDAT4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NDAT4 to value 0"]
impl crate::Resettable for NDAT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
