#[doc = "Register `GPR7` reader"]
pub struct R(crate::R<GPR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER7` reader - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
pub type GENERALPURPOSEREGISTER7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister7(&self) -> GENERALPURPOSEREGISTER7_R {
        GENERALPURPOSEREGISTER7_R::new(self.bits)
    }
}
#[doc = "General purpose Register. (Internal register with id = 7)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr7](index.html) module"]
pub struct GPR7_SPEC;
impl crate::RegisterSpec for GPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr7::R](R) reader structure"]
impl crate::Readable for GPR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR7 to value 0"]
impl crate::Resettable for GPR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
