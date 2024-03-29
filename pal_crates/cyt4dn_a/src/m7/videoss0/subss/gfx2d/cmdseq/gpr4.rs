#[doc = "Register `GPR4` reader"]
pub struct R(crate::R<GPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER4` reader - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
pub type GENERALPURPOSEREGISTER4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister4(&self) -> GENERALPURPOSEREGISTER4_R {
        GENERALPURPOSEREGISTER4_R::new(self.bits)
    }
}
#[doc = "GeneralPurpose Register. (Internal register with id = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr4](index.html) module"]
pub struct GPR4_SPEC;
impl crate::RegisterSpec for GPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr4::R](R) reader structure"]
impl crate::Readable for GPR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR4 to value 0"]
impl crate::Resettable for GPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
