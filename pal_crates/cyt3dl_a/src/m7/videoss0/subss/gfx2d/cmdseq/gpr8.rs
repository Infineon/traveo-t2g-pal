#[doc = "Register `GPR8` reader"]
pub struct R(crate::R<GPR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER8` reader - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
pub type GENERALPURPOSEREGISTER8_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister8(&self) -> GENERALPURPOSEREGISTER8_R {
        GENERALPURPOSEREGISTER8_R::new(self.bits)
    }
}
#[doc = "General purpose Register. (Internal register with id = 8)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr8](index.html) module"]
pub struct GPR8_SPEC;
impl crate::RegisterSpec for GPR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr8::R](R) reader structure"]
impl crate::Readable for GPR8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR8 to value 0"]
impl crate::Resettable for GPR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
