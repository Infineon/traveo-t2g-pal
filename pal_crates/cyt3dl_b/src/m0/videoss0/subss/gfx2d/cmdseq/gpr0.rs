#[doc = "Register `GPR0` reader"]
pub struct R(crate::R<GPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER0` reader - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
pub type GENERALPURPOSEREGISTER0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister0(&self) -> GENERALPURPOSEREGISTER0_R {
        GENERALPURPOSEREGISTER0_R::new(self.bits)
    }
}
#[doc = "General purpose Register. (Internal register with id = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr0](index.html) module"]
pub struct GPR0_SPEC;
impl crate::RegisterSpec for GPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr0::R](R) reader structure"]
impl crate::Readable for GPR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR0 to value 0"]
impl crate::Resettable for GPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
