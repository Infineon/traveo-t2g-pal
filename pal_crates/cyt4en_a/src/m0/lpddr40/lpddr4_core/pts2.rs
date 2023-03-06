#[doc = "Register `PTS2` reader"]
pub struct R(crate::R<PTS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DQSDQERR` reader - DQS-DQ training error on rank 0. One bit for each DQ signal. 1 means data eye training for corresponding DQ signal is not completed successfully."]
pub type DQSDQERR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DQS-DQ training error on rank 0. One bit for each DQ signal. 1 means data eye training for corresponding DQ signal is not completed successfully."]
    #[inline(always)]
    pub fn dqsdqerr(&self) -> DQSDQERR_R {
        DQSDQERR_R::new(self.bits)
    }
}
#[doc = "PHY Training Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pts2](index.html) module"]
pub struct PTS2_SPEC;
impl crate::RegisterSpec for PTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pts2::R](R) reader structure"]
impl crate::Readable for PTS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTS2 to value 0"]
impl crate::Resettable for PTS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
