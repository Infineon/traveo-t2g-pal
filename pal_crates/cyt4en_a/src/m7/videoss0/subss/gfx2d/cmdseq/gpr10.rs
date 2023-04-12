#[doc = "Register `GPR10` reader"]
pub struct R(crate::R<GPR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER10` reader - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
pub type GENERALPURPOSEREGISTER10_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister10(&self) -> GENERALPURPOSEREGISTER10_R {
        GENERALPURPOSEREGISTER10_R::new(self.bits)
    }
}
#[doc = "General purpose Register. (Internal register with id = 10)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr10](index.html) module"]
pub struct GPR10_SPEC;
impl crate::RegisterSpec for GPR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr10::R](R) reader structure"]
impl crate::Readable for GPR10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR10 to value 0"]
impl crate::Resettable for GPR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
