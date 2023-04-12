#[doc = "Register `MBSC1` reader"]
pub struct R(crate::R<MBSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MBC` reader - Message Buffer Status Changed MBC\\[31:0\\]
An MBC flag is set whenever the Message Handler changes one of the status flags VFRA, VFRB, SEOA, SEOB, CEOA, CEOB, SVOA, SVOB, TCIA, TCIB, ESA, ESB, MLST, FTA, FTB in the header section (see 4.11.5 Message Buffer Status (MBS) and 5.12.1 Header Partition, header 4) of the respective message buffer. An MBC flag is reset when the header section of the corresponding message buffer is reconfigured or when it has been transferred to the Output Buffer."]
pub type MBC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Buffer Status Changed MBC\\[31:0\\]
An MBC flag is set whenever the Message Handler changes one of the status flags VFRA, VFRB, SEOA, SEOB, CEOA, CEOB, SVOA, SVOB, TCIA, TCIB, ESA, ESB, MLST, FTA, FTB in the header section (see 4.11.5 Message Buffer Status (MBS) and 5.12.1 Header Partition, header 4) of the respective message buffer. An MBC flag is reset when the header section of the corresponding message buffer is reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(self.bits)
    }
}
#[doc = "Message Buffer Status Changed 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbsc1](index.html) module"]
pub struct MBSC1_SPEC;
impl crate::RegisterSpec for MBSC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbsc1::R](R) reader structure"]
impl crate::Readable for MBSC1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MBSC1 to value 0"]
impl crate::Resettable for MBSC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
