#[doc = "Register `GPR3` reader"]
pub struct R(crate::R<GPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER3` reader - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
pub type GENERALPURPOSEREGISTER3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister3(&self) -> GENERALPURPOSEREGISTER3_R {
        GENERALPURPOSEREGISTER3_R::new(self.bits)
    }
}
#[doc = "GeneralPurpose Register. (Internal register with id = 3)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr3](index.html) module"]
pub struct GPR3_SPEC;
impl crate::RegisterSpec for GPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr3::R](R) reader structure"]
impl crate::Readable for GPR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR3 to value 0"]
impl crate::Resettable for GPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
