#[doc = "Register `PCS_AN_EXP` reader"]
pub struct R(crate::R<PCS_AN_EXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCS_AN_EXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCS_AN_EXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCS_AN_EXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REMOVED_31_0` reader - Write ignore, read 0"]
pub type REMOVED_31_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_31_0(&self) -> REMOVED_31_0_R {
        REMOVED_31_0_R::new(self.bits)
    }
}
#[doc = "Not presents. Access to the register returns AHB error.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcs_an_exp](index.html) module"]
pub struct PCS_AN_EXP_SPEC;
impl crate::RegisterSpec for PCS_AN_EXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcs_an_exp::R](R) reader structure"]
impl crate::Readable for PCS_AN_EXP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCS_AN_EXP to value 0"]
impl crate::Resettable for PCS_AN_EXP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
