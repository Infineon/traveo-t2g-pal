#[doc = "Register `MRR_CH0` reader"]
pub struct R(crate::R<MRR_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRR_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRR_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRR_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE` reader - MRR complete - Channel 0"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `READOUT` reader - MRR data - Channel 0"]
pub type READOUT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - MRR complete - Channel 0"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - MRR data - Channel 0"]
    #[inline(always)]
    pub fn readout(&self) -> READOUT_R {
        READOUT_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
#[doc = "MR Readout Register - Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrr_ch0](index.html) module"]
pub struct MRR_CH0_SPEC;
impl crate::RegisterSpec for MRR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrr_ch0::R](R) reader structure"]
impl crate::Readable for MRR_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MRR_CH0 to value 0"]
impl crate::Resettable for MRR_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
