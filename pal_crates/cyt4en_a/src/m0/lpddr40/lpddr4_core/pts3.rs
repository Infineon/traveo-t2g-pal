#[doc = "Register `PTS3` reader"]
pub struct R(crate::R<PTS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DQSDMERR` reader - DQS-DM training error on rank 0. One bit for each DM signal. 1 means data eye training for corresponding DM signal is not completed successfully."]
pub type DQSDMERR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - DQS-DM training error on rank 0. One bit for each DM signal. 1 means data eye training for corresponding DM signal is not completed successfully."]
    #[inline(always)]
    pub fn dqsdmerr(&self) -> DQSDMERR_R {
        DQSDMERR_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "PHY Training Status Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pts3](index.html) module"]
pub struct PTS3_SPEC;
impl crate::RegisterSpec for PTS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pts3::R](R) reader structure"]
impl crate::Readable for PTS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTS3 to value 0"]
impl crate::Resettable for PTS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
