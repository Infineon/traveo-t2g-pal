#[doc = "Register `GPR13` reader"]
pub struct R(crate::R<GPR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER13` reader - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
pub type GENERALPURPOSEREGISTER13_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister13(&self) -> GENERALPURPOSEREGISTER13_R {
        GENERALPURPOSEREGISTER13_R::new(self.bits)
    }
}
#[doc = "GeneralPurpose Register. (Internal register with id = 13)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr13](index.html) module"]
pub struct GPR13_SPEC;
impl crate::RegisterSpec for GPR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr13::R](R) reader structure"]
impl crate::Readable for GPR13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR13 to value 0"]
impl crate::Resettable for GPR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
