#[doc = "Register `INTR_CAUSE` reader"]
pub struct R(crate::R<INTR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NORMAL` reader - Reflects the state of interrupt_normal (which is the OR of the elements in the normal INTR_MASKED)"]
pub type NORMAL_R = crate::BitReader<bool>;
#[doc = "Field `MPC` reader - Reflects the state of interrupt_mpc (which is the OR of the elements in the MPC'S INTR_MASKED)"]
pub type MPC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Reflects the state of interrupt_normal (which is the OR of the elements in the normal INTR_MASKED)"]
    #[inline(always)]
    pub fn normal(&self) -> NORMAL_R {
        NORMAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reflects the state of interrupt_mpc (which is the OR of the elements in the MPC'S INTR_MASKED)"]
    #[inline(always)]
    pub fn mpc(&self) -> MPC_R {
        MPC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Distinguishes normal vs. MPC interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](index.html) module"]
pub struct INTR_CAUSE_SPEC;
impl crate::RegisterSpec for INTR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for INTR_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
