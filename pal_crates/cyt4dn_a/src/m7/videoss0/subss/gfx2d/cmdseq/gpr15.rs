#[doc = "Register `GPR15` reader"]
pub struct R(crate::R<GPR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER15` reader - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
pub type GENERALPURPOSEREGISTER15_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for syncronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister15(&self) -> GENERALPURPOSEREGISTER15_R {
        GENERALPURPOSEREGISTER15_R::new(self.bits)
    }
}
#[doc = "GeneralPurpose Register. (Internal register with id = 15)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr15](index.html) module"]
pub struct GPR15_SPEC;
impl crate::RegisterSpec for GPR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr15::R](R) reader structure"]
impl crate::Readable for GPR15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR15 to value 0"]
impl crate::Resettable for GPR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
