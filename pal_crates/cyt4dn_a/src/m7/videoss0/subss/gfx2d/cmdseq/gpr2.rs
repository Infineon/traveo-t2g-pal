#[doc = "Register `GPR2` reader"]
pub struct R(crate::R<GPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER2` reader - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
pub type GENERALPURPOSEREGISTER2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister2(&self) -> GENERALPURPOSEREGISTER2_R {
        GENERALPURPOSEREGISTER2_R::new(self.bits)
    }
}
#[doc = "GeneralPurpose Register. (Internal register with id = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr2](index.html) module"]
pub struct GPR2_SPEC;
impl crate::RegisterSpec for GPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr2::R](R) reader structure"]
impl crate::Readable for GPR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR2 to value 0"]
impl crate::Resettable for GPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
