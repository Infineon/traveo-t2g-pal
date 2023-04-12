#[doc = "Register `GPR12` reader"]
pub struct R(crate::R<GPR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER12` reader - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
pub type GENERALPURPOSEREGISTER12_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister12(&self) -> GENERALPURPOSEREGISTER12_R {
        GENERALPURPOSEREGISTER12_R::new(self.bits)
    }
}
#[doc = "General purpose Register. (Internal register with id = 12)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr12](index.html) module"]
pub struct GPR12_SPEC;
impl crate::RegisterSpec for GPR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr12::R](R) reader structure"]
impl crate::Readable for GPR12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR12 to value 0"]
impl crate::Resettable for GPR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
