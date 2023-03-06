#[doc = "Register `GPR14` reader"]
pub struct R(crate::R<GPR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER14` reader - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
pub type GENERALPURPOSEREGISTER14_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister14(&self) -> GENERALPURPOSEREGISTER14_R {
        GENERALPURPOSEREGISTER14_R::new(self.bits)
    }
}
#[doc = "General purpose Register. (Internal register with id = 14)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr14](index.html) module"]
pub struct GPR14_SPEC;
impl crate::RegisterSpec for GPR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr14::R](R) reader structure"]
impl crate::Readable for GPR14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR14 to value 0"]
impl crate::Resettable for GPR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
