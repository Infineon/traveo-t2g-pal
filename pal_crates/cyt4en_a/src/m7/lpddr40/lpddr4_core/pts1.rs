#[doc = "Register `PTS1` reader"]
pub struct R(crate::R<PTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDLVLDQERR` reader - DQ read data eye training error on rank 0. One bit for each DQ signal. 1 means data eye training for corresponding DQ signal is not completed successfully."]
pub type RDLVLDQERR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DQ read data eye training error on rank 0. One bit for each DQ signal. 1 means data eye training for corresponding DQ signal is not completed successfully."]
    #[inline(always)]
    pub fn rdlvldqerr(&self) -> RDLVLDQERR_R {
        RDLVLDQERR_R::new(self.bits)
    }
}
#[doc = "PHY Training Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pts1](index.html) module"]
pub struct PTS1_SPEC;
impl crate::RegisterSpec for PTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pts1::R](R) reader structure"]
impl crate::Readable for PTS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTS1 to value 0"]
impl crate::Resettable for PTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
