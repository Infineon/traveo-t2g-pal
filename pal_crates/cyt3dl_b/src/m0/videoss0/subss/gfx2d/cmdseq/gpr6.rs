#[doc = "Register `GPR6` reader"]
pub struct R(crate::R<GPR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERALPURPOSEREGISTER6` reader - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
pub type GENERALPURPOSEREGISTER6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose Register used internally for CmdSeq and for synchronization with SW."]
    #[inline(always)]
    pub fn generalpurposeregister6(&self) -> GENERALPURPOSEREGISTER6_R {
        GENERALPURPOSEREGISTER6_R::new(self.bits)
    }
}
#[doc = "General purpose Register. (Internal register with id = 6)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr6](index.html) module"]
pub struct GPR6_SPEC;
impl crate::RegisterSpec for GPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr6::R](R) reader structure"]
impl crate::Readable for GPR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR6 to value 0"]
impl crate::Resettable for GPR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
